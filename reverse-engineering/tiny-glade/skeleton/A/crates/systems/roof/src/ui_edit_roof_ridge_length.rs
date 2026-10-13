// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: ui_edit_roof_ridge_length.rs.assembly.json.gz

// FUNCTION system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$0
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3199:7680 -> host 3199:4300 / rva_198ac80_len_1398; exact_symbolizer_function_frame
//   Function-frame RVA span [0x198b0b9, 0x198b149), 37 instructions
//
// FUNCTION system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$2
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3199:10992 -> host 3199:4300 / rva_198ac80_len_1398; exact_symbolizer_function_frame
// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.
//   Function-frame RVA span [0x198b63b, 0x198b6dc), 31 instructions
//   Function-frame RVA span [0x198b750, 0x198b7f9), 32 instructions
// Inline site 3199:11316 -> host 3199:4300 / rva_198ac80_len_1398; exact_symbolizer_function_frame
// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.
//   Function-frame RVA span [0x198b63b, 0x198b6dc), 31 instructions
//   Function-frame RVA span [0x198b750, 0x198b7f9), 32 instructions
//
// FUNCTION system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length
// Candidate interfaces: bd45d76fb685663246c3f782
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3199:4300 -> rva_198ac80_len_1398
//
// FUNCTION system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$1
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 3199:13352 -> rva_198c020_len_27f
//
// FUNCTION system_roof::ui_edit_roof_ridge_length::create_ui_widget
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3199:14616 -> rva_198c2a0_len_c5d
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_198ac80_len_1398: RVA 0x198ac80, 5016 PDB bytes; 1064 instructions
// Evidence: _assembly/roof-v1/A/pack-003/rva_198ac80_len_1398.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3199:4300: system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length
// Frames: F1
// 0x14198ac80  55                             pushq	%rbp
// 0x14198ac81  4157                           pushq	%r15
// 0x14198ac83  4156                           pushq	%r14
// 0x14198ac85  4155                           pushq	%r13
// 0x14198ac87  4154                           pushq	%r12
// 0x14198ac89  56                             pushq	%rsi
// 0x14198ac8a  57                             pushq	%rdi
// 0x14198ac8b  53                             pushq	%rbx
// 0x14198ac8c  4881ec78020000                 subq	$0x278, %rsp            # imm = 0x278
// 0x14198ac93  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14198ac9b  66440f29b5e0010000             movapd	%xmm14, 0x1e0(%rbp)
// 0x14198aca4  440f29add0010000               movaps	%xmm13, 0x1d0(%rbp)
// 0x14198acac  440f29a5c0010000               movaps	%xmm12, 0x1c0(%rbp)
// 0x14198acb4  440f299db0010000               movaps	%xmm11, 0x1b0(%rbp)
// 0x14198acbc  440f2995a0010000               movaps	%xmm10, 0x1a0(%rbp)
// 0x14198acc4  440f298d90010000               movaps	%xmm9, 0x190(%rbp)
// 0x14198accc  440f298580010000               movaps	%xmm8, 0x180(%rbp)
// 0x14198acd4  0f29bd70010000                 movaps	%xmm7, 0x170(%rbp)
// 0x14198acdb  0f29b560010000                 movaps	%xmm6, 0x160(%rbp)
// 0x14198ace2  48c78558010000feffffff         movq	$-0x2, 0x158(%rbp)
// 0x14198aced  4c89ce                         movq	%r9, %rsi
// 0x14198acf0  4d89c7                         movq	%r8, %r15
// 0x14198acf3  4889d7                         movq	%rdx, %rdi
// 0x14198acf6  4989ce                         movq	%rcx, %r14
// Frames: F2
// 0x14198acf9  e83202dcfe                     callq	0x14074af30 <.text+0x749f30>
// 0x14198acfe  84c0                           testb	%al, %al
// 0x14198ad00  745d                           je	0x14198ad5f <.text+0x1989d5f>
// Frames: F3 -> F2
// 0x14198ad02  41bc41000000                   movl	$0x41, %r12d
// 0x14198ad08  b941000000                     movl	$0x41, %ecx
// 0x14198ad0d  e85efaffff                     callq	0x14198a770 <.text+0x1989770>
// 0x14198ad12  488d1de787d600                 leaq	0xd687e7(%rip), %rbx    # 0x1426f3500
// 0x14198ad19  4883f801                       cmpq	$0x1, %rax
// 0x14198ad1d  0f85af000000                   jne	0x14198add2 <.text+0x1989dd2>
// Frames: F4 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x14198ad23  4989d1                         movq	%rdx, %r9
// 0x14198ad26  4885d2                         testq	%rdx, %rdx
// 0x14198ad29  7444                           je	0x14198ad6f <.text+0x1989d6f>
// Frames: F9 -> F10 -> F11 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x14198ad2b  4983f941                       cmpq	$0x41, %r9
// Frames: F11 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x14198ad2f  733c                           jae	0x14198ad6d <.text+0x1989d6d>
// Frames: F9 -> F10 -> F11 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x14198ad31  488d05c887d600                 leaq	0xd687c8(%rip), %rax    # 0x1426f3500
// Frames: F12 -> F13 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x14198ad38  41803c01bf                     cmpb	$-0x41, (%r9,%rax)
// Frames: F5 -> F6 -> F7 -> F8 -> F2
// 0x14198ad3d  7f30                           jg	0x14198ad6f <.text+0x1989d6f>
// Frames: F14 -> F7 -> F8 -> F2
// 0x14198ad3f  488d054289d600                 leaq	0xd68942(%rip), %rax    # 0x1426f3688
// 0x14198ad46  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14198ad4b  488d0dae87d600                 leaq	0xd687ae(%rip), %rcx    # 0x1426f3500
// 0x14198ad52  ba41000000                     movl	$0x41, %edx
// 0x14198ad57  4531c0                         xorl	%r8d, %r8d
// 0x14198ad5a  e8d1437200                     callq	0x1420af130 <.text+0x20ae130>
// 0x14198ad5f  31c0                           xorl	%eax, %eax
// 0x14198ad61  488985e8000000                 movq	%rax, 0xe8(%rbp)
// 0x14198ad68  e92c020000                     jmp	0x14198af99 <.text+0x1989f99>
// Frames: F5 -> F6 -> F7 -> F8 -> F2
// 0x14198ad6d  75d0                           jne	0x14198ad3f <.text+0x1989d3f>
// Frames: F8 -> F2
// 0x14198ad6f  4c89c9                         movq	%r9, %rcx
// 0x14198ad72  e8f9f9ffff                     callq	0x14198a770 <.text+0x1989770>
// 0x14198ad77  4883f801                       cmpq	$0x1, %rax
// 0x14198ad7b  7555                           jne	0x14198add2 <.text+0x1989dd2>
// Frames: F4 -> F15 -> F16 -> F7 -> F17 -> F2
// 0x14198ad7d  4989d0                         movq	%rdx, %r8
// 0x14198ad80  4983c002                       addq	$0x2, %r8
// 0x14198ad84  7439                           je	0x14198adbf <.text+0x1989dbf>
// Frames: F9 -> F10 -> F11 -> F15 -> F16 -> F7 -> F17 -> F2
// 0x14198ad86  4983f841                       cmpq	$0x41, %r8
// Frames: F11 -> F15 -> F16 -> F7 -> F17 -> F2
// 0x14198ad8a  7331                           jae	0x14198adbd <.text+0x1989dbd>
// Frames: F9 -> F10 -> F11 -> F15 -> F16 -> F7 -> F17 -> F2
// 0x14198ad8c  488d056d87d600                 leaq	0xd6876d(%rip), %rax    # 0x1426f3500
// Frames: F12 -> F13 -> F15 -> F16 -> F7 -> F17 -> F2
// 0x14198ad93  41803c00bf                     cmpb	$-0x41, (%r8,%rax)
// Frames: F15 -> F16 -> F7 -> F17 -> F2
// 0x14198ad98  7f25                           jg	0x14198adbf <.text+0x1989dbf>
// Frames: F18 -> F7 -> F17 -> F2
// 0x14198ad9a  488d05ff88d600                 leaq	0xd688ff(%rip), %rax    # 0x1426f36a0
// 0x14198ada1  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14198ada6  488d0d5387d600                 leaq	0xd68753(%rip), %rcx    # 0x1426f3500
// 0x14198adad  ba41000000                     movl	$0x41, %edx
// 0x14198adb2  41b941000000                   movl	$0x41, %r9d
// 0x14198adb8  e873437200                     callq	0x1420af130 <.text+0x20ae130>
// Frames: F15 -> F16 -> F7 -> F17 -> F2
// 0x14198adbd  75db                           jne	0x14198ad9a <.text+0x1989d9a>
// Frames: F19 -> F20 -> F21 -> F22 -> F16 -> F7 -> F17 -> F2
// 0x14198adbf  488d1d3a87d600                 leaq	0xd6873a(%rip), %rbx    # 0x1426f3500
// 0x14198adc6  4c01c3                         addq	%r8, %rbx
// 0x14198adc9  41bc3f000000                   movl	$0x3f, %r12d
// Frames: F23 -> F21 -> F22 -> F16 -> F7 -> F17 -> F2
// 0x14198adcf  4929d4                         subq	%rdx, %r12
// 0x14198add2  488d05ab89d600                 leaq	0xd689ab(%rip), %rax    # 0x1426f3784
// 0x14198add9  4c8d2d7089d600                 leaq	0xd68970(%rip), %r13    # 0x1426f3750
// 0x14198ade0  4c89b548010000                 movq	%r14, 0x148(%rbp)
// 0x14198ade7  660f1f840000000000             nopw	(%rax,%rax)
// Frames: F24 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198adf0  0fb648ff                       movzbl	-0x1(%rax), %ecx
// 0x14198adf4  84c9                           testb	%cl, %cl
// 0x14198adf6  7808                           js	0x14198ae00 <.text+0x1989e00>
// 0x14198adf8  48ffc8                         decq	%rax
// 0x14198adfb  0fb6c9                         movzbl	%cl, %ecx
// 0x14198adfe  eb5c                           jmp	0x14198ae5c <.text+0x1989e5c>
// Frames: F32 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae00  0fb650fe                       movzbl	-0x2(%rax), %edx
// Frames: F33 -> F34 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae04  80fac0                         cmpb	$-0x40, %dl
// Frames: F34 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae07  7d25                           jge	0x14198ae2e <.text+0x1989e2e>
// Frames: F35 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae09  440fb640fd                     movzbl	-0x3(%rax), %r8d
// Frames: F33 -> F36 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae0e  4180f8bf                       cmpb	$-0x41, %r8b
// Frames: F36 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae12  7f23                           jg	0x14198ae37 <.text+0x1989e37>
// Frames: F37 -> F38 -> F39 -> F40 -> F41 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae14  440fb648fc                     movzbl	-0x4(%rax), %r9d
// 0x14198ae19  4883c0fc                       addq	$-0x4, %rax
// Frames: F42 -> F43 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae1d  4183e107                       andl	$0x7, %r9d
// Frames: F44 -> F45 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae21  41c1e106                       shll	$0x6, %r9d
// 0x14198ae25  4183e03f                       andl	$0x3f, %r8d
// 0x14198ae29  4509c8                         orl	%r9d, %r8d
// 0x14198ae2c  eb11                           jmp	0x14198ae3f <.text+0x1989e3f>
// 0x14198ae2e  4883c0fe                       addq	$-0x2, %rax
// 0x14198ae32  83e21f                         andl	$0x1f, %edx
// 0x14198ae35  eb12                           jmp	0x14198ae49 <.text+0x1989e49>
// 0x14198ae37  4883c0fd                       addq	$-0x3, %rax
// 0x14198ae3b  4183e00f                       andl	$0xf, %r8d
// Frames: F44 -> F46 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae3f  41c1e006                       shll	$0x6, %r8d
// 0x14198ae43  83e23f                         andl	$0x3f, %edx
// 0x14198ae46  4409c2                         orl	%r8d, %edx
// Frames: F44 -> F47 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae49  c1e206                         shll	$0x6, %edx
// 0x14198ae4c  80e13f                         andb	$0x3f, %cl
// 0x14198ae4f  0fb6c9                         movzbl	%cl, %ecx
// 0x14198ae52  09d1                           orl	%edx, %ecx
// Frames: F48 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae54  81f900001100                   cmpl	$0x110000, %ecx         # imm = 0x110000
// 0x14198ae5a  740f                           je	0x14198ae6b <.text+0x1989e6b>
// Frames: F49 -> F50 -> F51 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae5c  83f95c                         cmpl	$0x5c, %ecx
// 0x14198ae5f  7462                           je	0x14198aec3 <.text+0x1989ec3>
// 0x14198ae61  83f92f                         cmpl	$0x2f, %ecx
// 0x14198ae64  745d                           je	0x14198aec3 <.text+0x1989ec3>
// Frames: F52 -> F53 -> F54 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae66  4c39e8                         cmpq	%r13, %rax
// Frames: F55 -> F54 -> F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F2
// 0x14198ae69  7585                           jne	0x14198adf0 <.text+0x1989df0>
// Frames: F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x14198ae6b  4d89fe                         movq	%r15, %r14
// 0x14198ae6e  4989ff                         movq	%rdi, %r15
// 0x14198ae71  bf34000000                     movl	$0x34, %edi
// 0x14198ae76  e8f511dcfe                     callq	0x14074c070 <.text+0x74b070>
// Frames: F61 -> F62 -> F63 -> F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x14198ae7b  48833800                       cmpq	$0x0, (%rax)
// 0x14198ae7f  0f85a2000000                   jne	0x14198af27 <.text+0x1989f27>
// Frames: F64 -> F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x14198ae85  4889c1                         movq	%rax, %rcx
// 0x14198ae88  31d2                           xorl	%edx, %edx
// 0x14198ae8a  e81158f4fe                     callq	0x1408d06a0 <.text+0x8cf6a0>
// Frames: F65 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x14198ae8f  4885c0                         testq	%rax, %rax
// 0x14198ae92  0f8593000000                   jne	0x14198af2b <.text+0x1989f2b>
// Frames: F66 -> F58 -> F59 -> F60 -> F2
// 0x14198ae98  488d054187d600                 leaq	0xd68741(%rip), %rax    # 0x1426f35e0
// 0x14198ae9f  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14198aea4  488d0d9986d600                 leaq	0xd68699(%rip), %rcx    # 0x1426f3544
// 0x14198aeab  4c8d0dde85d600                 leaq	0xd685de(%rip), %r9     # 0x1426f3490
// 0x14198aeb2  4c8d8500010000                 leaq	0x100(%rbp), %r8
// 0x14198aeb9  ba46000000                     movl	$0x46, %edx
// 0x14198aebe  e85d407200                     callq	0x1420aef20 <.text+0x20adf20>
// Frames: F67 -> F2
// 0x14198aec3  4c8d2d8688d600                 leaq	0xd68886(%rip), %r13    # 0x1426f3750
// 0x14198aeca  4c29e8                         subq	%r13, %rax
// 0x14198aecd  4989c0                         movq	%rax, %r8
// 0x14198aed0  49ffc0                         incq	%r8
// Frames: F4 -> F15 -> F16 -> F7 -> F67 -> F2
// 0x14198aed3  7432                           je	0x14198af07 <.text+0x1989f07>
// Frames: F9 -> F10 -> F11 -> F15 -> F16 -> F7 -> F67 -> F2
// 0x14198aed5  4983f834                       cmpq	$0x34, %r8
// Frames: F11 -> F15 -> F16 -> F7 -> F67 -> F2
// 0x14198aed9  732a                           jae	0x14198af05 <.text+0x1989f05>
// Frames: F12 -> F13 -> F15 -> F16 -> F7 -> F67 -> F2
// 0x14198aedb  43803c28bf                     cmpb	$-0x41, (%r8,%r13)
// Frames: F15 -> F16 -> F7 -> F67 -> F2
// 0x14198aee0  7f25                           jg	0x14198af07 <.text+0x1989f07>
// Frames: F18 -> F7 -> F67 -> F2
// 0x14198aee2  488d057f87d600                 leaq	0xd6877f(%rip), %rax    # 0x1426f3668
// 0x14198aee9  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14198aeee  488d0d5b88d600                 leaq	0xd6885b(%rip), %rcx    # 0x1426f3750
// 0x14198aef5  ba34000000                     movl	$0x34, %edx
// 0x14198aefa  41b934000000                   movl	$0x34, %r9d
// 0x14198af00  e82b427200                     callq	0x1420af130 <.text+0x20ae130>
// Frames: F15 -> F16 -> F7 -> F67 -> F2
// 0x14198af05  75db                           jne	0x14198aee2 <.text+0x1989ee2>
// Frames: F19 -> F20 -> F21 -> F22 -> F16 -> F7 -> F67 -> F2
// 0x14198af07  4d89fe                         movq	%r15, %r14
// 0x14198af0a  4989ff                         movq	%rdi, %r15
// 0x14198af0d  4d01c5                         addq	%r8, %r13
// 0x14198af10  bf33000000                     movl	$0x33, %edi
// Frames: F23 -> F21 -> F22 -> F16 -> F7 -> F67 -> F2
// 0x14198af15  4829c7                         subq	%rax, %rdi
// Frames: F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x14198af18  e85311dcfe                     callq	0x14074c070 <.text+0x74b070>
// Frames: F61 -> F62 -> F63 -> F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x14198af1d  48833800                       cmpq	$0x0, (%rax)
// 0x14198af21  0f845effffff                   je	0x14198ae85 <.text+0x1989e85>
// Frames: F68 -> F62 -> F63 -> F56 -> F57 -> F58 -> F59 -> F60 -> F2
// 0x14198af27  4883c008                       addq	$0x8, %rax
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x14198af2b  48833800                       cmpq	$0x0, (%rax)
// 0x14198af2f  0f854e0c0000                   jne	0x14198bb83 <.text+0x198ab83>
// Frames: F74 -> F75 -> F76 -> F77 -> F78 -> F70 -> F71 -> F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x14198af35  48c700ffffffff                 movq	$-0x1, (%rax)
// 0x14198af3c  48898540010000                 movq	%rax, 0x140(%rbp)
// Frames: F79 -> F80 -> F71 -> F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x14198af43  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F81 -> F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x14198af47  488d059284d600                 leaq	0xd68492(%rip), %rax    # 0x1426f33e0
// 0x14198af4e  4889442428                     movq	%rax, 0x28(%rsp)
// 0x14198af53  48897c2420                     movq	%rdi, 0x20(%rsp)
// 0x14198af58  48c744243000000000             movq	$0x0, 0x30(%rsp)
// 0x14198af61  4889da                         movq	%rbx, %rdx
// 0x14198af64  4d89e0                         movq	%r12, %r8
// 0x14198af67  4d89e9                         movq	%r13, %r9
// 0x14198af6a  e8c102dcfe                     callq	0x14074b230 <.text+0x74a230>
// 0x14198af6f  488985c8000000                 movq	%rax, 0xc8(%rbp)
// Frames: F74 -> F75 -> F76 -> F77 -> F82 -> F83 -> F83 -> F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x14198af76  488b8540010000                 movq	0x140(%rbp), %rax
// 0x14198af7d  48ff00                         incq	(%rax)
// 0x14198af80  b801000000                     movl	$0x1, %eax
// 0x14198af85  488985e8000000                 movq	%rax, 0xe8(%rbp)
// 0x14198af8c  4c89ff                         movq	%r15, %rdi
// 0x14198af8f  4d89f7                         movq	%r14, %r15
// 0x14198af92  4c8bb548010000                 movq	0x148(%rbp), %r14
// Frames: F84 -> F85 -> F86 -> F87 -> F88 -> F89 -> F90 -> F91 -> F92
// 0x14198af99  0fb60521fb2e01                 movzbl	0x12efb21(%rip), %eax   # 0x142c7aac1
// Frames: F93 -> F86 -> F87 -> F88 -> F89 -> F90 -> F91 -> F92
// 0x14198afa0  b930000000                     movl	$0x30, %ecx
// 0x14198afa5  ba08000000                     movl	$0x8, %edx
// 0x14198afaa  e8d1f7a4fe                     callq	0x1403da780 <.text+0x3d9780>
// Frames: F94 -> F89 -> F90 -> F91 -> F92
// 0x14198afaf  4885c0                         testq	%rax, %rax
// 0x14198afb2  0f84ba0b0000                   je	0x14198bb72 <.text+0x198ab72>
// Frames: F90 -> F91 -> F92
// 0x14198afb8  48c785d000000002000000         movq	$0x2, 0xd0(%rbp)
// 0x14198afc3  488985d8000000                 movq	%rax, 0xd8(%rbp)
// 0x14198afca  48c785e000000000000000         movq	$0x0, 0xe0(%rbp)
// 0x14198afd5  0f57c0                         xorps	%xmm0, %xmm0
// 0x14198afd8  0f294570                       movaps	%xmm0, 0x70(%rbp)
// 0x14198afdc  48c7858000000000000000         movq	$0x0, 0x80(%rbp)
// Frames: F95
// 0x14198afe7  48c785a000000000000000         movq	$0x0, 0xa0(%rbp)
// Frames: F96
// 0x14198aff2  4d8b26                         movq	(%r14), %r12
// 0x14198aff5  498b0424                       movq	(%r12), %rax
// 0x14198aff9  4885c0                         testq	%rax, %rax
// 0x14198affc  7412                           je	0x14198b010 <.text+0x198a010>
// Frames: F97
// 0x14198affe  48ffc8                         decq	%rax
// 0x14198b001  4883f801                       cmpq	$0x1, %rax
// 0x14198b005  0f86ae010000                   jbe	0x14198b1b9 <.text+0x198a1b9>
// 0x14198b00b  e998020000                     jmp	0x14198b2a8 <.text+0x198a2a8>
// Frames: F98
// 0x14198b010  4c8b37                         movq	(%rdi), %r14
// 0x14198b013  488d8d00010000                 leaq	0x100(%rbp), %rcx
// 0x14198b01a  4c89f2                         movq	%r14, %rdx
// 0x14198b01d  e8def2cdfe                     callq	0x14066a300 <.text+0x669300>
// Frames: F99 -> F100 -> F98
// 0x14198b022  4889bda8000000                 movq	%rdi, 0xa8(%rbp)
// 0x14198b029  4c89bd40010000                 movq	%r15, 0x140(%rbp)
// 0x14198b030  488b8500010000                 movq	0x100(%rbp), %rax
// 0x14198b037  48898538010000                 movq	%rax, 0x138(%rbp)
// 0x14198b03e  488b8508010000                 movq	0x108(%rbp), %rax
// 0x14198b045  48898548010000                 movq	%rax, 0x148(%rbp)
// 0x14198b04c  4c8bad10010000                 movq	0x110(%rbp), %r13
// Frames: F52 -> F101 -> F98
// 0x14198b053  4d85ed                         testq	%r13, %r13
// Frames: F101 -> F98
// 0x14198b056  0f841c010000                   je	0x14198b178 <.text+0x198a178>
// 0x14198b05c  488b8540010000                 movq	0x140(%rbp), %rax
// 0x14198b063  4c8b38                         movq	(%rax), %r15
// 0x14198b066  488b7808                       movq	0x8(%rax), %rdi
// 0x14198b06a  8b461c                         movl	0x1c(%rsi), %eax
// 0x14198b06d  8985b0000000                   movl	%eax, 0xb0(%rbp)
// 0x14198b073  488b06                         movq	(%rsi), %rax
// 0x14198b076  488985b8000000                 movq	%rax, 0xb8(%rbp)
// 0x14198b07d  488b4610                       movq	0x10(%rsi), %rax
// 0x14198b081  488985c0000000                 movq	%rax, 0xc0(%rbp)
// 0x14198b088  49c1e503                       shlq	$0x3, %r13
// 0x14198b08c  31db                           xorl	%ebx, %ebx
// 0x14198b08e  eb0d                           jmp	0x14198b09d <.text+0x198a09d>
// Frames: F52 -> F101 -> F98
// 0x14198b090  4883c308                       addq	$0x8, %rbx
// 0x14198b094  4939dd                         cmpq	%rbx, %r13
// Frames: F101 -> F98
// 0x14198b097  0f84db000000                   je	0x14198b178 <.text+0x198a178>
// Frames: F102 -> F103 -> F104 -> F98
// 0x14198b09d  488b8548010000                 movq	0x148(%rbp), %rax
// 0x14198b0a4  488b1418                       movq	(%rax,%rbx), %rdx
// Frames: F105
// 0x14198b0a8  4c89f1                         movq	%r14, %rcx
// 0x14198b0ab  e850f4cdfe                     callq	0x14066a500 <.text+0x669500>
// Frames: F106 -> F105
// 0x14198b0b0  4889c1                         movq	%rax, %rcx
// 0x14198b0b3  48c1e920                       shrq	$0x20, %rcx
// 0x14198b0b7  74d7                           je	0x14198b090 <.text+0x198a090>
// Frames: F107 -> F108 -> F109 -> F110 -> F111 -> F105
// 0x14198b0b9  89c0                           movl	%eax, %eax
// Frames: F9 -> F10 -> F107 -> F108 -> F109 -> F110 -> F111 -> F105
// 0x14198b0bb  483b4710                       cmpq	0x10(%rdi), %rax
// Frames: F107 -> F108 -> F109 -> F110 -> F111 -> F105
// 0x14198b0bf  73cf                           jae	0x14198b090 <.text+0x198a090>
// Frames: F112 -> F113 -> F114 -> F108 -> F109 -> F110 -> F111 -> F105
// 0x14198b0c1  488d0480                       leaq	(%rax,%rax,4), %rax
// 0x14198b0c5  48c1e002                       shlq	$0x2, %rax
// 0x14198b0c9  48034708                       addq	0x8(%rdi), %rax
// 0x14198b0cd  3908                           cmpl	%ecx, (%rax)
// Frames: F114 -> F108 -> F109 -> F110 -> F111 -> F105
// 0x14198b0cf  75bf                           jne	0x14198b090 <.text+0x198a090>
// Frames: F115 -> F116 -> F108 -> F109 -> F110 -> F111 -> F105
// 0x14198b0d1  8b4804                         movl	0x4(%rax), %ecx
// 0x14198b0d4  baffffffff                     movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
// 0x14198b0d9  4839d1                         cmpq	%rdx, %rcx
// Frames: F116 -> F108 -> F109 -> F110 -> F111 -> F105
// 0x14198b0dc  74b2                           je	0x14198b090 <.text+0x198a090>
// Frames: F117 -> F118 -> F109 -> F110 -> F111 -> F105
// 0x14198b0de  49394f38                       cmpq	%rcx, 0x38(%r15)
// Frames: F119 -> F120 -> F118 -> F109 -> F110 -> F111 -> F105
// 0x14198b0e2  76ac                           jbe	0x14198b090 <.text+0x198a090>
// Frames: F118 -> F109 -> F110 -> F111 -> F105
// 0x14198b0e4  498b5728                       movq	0x28(%r15), %rdx
// Frames: F121 -> F122 -> F123 -> F124 -> F119 -> F120 -> F118 -> F109 -> F110 -> F111 -> F105
// 0x14198b0e8  4189c8                         movl	%ecx, %r8d
// 0x14198b0eb  41c1e806                       shrl	$0x6, %r8d
// Frames: F125 -> F123 -> F124 -> F119 -> F120 -> F118 -> F109 -> F110 -> F111 -> F105
// 0x14198b0ef  4a8b14c2                       movq	(%rdx,%r8,8), %rdx
// Frames: F123 -> F124 -> F119 -> F120 -> F118 -> F109 -> F110 -> F111 -> F105
// 0x14198b0f3  480fa3ca                       btq	%rcx, %rdx
// Frames: F118 -> F109 -> F110 -> F111 -> F105
// 0x14198b0f7  7397                           jae	0x14198b090 <.text+0x198a090>
// Frames: F126 -> F108 -> F109 -> F110 -> F111 -> F105
// 0x14198b0f9  488b4808                       movq	0x8(%rax), %rcx
// 0x14198b0fd  8b4010                         movl	0x10(%rax), %eax
// 0x14198b100  48c1e920                       shrq	$0x20, %rcx
// Frames: F127 -> F128 -> F109 -> F110 -> F111 -> F105
// 0x14198b104  488b9728010000                 movq	0x128(%rdi), %rdx
// Frames: F129 -> F109 -> F110 -> F111 -> F105
// 0x14198b10b  4d8b87d8000000                 movq	0xd8(%r15), %r8
// Frames: F130 -> F131 -> F132 -> F129 -> F109 -> F110 -> F111 -> F105
// 0x14198b112  488d0cc9                       leaq	(%rcx,%rcx,8), %rcx
// Frames: F133 -> F130 -> F131 -> F132 -> F129 -> F109 -> F110 -> F111 -> F105
// 0x14198b116  4c8b4cca18                     movq	0x18(%rdx,%rcx,8), %r9
// 0x14198b11b  488b4cca38                     movq	0x38(%rdx,%rcx,8), %rcx
// Frames: F61 -> F134 -> F135 -> F136 -> F133 -> F130 -> F131 -> F132 -> F129 -> F109 -> F110 -> F111 -> F105
// 0x14198b120  4a8b0cc1                       movq	(%rcx,%r8,8), %rcx
// Frames: F137 -> F138 -> F135 -> F133 -> F130 -> F131 -> F132 -> F129 -> F109 -> F110 -> F111 -> F105
// 0x14198b124  48f7d1                         notq	%rcx
// Frames: F139 -> F140 -> F141 -> F132 -> F129 -> F109 -> F110 -> F111 -> F105
// 0x14198b127  488d1449                       leaq	(%rcx,%rcx,2), %rdx
// 0x14198b12b  48c1e205                       shlq	$0x5, %rdx
// Frames: F121 -> F142 -> F143 -> F144 -> F109 -> F110 -> F111 -> F105
// 0x14198b12f  488d0c80                       leaq	(%rax,%rax,4), %rcx
// 0x14198b133  48c1e104                       shlq	$0x4, %rcx
// 0x14198b137  49034c1110                     addq	0x10(%r9,%rdx), %rcx
// 0x14198b13c  8b85b0000000                   movl	0xb0(%rbp), %eax
// 0x14198b142  488b95c0000000                 movq	0xc0(%rbp), %rdx
// Frames: F145 -> F146 -> F147
// 0x14198b149  8902                           movl	%eax, (%rdx)
// Frames: F148
// 0x14198b14b  488d85d0000000                 leaq	0xd0(%rbp), %rax
// 0x14198b152  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14198b157  c644242800                     movb	$0x0, 0x28(%rsp)
// 0x14198b15c  488d5570                       leaq	0x70(%rbp), %rdx
// 0x14198b160  4c8b85b8000000                 movq	0xb8(%rbp), %r8
// 0x14198b167  4c8d8da0000000                 leaq	0xa0(%rbp), %r9
// 0x14198b16e  e82d110000                     callq	0x14198c2a0 <.text+0x198b2a0>
// 0x14198b173  e918ffffff                     jmp	0x14198b090 <.text+0x198a090>
// Frames: F149 -> F150 -> F83 -> F151 -> F83 -> F152 -> F83 -> F98
// 0x14198b178  488b9538010000                 movq	0x138(%rbp), %rdx
// 0x14198b17f  4885d2                         testq	%rdx, %rdx
// 0x14198b182  7416                           je	0x14198b19a <.text+0x198a19a>
// Frames: F153 -> F154 -> F150 -> F83 -> F151 -> F83 -> F152 -> F83 -> F98
// 0x14198b184  48c1e203                       shlq	$0x3, %rdx
// Frames: F155 -> F156 -> F157 -> F83 -> F151 -> F83 -> F152 -> F83 -> F98
// 0x14198b188  41b808000000                   movl	$0x8, %r8d
// 0x14198b18e  488b8d48010000                 movq	0x148(%rbp), %rcx
// 0x14198b195  e8f6f5a4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F97
// 0x14198b19a  498b0424                       movq	(%r12), %rax
// 0x14198b19e  4c8bbd40010000                 movq	0x140(%rbp), %r15
// 0x14198b1a5  488bbda8000000                 movq	0xa8(%rbp), %rdi
// 0x14198b1ac  48ffc8                         decq	%rax
// 0x14198b1af  4883f801                       cmpq	$0x1, %rax
// 0x14198b1b3  0f87ef000000                   ja	0x14198b2a8 <.text+0x198a2a8>
// 0x14198b1b9  498b542408                     movq	0x8(%r12), %rdx
// Frames: F158
// 0x14198b1be  488b0f                         movq	(%rdi), %rcx
// 0x14198b1c1  e83af3cdfe                     callq	0x14066a500 <.text+0x669500>
// 0x14198b1c6  4989c1                         movq	%rax, %r9
// 0x14198b1c9  49c1e920                       shrq	$0x20, %r9
// 0x14198b1cd  0f84e2080000                   je	0x14198bab5 <.text+0x198aab5>
// Frames: F159 -> F160
// 0x14198b1d3  498b4f08                       movq	0x8(%r15), %rcx
// Frames: F107 -> F108 -> F109 -> F160
// 0x14198b1d7  4189c0                         movl	%eax, %r8d
// 0x14198b1da  ba01000000                     movl	$0x1, %edx
// Frames: F9 -> F10 -> F107 -> F108 -> F109 -> F160
// 0x14198b1df  4c3b4110                       cmpq	0x10(%rcx), %r8
// Frames: F107 -> F108 -> F109 -> F160
// 0x14198b1e3  0f83a8090000                   jae	0x14198bb91 <.text+0x198ab91>
// Frames: F112 -> F113 -> F114 -> F108 -> F109 -> F160
// 0x14198b1e9  4f8d0480                       leaq	(%r8,%r8,4), %r8
// 0x14198b1ed  49c1e002                       shlq	$0x2, %r8
// 0x14198b1f1  4c034108                       addq	0x8(%rcx), %r8
// 0x14198b1f5  453908                         cmpl	%r9d, (%r8)
// Frames: F114 -> F108 -> F109 -> F160
// 0x14198b1f8  0f8593090000                   jne	0x14198bb91 <.text+0x198ab91>
// Frames: F115 -> F116 -> F108 -> F109 -> F160
// 0x14198b1fe  458b4804                       movl	0x4(%r8), %r9d
// 0x14198b202  41baffffffff                   movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
// 0x14198b208  4d39d1                         cmpq	%r10, %r9
// Frames: F116 -> F108 -> F109 -> F160
// 0x14198b20b  0f8480090000                   je	0x14198bb91 <.text+0x198ab91>
// Frames: F117 -> F118 -> F109 -> F160
// 0x14198b211  498b17                         movq	(%r15), %rdx
// 0x14198b214  4c394a38                       cmpq	%r9, 0x38(%rdx)
// Frames: F119 -> F120 -> F118 -> F109 -> F160
// 0x14198b218  0f8671090000                   jbe	0x14198bb8f <.text+0x198ab8f>
// Frames: F118 -> F109 -> F160
// 0x14198b21e  4c8b5228                       movq	0x28(%rdx), %r10
// Frames: F121 -> F122 -> F123 -> F124 -> F119 -> F120 -> F118 -> F109 -> F160
// 0x14198b222  4589cb                         movl	%r9d, %r11d
// 0x14198b225  41c1eb06                       shrl	$0x6, %r11d
// Frames: F125 -> F123 -> F124 -> F119 -> F120 -> F118 -> F109 -> F160
// 0x14198b229  4f8b14da                       movq	(%r10,%r11,8), %r10
// Frames: F123 -> F124 -> F119 -> F120 -> F118 -> F109 -> F160
// 0x14198b22d  4d0fa3ca                       btq	%r9, %r10
// Frames: F118 -> F109 -> F160
// 0x14198b231  0f8358090000                   jae	0x14198bb8f <.text+0x198ab8f>
// Frames: F126 -> F108 -> F109 -> F160
// 0x14198b237  498b4008                       movq	0x8(%r8), %rax
// 0x14198b23b  458b4010                       movl	0x10(%r8), %r8d
// 0x14198b23f  48c1e820                       shrq	$0x20, %rax
// Frames: F127 -> F128 -> F109 -> F160
// 0x14198b243  488b8928010000                 movq	0x128(%rcx), %rcx
// Frames: F129 -> F109 -> F160
// 0x14198b24a  488b92d8000000                 movq	0xd8(%rdx), %rdx
// Frames: F130 -> F131 -> F132 -> F129 -> F109 -> F160
// 0x14198b251  488d04c0                       leaq	(%rax,%rax,8), %rax
// Frames: F133 -> F130 -> F131 -> F132 -> F129 -> F109 -> F160
// 0x14198b255  4c8b4cc118                     movq	0x18(%rcx,%rax,8), %r9
// 0x14198b25a  488b44c138                     movq	0x38(%rcx,%rax,8), %rax
// Frames: F61 -> F134 -> F135 -> F136 -> F133 -> F130 -> F131 -> F132 -> F129 -> F109 -> F160
// 0x14198b25f  488b04d0                       movq	(%rax,%rdx,8), %rax
// Frames: F137 -> F138 -> F135 -> F133 -> F130 -> F131 -> F132 -> F129 -> F109 -> F160
// 0x14198b263  48f7d0                         notq	%rax
// Frames: F139 -> F140 -> F141 -> F132 -> F129 -> F109 -> F160
// 0x14198b266  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x14198b26a  48c1e005                       shlq	$0x5, %rax
// Frames: F121 -> F142 -> F143 -> F144 -> F109 -> F160
// 0x14198b26e  4b8d0c80                       leaq	(%r8,%r8,4), %rcx
// 0x14198b272  48c1e104                       shlq	$0x4, %rcx
// 0x14198b276  49034c0110                     addq	0x10(%r9,%rax), %rcx
// Frames: F145 -> F146 -> F161
// 0x14198b27b  8b461c                         movl	0x1c(%rsi), %eax
// 0x14198b27e  488b5610                       movq	0x10(%rsi), %rdx
// 0x14198b282  8902                           movl	%eax, (%rdx)
// Frames: F162 -> F161
// 0x14198b284  4c8b06                         movq	(%rsi), %r8
// Frames: F163
// 0x14198b287  488d85d0000000                 leaq	0xd0(%rbp), %rax
// 0x14198b28e  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14198b293  c644242801                     movb	$0x1, 0x28(%rsp)
// 0x14198b298  488d5570                       leaq	0x70(%rbp), %rdx
// 0x14198b29c  4c8d8da0000000                 leaq	0xa0(%rbp), %r9
// 0x14198b2a3  e8f80f0000                     callq	0x14198c2a0 <.text+0x198b2a0>
// Frames: F164
// 0x14198b2a8  4c8b85e0000000                 movq	0xe0(%rbp), %r8
// Frames: F165 -> F164
// 0x14198b2af  4d85c0                         testq	%r8, %r8
// Frames: F164
// 0x14198b2b2  0f84fd070000                   je	0x14198bab5 <.text+0x198aab5>
// Frames: F166
// 0x14198b2b8  488b85a0000000                 movq	0xa0(%rbp), %rax
// 0x14198b2bf  48898550010000                 movq	%rax, 0x150(%rbp)
// Frames: F167 -> F166
// 0x14198b2c6  4885c0                         testq	%rax, %rax
// 0x14198b2c9  0f84fd080000                   je	0x14198bbcc <.text+0x198abcc>
// Frames: F168
// 0x14198b2cf  4c8b95b0020000                 movq	0x2b0(%rbp), %r10
// 0x14198b2d6  488b95a8020000                 movq	0x2a8(%rbp), %rdx
// 0x14198b2dd  4c8b8da0020000                 movq	0x2a0(%rbp), %r9
// 0x14198b2e4  488b8d90020000                 movq	0x290(%rbp), %rcx
// 0x14198b2eb  488b8588020000                 movq	0x288(%rbp), %rax
// 0x14198b2f2  4c8b9d60020000                 movq	0x260(%rbp), %r11
// 0x14198b2f9  f30f104d70                     movss	0x70(%rbp), %xmm1
// 0x14198b2fe  f30f104574                     movss	0x74(%rbp), %xmm0
// 0x14198b303  f30f10a580000000               movss	0x80(%rbp), %xmm4
// 0x14198b30b  f30f109d84000000               movss	0x84(%rbp), %xmm3
// Frames: F169 -> F170 -> F168
// 0x14198b313  0f28f8                         movaps	%xmm0, %xmm7
// 0x14198b316  f30f58fc                       addss	%xmm4, %xmm7
// Frames: F168
// 0x14198b31a  f20f105578                     movsd	0x78(%rbp), %xmm2
// Frames: F169 -> F171 -> F168
// 0x14198b31f  440f28c3                       movaps	%xmm3, %xmm8
// 0x14198b323  440f14c1                       unpcklps	%xmm1, %xmm8            # xmm8 = xmm8[0],xmm1[0],xmm8[1],xmm1[1]
// 0x14198b327  440f58c2                       addps	%xmm2, %xmm8
// Frames: F172 -> F173 -> F168
// 0x14198b32b  f30f593dd1f07b00               mulss	0x7bf0d1(%rip), %xmm7   # 0x14214a404
// Frames: F172 -> F174 -> F168
// 0x14198b333  440f5905b5c88300               mulps	0x83c8b5(%rip), %xmm8   # 0x1421c7bf0
// 0x14198b33b  f3450f16c8                     movshdup	%xmm8, %xmm9            # xmm9 = xmm8[1,1,3,3]
// Frames: F175
// 0x14198b340  488b9dd8000000                 movq	0xd8(%rbp), %rbx
// Frames: F121 -> F176 -> F177 -> F175
// 0x14198b347  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x14198b34b  4e8d04c3                       leaq	(%rbx,%r8,8), %r8
// 0x14198b34f  4c898538010000                 movq	%r8, 0x138(%rbp)
// 0x14198b356  448b461c                       movl	0x1c(%rsi), %r8d
// 0x14198b35a  448985a8000000                 movl	%r8d, 0xa8(%rbp)
// 0x14198b361  4c8b06                         movq	(%rsi), %r8
// 0x14198b364  4c8985b0000000                 movq	%r8, 0xb0(%rbp)
// 0x14198b36b  4c8b4610                       movq	0x10(%rsi), %r8
// 0x14198b36f  4c8985b8000000                 movq	%r8, 0xb8(%rbp)
// 0x14198b376  4d8b03                         movq	(%r11), %r8
// 0x14198b379  4c8945e0                       movq	%r8, -0x20(%rbp)
// 0x14198b37d  4d8b02                         movq	(%r10), %r8
// 0x14198b380  4983c02c                       addq	$0x2c, %r8
// 0x14198b384  4c8985c0000000                 movq	%r8, 0xc0(%rbp)
// 0x14198b38b  458b411c                       movl	0x1c(%r9), %r8d
// 0x14198b38f  4489858c000000                 movl	%r8d, 0x8c(%rbp)
// 0x14198b396  4d8b01                         movq	(%r9), %r8
// 0x14198b399  4c8945e8                       movq	%r8, -0x18(%rbp)
// 0x14198b39d  4d8b4110                       movq	0x10(%r9), %r8
// 0x14198b3a1  4c8945f0                       movq	%r8, -0x10(%rbp)
// 0x14198b3a5  488b12                         movq	(%rdx), %rdx
// 0x14198b3a8  488955f8                       movq	%rdx, -0x8(%rbp)
// 0x14198b3ac  f30f16ea                       movshdup	%xmm2, %xmm5            # xmm5 = xmm2[1,1,3,3]
// 0x14198b3b0  f30f5ccd                       subss	%xmm5, %xmm1
// 0x14198b3b4  f30f5cc4                       subss	%xmm4, %xmm0
// 0x14198b3b8  f30f5cd3                       subss	%xmm3, %xmm2
// 0x14198b3bc  f30f59c9                       mulss	%xmm1, %xmm1
// 0x14198b3c0  f30f59c0                       mulss	%xmm0, %xmm0
// 0x14198b3c4  f30f58c1                       addss	%xmm1, %xmm0
// 0x14198b3c8  f30f59d2                       mulss	%xmm2, %xmm2
// 0x14198b3cc  f30f58d0                       addss	%xmm0, %xmm2
// 0x14198b3d0  f3440f51d2                     sqrtss	%xmm2, %xmm10
// 0x14198b3d5  488b09                         movq	(%rcx), %rcx
// 0x14198b3d8  48894d40                       movq	%rcx, 0x40(%rbp)
// 0x14198b3dc  488b00                         movq	(%rax), %rax
// 0x14198b3df  48894548                       movq	%rax, 0x48(%rbp)
// 0x14198b3e3  488b8568020000                 movq	0x268(%rbp), %rax
// 0x14198b3ea  488b00                         movq	(%rax), %rax
// 0x14198b3ed  48894550                       movq	%rax, 0x50(%rbp)
// 0x14198b3f1  f2440f101d067f8e00             movsd	0x8e7f06(%rip), %xmm11  # 0x142273300
// 0x14198b3fa  31c0                           xorl	%eax, %eax
// 0x14198b3fc  48894568                       movq	%rax, 0x68(%rbp)
// 0x14198b400  4889ee                         movq	%rbp, %rsi
// 0x14198b403  4c8d7db8                       leaq	-0x48(%rbp), %r15
// 0x14198b407  31ff                           xorl	%edi, %edi
// 0x14198b409  31c0                           xorl	%eax, %eax
// 0x14198b40b  48894560                       movq	%rax, 0x60(%rbp)
// 0x14198b40f  31c0                           xorl	%eax, %eax
// 0x14198b411  48894558                       movq	%rax, 0x58(%rbp)
// 0x14198b415  eb16                           jmp	0x14198b42d <.text+0x198a42d>
// 0x14198b417  660f1f840000000000             nopw	(%rax,%rax)
// 0x14198b420  b001                           movb	$0x1, %al
// 0x14198b422  48894558                       movq	%rax, 0x58(%rbp)
// 0x14198b426  4883c318                       addq	$0x18, %rbx
// 0x14198b42a  48ffc7                         incq	%rdi
// Frames: F52 -> F178 -> F179 -> F175
// 0x14198b42d  483b9d38010000                 cmpq	0x138(%rbp), %rbx
// Frames: F180 -> F179 -> F175
// 0x14198b434  0f8430040000                   je	0x14198b86a <.text+0x198a86a>
// Frames: F175
// 0x14198b43a  48897d38                       movq	%rdi, 0x38(%rbp)
// 0x14198b43e  488b13                         movq	(%rbx), %rdx
// 0x14198b441  f30f104308                     movss	0x8(%rbx), %xmm0
// 0x14198b446  f30f118534010000               movss	%xmm0, 0x134(%rbp)
// 0x14198b44e  488b430c                       movq	0xc(%rbx), %rax
// 0x14198b452  488985f0000000                 movq	%rax, 0xf0(%rbp)
// 0x14198b459  8b4314                         movl	0x14(%rbx), %eax
// 0x14198b45c  8985f8000000                   movl	%eax, 0xf8(%rbp)
// 0x14198b462  8b85a8000000                   movl	0xa8(%rbp), %eax
// 0x14198b468  488b8db8000000                 movq	0xb8(%rbp), %rcx
// Frames: F145 -> F146 -> F181
// 0x14198b46f  8901                           movl	%eax, (%rcx)
// Frames: F181
// 0x14198b471  488b8db0000000                 movq	0xb0(%rbp), %rcx
// 0x14198b478  e8130eabfe                     callq	0x14043c290 <.text+0x43b290>
// 0x14198b47d  488945d0                       movq	%rax, -0x30(%rbp)
// 0x14198b481  488955d8                       movq	%rdx, -0x28(%rbp)
// Frames: F182
// 0x14198b485  4c8b6538                       movq	0x38(%rbp), %r12
// 0x14198b489  4983fc02                       cmpq	$0x2, %r12
// 0x14198b48d  0f83b5060000                   jae	0x14198bb48 <.text+0x198ab48>
// Frames: F183
// 0x14198b493  488d4538                       leaq	0x38(%rbp), %rax
// 0x14198b497  48894500                       movq	%rax, (%rbp)
// 0x14198b49b  488d059e686ffe                 leaq	-0x1909762(%rip), %rax  # 0x140081d40 <.text+0x80d40>
// 0x14198b4a2  48894508                       movq	%rax, 0x8(%rbp)
// 0x14198b4a6  488b85c0000000                 movq	0xc0(%rbp), %rax
// 0x14198b4ad  48894510                       movq	%rax, 0x10(%rbp)
// 0x14198b4b1  488d0588e269fe                 leaq	-0x1961d78(%rip), %rax  # 0x140029740 <.text+0x28740>
// 0x14198b4b8  48894518                       movq	%rax, 0x18(%rbp)
// 0x14198b4bc  488d8534010000                 leaq	0x134(%rbp), %rax
// 0x14198b4c3  48894520                       movq	%rax, 0x20(%rbp)
// 0x14198b4c7  488d05120aeffe                 leaq	-0x110f5ee(%rip), %rax  # 0x14087bee0 <.text+0x87aee0>
// 0x14198b4ce  48894528                       movq	%rax, 0x28(%rbp)
// Frames: F184 -> F185 -> F186 -> F183
// 0x14198b4d2  488d052f83d600                 leaq	0xd6832f(%rip), %rax    # 0x1426f3808
// 0x14198b4d9  48898500010000                 movq	%rax, 0x100(%rbp)
// 0x14198b4e0  48c7850801000003000000         movq	$0x3, 0x108(%rbp)
// 0x14198b4eb  4889b510010000                 movq	%rsi, 0x110(%rbp)
// 0x14198b4f2  48c7851801000003000000         movq	$0x3, 0x118(%rbp)
// 0x14198b4fd  48c7852001000000000000         movq	$0x0, 0x120(%rbp)
// 0x14198b508  4c89f9                         movq	%r15, %rcx
// 0x14198b50b  488d9500010000                 leaq	0x100(%rbp), %rdx
// 0x14198b512  e809e3edfe                     callq	0x140869820 <.text+0x868820>
// Frames: F183
// 0x14198b517  4989f6                         movq	%rsi, %r14
// 0x14198b51a  488b45b8                       movq	-0x48(%rbp), %rax
// 0x14198b51e  48898548010000                 movq	%rax, 0x148(%rbp)
// 0x14198b525  4c8b6dc0                       movq	-0x40(%rbp), %r13
// 0x14198b529  488b75c8                       movq	-0x38(%rbp), %rsi
// Frames: F187
// 0x14198b52d  f3440f10a534010000             movss	0x134(%rbp), %xmm12
// Frames: F84 -> F85 -> F86 -> F87 -> F188 -> F189 -> F190 -> F187
// 0x14198b536  0fb60584f52e01                 movzbl	0x12ef584(%rip), %eax   # 0x142c7aac1
// Frames: F93 -> F86 -> F87 -> F188 -> F189 -> F190 -> F187
// 0x14198b53d  b930000000                     movl	$0x30, %ecx
// 0x14198b542  ba08000000                     movl	$0x8, %edx
// 0x14198b547  e834f2a4fe                     callq	0x1403da780 <.text+0x3d9780>
// Frames: F188 -> F189 -> F190 -> F187
// 0x14198b54c  4885c0                         testq	%rax, %rax
// 0x14198b54f  4c89ad40010000                 movq	%r13, 0x140(%rbp)
// 0x14198b556  0f8405060000                   je	0x14198bb61 <.text+0x198ab61>
// Frames: F189 -> F190 -> F187
// 0x14198b55c  4b8d0c64                       leaq	(%r12,%r12,2), %rcx
// 0x14198b560  488d0c8d70000000               leaq	0x70(,%rcx,4), %rcx
// 0x14198b568  4801e9                         addq	%rbp, %rcx
// 0x14198b56b  48c70001000000                 movq	$0x1, (%rax)
// 0x14198b572  48c7400801000000               movq	$0x1, 0x8(%rax)
// 0x14198b57a  f3440f116010                   movss	%xmm12, 0x10(%rax)
// 0x14198b580  488b11                         movq	(%rcx), %rdx
// 0x14198b583  48895014                       movq	%rdx, 0x14(%rax)
// 0x14198b587  8b4908                         movl	0x8(%rcx), %ecx
// 0x14198b58a  89481c                         movl	%ecx, 0x1c(%rax)
// 0x14198b58d  488b8df0000000                 movq	0xf0(%rbp), %rcx
// 0x14198b594  48894820                       movq	%rcx, 0x20(%rax)
// 0x14198b598  8b8df8000000                   movl	0xf8(%rbp), %ecx
// 0x14198b59e  894828                         movl	%ecx, 0x28(%rax)
// Frames: F191
// 0x14198b5a1  488d0d9082d600                 leaq	0xd68290(%rip), %rcx    # 0x1426f3838
// 0x14198b5a8  48894c2420                     movq	%rcx, 0x20(%rsp)
// 0x14198b5ad  488d8d00010000                 leaq	0x100(%rbp), %rcx
// 0x14198b5b4  4c89ea                         movq	%r13, %rdx
// 0x14198b5b7  4989f0                         movq	%rsi, %r8
// 0x14198b5ba  4989c1                         movq	%rax, %r9
// 0x14198b5bd  e89eb5ccfe                     callq	0x140656b60 <.text+0x655b60>
// Frames: F192
// 0x14198b5c2  4c89f6                         movq	%r14, %rsi
// 0x14198b5c5  488b8520010000                 movq	0x120(%rbp), %rax
// 0x14198b5cc  48894520                       movq	%rax, 0x20(%rbp)
// 0x14198b5d0  0f108500010000                 movups	0x100(%rbp), %xmm0
// 0x14198b5d7  0f108d10010000                 movups	0x110(%rbp), %xmm1
// 0x14198b5de  0f294d10                       movaps	%xmm1, 0x10(%rbp)
// 0x14198b5e2  0f294500                       movaps	%xmm0, (%rbp)
// 0x14198b5e6  488b4de0                       movq	-0x20(%rbp), %rcx
// 0x14198b5ea  4c89f2                         movq	%r14, %rdx
// 0x14198b5ed  e86e4436ff                     callq	0x140cefa60 <.text+0xceea60>
// Frames: F149 -> F150 -> F83 -> F83 -> F83 -> F193
// 0x14198b5f2  488b9548010000                 movq	0x148(%rbp), %rdx
// 0x14198b5f9  4885d2                         testq	%rdx, %rdx
// 0x14198b5fc  7412                           je	0x14198b610 <.text+0x198a610>
// Frames: F155 -> F156 -> F157 -> F83 -> F83 -> F83 -> F193
// 0x14198b5fe  41b801000000                   movl	$0x1, %r8d
// 0x14198b604  488b8d40010000                 movq	0x140(%rbp), %rcx
// 0x14198b60b  e880f1a4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F194
// 0x14198b610  488d4dd0                       leaq	-0x30(%rbp), %rcx
// 0x14198b614  e877adcffe                     callq	0x140686390 <.text+0x685390>
// 0x14198b619  04fe                           addb	$-0x2, %al
// 0x14198b61b  0fb6c0                         movzbl	%al, %eax
// 0x14198b61e  3c03                           cmpb	$0x3, %al
// 0x14198b620  b903000000                     movl	$0x3, %ecx
// 0x14198b625  0f43c1                         cmovael	%ecx, %eax
// 0x14198b628  0fb6c0                         movzbl	%al, %eax
// 0x14198b62b  488d0db682d600                 leaq	0xd682b6(%rip), %rcx    # 0x1426f38e8
// 0x14198b632  48630481                       movslq	(%rcx,%rax,4), %rax
// 0x14198b636  4801c8                         addq	%rcx, %rax
// 0x14198b639  ffe0                           jmpq	*%rax
// Frames: F195 -> F196
// 0x14198b63b  4c89f9                         movq	%r15, %rcx
// 0x14198b63e  488b5540                       movq	0x40(%rbp), %rdx
// 0x14198b642  e82991c5fe                     callq	0x1405e4770 <.text+0x5e3770>
// Frames: F197 -> F196
// 0x14198b647  488b4548                       movq	0x48(%rbp), %rax
// 0x14198b64b  f30f1008                       movss	(%rax), %xmm1
// 0x14198b64f  f30f105004                     movss	0x4(%rax), %xmm2
// 0x14198b654  4c89f9                         movq	%r15, %rcx
// 0x14198b657  e84491c5fe                     callq	0x1405e47a0 <.text+0x5e37a0>
// Frames: F198 -> F196
// 0x14198b65c  0f28d1                         movaps	%xmm1, %xmm2
// 0x14198b65f  f3440f118d00010000             movss	%xmm9, 0x100(%rbp)
// 0x14198b668  f30f11bd04010000               movss	%xmm7, 0x104(%rbp)
// 0x14198b670  f3440f118508010000             movss	%xmm8, 0x108(%rbp)
// 0x14198b679  8b85f8000000                   movl	0xf8(%rbp), %eax
// 0x14198b67f  488d8d0c010000                 leaq	0x10c(%rbp), %rcx
// 0x14198b686  894108                         movl	%eax, 0x8(%rcx)
// 0x14198b689  488b85f0000000                 movq	0xf0(%rbp), %rax
// 0x14198b690  488901                         movq	%rax, (%rcx)
// Frames: F199 -> F198 -> F196
// 0x14198b693  f2440f119d90000000             movsd	%xmm11, 0x90(%rbp)
// 0x14198b69c  c6859800000000                 movb	$0x0, 0x98(%rbp)
// Frames: F200 -> F196
// 0x14198b6a3  488d8590000000                 leaq	0x90(%rbp), %rax
// 0x14198b6aa  4889442430                     movq	%rax, 0x30(%rsp)
// 0x14198b6af  488d8500010000                 leaq	0x100(%rbp), %rax
// 0x14198b6b6  4889442428                     movq	%rax, 0x28(%rsp)
// 0x14198b6bb  488b4550                       movq	0x50(%rbp), %rax
// 0x14198b6bf  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14198b6c4  4889f1                         movq	%rsi, %rcx
// 0x14198b6c7  0f28c8                         movaps	%xmm0, %xmm1
// 0x14198b6ca  4d89f9                         movq	%r15, %r9
// 0x14198b6cd  e81e91c5fe                     callq	0x1405e47f0 <.text+0x5e37f0>
// 0x14198b6d2  f30f104504                     movss	0x4(%rbp), %xmm0
// 0x14198b6d7  f20f104d08                     movsd	0x8(%rbp), %xmm1
// Frames: F201 -> F202 -> F203 -> F196
// 0x14198b6dc  f30f105500                     movss	(%rbp), %xmm2
// Frames: F201 -> F204 -> F203 -> F196
// 0x14198b6e1  f30f5cc7                       subss	%xmm7, %xmm0
// Frames: F205 -> F206 -> F203 -> F196
// 0x14198b6e5  f30f59c0                       mulss	%xmm0, %xmm0
// Frames: F201 -> F202 -> F203 -> F196
// 0x14198b6e9  0f16d1                         movlhps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0]
// 0x14198b6ec  0fc6d142                       shufps	$0x42, %xmm1, %xmm2     # xmm2 = xmm2[2,0],xmm1[0,1]
// 0x14198b6f0  410f5cd0                       subps	%xmm8, %xmm2
// Frames: F205 -> F206 -> F203 -> F196
// 0x14198b6f4  0f59d2                         mulps	%xmm2, %xmm2
// 0x14198b6f7  f30f16ca                       movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
// 0x14198b6fb  f30f58c8                       addss	%xmm0, %xmm1
// 0x14198b6ff  f30f58ca                       addss	%xmm2, %xmm1
// Frames: F207 -> F208 -> F206 -> F203 -> F196
// 0x14198b703  0f57c0                         xorps	%xmm0, %xmm0
// 0x14198b706  f30f51c1                       sqrtss	%xmm1, %xmm0
// Frames: F196
// 0x14198b70a  f30f58c0                       addss	%xmm0, %xmm0
// Frames: F209
// 0x14198b70e  f3410f5cc2                     subss	%xmm10, %xmm0
// 0x14198b713  488b8598020000                 movq	0x298(%rbp), %rax
// 0x14198b71a  f30f1100                       movss	%xmm0, (%rax)
// 0x14198b71e  8b858c000000                   movl	0x8c(%rbp), %eax
// 0x14198b724  488b4df0                       movq	-0x10(%rbp), %rcx
// Frames: F145 -> F146 -> F210
// 0x14198b728  8901                           movl	%eax, (%rcx)
// 0x14198b72a  488b8550010000                 movq	0x150(%rbp), %rax
// Frames: F210
// 0x14198b731  488b10                         movq	(%rax), %rdx
// 0x14198b734  b001                           movb	$0x1, %al
// 0x14198b736  48894560                       movq	%rax, 0x60(%rbp)
// 0x14198b73a  488b4de8                       movq	-0x18(%rbp), %rcx
// 0x14198b73e  4c8b45f8                       movq	-0x8(%rbp), %r8
// 0x14198b742  e839e5c2fe                     callq	0x1405b9c80 <.text+0x5b8c80>
// 0x14198b747  e9dafcffff                     jmp	0x14198b426 <.text+0x198a426>
// 0x14198b74c  0f1f4000                       nopl	(%rax)
// Frames: F195 -> F211
// 0x14198b750  4c89f9                         movq	%r15, %rcx
// 0x14198b753  488b5540                       movq	0x40(%rbp), %rdx
// 0x14198b757  e81490c5fe                     callq	0x1405e4770 <.text+0x5e3770>
// Frames: F197 -> F211
// 0x14198b75c  488b4548                       movq	0x48(%rbp), %rax
// 0x14198b760  f30f1008                       movss	(%rax), %xmm1
// 0x14198b764  f30f105004                     movss	0x4(%rax), %xmm2
// 0x14198b769  4c89f9                         movq	%r15, %rcx
// 0x14198b76c  e82f90c5fe                     callq	0x1405e47a0 <.text+0x5e37a0>
// Frames: F198 -> F211
// 0x14198b771  0f28d1                         movaps	%xmm1, %xmm2
// 0x14198b774  f3440f118d00010000             movss	%xmm9, 0x100(%rbp)
// 0x14198b77d  f30f11bd04010000               movss	%xmm7, 0x104(%rbp)
// 0x14198b785  f3440f118508010000             movss	%xmm8, 0x108(%rbp)
// 0x14198b78e  8b85f8000000                   movl	0xf8(%rbp), %eax
// 0x14198b794  488d8d0c010000                 leaq	0x10c(%rbp), %rcx
// 0x14198b79b  894108                         movl	%eax, 0x8(%rcx)
// 0x14198b79e  488b85f0000000                 movq	0xf0(%rbp), %rax
// 0x14198b7a5  488901                         movq	%rax, (%rcx)
// Frames: F199 -> F198 -> F211
// 0x14198b7a8  f2440f119d90000000             movsd	%xmm11, 0x90(%rbp)
// 0x14198b7b1  c6859800000000                 movb	$0x0, 0x98(%rbp)
// Frames: F200 -> F211
// 0x14198b7b8  488d8590000000                 leaq	0x90(%rbp), %rax
// 0x14198b7bf  4889442430                     movq	%rax, 0x30(%rsp)
// 0x14198b7c4  488d8500010000                 leaq	0x100(%rbp), %rax
// 0x14198b7cb  4889442428                     movq	%rax, 0x28(%rsp)
// 0x14198b7d0  488b4550                       movq	0x50(%rbp), %rax
// 0x14198b7d4  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14198b7d9  4889f1                         movq	%rsi, %rcx
// 0x14198b7dc  0f28c8                         movaps	%xmm0, %xmm1
// 0x14198b7df  4d89f9                         movq	%r15, %r9
// 0x14198b7e2  e80990c5fe                     callq	0x1405e47f0 <.text+0x5e37f0>
// 0x14198b7e7  f3440f107500                   movss	(%rbp), %xmm14
// 0x14198b7ed  f3440f106d04                   movss	0x4(%rbp), %xmm13
// 0x14198b7f3  66440f286500                   movapd	(%rbp), %xmm12
// Frames: F212
// 0x14198b7f9  488b8598020000                 movq	0x298(%rbp), %rax
// 0x14198b800  f30f1030                       movss	(%rax), %xmm6
// 0x14198b804  488b8d50010000                 movq	0x150(%rbp), %rcx
// Frames: F213
// 0x14198b80b  448a613c                       movb	0x3c(%rcx), %r12b
// Frames: F214
// 0x14198b80f  e8bcf0d2fe                     callq	0x1406ba8d0 <.text+0x6b98d0>
// Frames: F201 -> F202 -> F203 -> F211
// 0x14198b814  0f28d0                         movaps	%xmm0, %xmm2
// 0x14198b817  f2450f10e6                     movsd	%xmm14, %xmm12          # xmm12 = xmm14[0],xmm12[1]
// 0x14198b81c  450fc6e4e2                     shufps	$0xe2, %xmm12, %xmm12   # xmm12 = xmm12[2,0,2,3]
// 0x14198b821  450f5ce0                       subps	%xmm8, %xmm12
// Frames: F201 -> F204 -> F203 -> F211
// 0x14198b825  f3440f5cef                     subss	%xmm7, %xmm13
// Frames: F205 -> F206 -> F203 -> F211
// 0x14198b82a  f3450f59ed                     mulss	%xmm13, %xmm13
// 0x14198b82f  450f59e4                       mulps	%xmm12, %xmm12
// 0x14198b833  f3410f16c4                     movshdup	%xmm12, %xmm0           # xmm0 = xmm12[1,1,3,3]
// 0x14198b838  f3410f58c5                     addss	%xmm13, %xmm0
// 0x14198b83d  f3410f58c4                     addss	%xmm12, %xmm0
// Frames: F207 -> F208 -> F206 -> F203 -> F211
// 0x14198b842  f30f51c0                       sqrtss	%xmm0, %xmm0
// Frames: F211
// 0x14198b846  f30f58c0                       addss	%xmm0, %xmm0
// Frames: F212
// 0x14198b84a  f30f5cc6                       subss	%xmm6, %xmm0
// Frames: F215
// 0x14198b84e  4489e2                         movl	%r12d, %edx
// 0x14198b851  0f28d9                         movaps	%xmm1, %xmm3
// 0x14198b854  e8a704c1fe                     callq	0x14059bd00 <.text+0x59ad00>
// 0x14198b859  0f28f0                         movaps	%xmm0, %xmm6
// 0x14198b85c  b801000000                     movl	$0x1, %eax
// 0x14198b861  48894568                       movq	%rax, 0x68(%rbp)
// 0x14198b865  e9bcfbffff                     jmp	0x14198b426 <.text+0x198a426>
// Frames: F216 -> F217
// 0x14198b86a  837d6800                       cmpl	$0x0, 0x68(%rbp)
// Frames: F217
// 0x14198b86e  0f8484000000                   je	0x14198b8f8 <.text+0x198a8f8>
// Frames: F218
// 0x14198b874  488b8550010000                 movq	0x150(%rbp), %rax
// 0x14198b87b  488b38                         movq	(%rax), %rdi
// 0x14198b87e  488b9580020000                 movq	0x280(%rbp), %rdx
// Frames: F219 -> F146 -> F220 -> F221
// 0x14198b885  8b421c                         movl	0x1c(%rdx), %eax
// 0x14198b888  488b4a10                       movq	0x10(%rdx), %rcx
// 0x14198b88c  8901                           movl	%eax, (%rcx)
// Frames: F222 -> F220 -> F221
// 0x14198b88e  488b32                         movq	(%rdx), %rsi
// Frames: F223 -> F224 -> F220 -> F221
// 0x14198b891  488b5630                       movq	0x30(%rsi), %rdx
// Frames: F225 -> F220 -> F221
// 0x14198b895  488b5e40                       movq	0x40(%rsi), %rbx
// Frames: F223 -> F224 -> F220 -> F221
// 0x14198b899  483b5620                       cmpq	0x20(%rsi), %rdx
// 0x14198b89d  750d                           jne	0x14198b8ac <.text+0x198a8ac>
// Frames: F224 -> F220 -> F221
// 0x14198b89f  488d4e20                       leaq	0x20(%rsi), %rcx
// Frames: F226 -> F224 -> F220 -> F221
// 0x14198b8a3  e8d8e58efe                     callq	0x140279e80 <.text+0x278e80>
// Frames: F227 -> F224 -> F220 -> F221
// 0x14198b8a8  488b5630                       movq	0x30(%rsi), %rdx
// Frames: F228 -> F229 -> F227 -> F224 -> F220 -> F221
// 0x14198b8ac  488b4628                       movq	0x28(%rsi), %rax
// Frames: F230 -> F227 -> F224 -> F220 -> F221
// 0x14198b8b0  48c1e206                       shlq	$0x6, %rdx
// Frames: F74 -> F231 -> F224 -> F220 -> F221
// 0x14198b8b4  c7041000000000                 movl	$0x0, (%rax,%rdx)
// 0x14198b8bb  c744100800000000               movl	$0x0, 0x8(%rax,%rdx)
// 0x14198b8c3  c744101001000000               movl	$0x1, 0x10(%rax,%rdx)
// 0x14198b8cb  f30f11741014                   movss	%xmm6, 0x14(%rax,%rdx)
// 0x14198b8d1  c744101800000000               movl	$0x0, 0x18(%rax,%rdx)
// 0x14198b8d9  48897c1020                     movq	%rdi, 0x20(%rax,%rdx)
// 0x14198b8de  c744102800000000               movl	$0x0, 0x28(%rax,%rdx)
// 0x14198b8e6  c644103400                     movb	$0x0, 0x34(%rax,%rdx)
// 0x14198b8eb  48895c1038                     movq	%rbx, 0x38(%rax,%rdx)
// Frames: F232 -> F224 -> F220 -> F221
// 0x14198b8f0  48ff4630                       incq	0x30(%rsi)
// Frames: F233 -> F220 -> F221
// 0x14198b8f4  48ff4640                       incq	0x40(%rsi)
// Frames: F234
// 0x14198b8f8  f6456001                       testb	$0x1, 0x60(%rbp)
// 0x14198b8fc  7454                           je	0x14198b952 <.text+0x198a952>
// Frames: F235
// 0x14198b8fe  488b8d50010000                 movq	0x150(%rbp), %rcx
// 0x14198b905  488b01                         movq	(%rcx), %rax
// Frames: F236
// 0x14198b908  0f104130                       movups	0x30(%rcx), %xmm0
// 0x14198b90c  0f11850c010000                 movups	%xmm0, 0x10c(%rbp)
// 0x14198b913  0f104124                       movups	0x24(%rcx), %xmm0
// 0x14198b917  0f298500010000                 movaps	%xmm0, 0x100(%rbp)
// 0x14198b91e  4c8b8578020000                 movq	0x278(%rbp), %r8
// Frames: F145 -> F146 -> F237
// 0x14198b925  418b481c                       movl	0x1c(%r8), %ecx
// 0x14198b929  498b5010                       movq	0x10(%r8), %rdx
// 0x14198b92d  890a                           movl	%ecx, (%rdx)
// Frames: F162 -> F237
// 0x14198b92f  498b08                         movq	(%r8), %rcx
// Frames: F237
// 0x14198b932  c70100000000                   movl	$0x0, (%rcx)
// 0x14198b938  48894110                       movq	%rax, 0x10(%rcx)
// 0x14198b93c  0f288500010000                 movaps	0x100(%rbp), %xmm0
// 0x14198b943  0f114118                       movups	%xmm0, 0x18(%rcx)
// 0x14198b947  0f10850c010000                 movups	0x10c(%rbp), %xmm0
// 0x14198b94e  0f114124                       movups	%xmm0, 0x24(%rcx)
// Frames: F238
// 0x14198b952  f6455801                       testb	$0x1, 0x58(%rbp)
// 0x14198b956  0f8459010000                   je	0x14198bab5 <.text+0x198aab5>
// Frames: F239
// 0x14198b95c  488b8578020000                 movq	0x278(%rbp), %rax
// 0x14198b963  488b00                         movq	(%rax), %rax
// Frames: F61 -> F239
// 0x14198b966  833802                         cmpl	$0x2, (%rax)
// Frames: F239
// 0x14198b969  0f8446010000                   je	0x14198bab5 <.text+0x198aab5>
// Frames: F240 -> F241 -> F242
// 0x14198b96f  f30f104018                     movss	0x18(%rax), %xmm0
// 0x14198b974  488b8d50010000                 movq	0x150(%rbp), %rcx
// 0x14198b97b  0f2e4124                       ucomiss	0x24(%rcx), %xmm0
// 0x14198b97f  757a                           jne	0x14198b9fb <.text+0x198a9fb>
// 0x14198b981  7a78                           jp	0x14198b9fb <.text+0x198a9fb>
// Frames: F243 -> F241 -> F242
// 0x14198b983  f30f10401c                     movss	0x1c(%rax), %xmm0
// 0x14198b988  488b8d50010000                 movq	0x150(%rbp), %rcx
// 0x14198b98f  0f2e4128                       ucomiss	0x28(%rcx), %xmm0
// 0x14198b993  7566                           jne	0x14198b9fb <.text+0x198a9fb>
// 0x14198b995  7a64                           jp	0x14198b9fb <.text+0x198a9fb>
// Frames: F244 -> F241 -> F242
// 0x14198b997  f30f104020                     movss	0x20(%rax), %xmm0
// 0x14198b99c  488b8d50010000                 movq	0x150(%rbp), %rcx
// Frames: F245 -> F244 -> F241 -> F242
// 0x14198b9a3  0f2e412c                       ucomiss	0x2c(%rcx), %xmm0
// Frames: F244 -> F241 -> F242
// 0x14198b9a7  7552                           jne	0x14198b9fb <.text+0x198a9fb>
// 0x14198b9a9  7a50                           jp	0x14198b9fb <.text+0x198a9fb>
// 0x14198b9ab  f30f104024                     movss	0x24(%rax), %xmm0
// 0x14198b9b0  488b8d50010000                 movq	0x150(%rbp), %rcx
// 0x14198b9b7  0f2e4130                       ucomiss	0x30(%rcx), %xmm0
// 0x14198b9bb  753e                           jne	0x14198b9fb <.text+0x198a9fb>
// 0x14198b9bd  7a3c                           jp	0x14198b9fb <.text+0x198a9fb>
// Frames: F246 -> F241 -> F242
// 0x14198b9bf  f30f104028                     movss	0x28(%rax), %xmm0
// 0x14198b9c4  488b8d50010000                 movq	0x150(%rbp), %rcx
// 0x14198b9cb  0f2e4134                       ucomiss	0x34(%rcx), %xmm0
// 0x14198b9cf  752a                           jne	0x14198b9fb <.text+0x198a9fb>
// 0x14198b9d1  7a28                           jp	0x14198b9fb <.text+0x198a9fb>
// Frames: F247 -> F241 -> F242
// 0x14198b9d3  0fb64830                       movzbl	0x30(%rax), %ecx
// 0x14198b9d7  488b9550010000                 movq	0x150(%rbp), %rdx
// Frames: F248 -> F247 -> F241 -> F242
// 0x14198b9de  3a4a3c                         cmpb	0x3c(%rdx), %cl
// Frames: F247 -> F241 -> F242
// 0x14198b9e1  7518                           jne	0x14198b9fb <.text+0x198a9fb>
// Frames: F249 -> F241 -> F242
// 0x14198b9e3  f30f10402c                     movss	0x2c(%rax), %xmm0
// 0x14198b9e8  488b8d50010000                 movq	0x150(%rbp), %rcx
// 0x14198b9ef  0f2e4138                       ucomiss	0x38(%rcx), %xmm0
// Frames: F242
// 0x14198b9f3  7506                           jne	0x14198b9fb <.text+0x198a9fb>
// 0x14198b9f5  0f8bba000000                   jnp	0x14198bab5 <.text+0x198aab5>
// Frames: F145 -> F146 -> F250
// 0x14198b9fb  4c8b8570020000                 movq	0x270(%rbp), %r8
// 0x14198ba02  418b481c                       movl	0x1c(%r8), %ecx
// 0x14198ba06  498b5010                       movq	0x10(%r8), %rdx
// 0x14198ba0a  890a                           movl	%ecx, (%rdx)
// Frames: F162 -> F250
// 0x14198ba0c  498b38                         movq	(%r8), %rdi
// Frames: F251
// 0x14198ba0f  488b7010                       movq	0x10(%rax), %rsi
// Frames: F252
// 0x14198ba13  f20f107818                     movsd	0x18(%rax), %xmm7
// Frames: F253
// 0x14198ba18  488b5820                       movq	0x20(%rax), %rbx
// 0x14198ba1c  488b8d50010000                 movq	0x150(%rbp), %rcx
// Frames: F254
// 0x14198ba23  f20f107124                     movsd	0x24(%rcx), %xmm6
// Frames: F255
// 0x14198ba28  4c8b712c                       movq	0x2c(%rcx), %r14
// Frames: F256
// 0x14198ba2c  f2440f104828                   movsd	0x28(%rax), %xmm9
// Frames: F257
// 0x14198ba32  f2440f104134                   movsd	0x34(%rcx), %xmm8
// Frames: F84 -> F85 -> F86 -> F87 -> F188 -> F189 -> F190 -> F250
// 0x14198ba38  0fb60582f02e01                 movzbl	0x12ef082(%rip), %eax   # 0x142c7aac1
// Frames: F93 -> F86 -> F87 -> F188 -> F189 -> F190 -> F250
// 0x14198ba3f  b950000000                     movl	$0x50, %ecx
// 0x14198ba44  ba08000000                     movl	$0x8, %edx
// 0x14198ba49  e832eda4fe                     callq	0x1403da780 <.text+0x3d9780>
// Frames: F188 -> F189 -> F190 -> F250
// 0x14198ba4e  4885c0                         testq	%rax, %rax
// 0x14198ba51  0f8483010000                   je	0x14198bbda <.text+0x198abda>
// Frames: F189 -> F190 -> F250
// 0x14198ba57  48c70001000000                 movq	$0x1, (%rax)
// 0x14198ba5e  48c7400801000000               movq	$0x1, 0x8(%rax)
// 0x14198ba66  c6401002                       movb	$0x2, 0x10(%rax)
// 0x14198ba6a  0fc6ffe1                       shufps	$0xe1, %xmm7, %xmm7     # xmm7 = xmm7[1,0,2,3]
// 0x14198ba6e  0f137814                       movlps	%xmm7, 0x14(%rax)
// 0x14198ba72  440fc6ce14                     shufps	$0x14, %xmm6, %xmm9     # xmm9 = xmm9[0,1],xmm6[1,0]
// 0x14198ba77  440f11481c                     movups	%xmm9, 0x1c(%rax)
// 0x14198ba7c  440f13402c                     movlps	%xmm8, 0x2c(%rax)
// 0x14198ba81  48897038                       movq	%rsi, 0x38(%rax)
// 0x14198ba85  48895840                       movq	%rbx, 0x40(%rax)
// 0x14198ba89  4c897048                       movq	%r14, 0x48(%rax)
// Frames: F250
// 0x14198ba8d  48898508010000                 movq	%rax, 0x108(%rbp)
// 0x14198ba94  48c7850001000005000000         movq	$0x5, 0x100(%rbp)
// 0x14198ba9f  4c8d05127dd600                 leaq	0xd67d12(%rip), %r8     # 0x1426f37b8
// 0x14198baa6  488d9500010000                 leaq	0x100(%rbp), %rdx
// 0x14198baad  4889f9                         movq	%rdi, %rcx
// 0x14198bab0  e8abbfc6fe                     callq	0x1405f7a60 <.text+0x5f6a60>
// Frames: F83 -> F258
// 0x14198bab5  488b85d0000000                 movq	0xd0(%rbp), %rax
// Frames: F149 -> F150 -> F83 -> F83 -> F258
// 0x14198babc  4885c0                         testq	%rax, %rax
// 0x14198babf  741a                           je	0x14198badb <.text+0x198aadb>
// Frames: F83 -> F258
// 0x14198bac1  488b8dd8000000                 movq	0xd8(%rbp), %rcx
// Frames: F153 -> F154 -> F150 -> F83 -> F83 -> F258
// 0x14198bac8  48c1e003                       shlq	$0x3, %rax
// 0x14198bacc  488d1440                       leaq	(%rax,%rax,2), %rdx
// Frames: F155 -> F156 -> F157 -> F83 -> F83 -> F258
// 0x14198bad0  41b808000000                   movl	$0x8, %r8d
// 0x14198bad6  e8b5eca4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F258
// 0x14198badb  488b95c8000000                 movq	0xc8(%rbp), %rdx
// 0x14198bae2  488b8de8000000                 movq	0xe8(%rbp), %rcx
// 0x14198bae9  e8c2ebffff                     callq	0x14198a6b0 <.text+0x19896b0>
// 0x14198baee  0f28b560010000                 movaps	0x160(%rbp), %xmm6
// 0x14198baf5  0f28bd70010000                 movaps	0x170(%rbp), %xmm7
// 0x14198bafc  440f288580010000               movaps	0x180(%rbp), %xmm8
// 0x14198bb04  440f288d90010000               movaps	0x190(%rbp), %xmm9
// 0x14198bb0c  440f2895a0010000               movaps	0x1a0(%rbp), %xmm10
// 0x14198bb14  440f289db0010000               movaps	0x1b0(%rbp), %xmm11
// 0x14198bb1c  440f28a5c0010000               movaps	0x1c0(%rbp), %xmm12
// 0x14198bb24  440f28add0010000               movaps	0x1d0(%rbp), %xmm13
// 0x14198bb2c  440f28b5e0010000               movaps	0x1e0(%rbp), %xmm14
// 0x14198bb34  4881c478020000                 addq	$0x278, %rsp            # imm = 0x278
// 0x14198bb3b  5b                             popq	%rbx
// 0x14198bb3c  5f                             popq	%rdi
// 0x14198bb3d  5e                             popq	%rsi
// 0x14198bb3e  415c                           popq	%r12
// 0x14198bb40  415d                           popq	%r13
// 0x14198bb42  415e                           popq	%r14
// 0x14198bb44  415f                           popq	%r15
// 0x14198bb46  5d                             popq	%rbp
// 0x14198bb47  c3                             retq
// Frames: F182
// 0x14198bb48  4c8d05817cd600                 leaq	0xd67c81(%rip), %r8     # 0x1426f37d0
// 0x14198bb4f  ba02000000                     movl	$0x2, %edx
// 0x14198bb54  4c89e1                         movq	%r12, %rcx
// 0x14198bb57  e8b4307200                     callq	0x1420aec10 <.text+0x20adc10>
// 0x14198bb5c  e988000000                     jmp	0x14198bbe9 <.text+0x198abe9>
// Frames: F259 -> F189 -> F190 -> F187
// 0x14198bb61  b908000000                     movl	$0x8, %ecx
// 0x14198bb66  ba30000000                     movl	$0x30, %edx
// 0x14198bb6b  e8f02b7200                     callq	0x1420ae760 <.text+0x20ad760>
// 0x14198bb70  eb77                           jmp	0x14198bbe9 <.text+0x198abe9>
// Frames: F260 -> F89 -> F90 -> F91 -> F92
// 0x14198bb72  b908000000                     movl	$0x8, %ecx
// 0x14198bb77  ba30000000                     movl	$0x30, %edx
// 0x14198bb7c  e8df2b7200                     callq	0x1420ae760 <.text+0x20ad760>
// 0x14198bb81  eb66                           jmp	0x14198bbe9 <.text+0x198abe9>
// Frames: F261 -> F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x14198bb83  488d0dc67ad600                 leaq	0xd67ac6(%rip), %rcx    # 0x1426f3650
// 0x14198bb8a  e8f12d7200                     callq	0x1420ae980 <.text+0x20ad980>
// 0x14198bb8f  31d2                           xorl	%edx, %edx
// Frames: F262 -> F160
// 0x14198bb91  48899500010000                 movq	%rdx, 0x100(%rbp)
// 0x14198bb98  48898508010000                 movq	%rax, 0x108(%rbp)
// 0x14198bb9f  488d05e27bd600                 leaq	0xd67be2(%rip), %rax    # 0x1426f3788
// 0x14198bba6  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14198bbab  488d0dfe78d600                 leaq	0xd678fe(%rip), %rcx    # 0x1426f34b0
// 0x14198bbb2  4c8d0d2779d600                 leaq	0xd67927(%rip), %r9     # 0x1426f34e0
// 0x14198bbb9  4c8d8500010000                 leaq	0x100(%rbp), %r8
// 0x14198bbc0  ba2b000000                     movl	$0x2b, %edx
// 0x14198bbc5  e856337200                     callq	0x1420aef20 <.text+0x20adf20>
// 0x14198bbca  eb1d                           jmp	0x14198bbe9 <.text+0x198abe9>
// Frames: F263 -> F166
// 0x14198bbcc  488d0dcd7bd600                 leaq	0xd67bcd(%rip), %rcx    # 0x1426f37a0
// 0x14198bbd3  e8682e7200                     callq	0x1420aea40 <.text+0x20ada40>
// 0x14198bbd8  eb0f                           jmp	0x14198bbe9 <.text+0x198abe9>
// Frames: F259 -> F189 -> F190 -> F250
// 0x14198bbda  b908000000                     movl	$0x8, %ecx
// 0x14198bbdf  ba50000000                     movl	$0x50, %edx
// 0x14198bbe4  e8772b7200                     callq	0x1420ae760 <.text+0x20ad760>
// 0x14198bbe9  0f0b                           ud2
// 0x14198bbeb  0f1f440000                     nopl	(%rax,%rax)
// 0x14198bbf0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198bbf5  55                             pushq	%rbp
// 0x14198bbf6  4157                           pushq	%r15
// 0x14198bbf8  4156                           pushq	%r14
// 0x14198bbfa  4155                           pushq	%r13
// 0x14198bbfc  4154                           pushq	%r12
// 0x14198bbfe  56                             pushq	%rsi
// 0x14198bbff  57                             pushq	%rdi
// 0x14198bc00  53                             pushq	%rbx
// 0x14198bc01  4881ecc8000000                 subq	$0xc8, %rsp
// 0x14198bc08  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14198bc0f  66440f29742430                 movapd	%xmm14, 0x30(%rsp)
// 0x14198bc16  440f296c2440                   movaps	%xmm13, 0x40(%rsp)
// 0x14198bc1c  440f29642450                   movaps	%xmm12, 0x50(%rsp)
// 0x14198bc22  440f295c2460                   movaps	%xmm11, 0x60(%rsp)
// 0x14198bc28  440f29542470                   movaps	%xmm10, 0x70(%rsp)
// 0x14198bc2e  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x14198bc37  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x14198bc40  0f29bc24a0000000               movaps	%xmm7, 0xa0(%rsp)
// 0x14198bc48  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// 0x14198bc50  488b8540010000                 movq	0x140(%rbp), %rax
// Frames: F74 -> F75 -> F76 -> F77 -> F82 -> F83 -> F83 -> F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x14198bc57  48ff00                         incq	(%rax)
// 0x14198bc5a  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x14198bc62  0f28bc24a0000000               movaps	0xa0(%rsp), %xmm7
// 0x14198bc6a  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x14198bc73  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x14198bc7c  440f28542470                   movaps	0x70(%rsp), %xmm10
// 0x14198bc82  440f285c2460                   movaps	0x60(%rsp), %xmm11
// 0x14198bc88  440f28642450                   movaps	0x50(%rsp), %xmm12
// 0x14198bc8e  440f286c2440                   movaps	0x40(%rsp), %xmm13
// 0x14198bc94  440f28742430                   movaps	0x30(%rsp), %xmm14
// Frames: F72 -> F73 -> F58 -> F59 -> F60 -> F2
// 0x14198bc9a  4881c4c8000000                 addq	$0xc8, %rsp
// 0x14198bca1  5b                             popq	%rbx
// 0x14198bca2  5f                             popq	%rdi
// 0x14198bca3  5e                             popq	%rsi
// 0x14198bca4  415c                           popq	%r12
// 0x14198bca6  415d                           popq	%r13
// 0x14198bca8  415e                           popq	%r14
// 0x14198bcaa  415f                           popq	%r15
// 0x14198bcac  5d                             popq	%rbp
// 0x14198bcad  c3                             retq
// 0x14198bcae  6690                           nop
// 0x14198bcb0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198bcb5  55                             pushq	%rbp
// 0x14198bcb6  4157                           pushq	%r15
// 0x14198bcb8  4156                           pushq	%r14
// 0x14198bcba  4155                           pushq	%r13
// 0x14198bcbc  4154                           pushq	%r12
// 0x14198bcbe  56                             pushq	%rsi
// 0x14198bcbf  57                             pushq	%rdi
// 0x14198bcc0  53                             pushq	%rbx
// 0x14198bcc1  4881ecc8000000                 subq	$0xc8, %rsp
// 0x14198bcc8  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14198bccf  66440f29742430                 movapd	%xmm14, 0x30(%rsp)
// 0x14198bcd6  440f296c2440                   movaps	%xmm13, 0x40(%rsp)
// 0x14198bcdc  440f29642450                   movaps	%xmm12, 0x50(%rsp)
// 0x14198bce2  440f295c2460                   movaps	%xmm11, 0x60(%rsp)
// 0x14198bce8  440f29542470                   movaps	%xmm10, 0x70(%rsp)
// 0x14198bcee  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x14198bcf7  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x14198bd00  0f29bc24a0000000               movaps	%xmm7, 0xa0(%rsp)
// 0x14198bd08  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// 0x14198bd10  488b8de8000000                 movq	0xe8(%rbp), %rcx
// 0x14198bd17  488b95c8000000                 movq	0xc8(%rbp), %rdx
// Frames: F258
// 0x14198bd1e  e88de9ffff                     callq	0x14198a6b0 <.text+0x19896b0>
// 0x14198bd23  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x14198bd2b  0f28bc24a0000000               movaps	0xa0(%rsp), %xmm7
// 0x14198bd33  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x14198bd3c  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x14198bd45  440f28542470                   movaps	0x70(%rsp), %xmm10
// 0x14198bd4b  440f285c2460                   movaps	0x60(%rsp), %xmm11
// 0x14198bd51  440f28642450                   movaps	0x50(%rsp), %xmm12
// 0x14198bd57  440f286c2440                   movaps	0x40(%rsp), %xmm13
// 0x14198bd5d  440f28742430                   movaps	0x30(%rsp), %xmm14
// Frames: F1
// 0x14198bd63  4881c4c8000000                 addq	$0xc8, %rsp
// 0x14198bd6a  5b                             popq	%rbx
// 0x14198bd6b  5f                             popq	%rdi
// 0x14198bd6c  5e                             popq	%rsi
// 0x14198bd6d  415c                           popq	%r12
// 0x14198bd6f  415d                           popq	%r13
// 0x14198bd71  415e                           popq	%r14
// 0x14198bd73  415f                           popq	%r15
// 0x14198bd75  5d                             popq	%rbp
// 0x14198bd76  c3                             retq
// 0x14198bd77  660f1f840000000000             nopw	(%rax,%rax)
// 0x14198bd80  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198bd85  55                             pushq	%rbp
// 0x14198bd86  4157                           pushq	%r15
// 0x14198bd88  4156                           pushq	%r14
// 0x14198bd8a  4155                           pushq	%r13
// 0x14198bd8c  4154                           pushq	%r12
// 0x14198bd8e  56                             pushq	%rsi
// 0x14198bd8f  57                             pushq	%rdi
// 0x14198bd90  53                             pushq	%rbx
// 0x14198bd91  4881ecc8000000                 subq	$0xc8, %rsp
// 0x14198bd98  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14198bd9f  66440f29742430                 movapd	%xmm14, 0x30(%rsp)
// 0x14198bda6  440f296c2440                   movaps	%xmm13, 0x40(%rsp)
// 0x14198bdac  440f29642450                   movaps	%xmm12, 0x50(%rsp)
// 0x14198bdb2  440f295c2460                   movaps	%xmm11, 0x60(%rsp)
// 0x14198bdb8  440f29542470                   movaps	%xmm10, 0x70(%rsp)
// 0x14198bdbe  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x14198bdc7  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x14198bdd0  0f29bc24a0000000               movaps	%xmm7, 0xa0(%rsp)
// 0x14198bdd8  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// Frames: F83 -> F258
// 0x14198bde0  488b85d0000000                 movq	0xd0(%rbp), %rax
// Frames: F149 -> F150 -> F83 -> F83 -> F258
// 0x14198bde7  4885c0                         testq	%rax, %rax
// 0x14198bdea  741a                           je	0x14198be06 <.text+0x198ae06>
// Frames: F83 -> F258
// 0x14198bdec  488b8dd8000000                 movq	0xd8(%rbp), %rcx
// Frames: F153 -> F154 -> F150 -> F83 -> F83 -> F258
// 0x14198bdf3  48c1e003                       shlq	$0x3, %rax
// 0x14198bdf7  488d1440                       leaq	(%rax,%rax,2), %rdx
// Frames: F155 -> F156 -> F157 -> F83 -> F83 -> F258
// 0x14198bdfb  41b808000000                   movl	$0x8, %r8d
// 0x14198be01  e88ae9a4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F258
// 0x14198be06  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x14198be0e  0f28bc24a0000000               movaps	0xa0(%rsp), %xmm7
// 0x14198be16  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x14198be1f  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x14198be28  440f28542470                   movaps	0x70(%rsp), %xmm10
// 0x14198be2e  440f285c2460                   movaps	0x60(%rsp), %xmm11
// 0x14198be34  440f28642450                   movaps	0x50(%rsp), %xmm12
// 0x14198be3a  440f286c2440                   movaps	0x40(%rsp), %xmm13
// 0x14198be40  66440f28742430                 movapd	0x30(%rsp), %xmm14
// 0x14198be47  4881c4c8000000                 addq	$0xc8, %rsp
// 0x14198be4e  5b                             popq	%rbx
// 0x14198be4f  5f                             popq	%rdi
// 0x14198be50  5e                             popq	%rsi
// 0x14198be51  415c                           popq	%r12
// 0x14198be53  415d                           popq	%r13
// 0x14198be55  415e                           popq	%r14
// 0x14198be57  415f                           popq	%r15
// 0x14198be59  5d                             popq	%rbp
// 0x14198be5a  c3                             retq
// 0x14198be5b  0f1f440000                     nopl	(%rax,%rax)
// 0x14198be60  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198be65  55                             pushq	%rbp
// 0x14198be66  4157                           pushq	%r15
// 0x14198be68  4156                           pushq	%r14
// 0x14198be6a  4155                           pushq	%r13
// 0x14198be6c  4154                           pushq	%r12
// 0x14198be6e  56                             pushq	%rsi
// 0x14198be6f  57                             pushq	%rdi
// 0x14198be70  53                             pushq	%rbx
// 0x14198be71  4881ecc8000000                 subq	$0xc8, %rsp
// 0x14198be78  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14198be7f  66440f29742430                 movapd	%xmm14, 0x30(%rsp)
// 0x14198be86  440f296c2440                   movaps	%xmm13, 0x40(%rsp)
// 0x14198be8c  440f29642450                   movaps	%xmm12, 0x50(%rsp)
// 0x14198be92  440f295c2460                   movaps	%xmm11, 0x60(%rsp)
// 0x14198be98  440f29542470                   movaps	%xmm10, 0x70(%rsp)
// 0x14198be9e  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x14198bea7  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x14198beb0  0f29bc24a0000000               movaps	%xmm7, 0xa0(%rsp)
// 0x14198beb8  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// Frames: F149 -> F150 -> F83 -> F151 -> F83 -> F152 -> F83 -> F98
// 0x14198bec0  4883bd3801000000               cmpq	$0x0, 0x138(%rbp)
// 0x14198bec8  741d                           je	0x14198bee7 <.text+0x198aee7>
// Frames: F153 -> F154 -> F150 -> F83 -> F151 -> F83 -> F152 -> F83 -> F98
// 0x14198beca  488b9538010000                 movq	0x138(%rbp), %rdx
// 0x14198bed1  48c1e203                       shlq	$0x3, %rdx
// Frames: F155 -> F156 -> F157 -> F83 -> F151 -> F83 -> F152 -> F83 -> F98
// 0x14198bed5  41b808000000                   movl	$0x8, %r8d
// 0x14198bedb  488b8d48010000                 movq	0x148(%rbp), %rcx
// 0x14198bee2  e8a9e8a4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F98
// 0x14198bee7  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x14198beef  0f28bc24a0000000               movaps	0xa0(%rsp), %xmm7
// 0x14198bef7  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x14198bf00  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x14198bf09  440f28542470                   movaps	0x70(%rsp), %xmm10
// 0x14198bf0f  440f285c2460                   movaps	0x60(%rsp), %xmm11
// 0x14198bf15  440f28642450                   movaps	0x50(%rsp), %xmm12
// 0x14198bf1b  440f286c2440                   movaps	0x40(%rsp), %xmm13
// 0x14198bf21  440f28742430                   movaps	0x30(%rsp), %xmm14
// 0x14198bf27  4881c4c8000000                 addq	$0xc8, %rsp
// 0x14198bf2e  5b                             popq	%rbx
// 0x14198bf2f  5f                             popq	%rdi
// 0x14198bf30  5e                             popq	%rsi
// 0x14198bf31  415c                           popq	%r12
// 0x14198bf33  415d                           popq	%r13
// 0x14198bf35  415e                           popq	%r14
// 0x14198bf37  415f                           popq	%r15
// 0x14198bf39  5d                             popq	%rbp
// 0x14198bf3a  c3                             retq
// 0x14198bf3b  0f1f440000                     nopl	(%rax,%rax)
// 0x14198bf40  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198bf45  55                             pushq	%rbp
// 0x14198bf46  4157                           pushq	%r15
// 0x14198bf48  4156                           pushq	%r14
// 0x14198bf4a  4155                           pushq	%r13
// 0x14198bf4c  4154                           pushq	%r12
// 0x14198bf4e  56                             pushq	%rsi
// 0x14198bf4f  57                             pushq	%rdi
// 0x14198bf50  53                             pushq	%rbx
// 0x14198bf51  4881ecc8000000                 subq	$0xc8, %rsp
// 0x14198bf58  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14198bf5f  66440f29742430                 movapd	%xmm14, 0x30(%rsp)
// 0x14198bf66  440f296c2440                   movaps	%xmm13, 0x40(%rsp)
// 0x14198bf6c  440f29642450                   movaps	%xmm12, 0x50(%rsp)
// 0x14198bf72  440f295c2460                   movaps	%xmm11, 0x60(%rsp)
// 0x14198bf78  440f29542470                   movaps	%xmm10, 0x70(%rsp)
// 0x14198bf7e  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x14198bf87  440f29842490000000             movaps	%xmm8, 0x90(%rsp)
// 0x14198bf90  0f29bc24a0000000               movaps	%xmm7, 0xa0(%rsp)
// 0x14198bf98  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// Frames: F149 -> F150 -> F83 -> F83 -> F83 -> F193
// 0x14198bfa0  4883bd4801000000               cmpq	$0x0, 0x148(%rbp)
// 0x14198bfa8  7419                           je	0x14198bfc3 <.text+0x198afc3>
// Frames: F155 -> F156 -> F157 -> F83 -> F83 -> F83 -> F193
// 0x14198bfaa  41b801000000                   movl	$0x1, %r8d
// 0x14198bfb0  488b8d40010000                 movq	0x140(%rbp), %rcx
// 0x14198bfb7  488b9548010000                 movq	0x148(%rbp), %rdx
// 0x14198bfbe  e8cde7a4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F193
// 0x14198bfc3  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x14198bfcb  0f28bc24a0000000               movaps	0xa0(%rsp), %xmm7
// 0x14198bfd3  440f28842490000000             movaps	0x90(%rsp), %xmm8
// 0x14198bfdc  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x14198bfe5  440f28542470                   movaps	0x70(%rsp), %xmm10
// 0x14198bfeb  440f285c2460                   movaps	0x60(%rsp), %xmm11
// 0x14198bff1  440f28642450                   movaps	0x50(%rsp), %xmm12
// 0x14198bff7  440f286c2440                   movaps	0x40(%rsp), %xmm13
// 0x14198bffd  66440f28742430                 movapd	0x30(%rsp), %xmm14
// 0x14198c004  4881c4c8000000                 addq	$0xc8, %rsp
// 0x14198c00b  5b                             popq	%rbx
// 0x14198c00c  5f                             popq	%rdi
// 0x14198c00d  5e                             popq	%rsi
// 0x14198c00e  415c                           popq	%r12
// 0x14198c010  415d                           popq	%r13
// 0x14198c012  415e                           popq	%r14
// 0x14198c014  415f                           popq	%r15
// 0x14198c016  5d                             popq	%rbp
// 0x14198c017  c3                             retq
//
// RANGE rva_198c020_len_27f: RVA 0x198c020, 639 PDB bytes; 152 instructions
// Evidence: _assembly/roof-v1/A/pack-003/rva_198c020_len_27f.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3199:13352: system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$1
// Frames: F264
// 0x14198c020  55                             pushq	%rbp
// 0x14198c021  4156                           pushq	%r14
// 0x14198c023  56                             pushq	%rsi
// 0x14198c024  57                             pushq	%rdi
// 0x14198c025  53                             pushq	%rbx
// 0x14198c026  4881ec90010000                 subq	$0x190, %rsp            # imm = 0x190
// 0x14198c02d  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14198c035  48c78508010000feffffff         movq	$-0x2, 0x108(%rbp)
// 0x14198c040  4c89cf                         movq	%r9, %rdi
// 0x14198c043  4c89c3                         movq	%r8, %rbx
// 0x14198c046  4989d6                         movq	%rdx, %r14
// 0x14198c049  4889ce                         movq	%rcx, %rsi
// Frames: F84 -> F85 -> F86 -> F87 -> F188 -> F265
// 0x14198c04c  0fb6056eea2e01                 movzbl	0x12eea6e(%rip), %eax   # 0x142c7aac1
// Frames: F93 -> F86 -> F87 -> F188 -> F265
// 0x14198c053  b908000000                     movl	$0x8, %ecx
// 0x14198c058  ba08000000                     movl	$0x8, %edx
// 0x14198c05d  e81ee7a4fe                     callq	0x1403da780 <.text+0x3d9780>
// Frames: F188 -> F265
// 0x14198c062  4885c0                         testq	%rax, %rax
// 0x14198c065  0f8488010000                   je	0x14198c1f3 <.text+0x198b1f3>
// Frames: F84 -> F85 -> F86 -> F87 -> F88 -> F89 -> F90 -> F266 -> F267 -> F268 -> F269 -> F270 -> F271 -> F272 -> F273 -> F274
// 0x14198c06b  488985f0000000                 movq	%rax, 0xf0(%rbp)
// 0x14198c072  0fb60548ea2e01                 movzbl	0x12eea48(%rip), %eax   # 0x142c7aac1
// Frames: F93 -> F86 -> F87 -> F88 -> F89 -> F90 -> F266 -> F267 -> F268 -> F269 -> F270 -> F271 -> F272 -> F273 -> F274
// 0x14198c079  b91e000000                     movl	$0x1e, %ecx
// 0x14198c07e  ba01000000                     movl	$0x1, %edx
// 0x14198c083  e8f8e6a4fe                     callq	0x1403da780 <.text+0x3d9780>
// Frames: F94 -> F89 -> F90 -> F266 -> F267 -> F268 -> F269 -> F270 -> F271 -> F272 -> F273 -> F274
// 0x14198c088  4885c0                         testq	%rax, %rax
// 0x14198c08b  0f8471010000                   je	0x14198c202 <.text+0x198b202>
// Frames: F275 -> F276 -> F277 -> F267 -> F268 -> F269 -> F270 -> F271 -> F272 -> F273 -> F274
// 0x14198c091  0f1005de77d600                 movups	0xd677de(%rip), %xmm0   # 0x1426f3876
// 0x14198c098  0f11400e                       movups	%xmm0, 0xe(%rax)
// 0x14198c09c  0f1005c577d600                 movups	0xd677c5(%rip), %xmm0   # 0x1426f3868
// 0x14198c0a3  0f1100                         movups	%xmm0, (%rax)
// Frames: F265
// 0x14198c0a6  48c745401e000000               movq	$0x1e, 0x40(%rbp)
// 0x14198c0ae  48894548                       movq	%rax, 0x48(%rbp)
// 0x14198c0b2  48c745501e000000               movq	$0x1e, 0x50(%rbp)
// 0x14198c0ba  48b80000000000000080           movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
// 0x14198c0c4  48894558                       movq	%rax, 0x58(%rbp)
// 0x14198c0c8  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x14198c0cc  488d5540                       leaq	0x40(%rbp), %rdx
// 0x14198c0d0  e87babccfe                     callq	0x140656c50 <.text+0x655c50>
// Frames: F278
// 0x14198c0d5  f3410f1016                     movss	(%r14), %xmm2
// 0x14198c0da  f30f58d2                       addss	%xmm2, %xmm2
// Frames: F265
// 0x14198c0de  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x14198c0e2  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x14198c0e6  e805aeccfe                     callq	0x140656ef0 <.text+0x655ef0>
// 0x14198c0eb  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x14198c0ef  488d5540                       leaq	0x40(%rbp), %rdx
// 0x14198c0f3  0f57d2                         xorps	%xmm2, %xmm2
// 0x14198c0f6  e825aeccfe                     callq	0x140656f20 <.text+0x655f20>
// Frames: F279
// 0x14198c0fb  f3410f10460c                   movss	0xc(%r14), %xmm0
// 0x14198c101  418b4618                       movl	0x18(%r14), %eax
// 0x14198c105  898500010000                   movl	%eax, 0x100(%rbp)
// 0x14198c10b  498b4610                       movq	0x10(%r14), %rax
// 0x14198c10f  488985f8000000                 movq	%rax, 0xf8(%rbp)
// 0x14198c116  f30f108d00010000               movss	0x100(%rbp), %xmm1
// Frames: F280 -> F281 -> F279
// 0x14198c11e  f30f590ddaec8200               mulss	0x82ecda(%rip), %xmm1   # 0x1421bae00
// Frames: F169 -> F282 -> F279
// 0x14198c126  f30f58c8                       addss	%xmm0, %xmm1
// Frames: F279
// 0x14198c12a  f20f1085f8000000               movsd	0xf8(%rbp), %xmm0
// Frames: F280 -> F283 -> F279
// 0x14198c132  0f5905a7ee8b00                 mulps	0x8beea7(%rip), %xmm0   # 0x14224afe0
// Frames: F279
// 0x14198c139  f2410f105604                   movsd	0x4(%r14), %xmm2
// Frames: F169 -> F171 -> F279
// 0x14198c13f  0f58c2                         addps	%xmm2, %xmm0
// Frames: F284 -> F279
// 0x14198c142  0f1385e0000000                 movlps	%xmm0, 0xe0(%rbp)
// 0x14198c149  f30f118de8000000               movss	%xmm1, 0xe8(%rbp)
// Frames: F265
// 0x14198c151  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x14198c155  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x14198c159  4c8d85e0000000                 leaq	0xe0(%rbp), %r8
// 0x14198c160  e82badccfe                     callq	0x140656e90 <.text+0x655e90>
// Frames: F285
// 0x14198c165  c6850701000001                 movb	$0x1, 0x107(%rbp)
// 0x14198c16c  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x14198c170  488d95f8000000                 leaq	0xf8(%rbp), %rdx
// 0x14198c177  e884dbe4fe                     callq	0x1407d9d00 <.text+0x7d8d00>
// 0x14198c17c  0f2845b0                       movaps	-0x50(%rbp), %xmm0
// Frames: F265
// 0x14198c180  0f2985d0000000                 movaps	%xmm0, 0xd0(%rbp)
// 0x14198c187  c6850701000000                 movb	$0x0, 0x107(%rbp)
// 0x14198c18e  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x14198c192  488d5540                       leaq	0x40(%rbp), %rdx
// 0x14198c196  4c8d85d0000000                 leaq	0xd0(%rbp), %r8
// 0x14198c19d  e81eadccfe                     callq	0x140656ec0 <.text+0x655ec0>
// 0x14198c1a2  488b8560010000                 movq	0x160(%rbp), %rax
// 0x14198c1a9  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14198c1ae  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x14198c1b2  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x14198c1b6  4989d8                         movq	%rbx, %r8
// 0x14198c1b9  4989f9                         movq	%rdi, %r9
// 0x14198c1bc  e81faeccfe                     callq	0x140656fe0 <.text+0x655fe0>
// 0x14198c1c1  488b4560                       movq	0x60(%rbp), %rax
// 0x14198c1c5  488b8df0000000                 movq	0xf0(%rbp), %rcx
// 0x14198c1cc  488901                         movq	%rax, (%rcx)
// Frames: F286 -> F287 -> F288 -> F265
// 0x14198c1cf  48c70601000000                 movq	$0x1, (%rsi)
// 0x14198c1d6  48894e08                       movq	%rcx, 0x8(%rsi)
// 0x14198c1da  48c7461001000000               movq	$0x1, 0x10(%rsi)
// Frames: F289
// 0x14198c1e2  4889f0                         movq	%rsi, %rax
// 0x14198c1e5  4881c490010000                 addq	$0x190, %rsp            # imm = 0x190
// 0x14198c1ec  5b                             popq	%rbx
// 0x14198c1ed  5f                             popq	%rdi
// 0x14198c1ee  5e                             popq	%rsi
// 0x14198c1ef  415e                           popq	%r14
// 0x14198c1f1  5d                             popq	%rbp
// 0x14198c1f2  c3                             retq
// Frames: F259 -> F265
// 0x14198c1f3  b908000000                     movl	$0x8, %ecx
// 0x14198c1f8  ba08000000                     movl	$0x8, %edx
// 0x14198c1fd  e85e257200                     callq	0x1420ae760 <.text+0x20ad760>
// Frames: F260 -> F89 -> F90 -> F266 -> F267 -> F268 -> F269 -> F270 -> F271 -> F272 -> F273 -> F274
// 0x14198c202  b901000000                     movl	$0x1, %ecx
// 0x14198c207  ba1e000000                     movl	$0x1e, %edx
// 0x14198c20c  e84f257200                     callq	0x1420ae760 <.text+0x20ad760>
// 0x14198c211  0f0b                           ud2
// 0x14198c213  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x14198c220  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198c225  55                             pushq	%rbp
// 0x14198c226  4156                           pushq	%r14
// 0x14198c228  56                             pushq	%rsi
// 0x14198c229  57                             pushq	%rdi
// 0x14198c22a  53                             pushq	%rbx
// 0x14198c22b  4883ec30                       subq	$0x30, %rsp
// 0x14198c22f  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F155 -> F156 -> F290 -> F83 -> F265
// 0x14198c236  ba08000000                     movl	$0x8, %edx
// 0x14198c23b  41b808000000                   movl	$0x8, %r8d
// 0x14198c241  488b8df0000000                 movq	0xf0(%rbp), %rcx
// 0x14198c248  e843e5a4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F264
// 0x14198c24d  90                             nop
// 0x14198c24e  4883c430                       addq	$0x30, %rsp
// 0x14198c252  5b                             popq	%rbx
// 0x14198c253  5f                             popq	%rdi
// 0x14198c254  5e                             popq	%rsi
// 0x14198c255  415e                           popq	%r14
// 0x14198c257  5d                             popq	%rbp
// 0x14198c258  c3                             retq
// 0x14198c259  0f1f8000000000                 nopl	(%rax)
// 0x14198c260  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198c265  55                             pushq	%rbp
// 0x14198c266  4156                           pushq	%r14
// 0x14198c268  56                             pushq	%rsi
// 0x14198c269  57                             pushq	%rdi
// 0x14198c26a  53                             pushq	%rbx
// 0x14198c26b  4883ec30                       subq	$0x30, %rsp
// 0x14198c26f  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F285
// 0x14198c276  80bd0701000000                 cmpb	$0x0, 0x107(%rbp)
// 0x14198c27d  7415                           je	0x14198c294 <.text+0x198b294>
// 0x14198c27f  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x14198c283  e80838f4fe                     callq	0x1408cfa90 <.text+0x8cea90>
// 0x14198c288  90                             nop
// 0x14198c289  4883c430                       addq	$0x30, %rsp
// 0x14198c28d  5b                             popq	%rbx
// 0x14198c28e  5f                             popq	%rdi
// 0x14198c28f  5e                             popq	%rsi
// 0x14198c290  415e                           popq	%r14
// 0x14198c292  5d                             popq	%rbp
// 0x14198c293  c3                             retq
// 0x14198c294  4883c430                       addq	$0x30, %rsp
// 0x14198c298  5b                             popq	%rbx
// 0x14198c299  5f                             popq	%rdi
// 0x14198c29a  5e                             popq	%rsi
// 0x14198c29b  415e                           popq	%r14
// 0x14198c29d  5d                             popq	%rbp
// 0x14198c29e  c3                             retq
//
// RANGE rva_198c2a0_len_c5d: RVA 0x198c2a0, 3165 PDB bytes; 626 instructions
// Evidence: _assembly/roof-v1/A/pack-003/rva_198c2a0_len_c5d.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3199:14616: system_roof::ui_edit_roof_ridge_length::create_ui_widget
// Frames: F291
// 0x14198c2a0  55                             pushq	%rbp
// 0x14198c2a1  4157                           pushq	%r15
// 0x14198c2a3  4156                           pushq	%r14
// 0x14198c2a5  4155                           pushq	%r13
// 0x14198c2a7  4154                           pushq	%r12
// 0x14198c2a9  56                             pushq	%rsi
// 0x14198c2aa  57                             pushq	%rdi
// 0x14198c2ab  53                             pushq	%rbx
// 0x14198c2ac  4881ec98030000                 subq	$0x398, %rsp            # imm = 0x398
// 0x14198c2b3  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14198c2bb  440f29b500030000               movaps	%xmm14, 0x300(%rbp)
// 0x14198c2c3  440f29adf0020000               movaps	%xmm13, 0x2f0(%rbp)
// 0x14198c2cb  440f29a5e0020000               movaps	%xmm12, 0x2e0(%rbp)
// 0x14198c2d3  440f299dd0020000               movaps	%xmm11, 0x2d0(%rbp)
// 0x14198c2db  440f2995c0020000               movaps	%xmm10, 0x2c0(%rbp)
// 0x14198c2e3  440f298db0020000               movaps	%xmm9, 0x2b0(%rbp)
// 0x14198c2eb  440f2985a0020000               movaps	%xmm8, 0x2a0(%rbp)
// 0x14198c2f3  0f29bd90020000                 movaps	%xmm7, 0x290(%rbp)
// 0x14198c2fa  0f29b580020000                 movaps	%xmm6, 0x280(%rbp)
// 0x14198c301  48c78578020000feffffff         movq	$-0x2, 0x278(%rbp)
// 0x14198c30c  4c89cb                         movq	%r9, %rbx
// 0x14198c30f  4c898550020000                 movq	%r8, 0x250(%rbp)
// 0x14198c316  4889d7                         movq	%rdx, %rdi
// 0x14198c319  4989cf                         movq	%rcx, %r15
// Frames: F292
// 0x14198c31c  e8aff3c0fe                     callq	0x14059b6d0 <.text+0x59a6d0>
// 0x14198c321  48898550010000                 movq	%rax, 0x150(%rbp)
// 0x14198c328  488d8d8c000000                 leaq	0x8c(%rbp), %rcx
// 0x14198c32f  488d9550010000                 leaq	0x150(%rbp), %rdx
// 0x14198c336  4d89f8                         movq	%r15, %r8
// 0x14198c339  e852f4c0fe                     callq	0x14059b790 <.text+0x59a790>
// Frames: F293
// 0x14198c33e  4c89f9                         movq	%r15, %rcx
// 0x14198c341  e8dae0d2fe                     callq	0x1406ba420 <.text+0x6b9420>
// 0x14198c346  84c0                           testb	%al, %al
// 0x14198c348  0f8477060000                   je	0x14198c9c5 <.text+0x198b9c5>
// 0x14198c34e  41837f0800                     cmpl	$0x0, 0x8(%r15)
// 0x14198c353  0f846c060000                   je	0x14198c9c5 <.text+0x198b9c5>
// Frames: F294
// 0x14198c359  498b471c                       movq	0x1c(%r15), %rax
// 0x14198c35d  48898510020000                 movq	%rax, 0x210(%rbp)
// 0x14198c364  410f10470c                     movups	0xc(%r15), %xmm0
// 0x14198c369  0f298500020000                 movaps	%xmm0, 0x200(%rbp)
// Frames: F295
// 0x14198c370  83bd8c00000000                 cmpl	$0x0, 0x8c(%rbp)
// 0x14198c377  488d8590000000                 leaq	0x90(%rbp), %rax
// 0x14198c37e  0f84a8000000                   je	0x14198c42c <.text+0x198b42c>
// Frames: F296
// 0x14198c384  488b4810                       movq	0x10(%rax), %rcx
// 0x14198c388  48898d40020000                 movq	%rcx, 0x240(%rbp)
// 0x14198c38f  0f1000                         movups	(%rax), %xmm0
// 0x14198c392  0f298530020000                 movaps	%xmm0, 0x230(%rbp)
// Frames: F297
// 0x14198c399  41807f3c00                     cmpb	$0x0, 0x3c(%r15)
// 0x14198c39e  0f84b9000000                   je	0x14198c45d <.text+0x198b45d>
// Frames: F298
// 0x14198c3a4  f30f108500020000               movss	0x200(%rbp), %xmm0
// Frames: F299 -> F298
// 0x14198c3ac  f30f108d04020000               movss	0x204(%rbp), %xmm1
// Frames: F300 -> F301 -> F298
// 0x14198c3b4  440f2805147e7f00               movaps	0x7f7e14(%rip), %xmm8   # 0x1421841d0
// Frames: F299 -> F298
// 0x14198c3bc  410f57c8                       xorps	%xmm8, %xmm1
// Frames: F298
// 0x14198c3c0  f30f114db0                     movss	%xmm1, -0x50(%rbp)
// 0x14198c3c5  f30f1145b4                     movss	%xmm0, -0x4c(%rbp)
// 0x14198c3ca  488d8d50010000                 leaq	0x150(%rbp), %rcx
// 0x14198c3d1  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x14198c3d5  0f57d2                         xorps	%xmm2, %xmm2
// 0x14198c3d8  e8a3e3e4fe                     callq	0x1407da780 <.text+0x7d9780>
// Frames: F301 -> F298
// 0x14198c3dd  f20f10b550010000               movsd	0x150(%rbp), %xmm6
// Frames: F300 -> F301 -> F298
// 0x14198c3e5  410f57f0                       xorps	%xmm8, %xmm6
// Frames: F300 -> F302 -> F298
// 0x14198c3e9  f30f10bd58010000               movss	0x158(%rbp), %xmm7
// 0x14198c3f1  410f57f8                       xorps	%xmm8, %xmm7
// Frames: F303
// 0x14198c3f5  f30f108500020000               movss	0x200(%rbp), %xmm0
// Frames: F299 -> F303
// 0x14198c3fd  f30f108d04020000               movss	0x204(%rbp), %xmm1
// 0x14198c405  410f57c8                       xorps	%xmm8, %xmm1
// Frames: F303
// 0x14198c409  f30f118d50010000               movss	%xmm1, 0x150(%rbp)
// 0x14198c411  f30f118554010000               movss	%xmm0, 0x154(%rbp)
// 0x14198c419  488d8d20020000                 leaq	0x220(%rbp), %rcx
// 0x14198c420  488d9550010000                 leaq	0x150(%rbp), %rdx
// 0x14198c427  0f57d2                         xorps	%xmm2, %xmm2
// 0x14198c42a  eb74                           jmp	0x14198c4a0 <.text+0x198b4a0>
// Frames: F304
// 0x14198c42c  8b4808                         movl	0x8(%rax), %ecx
// 0x14198c42f  898d44020000                   movl	%ecx, 0x244(%rbp)
// 0x14198c435  488b08                         movq	(%rax), %rcx
// 0x14198c438  48898d3c020000                 movq	%rcx, 0x23c(%rbp)
// 0x14198c43f  488b08                         movq	(%rax), %rcx
// 0x14198c442  48898d30020000                 movq	%rcx, 0x230(%rbp)
// 0x14198c449  8b4008                         movl	0x8(%rax), %eax
// 0x14198c44c  898538020000                   movl	%eax, 0x238(%rbp)
// Frames: F297
// 0x14198c452  41807f3c00                     cmpb	$0x0, 0x3c(%r15)
// 0x14198c457  0f8547ffffff                   jne	0x14198c3a4 <.text+0x198b3a4>
// Frames: F305
// 0x14198c45d  488d8d50010000                 leaq	0x150(%rbp), %rcx
// 0x14198c464  488db500020000                 leaq	0x200(%rbp), %rsi
// 0x14198c46b  0f57d2                         xorps	%xmm2, %xmm2
// 0x14198c46e  4889f2                         movq	%rsi, %rdx
// 0x14198c471  e80ae3e4fe                     callq	0x1407da780 <.text+0x7d9780>
// Frames: F301 -> F305
// 0x14198c476  f20f10b550010000               movsd	0x150(%rbp), %xmm6
// Frames: F300 -> F301 -> F305
// 0x14198c47e  0f28054b7d7f00                 movaps	0x7f7d4b(%rip), %xmm0   # 0x1421841d0
// 0x14198c485  0f57f0                         xorps	%xmm0, %xmm6
// Frames: F300 -> F302 -> F305
// 0x14198c488  f30f10bd58010000               movss	0x158(%rbp), %xmm7
// 0x14198c490  0f57f8                         xorps	%xmm0, %xmm7
// 0x14198c493  488d8d20020000                 leaq	0x220(%rbp), %rcx
// Frames: F305
// 0x14198c49a  0f57d2                         xorps	%xmm2, %xmm2
// 0x14198c49d  4889f2                         movq	%rsi, %rdx
// Frames: F84 -> F85 -> F86 -> F87 -> F88 -> F89 -> F90 -> F91 -> F306
// 0x14198c4a0  e8dbe2e4fe                     callq	0x1407da780 <.text+0x7d9780>
// 0x14198c4a5  48899df8010000                 movq	%rbx, 0x1f8(%rbp)
// 0x14198c4ac  4889bdf0010000                 movq	%rdi, 0x1f0(%rbp)
// 0x14198c4b3  0fb60507e62e01                 movzbl	0x12ee607(%rip), %eax   # 0x142c7aac1
// Frames: F93 -> F86 -> F87 -> F88 -> F89 -> F90 -> F91 -> F306
// 0x14198c4ba  b930000000                     movl	$0x30, %ecx
// 0x14198c4bf  ba08000000                     movl	$0x8, %edx
// 0x14198c4c4  e8b7e2a4fe                     callq	0x1403da780 <.text+0x3d9780>
// Frames: F94 -> F89 -> F90 -> F91 -> F306
// 0x14198c4c9  4885c0                         testq	%rax, %rax
// 0x14198c4cc  0f849d050000                   je	0x14198ca6f <.text+0x198ba6f>
// Frames: F90 -> F91 -> F306
// 0x14198c4d2  0fb68d88030000                 movzbl	0x388(%rbp), %ecx
// 0x14198c4d9  48c7855802000002000000         movq	$0x2, 0x258(%rbp)
// 0x14198c4e4  48898560020000                 movq	%rax, 0x260(%rbp)
// 0x14198c4eb  48c7856802000000000000         movq	$0x0, 0x268(%rbp)
// Frames: F307
// 0x14198c4f6  488b8540020000                 movq	0x240(%rbp), %rax
// 0x14198c4fd  488985c8000000                 movq	%rax, 0xc8(%rbp)
// 0x14198c504  0f288530020000                 movaps	0x230(%rbp), %xmm0
// 0x14198c50b  0f1185b8000000                 movups	%xmm0, 0xb8(%rbp)
// Frames: F308 -> F309 -> F310 -> F307
// 0x14198c512  0f57c0                         xorps	%xmm0, %xmm0
// 0x14198c515  0f1185f8000000                 movups	%xmm0, 0xf8(%rbp)
// 0x14198c51c  48c7850801000000000000         movq	$0x0, 0x108(%rbp)
// Frames: F307
// 0x14198c527  48c785a800000000000000         movq	$0x0, 0xa8(%rbp)
// 0x14198c532  48c785b000000002000000         movq	$0x2, 0xb0(%rbp)
// 0x14198c53d  48c785d800000002000000         movq	$0x2, 0xd8(%rbp)
// 0x14198c548  0f13b5e0000000                 movlps	%xmm6, 0xe0(%rbp)
// 0x14198c54f  f30f11bde8000000               movss	%xmm7, 0xe8(%rbp)
// 0x14198c557  488b8520020000                 movq	0x220(%rbp), %rax
// 0x14198c55e  488985ec000000                 movq	%rax, 0xec(%rbp)
// 0x14198c565  8b8528020000                   movl	0x228(%rbp), %eax
// 0x14198c56b  8985f4000000                   movl	%eax, 0xf4(%rbp)
// 0x14198c571  80f101                         xorb	$0x1, %cl
// 0x14198c574  888d74020000                   movb	%cl, 0x274(%rbp)
// 0x14198c57a  31c0                           xorl	%eax, %eax
// 0x14198c57c  4c8d6db0                       leaq	-0x50(%rbp), %r13
// 0x14198c580  488d7d48                       leaq	0x48(%rbp), %rdi
// 0x14198c584  f3440f1005f3e78200             movss	0x82e7f3(%rip), %xmm8   # 0x1421bad80
// 0x14198c58d  440f280ddb829d00               movaps	0x9d82db(%rip), %xmm9   # 0x142364870
// 0x14198c595  f30f103523c27b00               movss	0x7bc223(%rip), %xmm6   # 0x1421487c0
// 0x14198c59d  f30f103d9f0a8300               movss	0x830a9f(%rip), %xmm7   # 0x1421bd044
// 0x14198c5a5  f3440f1015e6fe8b00             movss	0x8bfee6(%rip), %xmm10  # 0x14224c494
// 0x14198c5ae  31c9                           xorl	%ecx, %ecx
// 0x14198c5b0  31d2                           xorl	%edx, %edx
// 0x14198c5b2  31db                           xorl	%ebx, %ebx
// 0x14198c5b4  eb47                           jmp	0x14198c5fd <.text+0x198b5fd>
// 0x14198c5b6  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F228 -> F229 -> F227 -> F311
// 0x14198c5c0  48ffc3                         incq	%rbx
// 0x14198c5c3  488b8560020000                 movq	0x260(%rbp), %rax
// Frames: F230 -> F227 -> F311
// 0x14198c5ca  488d0c52                       leaq	(%rdx,%rdx,2), %rcx
// Frames: F74 -> F231 -> F311
// 0x14198c5ce  488934c8                       movq	%rsi, (%rax,%rcx,8)
// 0x14198c5d2  f3440f116cc808                 movss	%xmm13, 0x8(%rax,%rcx,8)
// 0x14198c5d9  440f135cc80c                   movlps	%xmm11, 0xc(%rax,%rcx,8)
// 0x14198c5df  f3440f1164c814                 movss	%xmm12, 0x14(%rax,%rcx,8)
// Frames: F232 -> F311
// 0x14198c5e6  488b8568020000                 movq	0x268(%rbp), %rax
// 0x14198c5ed  48ffc0                         incq	%rax
// 0x14198c5f0  48898568020000                 movq	%rax, 0x268(%rbp)
// 0x14198c5f7  4c89f1                         movq	%r14, %rcx
// 0x14198c5fa  4c89e2                         movq	%r12, %rdx
// Frames: F312 -> F313 -> F314 -> F315 -> F179 -> F307
// 0x14198c5fd  4883fb02                       cmpq	$0x2, %rbx
// 0x14198c601  0f84ad020000                   je	0x14198c8b4 <.text+0x198b8b4>
// Frames: F312 -> F313 -> F316 -> F315 -> F179 -> F307
// 0x14198c607  4883fa02                       cmpq	$0x2, %rdx
// 0x14198c60b  0f84a3020000                   je	0x14198c8b4 <.text+0x198b8b4>
// Frames: F313 -> F317 -> F315 -> F179 -> F307
// 0x14198c611  488d045b                       leaq	(%rbx,%rbx,2), %rax
// 0x14198c615  4c8d85b8000000                 leaq	0xb8(%rbp), %r8
// 0x14198c61c  f2450f103480                   movsd	(%r8,%rax,4), %xmm14
// 0x14198c622  f3450f106c8008                 movss	0x8(%r8,%rax,4), %xmm13
// Frames: F318 -> F319 -> F313 -> F316 -> F315 -> F179 -> F307
// 0x14198c629  4c8d6201                       leaq	0x1(%rdx), %r12
// 0x14198c62d  4c89a5d0000000                 movq	%r12, 0xd0(%rbp)
// Frames: F19 -> F320 -> F321 -> F322 -> F323 -> F313 -> F316 -> F315 -> F179 -> F307
// 0x14198c634  488d0452                       leaq	(%rdx,%rdx,2), %rax
// Frames: F323 -> F313 -> F316 -> F315 -> F179 -> F307
// 0x14198c638  f2450f105c8028                 movsd	0x28(%r8,%rax,4), %xmm11
// 0x14198c63f  f3450f10648030                 movss	0x30(%r8,%rax,4), %xmm12
// Frames: F324 -> F307
// 0x14198c646  4c8d7101                       leaq	0x1(%rcx), %r14
// 0x14198c64a  4c89b510010000                 movq	%r14, 0x110(%rbp)
// Frames: F307
// 0x14198c651  48898de8010000                 movq	%rcx, 0x1e8(%rbp)
// Frames: F325
// 0x14198c658  f2440f115d48                   movsd	%xmm11, 0x48(%rbp)
// 0x14198c65e  f3440f116550                   movss	%xmm12, 0x50(%rbp)
// 0x14198c664  4c89e9                         movq	%r13, %rcx
// 0x14198c667  4889fe                         movq	%rdi, %rsi
// 0x14198c66a  4889fa                         movq	%rdi, %rdx
// 0x14198c66d  e88ed6e4fe                     callq	0x1407d9d00 <.text+0x7d8d00>
// 0x14198c672  488dbd50010000                 leaq	0x150(%rbp), %rdi
// Frames: F280 -> F281 -> F326
// 0x14198c679  410f28c4                       movaps	%xmm12, %xmm0
// 0x14198c67d  f3410f59c0                     mulss	%xmm8, %xmm0
// Frames: F169 -> F282 -> F326
// 0x14198c682  f3440f58e8                     addss	%xmm0, %xmm13
// Frames: F325
// 0x14198c687  0f2845b0                       movaps	-0x50(%rbp), %xmm0
// Frames: F280 -> F283 -> F326
// 0x14198c68b  410f28cb                       movaps	%xmm11, %xmm1
// 0x14198c68f  410f59c9                       mulps	%xmm9, %xmm1
// Frames: F169 -> F171 -> F326
// 0x14198c693  440f58f1                       addps	%xmm1, %xmm14
// Frames: F327
// 0x14198c697  440f13b560010000               movlps	%xmm14, 0x160(%rbp)
// 0x14198c69f  f3440f11ad68010000             movss	%xmm13, 0x168(%rbp)
// 0x14198c6a8  0f298550010000                 movaps	%xmm0, 0x150(%rbp)
// 0x14198c6af  488d856c010000                 leaq	0x16c(%rbp), %rax
// 0x14198c6b6  48b90000803f0000803f           movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
// 0x14198c6c0  488908                         movq	%rcx, (%rax)
// 0x14198c6c3  c740080000803f                 movl	$0x3f800000, 0x8(%rax)  # imm = 0x3F800000
// Frames: F328
// 0x14198c6ca  48897c2420                     movq	%rdi, 0x20(%rsp)
// 0x14198c6cf  488d8d18010000                 leaq	0x118(%rbp), %rcx
// 0x14198c6d6  0f28ce                         movaps	%xmm6, %xmm1
// 0x14198c6d9  0f28d6                         movaps	%xmm6, %xmm2
// 0x14198c6dc  0f28df                         movaps	%xmm7, %xmm3
// 0x14198c6df  e81c52bcfe                     callq	0x140551900 <.text+0x550900>
// Frames: F329
// 0x14198c6e4  4c897db0                       movq	%r15, -0x50(%rbp)
// 0x14198c6e8  488d05c1e4ffff                 leaq	-0x1b3f(%rip), %rax     # 0x14198abb0 <.text+0x1989bb0>
// 0x14198c6ef  488945b8                       movq	%rax, -0x48(%rbp)
// 0x14198c6f3  488d85e8010000                 leaq	0x1e8(%rbp), %rax
// 0x14198c6fa  488945c0                       movq	%rax, -0x40(%rbp)
// 0x14198c6fe  488d053b566ffe                 leaq	-0x190a9c5(%rip), %rax  # 0x140081d40 <.text+0x80d40>
// 0x14198c705  488945c8                       movq	%rax, -0x38(%rbp)
// Frames: F184 -> F185 -> F186 -> F329
// 0x14198c709  488d05b871d600                 leaq	0xd671b8(%rip), %rax    # 0x1426f38c8
// 0x14198c710  48898550010000                 movq	%rax, 0x150(%rbp)
// 0x14198c717  48c7855801000002000000         movq	$0x2, 0x158(%rbp)
// 0x14198c722  4c89ad60010000                 movq	%r13, 0x160(%rbp)
// 0x14198c729  48c7856801000002000000         movq	$0x2, 0x168(%rbp)
// 0x14198c734  48c7857001000000000000         movq	$0x0, 0x170(%rbp)
// 0x14198c73f  c6857702000001                 movb	$0x1, 0x277(%rbp)
// 0x14198c746  4889f1                         movq	%rsi, %rcx
// 0x14198c749  4889fa                         movq	%rdi, %rdx
// 0x14198c74c  4889f7                         movq	%rsi, %rdi
// 0x14198c74f  e8ccd0edfe                     callq	0x140869820 <.text+0x868820>
// Frames: F329
// 0x14198c754  0f104548                       movups	0x48(%rbp), %xmm0
// 0x14198c758  0f298530010000                 movaps	%xmm0, 0x130(%rbp)
// 0x14198c75f  488b4558                       movq	0x58(%rbp), %rax
// 0x14198c763  48898540010000                 movq	%rax, 0x140(%rbp)
// Frames: F330
// 0x14198c76a  488b8528010000                 movq	0x128(%rbp), %rax
// 0x14198c771  488d4db8                       leaq	-0x48(%rbp), %rcx
// 0x14198c775  48894110                       movq	%rax, 0x10(%rcx)
// 0x14198c779  0f108518010000                 movups	0x118(%rbp), %xmm0
// 0x14198c780  0f1101                         movups	%xmm0, (%rcx)
// 0x14198c783  c645b002                       movb	$0x2, -0x50(%rbp)
// 0x14198c787  c6454800                       movb	$0x0, 0x48(%rbp)
// 0x14198c78b  c6857602000001                 movb	$0x1, 0x276(%rbp)
// 0x14198c792  488db550010000                 leaq	0x150(%rbp), %rsi
// 0x14198c799  4889f1                         movq	%rsi, %rcx
// 0x14198c79c  4c89ea                         movq	%r13, %rdx
// 0x14198c79f  4989f8                         movq	%rdi, %r8
// 0x14198c7a2  e879fad2fe                     callq	0x1406bc220 <.text+0x6bb220>
// Frames: F331
// 0x14198c7a7  c6857502000001                 movb	$0x1, 0x275(%rbp)
// 0x14198c7ae  4c89e9                         movq	%r13, %rcx
// 0x14198c7b1  e8bafbd2fe                     callq	0x1406bc370 <.text+0x6bb370>
// 0x14198c7b6  c6857502000001                 movb	$0x1, 0x275(%rbp)
// 0x14198c7bd  41b801000000                   movl	$0x1, %r8d
// 0x14198c7c3  41b902000000                   movl	$0x2, %r9d
// 0x14198c7c9  4889f9                         movq	%rdi, %rcx
// 0x14198c7cc  4c89ea                         movq	%r13, %rdx
// 0x14198c7cf  e8fcfbd2fe                     callq	0x1406bc3d0 <.text+0x6bb3d0>
// Frames: F330
// 0x14198c7d4  c6857502000000                 movb	$0x0, 0x275(%rbp)
// 0x14198c7db  4c89e9                         movq	%r13, %rcx
// 0x14198c7de  4889f2                         movq	%rsi, %rdx
// 0x14198c7e1  4989f8                         movq	%rdi, %r8
// 0x14198c7e4  e8a7fad2fe                     callq	0x1406bc290 <.text+0x6bb290>
// 0x14198c7e9  c6857602000001                 movb	$0x1, 0x276(%rbp)
// 0x14198c7f0  4889f1                         movq	%rsi, %rcx
// 0x14198c7f3  4c89ea                         movq	%r13, %rdx
// 0x14198c7f6  440fb68574020000               movzbl	0x274(%rbp), %r8d
// 0x14198c7fe  e8cdfad2fe                     callq	0x1406bc2d0 <.text+0x6bb2d0>
// Frames: F332
// 0x14198c803  c6857602000000                 movb	$0x0, 0x276(%rbp)
// 0x14198c80a  488b8d50020000                 movq	0x250(%rbp), %rcx
// 0x14198c811  488d9530010000                 leaq	0x130(%rbp), %rdx
// 0x14198c818  4989f0                         movq	%rsi, %r8
// 0x14198c81b  e8d01d36ff                     callq	0x140cee5f0 <.text+0xced5f0>
// 0x14198c820  488945b0                       movq	%rax, -0x50(%rbp)
// 0x14198c824  488955b8                       movq	%rdx, -0x48(%rbp)
// 0x14198c828  c6857702000000                 movb	$0x0, 0x277(%rbp)
// Frames: F333
// 0x14198c82f  4c89e9                         movq	%r13, %rcx
// 0x14198c832  e8399bcffe                     callq	0x140686370 <.text+0x685370>
// 0x14198c837  888550010000                   movb	%al, 0x150(%rbp)
// 0x14198c83d  c6857702000000                 movb	$0x0, 0x277(%rbp)
// 0x14198c844  488d8d50010000                 leaq	0x150(%rbp), %rcx
// 0x14198c84b  e81015befe                     callq	0x14056dd60 <.text+0x56cd60>
// 0x14198c850  84c0                           testb	%al, %al
// 0x14198c852  7517                           jne	0x14198c86b <.text+0x198b86b>
// 0x14198c854  c6857702000000                 movb	$0x0, 0x277(%rbp)
// 0x14198c85b  4c89e9                         movq	%r13, %rcx
// 0x14198c85e  e82d9bcffe                     callq	0x140686390 <.text+0x685390>
// 0x14198c863  440f28ee                       movaps	%xmm6, %xmm13
// 0x14198c867  3c02                           cmpb	$0x2, %al
// 0x14198c869  7404                           je	0x14198c86f <.text+0x198b86f>
// 0x14198c86b  450f28ea                       movaps	%xmm10, %xmm13
// Frames: F311
// 0x14198c86f  c6857702000000                 movb	$0x0, 0x277(%rbp)
// 0x14198c876  4c89e9                         movq	%r13, %rcx
// 0x14198c879  e89229abfe                     callq	0x14043f210 <.text+0x43e210>
// 0x14198c87e  4889c6                         movq	%rax, %rsi
// Frames: F223 -> F311
// 0x14198c881  488b9568020000                 movq	0x268(%rbp), %rdx
// 0x14198c888  483b9558020000                 cmpq	0x258(%rbp), %rdx
// 0x14198c88f  0f852bfdffff                   jne	0x14198c5c0 <.text+0x198b5c0>
// Frames: F226 -> F311
// 0x14198c895  c6857702000000                 movb	$0x0, 0x277(%rbp)
// 0x14198c89c  488d8d58020000                 leaq	0x258(%rbp), %rcx
// 0x14198c8a3  e8f8da8efe                     callq	0x14027a3a0 <.text+0x2793a0>
// Frames: F227 -> F311
// 0x14198c8a8  488b9568020000                 movq	0x268(%rbp), %rdx
// 0x14198c8af  e90cfdffff                     jmp	0x14198c5c0 <.text+0x198b5c0>
// Frames: F334
// 0x14198c8b4  80bd8803000000                 cmpb	$0x0, 0x388(%rbp)
// 0x14198c8bb  0f8587000000                   jne	0x14198c948 <.text+0x198b948>
// Frames: F335 -> F336 -> F337 -> F338
// 0x14198c8c1  4885c0                         testq	%rax, %rax
// 0x14198c8c4  0f8480010000                   je	0x14198ca4a <.text+0x198ba4a>
// Frames: F338
// 0x14198c8ca  488b8560020000                 movq	0x260(%rbp), %rax
// 0x14198c8d1  488b10                         movq	(%rax), %rdx
// 0x14198c8d4  488b8d50020000                 movq	0x250(%rbp), %rcx
// 0x14198c8db  e8b0f9aafe                     callq	0x14043c290 <.text+0x43b290>
// 0x14198c8e0  48898550010000                 movq	%rax, 0x150(%rbp)
// 0x14198c8e7  48899558010000                 movq	%rdx, 0x158(%rbp)
// 0x14198c8ee  488d8d50010000                 leaq	0x150(%rbp), %rcx
// 0x14198c8f5  e8f69acffe                     callq	0x1406863f0 <.text+0x6853f0>
// 0x14198c8fa  84c0                           testb	%al, %al
// 0x14198c8fc  754a                           jne	0x14198c948 <.text+0x198b948>
// 0x14198c8fe  488b9568020000                 movq	0x268(%rbp), %rdx
// Frames: F335 -> F336 -> F337 -> F338
// 0x14198c905  4883fa01                       cmpq	$0x1, %rdx
// 0x14198c909  0f864d010000                   jbe	0x14198ca5c <.text+0x198ba5c>
// Frames: F338
// 0x14198c90f  488b8560020000                 movq	0x260(%rbp), %rax
// 0x14198c916  488b5018                       movq	0x18(%rax), %rdx
// 0x14198c91a  488b8d50020000                 movq	0x250(%rbp), %rcx
// 0x14198c921  e86af9aafe                     callq	0x14043c290 <.text+0x43b290>
// 0x14198c926  48898550010000                 movq	%rax, 0x150(%rbp)
// 0x14198c92d  48899558010000                 movq	%rdx, 0x158(%rbp)
// 0x14198c934  488d8d50010000                 leaq	0x150(%rbp), %rcx
// 0x14198c93b  e8b09acffe                     callq	0x1406863f0 <.text+0x6853f0>
// 0x14198c940  84c0                           testb	%al, %al
// 0x14198c942  0f84d7000000                   je	0x14198ca1f <.text+0x198ba1f>
// Frames: F339
// 0x14198c948  488b85f8010000                 movq	0x1f8(%rbp), %rax
// 0x14198c94f  4c8938                         movq	%r15, (%rax)
// Frames: F340
// 0x14198c952  488b8568020000                 movq	0x268(%rbp), %rax
// 0x14198c959  48898560010000                 movq	%rax, 0x160(%rbp)
// 0x14198c960  0f108558020000                 movups	0x258(%rbp), %xmm0
// 0x14198c967  0f298550010000                 movaps	%xmm0, 0x150(%rbp)
// 0x14198c96e  488bb580030000                 movq	0x380(%rbp), %rsi
// Frames: F83 -> F340
// 0x14198c975  488b06                         movq	(%rsi), %rax
// Frames: F149 -> F150 -> F83 -> F83 -> F340
// 0x14198c978  4885c0                         testq	%rax, %rax
// 0x14198c97b  7417                           je	0x14198c994 <.text+0x198b994>
// Frames: F83 -> F340
// 0x14198c97d  488b4e08                       movq	0x8(%rsi), %rcx
// Frames: F153 -> F154 -> F150 -> F83 -> F83 -> F340
// 0x14198c981  48c1e003                       shlq	$0x3, %rax
// 0x14198c985  488d1440                       leaq	(%rax,%rax,2), %rdx
// Frames: F155 -> F156 -> F157 -> F83 -> F83 -> F340
// 0x14198c989  41b808000000                   movl	$0x8, %r8d
// 0x14198c98f  e8fcdda4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F340
// 0x14198c994  488b8560010000                 movq	0x160(%rbp), %rax
// 0x14198c99b  48894610                       movq	%rax, 0x10(%rsi)
// 0x14198c99f  0f288550010000                 movaps	0x150(%rbp), %xmm0
// 0x14198c9a6  0f1106                         movups	%xmm0, (%rsi)
// Frames: F341
// 0x14198c9a9  488b8540020000                 movq	0x240(%rbp), %rax
// 0x14198c9b0  488b8df0010000                 movq	0x1f0(%rbp), %rcx
// 0x14198c9b7  48894110                       movq	%rax, 0x10(%rcx)
// 0x14198c9bb  0f288530020000                 movaps	0x230(%rbp), %xmm0
// 0x14198c9c2  0f1101                         movups	%xmm0, (%rcx)
// Frames: F342
// 0x14198c9c5  0f28b580020000                 movaps	0x280(%rbp), %xmm6
// 0x14198c9cc  0f28bd90020000                 movaps	0x290(%rbp), %xmm7
// 0x14198c9d3  440f2885a0020000               movaps	0x2a0(%rbp), %xmm8
// 0x14198c9db  440f288db0020000               movaps	0x2b0(%rbp), %xmm9
// 0x14198c9e3  440f2895c0020000               movaps	0x2c0(%rbp), %xmm10
// 0x14198c9eb  440f289dd0020000               movaps	0x2d0(%rbp), %xmm11
// 0x14198c9f3  440f28a5e0020000               movaps	0x2e0(%rbp), %xmm12
// 0x14198c9fb  440f28adf0020000               movaps	0x2f0(%rbp), %xmm13
// 0x14198ca03  440f28b500030000               movaps	0x300(%rbp), %xmm14
// 0x14198ca0b  4881c498030000                 addq	$0x398, %rsp            # imm = 0x398
// 0x14198ca12  5b                             popq	%rbx
// 0x14198ca13  5f                             popq	%rdi
// 0x14198ca14  5e                             popq	%rsi
// 0x14198ca15  415c                           popq	%r12
// 0x14198ca17  415d                           popq	%r13
// 0x14198ca19  415e                           popq	%r14
// 0x14198ca1b  415f                           popq	%r15
// 0x14198ca1d  5d                             popq	%rbp
// 0x14198ca1e  c3                             retq
// Frames: F83 -> F343
// 0x14198ca1f  488b8558020000                 movq	0x258(%rbp), %rax
// Frames: F149 -> F150 -> F83 -> F83 -> F343
// 0x14198ca26  4885c0                         testq	%rax, %rax
// 0x14198ca29  749a                           je	0x14198c9c5 <.text+0x198b9c5>
// Frames: F83 -> F343
// 0x14198ca2b  488b8d60020000                 movq	0x260(%rbp), %rcx
// Frames: F153 -> F154 -> F150 -> F83 -> F83 -> F343
// 0x14198ca32  48c1e003                       shlq	$0x3, %rax
// 0x14198ca36  488d1440                       leaq	(%rax,%rax,2), %rdx
// Frames: F155 -> F156 -> F157 -> F83 -> F83 -> F343
// 0x14198ca3a  41b808000000                   movl	$0x8, %r8d
// 0x14198ca40  e84bdda4fe                     callq	0x1403da790 <.text+0x3d9790>
// 0x14198ca45  e97bffffff                     jmp	0x14198c9c5 <.text+0x198b9c5>
// Frames: F335 -> F336 -> F337 -> F338
// 0x14198ca4a  4c8d05376ed600                 leaq	0xd66e37(%rip), %r8     # 0x1426f3888
// 0x14198ca51  31c9                           xorl	%ecx, %ecx
// 0x14198ca53  31d2                           xorl	%edx, %edx
// 0x14198ca55  e8b6217200                     callq	0x1420aec10 <.text+0x20adc10>
// 0x14198ca5a  eb11                           jmp	0x14198ca6d <.text+0x198ba6d>
// 0x14198ca5c  4c8d053d6ed600                 leaq	0xd66e3d(%rip), %r8     # 0x1426f38a0
// 0x14198ca63  b901000000                     movl	$0x1, %ecx
// 0x14198ca68  e8a3217200                     callq	0x1420aec10 <.text+0x20adc10>
// 0x14198ca6d  0f0b                           ud2
// Frames: F260 -> F89 -> F90 -> F91 -> F306
// 0x14198ca6f  b908000000                     movl	$0x8, %ecx
// 0x14198ca74  ba30000000                     movl	$0x30, %edx
// 0x14198ca79  e8e21c7200                     callq	0x1420ae760 <.text+0x20ad760>
// 0x14198ca7e  cc                             int3
// 0x14198ca7f  90                             nop
// 0x14198ca80  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198ca85  55                             pushq	%rbp
// 0x14198ca86  4157                           pushq	%r15
// 0x14198ca88  4156                           pushq	%r14
// 0x14198ca8a  4155                           pushq	%r13
// 0x14198ca8c  4154                           pushq	%r12
// 0x14198ca8e  56                             pushq	%rsi
// 0x14198ca8f  57                             pushq	%rdi
// 0x14198ca90  53                             pushq	%rbx
// 0x14198ca91  4881ecb8000000                 subq	$0xb8, %rsp
// 0x14198ca98  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14198ca9f  440f29742420                   movaps	%xmm14, 0x20(%rsp)
// 0x14198caa5  440f296c2430                   movaps	%xmm13, 0x30(%rsp)
// 0x14198caab  440f29642440                   movaps	%xmm12, 0x40(%rsp)
// 0x14198cab1  440f295c2450                   movaps	%xmm11, 0x50(%rsp)
// 0x14198cab7  440f29542460                   movaps	%xmm10, 0x60(%rsp)
// 0x14198cabd  440f294c2470                   movaps	%xmm9, 0x70(%rsp)
// 0x14198cac3  440f29842480000000             movaps	%xmm8, 0x80(%rsp)
// 0x14198cacc  0f29bc2490000000               movaps	%xmm7, 0x90(%rsp)
// 0x14198cad4  0f29b424a0000000               movaps	%xmm6, 0xa0(%rsp)
// Frames: F83 -> F343
// 0x14198cadc  488b8558020000                 movq	0x258(%rbp), %rax
// Frames: F149 -> F150 -> F83 -> F83 -> F343
// 0x14198cae3  4885c0                         testq	%rax, %rax
// 0x14198cae6  741a                           je	0x14198cb02 <.text+0x198bb02>
// Frames: F83 -> F343
// 0x14198cae8  488b8d60020000                 movq	0x260(%rbp), %rcx
// Frames: F153 -> F154 -> F150 -> F83 -> F83 -> F343
// 0x14198caef  48c1e003                       shlq	$0x3, %rax
// 0x14198caf3  488d1440                       leaq	(%rax,%rax,2), %rdx
// Frames: F155 -> F156 -> F157 -> F83 -> F83 -> F343
// 0x14198caf7  41b808000000                   movl	$0x8, %r8d
// 0x14198cafd  e88edca4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F291
// 0x14198cb02  0f28b424a0000000               movaps	0xa0(%rsp), %xmm6
// 0x14198cb0a  0f28bc2490000000               movaps	0x90(%rsp), %xmm7
// 0x14198cb12  440f28842480000000             movaps	0x80(%rsp), %xmm8
// 0x14198cb1b  440f284c2470                   movaps	0x70(%rsp), %xmm9
// 0x14198cb21  440f28542460                   movaps	0x60(%rsp), %xmm10
// 0x14198cb27  440f285c2450                   movaps	0x50(%rsp), %xmm11
// 0x14198cb2d  440f28642440                   movaps	0x40(%rsp), %xmm12
// 0x14198cb33  440f286c2430                   movaps	0x30(%rsp), %xmm13
// 0x14198cb39  440f28742420                   movaps	0x20(%rsp), %xmm14
// 0x14198cb3f  4881c4b8000000                 addq	$0xb8, %rsp
// 0x14198cb46  5b                             popq	%rbx
// 0x14198cb47  5f                             popq	%rdi
// 0x14198cb48  5e                             popq	%rsi
// 0x14198cb49  415c                           popq	%r12
// 0x14198cb4b  415d                           popq	%r13
// 0x14198cb4d  415e                           popq	%r14
// 0x14198cb4f  415f                           popq	%r15
// 0x14198cb51  5d                             popq	%rbp
// 0x14198cb52  c3                             retq
// 0x14198cb53  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x14198cb60  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198cb65  55                             pushq	%rbp
// 0x14198cb66  4157                           pushq	%r15
// 0x14198cb68  4156                           pushq	%r14
// 0x14198cb6a  4155                           pushq	%r13
// 0x14198cb6c  4154                           pushq	%r12
// 0x14198cb6e  56                             pushq	%rsi
// 0x14198cb6f  57                             pushq	%rdi
// 0x14198cb70  53                             pushq	%rbx
// 0x14198cb71  4881ecb8000000                 subq	$0xb8, %rsp
// 0x14198cb78  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14198cb7f  440f29742420                   movaps	%xmm14, 0x20(%rsp)
// 0x14198cb85  440f296c2430                   movaps	%xmm13, 0x30(%rsp)
// 0x14198cb8b  440f29642440                   movaps	%xmm12, 0x40(%rsp)
// 0x14198cb91  440f295c2450                   movaps	%xmm11, 0x50(%rsp)
// 0x14198cb97  440f29542460                   movaps	%xmm10, 0x60(%rsp)
// 0x14198cb9d  440f294c2470                   movaps	%xmm9, 0x70(%rsp)
// 0x14198cba3  440f29842480000000             movaps	%xmm8, 0x80(%rsp)
// 0x14198cbac  0f29bc2490000000               movaps	%xmm7, 0x90(%rsp)
// 0x14198cbb4  0f29b424a0000000               movaps	%xmm6, 0xa0(%rsp)
// Frames: F331
// 0x14198cbbc  80bd7502000000                 cmpb	$0x0, 0x275(%rbp)
// 0x14198cbc3  7464                           je	0x14198cc29 <.text+0x198bc29>
// 0x14198cbc5  488d8d50010000                 leaq	0x150(%rbp), %rcx
// 0x14198cbcc  e8af0fd6fe                     callq	0x1406edb80 <.text+0x6ecb80>
// 0x14198cbd1  c6857602000001                 movb	$0x1, 0x276(%rbp)
// 0x14198cbd8  0f28b424a0000000               movaps	0xa0(%rsp), %xmm6
// 0x14198cbe0  0f28bc2490000000               movaps	0x90(%rsp), %xmm7
// 0x14198cbe8  440f28842480000000             movaps	0x80(%rsp), %xmm8
// 0x14198cbf1  440f284c2470                   movaps	0x70(%rsp), %xmm9
// 0x14198cbf7  440f28542460                   movaps	0x60(%rsp), %xmm10
// 0x14198cbfd  440f285c2450                   movaps	0x50(%rsp), %xmm11
// 0x14198cc03  440f28642440                   movaps	0x40(%rsp), %xmm12
// 0x14198cc09  440f286c2430                   movaps	0x30(%rsp), %xmm13
// 0x14198cc0f  440f28742420                   movaps	0x20(%rsp), %xmm14
// 0x14198cc15  4881c4b8000000                 addq	$0xb8, %rsp
// 0x14198cc1c  5b                             popq	%rbx
// 0x14198cc1d  5f                             popq	%rdi
// 0x14198cc1e  5e                             popq	%rsi
// 0x14198cc1f  415c                           popq	%r12
// 0x14198cc21  415d                           popq	%r13
// 0x14198cc23  415e                           popq	%r14
// 0x14198cc25  415f                           popq	%r15
// 0x14198cc27  5d                             popq	%rbp
// 0x14198cc28  c3                             retq
// 0x14198cc29  c6857602000001                 movb	$0x1, 0x276(%rbp)
// 0x14198cc30  0f28b424a0000000               movaps	0xa0(%rsp), %xmm6
// 0x14198cc38  0f28bc2490000000               movaps	0x90(%rsp), %xmm7
// 0x14198cc40  440f28842480000000             movaps	0x80(%rsp), %xmm8
// 0x14198cc49  440f284c2470                   movaps	0x70(%rsp), %xmm9
// 0x14198cc4f  440f28542460                   movaps	0x60(%rsp), %xmm10
// 0x14198cc55  440f285c2450                   movaps	0x50(%rsp), %xmm11
// 0x14198cc5b  440f28642440                   movaps	0x40(%rsp), %xmm12
// 0x14198cc61  440f286c2430                   movaps	0x30(%rsp), %xmm13
// 0x14198cc67  440f28742420                   movaps	0x20(%rsp), %xmm14
// 0x14198cc6d  4881c4b8000000                 addq	$0xb8, %rsp
// 0x14198cc74  5b                             popq	%rbx
// 0x14198cc75  5f                             popq	%rdi
// 0x14198cc76  5e                             popq	%rsi
// 0x14198cc77  415c                           popq	%r12
// 0x14198cc79  415d                           popq	%r13
// 0x14198cc7b  415e                           popq	%r14
// 0x14198cc7d  415f                           popq	%r15
// 0x14198cc7f  5d                             popq	%rbp
// 0x14198cc80  c3                             retq
// 0x14198cc81  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// 0x14198cc90  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198cc95  55                             pushq	%rbp
// 0x14198cc96  4157                           pushq	%r15
// 0x14198cc98  4156                           pushq	%r14
// 0x14198cc9a  4155                           pushq	%r13
// 0x14198cc9c  4154                           pushq	%r12
// 0x14198cc9e  56                             pushq	%rsi
// 0x14198cc9f  57                             pushq	%rdi
// 0x14198cca0  53                             pushq	%rbx
// 0x14198cca1  4881ecb8000000                 subq	$0xb8, %rsp
// 0x14198cca8  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14198ccaf  440f29742420                   movaps	%xmm14, 0x20(%rsp)
// 0x14198ccb5  440f296c2430                   movaps	%xmm13, 0x30(%rsp)
// 0x14198ccbb  440f29642440                   movaps	%xmm12, 0x40(%rsp)
// 0x14198ccc1  440f295c2450                   movaps	%xmm11, 0x50(%rsp)
// 0x14198ccc7  440f29542460                   movaps	%xmm10, 0x60(%rsp)
// 0x14198cccd  440f294c2470                   movaps	%xmm9, 0x70(%rsp)
// 0x14198ccd3  440f29842480000000             movaps	%xmm8, 0x80(%rsp)
// 0x14198ccdc  0f29bc2490000000               movaps	%xmm7, 0x90(%rsp)
// 0x14198cce4  0f29b424a0000000               movaps	%xmm6, 0xa0(%rsp)
// Frames: F344
// 0x14198ccec  80bd7602000000                 cmpb	$0x0, 0x276(%rbp)
// 0x14198ccf3  7476                           je	0x14198cd6b <.text+0x198bd6b>
// Frames: F83 -> F344
// 0x14198ccf5  488b9530010000                 movq	0x130(%rbp), %rdx
// Frames: F149 -> F150 -> F83 -> F83 -> F83 -> F344
// 0x14198ccfc  4885d2                         testq	%rdx, %rdx
// 0x14198ccff  7412                           je	0x14198cd13 <.text+0x198bd13>
// Frames: F83 -> F344
// 0x14198cd01  488b8d38010000                 movq	0x138(%rbp), %rcx
// Frames: F155 -> F156 -> F157 -> F83 -> F83 -> F83 -> F344
// 0x14198cd08  41b801000000                   movl	$0x1, %r8d
// 0x14198cd0e  e87ddaa4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F344
// 0x14198cd13  c6857702000000                 movb	$0x0, 0x277(%rbp)
// 0x14198cd1a  0f28b424a0000000               movaps	0xa0(%rsp), %xmm6
// 0x14198cd22  0f28bc2490000000               movaps	0x90(%rsp), %xmm7
// 0x14198cd2a  440f28842480000000             movaps	0x80(%rsp), %xmm8
// 0x14198cd33  440f284c2470                   movaps	0x70(%rsp), %xmm9
// 0x14198cd39  440f28542460                   movaps	0x60(%rsp), %xmm10
// 0x14198cd3f  440f285c2450                   movaps	0x50(%rsp), %xmm11
// 0x14198cd45  440f28642440                   movaps	0x40(%rsp), %xmm12
// 0x14198cd4b  440f286c2430                   movaps	0x30(%rsp), %xmm13
// 0x14198cd51  440f28742420                   movaps	0x20(%rsp), %xmm14
// 0x14198cd57  4881c4b8000000                 addq	$0xb8, %rsp
// 0x14198cd5e  5b                             popq	%rbx
// 0x14198cd5f  5f                             popq	%rdi
// 0x14198cd60  5e                             popq	%rsi
// 0x14198cd61  415c                           popq	%r12
// 0x14198cd63  415d                           popq	%r13
// 0x14198cd65  415e                           popq	%r14
// 0x14198cd67  415f                           popq	%r15
// 0x14198cd69  5d                             popq	%rbp
// 0x14198cd6a  c3                             retq
// 0x14198cd6b  c6857702000000                 movb	$0x0, 0x277(%rbp)
// 0x14198cd72  0f28b424a0000000               movaps	0xa0(%rsp), %xmm6
// 0x14198cd7a  0f28bc2490000000               movaps	0x90(%rsp), %xmm7
// 0x14198cd82  440f28842480000000             movaps	0x80(%rsp), %xmm8
// 0x14198cd8b  440f284c2470                   movaps	0x70(%rsp), %xmm9
// 0x14198cd91  440f28542460                   movaps	0x60(%rsp), %xmm10
// 0x14198cd97  440f285c2450                   movaps	0x50(%rsp), %xmm11
// 0x14198cd9d  440f28642440                   movaps	0x40(%rsp), %xmm12
// 0x14198cda3  440f286c2430                   movaps	0x30(%rsp), %xmm13
// 0x14198cda9  440f28742420                   movaps	0x20(%rsp), %xmm14
// 0x14198cdaf  4881c4b8000000                 addq	$0xb8, %rsp
// 0x14198cdb6  5b                             popq	%rbx
// 0x14198cdb7  5f                             popq	%rdi
// 0x14198cdb8  5e                             popq	%rsi
// 0x14198cdb9  415c                           popq	%r12
// 0x14198cdbb  415d                           popq	%r13
// 0x14198cdbd  415e                           popq	%r14
// 0x14198cdbf  415f                           popq	%r15
// 0x14198cdc1  5d                             popq	%rbp
// 0x14198cdc2  c3                             retq
// 0x14198cdc3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x14198cdd0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14198cdd5  55                             pushq	%rbp
// 0x14198cdd6  4157                           pushq	%r15
// 0x14198cdd8  4156                           pushq	%r14
// 0x14198cdda  4155                           pushq	%r13
// 0x14198cddc  4154                           pushq	%r12
// 0x14198cdde  56                             pushq	%rsi
// 0x14198cddf  57                             pushq	%rdi
// 0x14198cde0  53                             pushq	%rbx
// 0x14198cde1  4881ecb8000000                 subq	$0xb8, %rsp
// 0x14198cde8  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14198cdef  440f29742420                   movaps	%xmm14, 0x20(%rsp)
// 0x14198cdf5  440f296c2430                   movaps	%xmm13, 0x30(%rsp)
// 0x14198cdfb  440f29642440                   movaps	%xmm12, 0x40(%rsp)
// 0x14198ce01  440f295c2450                   movaps	%xmm11, 0x50(%rsp)
// 0x14198ce07  440f29542460                   movaps	%xmm10, 0x60(%rsp)
// 0x14198ce0d  440f294c2470                   movaps	%xmm9, 0x70(%rsp)
// 0x14198ce13  440f29842480000000             movaps	%xmm8, 0x80(%rsp)
// 0x14198ce1c  0f29bc2490000000               movaps	%xmm7, 0x90(%rsp)
// 0x14198ce24  0f29b424a0000000               movaps	%xmm6, 0xa0(%rsp)
// Frames: F345
// 0x14198ce2c  80bd7702000000                 cmpb	$0x0, 0x277(%rbp)
// 0x14198ce33  7477                           je	0x14198ceac <.text+0x198beac>
// Frames: F83 -> F345
// 0x14198ce35  488b8518010000                 movq	0x118(%rbp), %rax
// Frames: F149 -> F150 -> F83 -> F83 -> F345
// 0x14198ce3c  4885c0                         testq	%rax, %rax
// 0x14198ce3f  741a                           je	0x14198ce5b <.text+0x198be5b>
// Frames: F83 -> F345
// 0x14198ce41  488b8d20010000                 movq	0x120(%rbp), %rcx
// Frames: F153 -> F154 -> F150 -> F83 -> F83 -> F345
// 0x14198ce48  48c1e002                       shlq	$0x2, %rax
// 0x14198ce4c  488d14c0                       leaq	(%rax,%rax,8), %rdx
// Frames: F155 -> F156 -> F157 -> F83 -> F83 -> F345
// 0x14198ce50  41b804000000                   movl	$0x4, %r8d
// 0x14198ce56  e835d9a4fe                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F345
// 0x14198ce5b  0f28b424a0000000               movaps	0xa0(%rsp), %xmm6
// 0x14198ce63  0f28bc2490000000               movaps	0x90(%rsp), %xmm7
// 0x14198ce6b  440f28842480000000             movaps	0x80(%rsp), %xmm8
// 0x14198ce74  440f284c2470                   movaps	0x70(%rsp), %xmm9
// 0x14198ce7a  440f28542460                   movaps	0x60(%rsp), %xmm10
// 0x14198ce80  440f285c2450                   movaps	0x50(%rsp), %xmm11
// 0x14198ce86  440f28642440                   movaps	0x40(%rsp), %xmm12
// 0x14198ce8c  440f286c2430                   movaps	0x30(%rsp), %xmm13
// 0x14198ce92  440f28742420                   movaps	0x20(%rsp), %xmm14
// 0x14198ce98  4881c4b8000000                 addq	$0xb8, %rsp
// 0x14198ce9f  5b                             popq	%rbx
// 0x14198cea0  5f                             popq	%rdi
// 0x14198cea1  5e                             popq	%rsi
// 0x14198cea2  415c                           popq	%r12
// 0x14198cea4  415d                           popq	%r13
// 0x14198cea6  415e                           popq	%r14
// 0x14198cea8  415f                           popq	%r15
// 0x14198ceaa  5d                             popq	%rbp
// 0x14198ceab  c3                             retq
// 0x14198ceac  0f28b424a0000000               movaps	0xa0(%rsp), %xmm6
// 0x14198ceb4  0f28bc2490000000               movaps	0x90(%rsp), %xmm7
// 0x14198cebc  440f28842480000000             movaps	0x80(%rsp), %xmm8
// 0x14198cec5  440f284c2470                   movaps	0x70(%rsp), %xmm9
// 0x14198cecb  440f28542460                   movaps	0x60(%rsp), %xmm10
// 0x14198ced1  440f285c2450                   movaps	0x50(%rsp), %xmm11
// 0x14198ced7  440f28642440                   movaps	0x40(%rsp), %xmm12
// 0x14198cedd  440f286c2430                   movaps	0x30(%rsp), %xmm13
// 0x14198cee3  440f28742420                   movaps	0x20(%rsp), %xmm14
// 0x14198cee9  4881c4b8000000                 addq	$0xb8, %rsp
// 0x14198cef0  5b                             popq	%rbx
// 0x14198cef1  5f                             popq	%rdi
// 0x14198cef2  5e                             popq	%rsi
// 0x14198cef3  415c                           popq	%r12
// 0x14198cef5  415d                           popq	%r13
// 0x14198cef7  415e                           popq	%r14
// 0x14198cef9  415f                           popq	%r15
// 0x14198cefb  5d                             popq	%rbp
// 0x14198cefc  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:37:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F2: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:63:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F3: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:627:0 puffin::clean_function_name
// F4: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/mod.rs:222:0 core::str::impl$0::is_char_boundary
// F5: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:334:0 core::str::traits::impl$9::get
// F6: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:365:0 core::str::traits::impl$9::index
// F7: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:61:0 core::str::traits::impl$4::index
// F8: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:628:0 puffin::clean_function_name
// F9: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:222:0 core::slice::index::impl$2::get
// F10: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:621:0 core::slice::impl$0::get
// F11: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/mod.rs:226:0 core::str::impl$0::is_char_boundary
// F12: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1103:0 core::num::impl$6::is_utf8_char_boundary
// F13: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/mod.rs:238:0 core::str::impl$0::is_char_boundary
// F14: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:367:0 core::str::traits::impl$9::index
// F15: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:403:0 core::str::traits::impl$10::get
// F16: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:436:0 core::str::traits::impl$10::index
// F17: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:630:0 puffin::clean_function_name
// F18: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:438:0 core::str::traits::impl$10::index
// F19: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/const_ptr.rs:964:0 core::ptr::const_ptr::impl$0::add
// F20: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:213:0 core::str::traits::impl$7::get_unchecked
// F21: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:425:0 core::str::traits::impl$10::get_unchecked
// F22: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:406:0 core::str::traits::impl$10::get
// F23: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/traits.rs:214:0 core::str::traits::impl$7::get_unchecked
// F24: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:85:0 core::str::validations::next_code_point_reverse
// F25: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/iter.rs:134:0 core::str::iter::impl$2::next_back
// F26: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/iter.rs:222:0 core::str::iter::impl$6::next_back
// F27: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:695:0 core::str::pattern::impl$11::next_back
// F28: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:303:0 core::str::pattern::ReverseSearcher::next_match_back
// F29: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:780:0 core::str::pattern::impl$22::next_match_back
// F30: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/mod.rs:1314:0 core::str::impl$0::rfind
// F31: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:662:0 puffin::short_file_name
// F32: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:94:0 core::str::validations::next_code_point_reverse
// F33: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:25:0 core::str::validations::utf8_is_cont_byte
// F34: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:96:0 core::str::validations::next_code_point_reverse
// F35: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:99:0 core::str::validations::next_code_point_reverse
// F36: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:101:0 core::str::validations::next_code_point_reverse
// F37: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:539:0 core::ptr::non_null::NonNull::offset
// F38: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:720:0 core::ptr::non_null::NonNull::sub
// F39: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:131:0 core::slice::iter::Iter::pre_dec_end
// F40: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:401:0 core::slice::iter::impl$182::next_back
// F41: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:104:0 core::str::validations::next_code_point_reverse
// F42: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:12:0 core::str::validations::utf8_first_byte
// F43: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:105:0 core::str::validations::next_code_point_reverse
// F44: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:18:0 core::str::validations::utf8_acc_cont_byte
// F45: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:106:0 core::str::validations::next_code_point_reverse
// F46: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:108:0 core::str::validations::next_code_point_reverse
// F47: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:110:0 core::str::validations::next_code_point_reverse
// F48: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1072:0 core::option::Option::map
// F49: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:285:0 core::slice::iter::impl$181::any
// F50: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:641:0 core::str::pattern::impl$8::matches
// F51: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/pattern.rs:698:0 core::str::pattern::impl$11::next_back
// F52: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:1798:0 core::ptr::non_null::impl$12::eq
// F53: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$182::next_back
// F54: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/str/validations.rs:84:0 core::str::validations::next_code_point_reverse
// F55: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:398:0 core::slice::iter::impl$182::next_back
// F56: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/thread_local/fast_local.rs:93:0 puffin::impl$6::call::THREAD_PROFILER::__getit
// F57: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/thread/local.rs:283:0 std::thread::local::LocalKey::try_with
// F58: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/thread/local.rs:260:0 std::thread::local::LocalKey::with
// F59: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:429:0 puffin::ThreadProfiler::call
// F60: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:591:0 puffin::ProfilerScope::new
// F61: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:683:0 core::option::Option::as_ref
// F62: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/thread_local/mod.rs:46:0 std::sys::thread_local::lazy::LazyKeyInner::get
// F63: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/thread_local/fast_local.rs:174:0 std::sys::thread_local::fast_local::Key::get
// F64: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/sys/thread_local/fast_local.rs:176:0 std::sys::thread_local::fast_local::Key::get
// F65: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1207:0 core::option::Option::ok_or
// F66: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1034:0 core::result::Result::expect
// F67: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:664:0 puffin::short_file_name
// F68: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:684:0 core::option::Option::as_ref
// F69: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1787:0 core::cell::BorrowRefMut::new
// F70: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1091:0 core::cell::RefCell::try_borrow_mut
// F71: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1059:0 core::cell::RefCell::borrow_mut
// F72: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:429:0 puffin::impl$6::call::closure$0
// F73: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/thread/local.rs:284:0 std::thread::local::LocalKey::try_with
// F74: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1493:0 core::ptr::write
// F75: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/mem/mod.rs:922:0 core::mem::replace
// F76: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:482:0 core::cell::Cell::replace
// F77: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:411:0 core::cell::Cell::set
// F78: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1789:0 core::cell::BorrowRefMut::new
// F79: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:2126:0 core::cell::UnsafeCell::get
// F80: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1099:0 core::cell::RefCell::try_borrow_mut
// F81: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs:592:0 puffin::impl$9::new::closure$0
// F82: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1776:0 core::cell::impl$42::drop
// F83: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:515:0 core::ptr::drop_in_place
// F84: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1665:0 core::ptr::read_volatile
// F85: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:98:0 alloc::alloc::alloc
// F86: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:183:0 alloc::alloc::Global::alloc_impl
// F87: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:243:0 alloc::alloc::impl$1::allocate
// F88: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:221:0 alloc::raw_vec::RawVec::try_allocate_in
// F89: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:155:0 alloc::raw_vec::RawVec::with_capacity_in
// F90: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:688:0 alloc::vec::Vec::with_capacity_in
// F91: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:481:0 alloc::vec::Vec::with_capacity
// F92: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:65:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F93: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:100:0 alloc::alloc::alloc
// F94: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:225:0 alloc::raw_vec::RawVec::try_allocate_in
// F95: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:67:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F96: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:73:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F97: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:91:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F98: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:74:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F99: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/mem/manually_drop.rs:71:0 core::mem::manually_drop::ManuallyDrop::new
// F100: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2922:0 alloc::vec::impl$15::into_iter
// F101: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/into_iter.rs:212:0 alloc::vec::into_iter::impl$5::next
// F102: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1284:0 core::ptr::read
// F103: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:976:0 core::ptr::non_null::NonNull::read
// F104: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/into_iter.rs:219:0 alloc::vec::into_iter::impl$5::next
// F105: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:75:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F106: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1412:0 core::option::Option::and_then
// F107: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:740:0 bevy_ecs::entity::Entities::get
// F108: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:866:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F109: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:850:0 bevy_ecs::system::query::Query::get
// F110: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:77:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$0
// F111: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1413:0 core::option::Option::and_then
// F112: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1488:0 core::cmp::impls::impl$24::ne
// F113: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/nonzero.rs:137:0 core::num::nonzero::impl$2::ne
// F114: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:741:0 bevy_ecs::entity::Entities::get
// F115: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs:74:0 bevy_ecs::archetype::impl$24::eq
// F116: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs:742:0 bevy_ecs::entity::Entities::get
// F117: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:393:0 fixedbitset::FixedBitSet::contains
// F118: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:870:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F119: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/bool.rs:60:0 core::bool::impl$0::then
// F120: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::FixedBitSet::contains
// F121: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 core::ptr::mut_ptr::impl$0::add
// F122: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F123: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F124: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F125: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/bit.rs:178:0 fixedbitset::FixedBitSet::get_unchecked
// F126: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1952:0 core::result::impl$26::branch
// F127: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:835:0 bevy_ecs::storage::table::Tables::get
// F128: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:886:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F129: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:888:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F130: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:693:0 bevy_ecs::storage::table::Table::get_column
// F131: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:955:0 bevy_ecs::query::fetch::impl$21::set_table
// F132: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:942:0 bevy_ecs::query::fetch::impl$21::set_archetype
// F133: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:370:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F134: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::impl$11::get::closure$0
// F135: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1073:0 core::option::Option::map
// F136: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F137: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs:398:0 nonmax::NonMaxUsize::get
// F138: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:372:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F139: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs:412:0 bevy_ecs::storage::blob_vec::BlobVec::get_slice
// F140: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:322:0 bevy_ecs::storage::table::Column::get_data_slice
// F141: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:957:0 bevy_ecs::query::fetch::impl$21::set_table
// F142: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs:506:0 bevy_ptr::ThinSlicePtr::get
// F143: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:973:0 bevy_ecs::query::fetch::impl$21::fetch
// F144: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:892:0 bevy_ecs::query::state::QueryState::get_unchecked_manual
// F145: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:310:0 bevy_ecs::change_detection::impl$29::set_changed
// F146: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$30::deref_mut
// F147: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:82:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F148: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:79:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F149: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:280:0 alloc::raw_vec::RawVec::current_memory
// F150: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:554:0 alloc::raw_vec::impl$4::drop
// F151: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/into_iter.rs:436:0 alloc::vec::into_iter::impl$15::drop::impl$0::drop
// F152: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/into_iter.rs:447:0 alloc::vec::into_iter::impl$15::drop
// F153: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 core::num::impl$11::unchecked_mul
// F154: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:290:0 alloc::raw_vec::RawVec::current_memory
// F155: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:119:0 alloc::alloc::dealloc
// F156: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:256:0 alloc::alloc::impl$1::deallocate
// F157: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:555:0 alloc::raw_vec::impl$4::drop
// F158: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:92:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F159: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:851:0 bevy_ecs::system::query::Query::get
// F160: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:95:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F161: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:100:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F162: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:328:0 bevy_ecs::change_detection::impl$30::deref_mut
// F163: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:97:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F164: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:111:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F165: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2219:0 alloc::vec::Vec::is_empty
// F166: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:116:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F167: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:931:0 core::option::Option::unwrap
// F168: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:117:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F169: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:112:0 core::ops::arith::impl$12::add
// F170: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1084:0 glam::f32::vec3::impl$12::add
// F171: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1083:0 glam::f32::vec3::impl$12::add
// F172: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:509:0 core::ops::arith::impl$216::div
// F173: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:997:0 glam::f32::vec3::impl$4::div
// F174: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:996:0 glam::f32::vec3::impl$4::div
// F175: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:119:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F176: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter.rs:95:0 core::slice::iter::Iter::new
// F177: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:1035:0 core::slice::impl$0::iter
// F178: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$181::next
// F179: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/enumerate.rs:47:0 core::iter::adapters::enumerate::impl$1::next
// F180: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:162:0 core::slice::iter::impl$181::next
// F181: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:120:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F182: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:121:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F183: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:125:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F184: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/fmt.rs:637:0 alloc::fmt::format::closure$0
// F185: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1179:0 core::option::Option::map_or_else
// F186: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/fmt.rs:637:0 alloc::fmt::format
// F187: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:129:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F188: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:332:0 alloc::alloc::exchange_malloc
// F189: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/boxed.rs:219:0 alloc::boxed::impl$0::new
// F190: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/sync.rs:395:0 alloc::sync::Arc::new
// F191: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:124:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F192: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:123:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F193: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:141:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F194: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:154:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F195: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:146:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$2
// F196: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:159:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F197: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:147:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$2
// F198: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:150:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$2
// F199: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/range.rs:379:0 core::ops::range::RangeInclusive::new
// F200: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:149:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$2
// F201: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:221:0 core::ops::arith::impl$68::sub
// F202: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1137:0 glam::f32::vec3::impl$17::sub
// F203: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:463:0 glam::f32::vec3::Vec3::distance
// F204: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1138:0 glam::f32::vec3::impl$17::sub
// F205: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:196:0 glam::f32::vec3::Vec3::dot
// F206: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:437:0 glam::f32::vec3::impl$17::sub
// F207: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/f32.rs:442:0 std::f32::impl$0::sqrt
// F208: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs:188:0 glam::f32::vec3::Vec3::dot
// F209: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:162:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F210: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:164:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F211: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:169:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F212: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:170:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F213: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:174:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F214: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:175:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F215: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:172:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F216: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:608:0 core::option::Option::is_some
// F217: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:187:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F218: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:189:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F219: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:592:0 bevy_ecs::change_detection::impl$29::set_changed
// F220: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:625:0 bevy_ecs::event::EventWriter::send
// F221: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:188:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F222: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:592:0 bevy_ecs::change_detection::impl$30::deref_mut
// F223: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1970:0 alloc::vec::Vec::push
// F224: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:240:0 bevy_ecs::event::Events::send
// F225: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:233:0 bevy_ecs::event::Events::send
// F226: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1971:0 alloc::vec::Vec::push
// F227: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1974:0 alloc::vec::Vec::push
// F228: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:258:0 alloc::raw_vec::RawVec::ptr
// F229: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1365:0 alloc::vec::Vec::as_mut_ptr
// F230: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 alloc::vec::Vec::as_mut_ptr
// F231: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1975:0 alloc::vec::Vec::push
// F232: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1976:0 alloc::vec::Vec::push
// F233: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:241:0 bevy_ecs::event::Events::send
// F234: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:200:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F235: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:203:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F236: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:204:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F237: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:202:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F238: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:210:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F239: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:211:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F240: crates/country-core/src/resources/roofs/roof_shape.rs:219:0 country_core::resources::roofs::roof_shape::impl$22::eq
// F241: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:264:0 core::cmp::PartialEq::ne
// F242: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:212:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F243: crates/country-core/src/resources/roofs/roof_shape.rs:221:0 country_core::resources::roofs::roof_shape::impl$22::eq
// F244: crates/country-core/src/resources/roofs/roof_shape.rs:226:0 country_core::resources::roofs::roof_shape::impl$22::eq
// F245: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:23:0 glam::f32::vec2::impl$46::eq
// F246: crates/country-core/src/resources/roofs/roof_shape.rs:231:0 country_core::resources::roofs::roof_shape::impl$22::eq
// F247: crates/country-core/src/resources/roofs/roof_shape.rs:233:0 country_core::resources::roofs::roof_shape::impl$22::eq
// F248: crates/country-core/src/resources/roofs/roof_shape.rs:239:0 country_core::resources::roofs::roof_shape::impl$27::eq
// F249: crates/country-core/src/resources/roofs/roof_shape.rs:236:0 country_core::resources::roofs::roof_shape::impl$22::eq
// F250: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:213:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F251: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:214:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F252: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:216:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F253: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:217:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F254: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:219:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F255: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:220:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F256: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:221:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F257: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:222:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F258: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:230:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::hc9327c92dc8f0371
// F259: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:334:0 alloc::alloc::exchange_malloc
// F260: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:567:0 alloc::raw_vec::handle_reserve
// F261: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cell.rs:1061:0 core::cell::RefCell::borrow_mut
// F262: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1077:0 core::result::Result::unwrap
// F263: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:933:0 core::option::Option::unwrap
// F264: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:129:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$1
// F265: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:130:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$1
// F266: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:162:0 alloc::slice::hack::impl$1::to_vec
// F267: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:111:0 alloc::slice::hack::to_vec
// F268: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:441:0 alloc::slice::impl$0::to_vec_in
// F269: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:416:0 alloc::slice::impl$0::to_vec
// F270: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:823:0 alloc::slice::impl$9::to_owned
// F271: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/str.rs:211:0 alloc::str::impl$4::to_owned
// F272: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/string.rs:2706:0 alloc::string::impl$44::from
// F273: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/convert/mod.rs:759:0 core::convert::impl$3::into
// F274: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:131:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$1
// F275: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/intrinsics.rs:2987:0 core::intrinsics::copy_nonoverlapping
// F276: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/const_ptr.rs:1336:0 core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
// F277: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:167:0 alloc::slice::hack::impl$1::to_vec
// F278: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:133:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$1
// F279: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:135:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$1
// F280: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:351:0 core::ops::arith::impl$124::mul
// F281: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1052:0 glam::f32::vec3::impl$9::mul
// F282: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1085:0 glam::f32::vec3::impl$12::add
// F283: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1050:0 glam::f32::vec3::impl$9::mul
// F284: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1082:0 glam::f32::vec3::impl$12::add
// F285: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:136:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$1
// F286: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:820:0 alloc::vec::Vec::from_raw_parts_in
// F287: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:104:0 alloc::slice::hack::into_vec
// F288: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/slice.rs:463:0 alloc::slice::impl$0::into_vec
// F289: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:139:0 system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$1
// F290: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/boxed.rs:1248:0 alloc::boxed::impl$8::drop
// F291: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:232:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F292: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:240:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F293: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:242:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F294: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:243:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F295: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:244:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F296: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:246:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F297: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:249:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F298: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:256:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F299: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:816:0 glam::f32::vec2::Vec2::perp
// F300: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:701:0 core::ops::arith::impl$286::neg
// F301: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1301:0 glam::f32::vec3::impl$33::neg
// F302: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs:1303:0 glam::f32::vec3::impl$33::neg
// F303: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:257:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F304: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:245:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F305: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:260:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F306: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:263:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F307: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:265:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F308: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/zip.rs:155:0 core::iter::adapters::zip::impl$4::new
// F309: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/zip.rs:24:0 core::iter::adapters::zip::Zip::new
// F310: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/traits/iterator.rs:604:0 core::iter::traits::iterator::Iterator::zip
// F311: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:290:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F312: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/index_range.rs:119:0 core::ops::index_range::impl$1::next
// F313: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/array/iter.rs:245:0 core::array::iter::impl$2::next
// F314: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/zip.rs:166:0 core::iter::adapters::zip::impl$4::next
// F315: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/zip.rs:85:0 core::iter::adapters::zip::impl$1::next
// F316: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/zip.rs:167:0 core::iter::adapters::zip::impl$4::next
// F317: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/zip.rs:0:0 core::iter::adapters::zip::impl$4::next
// F318: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/index_range.rs:59:0 core::ops::index_range::IndexRange::next_unchecked
// F319: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/index_range.rs:121:0 core::ops::index_range::impl$1::next
// F320: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:246:0 core::slice::index::impl$2::get_unchecked
// F321: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:686:0 core::slice::impl$0::get_unchecked
// F322: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/array/iter.rs:252:0 core::array::iter::impl$2::next::closure$0
// F323: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1073:0 core::ops::index_range::impl$1::next
// F324: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/enumerate.rs:49:0 core::iter::adapters::enumerate::impl$1::next
// F325: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:272:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F326: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:271:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F327: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:270:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F328: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:266:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F329: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:278:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F330: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:279:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F331: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:280:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F332: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:277:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F333: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:284:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F334: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:293:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F335: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:264:0 core::slice::index::impl$2::index
// F336: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:18:0 core::slice::index::impl$0::index
// F337: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2829:0 alloc::vec::impl$12::index
// F338: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:295:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F339: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:297:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F340: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:298:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F341: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:299:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F342: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:303:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F343: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:301:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F344: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:282:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// F345: crates/systems/roof/src/ui_edit_roof_ridge_length.rs:291:0 system_roof::ui_edit_roof_ridge_length::create_ui_widget
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/ui_edit_roof_ridge_length.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 90a0e741e06756f94f62890d093e2c656bcc8ca0f0b657339dab6bdb6f402fc8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_edit_roof_ridge_length.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3199
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
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_roof; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3199,"record_offset":4300,"rva":26782848,"va":5395491968}
// 编译实例 RVAs: 0x198ac80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: ui_edit_roof_ridge_length.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22743087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8d1f7a4fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5395492778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: ui_edit_roof_ridge_length.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22743562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8f6f5a4fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5395493269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: ui_edit_roof_ridge_length.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22344173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8130eabfe","size":5,"soft_fail":false,"target_va":5373149840,"va":5395494008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7482352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8f02b7200","size":5,"soft_fail":false,"target_va":5402978144,"va":5395495787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17964279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e809e3edfe","size":5,"soft_fail":false,"target_va":5377529888,"va":5395494162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24189480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8d8e58efe","size":5,"soft_fail":false,"target_va":5371305600,"va":5395495075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7482865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8f12d7200","size":5,"soft_fail":false,"target_va":5402978688,"va":5395495818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7482984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8682e7200","size":5,"soft_fail":false,"target_va":5402978880,"va":5395495891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7483572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8b4307200","size":5,"soft_fail":false,"target_va":5402979344,"va":5395495767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8c2ebffff","size":5,"soft_fail":false,"target_va":5395490480,"va":5395495657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7487581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e85d407200","size":5,"soft_fail":false,"target_va":5402980128,"va":5395492542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e85efaffff","size":5,"soft_fail":false,"target_va":5395490672,"va":5395492109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7488465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8d1437200","size":5,"soft_fail":false,"target_va":5402980656,"va":5395492186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20529237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8abbfc6fe","size":5,"soft_fail":false,"target_va":5374966368,"va":5395495600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::inverse_ridge_01_param
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20904793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8a704c1fe","size":5,"soft_fail":false,"target_va":5374590208,"va":5395494996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::shape_dims
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19730244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8bcf0d2fe","size":5,"soft_fail":false,"target_va":5375764688,"va":5395494927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20781767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e839e5c2fe","size":5,"soft_fail":false,"target_va":5374712960,"va":5395494722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20057008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e850f4cdfe","size":5,"soft_fail":false,"target_va":5375436032,"va":5395493035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::wall_ids
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20057378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8def2cdfe","size":5,"soft_fail":false,"target_va":5375435520,"va":5395492893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19944073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e877adcffe","size":5,"soft_fail":false,"target_va":5375550352,"va":5395494420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20138594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e89eb5ccfe","size":5,"soft_fail":false,"target_va":5375355744,"va":5395494333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13220754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e86e4436ff","size":5,"soft_fail":false,"target_va":5382273632,"va":5395494381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20606679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e82991c5fe","size":5,"soft_fail":false,"target_va":5374887792,"va":5395494466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20606652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e84491c5fe","size":5,"soft_fail":false,"target_va":5374887840,"va":5395494487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20606690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e81e91c5fe","size":5,"soft_fail":false,"target_va":5374887920,"va":5395494605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19135807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8c102dcfe","size":5,"soft_fail":false,"target_va":5376356912,"va":5395492714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-19131915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e8f511dcfe","size":5,"soft_fail":false,"target_va":5376360560,"va":5395492470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19135950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e83202dcfe","size":5,"soft_fail":false,"target_va":5376356144,"va":5395492089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h8ca862e57c4aa143 (.llvm.3516157794931125432)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17541103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e81158f4fe","size":5,"soft_fail":false,"target_va":5377951392,"va":5395492490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_roof::ui_edit_roof_ridge_length::create_ui_widget
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395491968,"raw_hex":"e82d110000","size":5,"soft_fail":false,"target_va":5395497632,"va":5395493230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3199,"record_offset":10492,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3199,"record_offset":10576,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3199,"record_offset":10020,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3199,"record_offset":7204,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3199,"record_offset":7476,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3199,"record_offset":7532,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3199,"record_offset":8660,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3199,"record_offset":11596,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3199,"record_offset":9008,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3199,"record_offset":11952,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3199,"record_offset":10344,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3199,"record_offset":7652,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3199,"record_offset":11924,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3199,"record_offset":11568,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3199,"record_offset":10048,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3199,"record_offset":8716,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3199,"record_offset":12580,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3199,"record_offset":10260,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3199,"record_offset":11060,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3199,"record_offset":10084,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3199,"record_offset":10172,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3199,"record_offset":6136,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3199,"record_offset":4548,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3199,"record_offset":5124,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/ui_edit_roof_ridge_length.rs
//     evidence={"module":3199,"record_offset":10992,"parent_record_offset":4300,"ancestor_record_offset":4300,"inlinee":101548,"source_line":145,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":7340,"parent_record_offset":7316,"ancestor_record_offset":4300,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":7364,"parent_record_offset":7340,"ancestor_record_offset":4300,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":8952,"parent_record_offset":8928,"ancestor_record_offset":4300,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":10632,"parent_record_offset":10604,"ancestor_record_offset":4300,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":7316,"parent_record_offset":7288,"ancestor_record_offset":4300,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":8928,"parent_record_offset":8836,"ancestor_record_offset":4300,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3199,"record_offset":10604,"parent_record_offset":10576,"ancestor_record_offset":4300,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3199,"record_offset":10540,"parent_record_offset":10516,"ancestor_record_offset":4300,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":8864,"parent_record_offset":8836,"ancestor_record_offset":4300,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":11792,"parent_record_offset":11760,"ancestor_record_offset":4300,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":7288,"parent_record_offset":7260,"ancestor_record_offset":4300,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":7260,"parent_record_offset":7232,"ancestor_record_offset":4300,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":7436,"parent_record_offset":7260,"ancestor_record_offset":4300,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":8836,"parent_record_offset":8812,"ancestor_record_offset":4300,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3199,"record_offset":11760,"parent_record_offset":11716,"ancestor_record_offset":4300,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3199,"record_offset":11716,"parent_record_offset":11680,"ancestor_record_offset":4300,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3199,"record_offset":7232,"parent_record_offset":7204,"ancestor_record_offset":4300,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3199,"record_offset":8740,"parent_record_offset":8716,"ancestor_record_offset":4300,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3199,"record_offset":8788,"parent_record_offset":8764,"ancestor_record_offset":4300,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":3199,"record_offset":7916,"parent_record_offset":7772,"ancestor_record_offset":4300,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3199,"record_offset":8684,"parent_record_offset":8660,"ancestor_record_offset":4300,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3199,"record_offset":11620,"parent_record_offset":11596,"ancestor_record_offset":4300,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":3199,"record_offset":7772,"parent_record_offset":7728,"ancestor_record_offset":4300,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3199,"record_offset":11680,"parent_record_offset":11596,"ancestor_record_offset":4300,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3199,"record_offset":8560,"parent_record_offset":7728,"ancestor_record_offset":4300,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3199,"record_offset":8192,"parent_record_offset":7728,"ancestor_record_offset":4300,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3199,"record_offset":8216,"parent_record_offset":8192,"ancestor_record_offset":4300,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3199,"record_offset":7728,"parent_record_offset":7704,"ancestor_record_offset":4300,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3199,"record_offset":8520,"parent_record_offset":8496,"ancestor_record_offset":4300,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3199,"record_offset":8292,"parent_record_offset":8268,"ancestor_record_offset":4300,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3199,"record_offset":8268,"parent_record_offset":8244,"ancestor_record_offset":4300,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3199,"record_offset":8340,"parent_record_offset":8316,"ancestor_record_offset":4300,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3199,"record_offset":8428,"parent_record_offset":8404,"ancestor_record_offset":4300,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3199,"record_offset":8496,"parent_record_offset":8216,"ancestor_record_offset":4300,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3199,"record_offset":8244,"parent_record_offset":8216,"ancestor_record_offset":4300,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3199,"record_offset":8164,"parent_record_offset":7728,"ancestor_record_offset":4300,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3199,"record_offset":7704,"parent_record_offset":7680,"ancestor_record_offset":4300,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3199,"record_offset":8584,"parent_record_offset":8560,"ancestor_record_offset":4300,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3199,"record_offset":7980,"parent_record_offset":7948,"ancestor_record_offset":4300,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3199,"record_offset":6556,"parent_record_offset":6528,"ancestor_record_offset":4300,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3199,"record_offset":6608,"parent_record_offset":6584,"ancestor_record_offset":4300,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3199,"record_offset":6584,"parent_record_offset":6556,"ancestor_record_offset":4300,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3199,"record_offset":6496,"parent_record_offset":6464,"ancestor_record_offset":4300,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3199,"record_offset":6528,"parent_record_offset":6496,"ancestor_record_offset":4300,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3199,"record_offset":6700,"parent_record_offset":6528,"ancestor_record_offset":4300,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3199,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":4300,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3199,"record_offset":7884,"parent_record_offset":7860,"ancestor_record_offset":4300,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":3199,"record_offset":7500,"parent_record_offset":7476,"ancestor_record_offset":4300,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3199,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":4300,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3199,"record_offset":8892,"parent_record_offset":8864,"ancestor_record_offset":4300,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3199,"record_offset":4760,"parent_record_offset":4664,"ancestor_record_offset":4300,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3199,"record_offset":7860,"parent_record_offset":7772,"ancestor_record_offset":4300,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3199,"record_offset":10112,"parent_record_offset":10084,"ancestor_record_offset":4300,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3199,"record_offset":10200,"parent_record_offset":10172,"ancestor_record_offset":4300,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3199,"record_offset":11124,"parent_record_offset":11084,"ancestor_record_offset":4300,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3199,"record_offset":11028,"parent_record_offset":10992,"ancestor_record_offset":4300,"inlinee":5483,"source_line":378,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3199,"record_offset":6388,"parent_record_offset":6360,"ancestor_record_offset":4300,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3199,"record_offset":5720,"parent_record_offset":5260,"ancestor_record_offset":4300,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3199,"record_offset":10516,"parent_record_offset":10492,"ancestor_record_offset":4300,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3199,"record_offset":6436,"parent_record_offset":6256,"ancestor_record_offset":4300,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3199,"record_offset":5072,"parent_record_offset":5044,"ancestor_record_offset":4300,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3199,"record_offset":6764,"parent_record_offset":6464,"ancestor_record_offset":4300,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3199,"record_offset":8088,"parent_record_offset":8056,"ancestor_record_offset":4300,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3199,"record_offset":5496,"parent_record_offset":5472,"ancestor_record_offset":4300,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3199,"record_offset":7592,"parent_record_offset":7532,"ancestor_record_offset":4300,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3199,"record_offset":5472,"parent_record_offset":5448,"ancestor_record_offset":4300,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3199,"record_offset":5680,"parent_record_offset":5648,"ancestor_record_offset":4300,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3199,"record_offset":7616,"parent_record_offset":7592,"ancestor_record_offset":4300,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3199,"record_offset":7392,"parent_record_offset":7364,"ancestor_record_offset":4300,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3199,"record_offset":6656,"parent_record_offset":6632,"ancestor_record_offset":4300,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3199,"record_offset":7164,"parent_record_offset":6216,"ancestor_record_offset":4300,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3199,"record_offset":8136,"parent_record_offset":7728,"ancestor_record_offset":4300,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3199,"record_offset":4696,"parent_record_offset":4664,"ancestor_record_offset":4300,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3199,"record_offset":4724,"parent_record_offset":4696,"ancestor_record_offset":4300,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3199,"record_offset":10284,"parent_record_offset":10260,"ancestor_record_offset":4300,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3199,"record_offset":5448,"parent_record_offset":5424,"ancestor_record_offset":4300,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3199,"record_offset":5776,"parent_record_offset":5752,"ancestor_record_offset":4300,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3199,"record_offset":10368,"parent_record_offset":10344,"ancestor_record_offset":4300,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3199,"record_offset":5424,"parent_record_offset":5316,"ancestor_record_offset":4300,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3199,"record_offset":4664,"parent_record_offset":4636,"ancestor_record_offset":4300,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3199,"record_offset":5156,"parent_record_offset":5124,"ancestor_record_offset":4300,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3199,"record_offset":5288,"parent_record_offset":5260,"ancestor_record_offset":4300,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3199,"record_offset":5260,"parent_record_offset":5228,"ancestor_record_offset":4300,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3199,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":4300,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3199,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":4300,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3199,"record_offset":5180,"parent_record_offset":5156,"ancestor_record_offset":4300,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3199,"record_offset":5752,"parent_record_offset":5228,"ancestor_record_offset":4300,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3199,"record_offset":4860,"parent_record_offset":4828,"ancestor_record_offset":4300,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3199,"record_offset":5020,"parent_record_offset":4860,"ancestor_record_offset":4300,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3199,"record_offset":4828,"parent_record_offset":4804,"ancestor_record_offset":4300,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3199,"record_offset":4580,"parent_record_offset":4548,"ancestor_record_offset":4300,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3199,"record_offset":5044,"parent_record_offset":5020,"ancestor_record_offset":4300,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3199,"record_offset":4636,"parent_record_offset":4604,"ancestor_record_offset":4300,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3199,"record_offset":4604,"parent_record_offset":4580,"ancestor_record_offset":4300,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3199,"record_offset":5316,"parent_record_offset":5288,"ancestor_record_offset":4300,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3199,"record_offset":5564,"parent_record_offset":5316,"ancestor_record_offset":4300,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3199,"record_offset":5536,"parent_record_offset":5316,"ancestor_record_offset":4300,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3199,"record_offset":5368,"parent_record_offset":5316,"ancestor_record_offset":4300,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$22::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3199,"record_offset":11976,"parent_record_offset":11952,"ancestor_record_offset":4300,"inlinee":77167,"source_line":216,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$27::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3199,"record_offset":12048,"parent_record_offset":11976,"ancestor_record_offset":4300,"inlinee":77169,"source_line":239,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3199,"record_offset":7948,"parent_record_offset":7728,"ancestor_record_offset":4300,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3199,"record_offset":8032,"parent_record_offset":8008,"ancestor_record_offset":4300,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3199,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":4300,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3199,"record_offset":8008,"parent_record_offset":7980,"ancestor_record_offset":4300,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$46::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3199,"record_offset":12020,"parent_record_offset":11976,"ancestor_record_offset":4300,"inlinee":5676,"source_line":18,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3199,"record_offset":11184,"parent_record_offset":11084,"ancestor_record_offset":4300,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3199,"record_offset":11084,"parent_record_offset":11060,"ancestor_record_offset":4300,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3199,"record_offset":8452,"parent_record_offset":8428,"ancestor_record_offset":4300,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3199,"record_offset":6176,"parent_record_offset":6136,"ancestor_record_offset":4300,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3199,"record_offset":6300,"parent_record_offset":6256,"ancestor_record_offset":4300,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3199,"record_offset":6464,"parent_record_offset":6256,"ancestor_record_offset":4300,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3199,"record_offset":6736,"parent_record_offset":6464,"ancestor_record_offset":4300,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3199,"record_offset":11220,"parent_record_offset":11184,"ancestor_record_offset":4300,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3199,"record_offset":6328,"parent_record_offset":6300,"ancestor_record_offset":4300,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3199,"record_offset":6360,"parent_record_offset":6328,"ancestor_record_offset":4300,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3199,"record_offset":6256,"parent_record_offset":6216,"ancestor_record_offset":4300,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3199,"record_offset":6216,"parent_record_offset":6176,"ancestor_record_offset":4300,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/ui_edit_roof_ridge_length.rs
//     evidence={"module":3199,"record_offset":7680,"parent_record_offset":7652,"ancestor_record_offset":4300,"inlinee":101547,"source_line":77,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18CAB
// 首条位置: {"module":3199,"inlinee":101547}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3199,"record_offset":7704,"parent_record_offset":7680,"ancestor_record_offset":7680,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":3199,"record_offset":7916,"parent_record_offset":7772,"ancestor_record_offset":7680,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":3199,"record_offset":7772,"parent_record_offset":7728,"ancestor_record_offset":7680,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3199,"record_offset":8560,"parent_record_offset":7728,"ancestor_record_offset":7680,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3199,"record_offset":8192,"parent_record_offset":7728,"ancestor_record_offset":7680,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3199,"record_offset":8216,"parent_record_offset":8192,"ancestor_record_offset":7680,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3199,"record_offset":7728,"parent_record_offset":7704,"ancestor_record_offset":7680,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3199,"record_offset":8520,"parent_record_offset":8496,"ancestor_record_offset":7680,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3199,"record_offset":8292,"parent_record_offset":8268,"ancestor_record_offset":7680,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3199,"record_offset":8268,"parent_record_offset":8244,"ancestor_record_offset":7680,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3199,"record_offset":8340,"parent_record_offset":8316,"ancestor_record_offset":7680,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3199,"record_offset":8428,"parent_record_offset":8404,"ancestor_record_offset":7680,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3199,"record_offset":8496,"parent_record_offset":8216,"ancestor_record_offset":7680,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3199,"record_offset":8244,"parent_record_offset":8216,"ancestor_record_offset":7680,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3199,"record_offset":8164,"parent_record_offset":7728,"ancestor_record_offset":7680,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3199,"record_offset":8584,"parent_record_offset":8560,"ancestor_record_offset":7680,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3199,"record_offset":7980,"parent_record_offset":7948,"ancestor_record_offset":7680,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3199,"record_offset":7884,"parent_record_offset":7860,"ancestor_record_offset":7680,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3199,"record_offset":7860,"parent_record_offset":7772,"ancestor_record_offset":7680,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3199,"record_offset":8364,"parent_record_offset":8340,"ancestor_record_offset":7680,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3199,"record_offset":8316,"parent_record_offset":8292,"ancestor_record_offset":7680,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3199,"record_offset":8088,"parent_record_offset":8056,"ancestor_record_offset":7680,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3199,"record_offset":8136,"parent_record_offset":7728,"ancestor_record_offset":7680,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3199,"record_offset":7804,"parent_record_offset":7772,"ancestor_record_offset":7680,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3199,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":7680,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3199,"record_offset":7948,"parent_record_offset":7728,"ancestor_record_offset":7680,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3199,"record_offset":8032,"parent_record_offset":8008,"ancestor_record_offset":7680,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3199,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":7680,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3199,"record_offset":8008,"parent_record_offset":7980,"ancestor_record_offset":7680,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3199,"record_offset":8452,"parent_record_offset":8428,"ancestor_record_offset":7680,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3199,"record_offset":13352,"rva":26787872,"va":5395496992}
// 编译实例 RVAs: 0x198c020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_edit_roof_ridge_length.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22747362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395496992,"raw_hex":"e81ee7a4fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5395497053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_edit_roof_ridge_length.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22747837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395496992,"raw_hex":"e843e5a4fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5395497544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408cfa90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_ridge_length.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17549304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395496992,"raw_hex":"e80838f4fe","size":5,"soft_fail":false,"target_va":5377948304,"va":5395497603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7480670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395496992,"raw_hex":"e85e257200","size":5,"soft_fail":false,"target_va":5402978144,"va":5395497469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20140507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395496992,"raw_hex":"e825aeccfe","size":5,"soft_fail":false,"target_va":5375356704,"va":5395497206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20140513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395496992,"raw_hex":"e81faeccfe","size":5,"soft_fail":false,"target_va":5375356896,"va":5395497404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20141189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395496992,"raw_hex":"e87babccfe","size":5,"soft_fail":false,"target_va":5375355984,"va":5395497168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20140757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395496992,"raw_hex":"e82badccfe","size":5,"soft_fail":false,"target_va":5375356560,"va":5395497312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20140770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395496992,"raw_hex":"e81eadccfe","size":5,"soft_fail":false,"target_va":5375356608,"va":5395497373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20140539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395496992,"raw_hex":"e805aeccfe","size":5,"soft_fail":false,"target_va":5375356656,"va":5395497190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18556028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395496992,"raw_hex":"e884dbe4fe","size":5,"soft_fail":false,"target_va":5376941312,"va":5395497335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":13532,"parent_record_offset":13352,"ancestor_record_offset":13352,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3199,"record_offset":14416,"parent_record_offset":13352,"ancestor_record_offset":13352,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3199,"record_offset":13680,"parent_record_offset":13352,"ancestor_record_offset":13352,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3199,"record_offset":14500,"parent_record_offset":13352,"ancestor_record_offset":13352,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3199,"record_offset":14324,"parent_record_offset":13352,"ancestor_record_offset":13352,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3199,"record_offset":14236,"parent_record_offset":13352,"ancestor_record_offset":13352,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":13588,"parent_record_offset":13564,"ancestor_record_offset":13352,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":13612,"parent_record_offset":13588,"ancestor_record_offset":13352,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":14572,"parent_record_offset":14548,"ancestor_record_offset":13352,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":13564,"parent_record_offset":13532,"ancestor_record_offset":13352,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":14548,"parent_record_offset":14524,"ancestor_record_offset":13352,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3199,"record_offset":14524,"parent_record_offset":14500,"ancestor_record_offset":13352,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":13968,"parent_record_offset":13940,"ancestor_record_offset":13352,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":13940,"parent_record_offset":13912,"ancestor_record_offset":13352,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":14112,"parent_record_offset":13940,"ancestor_record_offset":13352,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3199,"record_offset":13876,"parent_record_offset":13848,"ancestor_record_offset":13352,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3199,"record_offset":14440,"parent_record_offset":14416,"ancestor_record_offset":13352,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3199,"record_offset":13848,"parent_record_offset":13820,"ancestor_record_offset":13352,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3199,"record_offset":13792,"parent_record_offset":13764,"ancestor_record_offset":13352,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3199,"record_offset":13820,"parent_record_offset":13792,"ancestor_record_offset":13352,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3199,"record_offset":13764,"parent_record_offset":13736,"ancestor_record_offset":13352,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3199,"record_offset":13736,"parent_record_offset":13708,"ancestor_record_offset":13352,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3199,"record_offset":13708,"parent_record_offset":13680,"ancestor_record_offset":13352,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3199,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":13352,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3199,"record_offset":13912,"parent_record_offset":13876,"ancestor_record_offset":13352,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3199,"record_offset":14172,"parent_record_offset":14148,"ancestor_record_offset":13352,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3199,"record_offset":14264,"parent_record_offset":14236,"ancestor_record_offset":13352,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3199,"record_offset":14356,"parent_record_offset":14324,"ancestor_record_offset":13352,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3199,"record_offset":14148,"parent_record_offset":13876,"ancestor_record_offset":13352,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3199,"record_offset":13636,"parent_record_offset":13612,"ancestor_record_offset":13352,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18CAC
// 首条位置: {"module":3199,"inlinee":101548}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3199,"record_offset":11028,"parent_record_offset":10992,"ancestor_record_offset":10992,"inlinee":5483,"source_line":378,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::ui_edit_roof_ridge_length::create_ui_widget
// 记录形式: compiled_procedure; instances=1
// 源码行: 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3199,"record_offset":14616,"rva":26788512,"va":5395497632}
// 编译实例 RVAs: 0x198c2a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_edit_roof_ridge_length.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22748489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8b7e2a4fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5395498180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: ui_edit_roof_ridge_length.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22749700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8fcdda4fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5395499407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: ui_edit_roof_ridge_length.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22349392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8b0f9aafe","size":5,"soft_fail":false,"target_va":5373149840,"va":5395499227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043f210
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: ui_edit_roof_ridge_length.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22337134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e89229abfe","size":5,"soft_fail":false,"target_va":5373162000,"va":5395499129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056dd60
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: ui_edit_roof_ridge_length.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21097200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e81015befe","size":5,"soft_fail":false,"target_va":5374401888,"va":5395499083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406edb80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_ridge_length.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19525713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8af0fd6fe","size":5,"soft_fail":false,"target_va":5375974272,"va":5395499980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_edit_roof_ridge_length.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18553949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8a3e3e4fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5395497944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cee5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_edit_roof_ridge_length.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13230640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8d01d36ff","size":5,"soft_fail":false,"target_va":5382268400,"va":5395499035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7478498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8e21c7200","size":5,"soft_fail":false,"target_va":5402978144,"va":5395499641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17968948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8ccd0edfe","size":5,"soft_fail":false,"target_va":5377529888,"va":5395498831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24192264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8f8da8efe","size":5,"soft_fail":false,"target_va":5371306912,"va":5395499171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7479734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8b6217200","size":5,"soft_fail":false,"target_va":5402979344,"va":5395499605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::RoofRidgeDims::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20909137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8aff3c0fe","size":5,"soft_fail":false,"target_va":5374588624,"va":5395497756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20908974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e852f4c0fe","size":5,"soft_fail":false,"target_va":5374588816,"va":5395497785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19734310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8dae0d2fe","size":5,"soft_fail":false,"target_va":5375763488,"va":5395497793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19948755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e82d9bcffe","size":5,"soft_fail":false,"target_va":5375550352,"va":5395499102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::hovered
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19948743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8399bcffe","size":5,"soft_fail":false,"target_va":5375550320,"va":5395499058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-19948810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8f69acffe","size":5,"soft_fail":false,"target_va":5375550448,"va":5395499253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::boop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19727430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8bafbd2fe","size":5,"soft_fail":false,"target_va":5375771504,"va":5395498929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19727364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8fcfbd2fe","size":5,"soft_fail":false,"target_va":5375771600,"va":5395498959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19727751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e879fad2fe","size":5,"soft_fail":false,"target_va":5375771168,"va":5395498914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19727667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8cdfad2fe","size":5,"soft_fail":false,"target_va":5375771344,"va":5395499006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19727705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e8a7fad2fe","size":5,"soft_fail":false,"target_va":5375771280,"va":5395498980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21212644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e81c52bcfe","size":5,"soft_fail":false,"target_va":5374286080,"va":5395498719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18557298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395497632,"raw_hex":"e88ed6e4fe","size":5,"soft_fail":false,"target_va":5376941312,"va":5395498605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3199,"record_offset":16112,"parent_record_offset":14616,"ancestor_record_offset":14616,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3199,"record_offset":15396,"parent_record_offset":14616,"ancestor_record_offset":14616,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3199,"record_offset":15040,"parent_record_offset":14616,"ancestor_record_offset":14616,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3199,"record_offset":16196,"parent_record_offset":14616,"ancestor_record_offset":14616,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3199,"record_offset":15536,"parent_record_offset":14616,"ancestor_record_offset":14616,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3199,"record_offset":15312,"parent_record_offset":14616,"ancestor_record_offset":14616,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3199,"record_offset":16388,"parent_record_offset":14616,"ancestor_record_offset":14616,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3199,"record_offset":14796,"parent_record_offset":14616,"ancestor_record_offset":14616,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3199,"record_offset":16024,"parent_record_offset":14616,"ancestor_record_offset":14616,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$33::neg
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3199,"record_offset":14828,"parent_record_offset":14616,"ancestor_record_offset":14616,"inlinee":14666,"source_line":1299,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3199,"record_offset":15936,"parent_record_offset":14616,"ancestor_record_offset":14616,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":15176,"parent_record_offset":15152,"ancestor_record_offset":14616,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":15200,"parent_record_offset":15176,"ancestor_record_offset":14616,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":16564,"parent_record_offset":16540,"ancestor_record_offset":14616,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":15152,"parent_record_offset":15124,"ancestor_record_offset":14616,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3199,"record_offset":16540,"parent_record_offset":16440,"ancestor_record_offset":14616,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3199,"record_offset":16160,"parent_record_offset":16136,"ancestor_record_offset":14616,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":16472,"parent_record_offset":16440,"ancestor_record_offset":14616,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":15472,"parent_record_offset":15440,"ancestor_record_offset":14616,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":15124,"parent_record_offset":15096,"ancestor_record_offset":14616,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":15096,"parent_record_offset":15068,"ancestor_record_offset":14616,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":15272,"parent_record_offset":15096,"ancestor_record_offset":14616,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3199,"record_offset":16440,"parent_record_offset":16412,"ancestor_record_offset":14616,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3199,"record_offset":15440,"parent_record_offset":15396,"ancestor_record_offset":14616,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3199,"record_offset":15068,"parent_record_offset":15040,"ancestor_record_offset":14616,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3199,"record_offset":15624,"parent_record_offset":15588,"ancestor_record_offset":14616,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3199,"record_offset":15776,"parent_record_offset":15708,"ancestor_record_offset":14616,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3199,"record_offset":15336,"parent_record_offset":15312,"ancestor_record_offset":14616,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3199,"record_offset":15564,"parent_record_offset":15536,"ancestor_record_offset":14616,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3199,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":14616,"inlinee":4243,"source_line":154,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3199,"record_offset":15588,"parent_record_offset":15564,"ancestor_record_offset":14616,"inlinee":8595,"source_line":165,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3199,"record_offset":16504,"parent_record_offset":16472,"ancestor_record_offset":14616,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3199,"record_offset":15964,"parent_record_offset":15936,"ancestor_record_offset":14616,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3199,"record_offset":16052,"parent_record_offset":16024,"ancestor_record_offset":14616,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$286::neg
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3199,"record_offset":14860,"parent_record_offset":14828,"ancestor_record_offset":14616,"inlinee":14668,"source_line":694,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3199,"record_offset":15748,"parent_record_offset":15708,"ancestor_record_offset":14616,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3199,"record_offset":15648,"parent_record_offset":15624,"ancestor_record_offset":14616,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3199,"record_offset":16136,"parent_record_offset":16112,"ancestor_record_offset":14616,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3199,"record_offset":15848,"parent_record_offset":15824,"ancestor_record_offset":14616,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3199,"record_offset":16412,"parent_record_offset":16388,"ancestor_record_offset":14616,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3199,"record_offset":15228,"parent_record_offset":15200,"ancestor_record_offset":14616,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3199,"record_offset":15504,"parent_record_offset":15396,"ancestor_record_offset":14616,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3199,"record_offset":15800,"parent_record_offset":15776,"ancestor_record_offset":14616,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3199,"record_offset":16224,"parent_record_offset":16196,"ancestor_record_offset":14616,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3199,"record_offset":15824,"parent_record_offset":15800,"ancestor_record_offset":14616,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3199,"record_offset":16252,"parent_record_offset":16224,"ancestor_record_offset":14616,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03199.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_edit_roof_ridge_length.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_roof::ui_edit_roof_ridge_length::RidgeGrabParams
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 30
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["system_roof::ui_edit_roof_ridge_length::RidgeGrabParams"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)>", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >)", "void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::SystemConfigTupleMarker", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)>,tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> > >", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred", "arguments": ["tuple$<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >),void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)>", "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::system_param::Local<system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventReader<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_edit_roof_ridge_length.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_edit_roof_ridge_length.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

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

// ------------------------------------------------------------------
// 函数: system_roof::ui_edit_roof_ridge_length::create_ui_widget
// 信息 ID: bbd45d64b7d64d9460667fe1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length @ 0x198b2a3 -> 0x198c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length @ 0x198b16e -> 0x198c2a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length
// 信息 ID: d527693d62cb73bb7d1ef61f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] bd45d76fb685663246c3f782: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x12d433c -> 0x198ac80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x12dad01 -> 0x198ac80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$0
// 信息 ID: 7e24adf9adb81483f44e28da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length; module=3199, inline_record=7680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$1
// 信息 ID: 4d8b4e709726600c5c660bd0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length", "captures": "unknown"}
// [闭包环境类型引用] system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure_env$1 <- core::ops::function::FnOnce::call_once<system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure_env$1 <- core::ptr::drop_in_place<system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x198a6a2 -> 0x198c020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length::closure$2
// 信息 ID: 31923bc7f57005ec9d84fa19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length; module=3199, inline_record=10992; 不是独立调用边。
// [PDB 内联宿主] system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length; module=3199, inline_record=11316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
