// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: delete_roof.rs.assembly.json.gz

// FUNCTION system_roof::delete_roof::process_invalid_roof_notifications
// Candidate interfaces: 966a6da9871e5ffa21dca45d
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 270:260 -> rva_62eb80_len_84a
//
// FUNCTION system_roof::delete_roof::mark_roof_for_deletion
// Candidate interfaces: 634f59edb43c20e01e8dc3ba
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 270:10664 -> rva_62f3d0_len_4ea
//
// FUNCTION system_roof::delete_roof::delete_roofs
// Candidate interfaces: 22f506c314bfeff5269f4bf0
// Static incoming reference records: 3; complete records in enriched/assembly sidecars.
// Compiled procedure 270:16556 -> rva_62f8c0_len_3f0
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_62eb80_len_84a: RVA 0x62eb80, 2122 PDB bytes; 530 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_62eb80_len_84a.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 270:260: system_roof::delete_roof::process_invalid_roof_notifications
// Frames: F1
// 0x14062eb80  55                             pushq	%rbp
// 0x14062eb81  4157                           pushq	%r15
// 0x14062eb83  4156                           pushq	%r14
// 0x14062eb85  4155                           pushq	%r13
// 0x14062eb87  4154                           pushq	%r12
// 0x14062eb89  56                             pushq	%rsi
// 0x14062eb8a  57                             pushq	%rdi
// 0x14062eb8b  53                             pushq	%rbx
// 0x14062eb8c  4881ec08010000                 subq	$0x108, %rsp            # imm = 0x108
// 0x14062eb93  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14062eb9b  48c78580000000feffffff         movq	$-0x2, 0x80(%rbp)
// 0x14062eba6  4c89cb                         movq	%r9, %rbx
// 0x14062eba9  4889cf                         movq	%rcx, %rdi
// 0x14062ebac  4c8badf0000000                 movq	0xf0(%rbp), %r13
// Frames: F2 -> F3 -> F4
// 0x14062ebb3  498b00                         movq	(%r8), %rax
// Frames: F5 -> F3 -> F4
// 0x14062ebb6  498b4808                       movq	0x8(%r8), %rcx
// Frames: F6 -> F7 -> F8 -> F3 -> F4
// 0x14062ebba  4c8b00                         movq	(%rax), %r8
// 0x14062ebbd  4531c9                         xorl	%r9d, %r9d
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F4
// 0x14062ebc0  4d89c2                         movq	%r8, %r10
// 0x14062ebc3  4c2b5118                       subq	0x18(%rcx), %r10
// 0x14062ebc7  4d0f42d1                       cmovbq	%r9, %r10
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F4
// 0x14062ebcb  4c2b4138                       subq	0x38(%rcx), %r8
// 0x14062ebcf  4d0f42c1                       cmovbq	%r9, %r8
// Frames: F12 -> F13 -> F14 -> F7 -> F8 -> F3 -> F4
// 0x14062ebd3  4c8b5930                       movq	0x30(%rcx), %r11
// Frames: F15 -> F16 -> F17 -> F18 -> F14 -> F7 -> F8 -> F3 -> F4
// 0x14062ebd7  4d29c3                         subq	%r8, %r11
// 0x14062ebda  4d0f42d9                       cmovbq	%r9, %r11
// Frames: F19 -> F20 -> F21 -> F22 -> F4
// 0x14062ebde  410f96c0                       setbe	%r8b
// Frames: F12 -> F13 -> F23 -> F7 -> F8 -> F3 -> F4
// 0x14062ebe2  488b7110                       movq	0x10(%rcx), %rsi
// Frames: F15 -> F16 -> F17 -> F18 -> F23 -> F7 -> F8 -> F3 -> F4
// 0x14062ebe6  4c29d6                         subq	%r10, %rsi
// 0x14062ebe9  490f42f1                       cmovbq	%r9, %rsi
// Frames: F24 -> F7 -> F8 -> F3 -> F4
// 0x14062ebed  488b4940                       movq	0x40(%rcx), %rcx
// Frames: F19 -> F25 -> F21 -> F22 -> F4
// 0x14062ebf1  410f96c1                       setbe	%r9b
// Frames: F26 -> F7 -> F8 -> F3 -> F4
// 0x14062ebf5  4c01de                         addq	%r11, %rsi
// Frames: F24 -> F7 -> F8 -> F3 -> F4
// 0x14062ebf8  4989ca                         movq	%rcx, %r10
// 0x14062ebfb  4929f2                         subq	%rsi, %r10
// 0x14062ebfe  4c8910                         movq	%r10, (%rax)
// Frames: F27 -> F21 -> F22 -> F4
// 0x14062ec01  4584c1                         testb	%r8b, %r9b
// 0x14062ec04  0f848f040000                   je	0x14062f099 <.text+0x62e099>
// Frames: F2 -> F3 -> F28
// 0x14062ec0a  488b02                         movq	(%rdx), %rax
// Frames: F5 -> F3 -> F28
// 0x14062ec0d  488b5208                       movq	0x8(%rdx), %rdx
// Frames: F6 -> F7 -> F8 -> F3 -> F28
// 0x14062ec11  4c8b08                         movq	(%rax), %r9
// 0x14062ec14  4531e4                         xorl	%r12d, %r12d
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F28
// 0x14062ec17  4d89ca                         movq	%r9, %r10
// 0x14062ec1a  4c2b5218                       subq	0x18(%rdx), %r10
// 0x14062ec1e  4d0f42d4                       cmovbq	%r12, %r10
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F28
// 0x14062ec22  4c2b4a38                       subq	0x38(%rdx), %r9
// 0x14062ec26  4d0f42cc                       cmovbq	%r12, %r9
// Frames: F15 -> F16 -> F17 -> F18 -> F23 -> F7 -> F8 -> F3 -> F28
// 0x14062ec2a  4c89d1                         movq	%r10, %rcx
// 0x14062ec2d  48c1e104                       shlq	$0x4, %rcx
// 0x14062ec31  48034a08                       addq	0x8(%rdx), %rcx
// Frames: F12 -> F13 -> F23 -> F7 -> F8 -> F3 -> F28
// 0x14062ec35  4c8b4210                       movq	0x10(%rdx), %r8
// Frames: F15 -> F16 -> F17 -> F18 -> F23 -> F7 -> F8 -> F3 -> F28
// 0x14062ec39  4d29d0                         subq	%r10, %r8
// 0x14062ec3c  4d0f42c4                       cmovbq	%r12, %r8
// 0x14062ec40  be08000000                     movl	$0x8, %esi
// 0x14062ec45  480f42ce                       cmovbq	%rsi, %rcx
// Frames: F15 -> F16 -> F17 -> F18 -> F14 -> F7 -> F8 -> F3 -> F28
// 0x14062ec49  4d89ca                         movq	%r9, %r10
// 0x14062ec4c  49c1e204                       shlq	$0x4, %r10
// 0x14062ec50  4c035228                       addq	0x28(%rdx), %r10
// Frames: F12 -> F13 -> F14 -> F7 -> F8 -> F3 -> F28
// 0x14062ec54  4c8b5a30                       movq	0x30(%rdx), %r11
// Frames: F15 -> F16 -> F17 -> F18 -> F14 -> F7 -> F8 -> F3 -> F28
// 0x14062ec58  4d29cb                         subq	%r9, %r11
// 0x14062ec5b  4d0f42dc                       cmovbq	%r12, %r11
// 0x14062ec5f  41b908000000                   movl	$0x8, %r9d
// 0x14062ec65  4c894d30                       movq	%r9, 0x30(%rbp)
// 0x14062ec69  4c0f42d6                       cmovbq	%rsi, %r10
// Frames: F26 -> F7 -> F8 -> F3 -> F28
// 0x14062ec6d  4f8d0c03                       leaq	(%r11,%r8), %r9
// Frames: F24 -> F7 -> F8 -> F3 -> F28
// 0x14062ec71  488b5240                       movq	0x40(%rdx), %rdx
// 0x14062ec75  4c29ca                         subq	%r9, %rdx
// 0x14062ec78  488910                         movq	%rdx, (%rax)
// Frames: F29 -> F30 -> F31 -> F32 -> F7 -> F8 -> F3 -> F28
// 0x14062ec7b  49c1e004                       shlq	$0x4, %r8
// 0x14062ec7f  4901c8                         addq	%rcx, %r8
// 0x14062ec82  49c1e304                       shlq	$0x4, %r11
// 0x14062ec86  4d01d3                         addq	%r10, %r11
// Frames: F33 -> F7 -> F8 -> F3 -> F28
// 0x14062ec89  488945e0                       movq	%rax, -0x20(%rbp)
// 0x14062ec8d  48894de8                       movq	%rcx, -0x18(%rbp)
// 0x14062ec91  4c8945f0                       movq	%r8, -0x10(%rbp)
// 0x14062ec95  4c8955f8                       movq	%r10, -0x8(%rbp)
// 0x14062ec99  4c895d00                       movq	%r11, (%rbp)
// 0x14062ec9d  4c894d08                       movq	%r9, 0x8(%rbp)
// Frames: F34 -> F35 -> F28
// 0x14062eca1  4c8d0588da3d02                 leaq	0x23dda88(%rip), %r8    # 0x142a0c730
// 0x14062eca8  488d4da0                       leaq	-0x60(%rbp), %rcx
// 0x14062ecac  488d55e0                       leaq	-0x20(%rbp), %rdx
// 0x14062ecb0  e8dbf97900                     callq	0x140dce690 <.text+0xdcd690>
// Frames: F36 -> F37 -> F38 -> F39 -> F40
// 0x14062ecb5  488b0f                         movq	(%rdi), %rcx
// 0x14062ecb8  488b4708                       movq	0x8(%rdi), %rax
// 0x14062ecbc  48894550                       movq	%rax, 0x50(%rbp)
// Frames: F41 -> F42 -> F43 -> F44 -> F12 -> F13 -> F45 -> F46 -> F47 -> F39 -> F40
// 0x14062ecc0  4c8bb9f8000000                 movq	0xf8(%rcx), %r15
// Frames: F12 -> F13 -> F45 -> F46 -> F47 -> F39 -> F40
// 0x14062ecc7  488b8100010000                 movq	0x100(%rcx), %rax
// Frames: F29 -> F30 -> F48 -> F45 -> F46 -> F47 -> F39 -> F40
// 0x14062ecce  498d0487                       leaq	(%r15,%rax,4), %rax
// 0x14062ecd2  48894558                       movq	%rax, 0x58(%rbp)
// 0x14062ecd6  48894d70                       movq	%rcx, 0x70(%rbp)
// Frames: F49 -> F46 -> F47 -> F39 -> F40
// 0x14062ecda  0fb68108010000                 movzbl	0x108(%rcx), %eax
// 0x14062ece1  884548                         movb	%al, 0x48(%rbp)
// 0x14062ece4  8b431c                         movl	0x1c(%rbx), %eax
// 0x14062ece7  894540                         movl	%eax, 0x40(%rbp)
// 0x14062ecea  488b03                         movq	(%rbx), %rax
// 0x14062eced  488945c0                       movq	%rax, -0x40(%rbp)
// 0x14062ecf1  488b4310                       movq	0x10(%rbx), %rax
// 0x14062ecf5  488945b8                       movq	%rax, -0x48(%rbp)
// 0x14062ecf9  4c8b4da8                       movq	-0x58(%rbp), %r9
// 0x14062ecfd  4c8b5db0                       movq	-0x50(%rbp), %r11
// 0x14062ed01  418b7d1c                       movl	0x1c(%r13), %edi
// 0x14062ed05  498b4500                       movq	(%r13), %rax
// 0x14062ed09  498b4d10                       movq	0x10(%r13), %rcx
// 0x14062ed0d  48894d68                       movq	%rcx, 0x68(%rbp)
// Frames: F40
// 0x14062ed11  49c1e303                       shlq	$0x3, %r11
// 0x14062ed15  48894560                       movq	%rax, 0x60(%rbp)
// 0x14062ed19  4883c020                       addq	$0x20, %rax
// 0x14062ed1d  488945d8                       movq	%rax, -0x28(%rbp)
// 0x14062ed21  31db                           xorl	%ebx, %ebx
// 0x14062ed23  4531f6                         xorl	%r14d, %r14d
// 0x14062ed26  4531ed                         xorl	%r13d, %r13d
// 0x14062ed29  4c897578                       movq	%r14, 0x78(%rbp)
// 0x14062ed2d  440fb65548                     movzbl	0x48(%rbp), %r10d
// 0x14062ed32  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x14062ed40  4889d8                         movq	%rbx, %rax
// Frames: F50 -> F51 -> F40
// 0x14062ed43  4584d2                         testb	%r10b, %r10b
// 0x14062ed46  7418                           je	0x14062ed60 <.text+0x62dd60>
// Frames: F52 -> F51 -> F40
// 0x14062ed48  4939c4                         cmpq	%rax, %r12
// 0x14062ed4b  0f841f010000                   je	0x14062ee70 <.text+0x62de70>
// Frames: F53
// 0x14062ed51  488d5801                       leaq	0x1(%rax), %rbx
// 0x14062ed55  41f6c501                       testb	$0x1, %r13b
// 0x14062ed59  75e5                           jne	0x14062ed40 <.text+0x62dd40>
// 0x14062ed5b  e9a0010000                     jmp	0x14062ef00 <.text+0x62df00>
// Frames: F54 -> F51 -> F40
// 0x14062ed60  4939c4                         cmpq	%rax, %r12
// 0x14062ed63  488b5550                       movq	0x50(%rbp), %rdx
// 0x14062ed67  4c8b4558                       movq	0x58(%rbp), %r8
// 0x14062ed6b  0f85d9000000                   jne	0x14062ee4a <.text+0x62de4a>
// 0x14062ed71  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F55 -> F56 -> F57 -> F51 -> F40
// 0x14062ed80  4d39c7                         cmpq	%r8, %r15
// Frames: F58 -> F57 -> F51 -> F40
// 0x14062ed83  0f8456040000                   je	0x14062f1df <.text+0x62e1df>
// Frames: F57 -> F51 -> F40
// 0x14062ed89  418b0f                         movl	(%r15), %ecx
// Frames: F59 -> F60 -> F61 -> F62 -> F57 -> F51 -> F40
// 0x14062ed8c  4983c704                       addq	$0x4, %r15
// Frames: F63 -> F18 -> F64 -> F65 -> F51 -> F40
// 0x14062ed90  488b8200010000                 movq	0x100(%rdx), %rax
// 0x14062ed97  488d0c89                       leaq	(%rcx,%rcx,4), %rcx
// 0x14062ed9b  48c1e105                       shlq	$0x5, %rcx
// Frames: F66 -> F67 -> F68 -> F69 -> F51 -> F40
// 0x14062ed9f  4c8b640858                     movq	0x58(%rax,%rcx), %r12
// Frames: F69 -> F51 -> F40
// 0x14062eda4  4d85e4                         testq	%r12, %r12
// 0x14062eda7  74d7                           je	0x14062ed80 <.text+0x62dd80>
// Frames: F70 -> F71 -> F51 -> F40
// 0x14062eda9  4801c8                         addq	%rcx, %rax
// 0x14062edac  8b8894000000                   movl	0x94(%rax), %ecx
// Frames: F63 -> F18 -> F72 -> F71 -> F51 -> F40
// 0x14062edb2  488b92a8010000                 movq	0x1a8(%rdx), %rdx
// 0x14062edb9  4c8d04c9                       leaq	(%rcx,%rcx,8), %r8
// 0x14062edbd  488b4d70                       movq	0x70(%rbp), %rcx
// Frames: F73 -> F74 -> F51 -> F40
// 0x14062edc1  488b8910010000                 movq	0x110(%rcx), %rcx
// Frames: F63 -> F18 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F40
// 0x14062edc8  4a394cc240                     cmpq	%rcx, 0x40(%rdx,%r8,8)
// 0x14062edcd  763f                           jbe	0x14062ee0e <.text+0x62de0e>
// Frames: F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F40
// 0x14062edcf  4a8d14c2                       leaq	(%rdx,%r8,8), %rdx
// 0x14062edd3  4c8b4238                       movq	0x38(%rdx), %r8
// Frames: F81 -> F82 -> F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F40
// 0x14062edd7  498b0cc8                       movq	(%r8,%rcx,8), %rcx
// 0x14062eddb  4885c9                         testq	%rcx, %rcx
// Frames: F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F40
// 0x14062edde  742e                           je	0x14062ee0e <.text+0x62de0e>
// Frames: F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F40
// 0x14062ede0  488b5218                       movq	0x18(%rdx), %rdx
// Frames: F84 -> F85 -> F86 -> F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F40
// 0x14062ede4  48f7d1                         notq	%rcx
// Frames: F87 -> F88 -> F89 -> F86 -> F90 -> F79 -> F80 -> F73 -> F74 -> F51 -> F40
// 0x14062ede7  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x14062edeb  48c1e104                       shlq	$0x4, %rcx
// 0x14062edef  488b4c0a10                     movq	0x10(%rdx,%rcx), %rcx
// 0x14062edf4  48894d78                       movq	%rcx, 0x78(%rbp)
// 0x14062edf8  488b4d70                       movq	0x70(%rbp), %rcx
// Frames: F91 -> F92 -> F73 -> F74 -> F51 -> F40
// 0x14062edfc  488b8918010000                 movq	0x118(%rcx), %rcx
// Frames: F63 -> F18 -> F93 -> F94 -> F95 -> F96 -> F97 -> F92 -> F73 -> F74 -> F51 -> F40
// 0x14062ee03  48398888000000                 cmpq	%rcx, 0x88(%rax)
// 0x14062ee0a  771e                           ja	0x14062ee2a <.text+0x62de2a>
// 0x14062ee0c  eb2f                           jmp	0x14062ee3d <.text+0x62de3d>
// Frames: F91 -> F92 -> F73 -> F74 -> F51 -> F40
// 0x14062ee0e  48c7457800000000               movq	$0x0, 0x78(%rbp)
// 0x14062ee16  488b4d70                       movq	0x70(%rbp), %rcx
// 0x14062ee1a  488b8918010000                 movq	0x118(%rcx), %rcx
// Frames: F63 -> F18 -> F93 -> F94 -> F95 -> F96 -> F97 -> F92 -> F73 -> F74 -> F51 -> F40
// 0x14062ee21  48398888000000                 cmpq	%rcx, 0x88(%rax)
// 0x14062ee28  7613                           jbe	0x14062ee3d <.text+0x62de3d>
// Frames: F94 -> F95 -> F96 -> F97 -> F92 -> F73 -> F74 -> F51 -> F40
// 0x14062ee2a  488b9080000000                 movq	0x80(%rax), %rdx
// Frames: F98 -> F82 -> F99 -> F93 -> F94 -> F95 -> F96 -> F97 -> F92 -> F73 -> F74 -> F51 -> F40
// 0x14062ee31  48833cca00                     cmpq	$0x0, (%rdx,%rcx,8)
// Frames: F100 -> F73 -> F74 -> F51 -> F40
// 0x14062ee36  7405                           je	0x14062ee3d <.text+0x62de3d>
// 0x14062ee38  41b501                         movb	$0x1, %r13b
// 0x14062ee3b  eb03                           jmp	0x14062ee40 <.text+0x62de40>
// 0x14062ee3d  4531ed                         xorl	%r13d, %r13d
// Frames: F41 -> F42 -> F43 -> F44 -> F12 -> F13 -> F101 -> F102 -> F51 -> F40
// 0x14062ee40  488b4050                       movq	0x50(%rax), %rax
// 0x14062ee44  48894530                       movq	%rax, 0x30(%rbp)
// 0x14062ee48  31c0                           xorl	%eax, %eax
// Frames: F53
// 0x14062ee4a  488d5801                       leaq	0x1(%rax), %rbx
// 0x14062ee4e  41f6c501                       testb	$0x1, %r13b
// 0x14062ee52  0f85e8feffff                   jne	0x14062ed40 <.text+0x62dd40>
// Frames: F103 -> F104 -> F51 -> F40
// 0x14062ee58  48c1e004                       shlq	$0x4, %rax
// 0x14062ee5c  488b4d30                       movq	0x30(%rbp), %rcx
// Frames: F103 -> F105 -> F51 -> F40
// 0x14062ee60  8b440108                       movl	0x8(%rcx,%rax), %eax
// 0x14062ee64  e999000000                     jmp	0x14062ef02 <.text+0x62df02>
// 0x14062ee69  0f1f8000000000                 nopl	(%rax)
// 0x14062ee70  488b5550                       movq	0x50(%rbp), %rdx
// 0x14062ee74  4c8b4558                       movq	0x58(%rbp), %r8
// 0x14062ee78  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F55 -> F56 -> F106 -> F51 -> F40
// 0x14062ee80  4d39c7                         cmpq	%r8, %r15
// Frames: F58 -> F106 -> F51 -> F40
// 0x14062ee83  0f8456030000                   je	0x14062f1df <.text+0x62e1df>
// Frames: F106 -> F51 -> F40
// 0x14062ee89  418b0f                         movl	(%r15), %ecx
// Frames: F59 -> F60 -> F61 -> F62 -> F106 -> F51 -> F40
// 0x14062ee8c  4983c704                       addq	$0x4, %r15
// Frames: F63 -> F18 -> F107 -> F108 -> F51 -> F40
// 0x14062ee90  488b82a8010000                 movq	0x1a8(%rdx), %rax
// 0x14062ee97  488d0cc9                       leaq	(%rcx,%rcx,8), %rcx
// Frames: F66 -> F67 -> F109 -> F110 -> F51 -> F40
// 0x14062ee9b  4c8b64c810                     movq	0x10(%rax,%rcx,8), %r12
// Frames: F110 -> F51 -> F40
// 0x14062eea0  4d85e4                         testq	%r12, %r12
// 0x14062eea3  74db                           je	0x14062ee80 <.text+0x62de80>
// Frames: F111 -> F112 -> F51 -> F40
// 0x14062eea5  488d04c8                       leaq	(%rax,%rcx,8), %rax
// 0x14062eea9  488b4d70                       movq	0x70(%rbp), %rcx
// 0x14062eead  488b9110010000                 movq	0x110(%rcx), %rdx
// Frames: F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F40
// 0x14062eeb4  488b4840                       movq	0x40(%rax), %rcx
// Frames: F63 -> F18 -> F75 -> F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F40
// 0x14062eeb8  4839d1                         cmpq	%rdx, %rcx
// 0x14062eebb  764f                           jbe	0x14062ef0c <.text+0x62df0c>
// Frames: F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F40
// 0x14062eebd  4c8b4038                       movq	0x38(%rax), %r8
// Frames: F81 -> F82 -> F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F40
// 0x14062eec1  498b14d0                       movq	(%r8,%rdx,8), %rdx
// 0x14062eec5  4885d2                         testq	%rdx, %rdx
// Frames: F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F40
// 0x14062eec8  7442                           je	0x14062ef0c <.text+0x62df0c>
// Frames: F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F40
// 0x14062eeca  4c8b4018                       movq	0x18(%rax), %r8
// Frames: F84 -> F85 -> F86 -> F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F40
// 0x14062eece  48f7d2                         notq	%rdx
// Frames: F87 -> F88 -> F89 -> F86 -> F90 -> F79 -> F111 -> F112 -> F51 -> F40
// 0x14062eed1  488d1452                       leaq	(%rdx,%rdx,2), %rdx
// 0x14062eed5  48c1e204                       shlq	$0x4, %rdx
// 0x14062eed9  4d8b741010                     movq	0x10(%r8,%rdx), %r14
// 0x14062eede  488b5570                       movq	0x70(%rbp), %rdx
// Frames: F91 -> F113 -> F111 -> F112 -> F51 -> F40
// 0x14062eee2  488b9218010000                 movq	0x118(%rdx), %rdx
// Frames: F63 -> F18 -> F93 -> F94 -> F114 -> F115 -> F97 -> F113 -> F111 -> F112 -> F51 -> F40
// 0x14062eee9  4839d1                         cmpq	%rdx, %rcx
// 0x14062eeec  7731                           ja	0x14062ef1f <.text+0x62df1f>
// 0x14062eeee  bb01000000                     movl	$0x1, %ebx
// 0x14062eef3  eb41                           jmp	0x14062ef36 <.text+0x62df36>
// 0x14062eef5  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F116 -> F117 -> F118 -> F119 -> F120 -> F121 -> F51 -> F40
// 0x14062ef00  89c0                           movl	%eax, %eax
// Frames: F29 -> F122 -> F117 -> F118 -> F119 -> F120 -> F123 -> F51 -> F40
// 0x14062ef02  4c6bf058                       imulq	$0x58, %rax, %r14
// 0x14062ef06  4c037578                       addq	0x78(%rbp), %r14
// 0x14062ef0a  eb31                           jmp	0x14062ef3d <.text+0x62df3d>
// Frames: F91 -> F113 -> F111 -> F112 -> F51 -> F40
// 0x14062ef0c  4531f6                         xorl	%r14d, %r14d
// 0x14062ef0f  488b5570                       movq	0x70(%rbp), %rdx
// 0x14062ef13  488b9218010000                 movq	0x118(%rdx), %rdx
// Frames: F63 -> F18 -> F93 -> F94 -> F114 -> F115 -> F97 -> F113 -> F111 -> F112 -> F51 -> F40
// 0x14062ef1a  4839d1                         cmpq	%rdx, %rcx
// 0x14062ef1d  76cf                           jbe	0x14062eeee <.text+0x62deee>
// Frames: F94 -> F114 -> F115 -> F97 -> F113 -> F111 -> F112 -> F51 -> F40
// 0x14062ef1f  488b4038                       movq	0x38(%rax), %rax
// 0x14062ef23  41b501                         movb	$0x1, %r13b
// 0x14062ef26  bb01000000                     movl	$0x1, %ebx
// Frames: F98 -> F82 -> F99 -> F93 -> F94 -> F114 -> F115 -> F97 -> F113 -> F111 -> F112 -> F51 -> F40
// 0x14062ef2b  48833cd000                     cmpq	$0x0, (%rax,%rdx,8)
// Frames: F124 -> F111 -> F112 -> F51 -> F40
// 0x14062ef30  0f85f3fdffff                   jne	0x14062ed29 <.text+0x62dd29>
// 0x14062ef36  4531ed                         xorl	%r13d, %r13d
// 0x14062ef39  4c897578                       movq	%r14, 0x78(%rbp)
// 0x14062ef3d  498b36                         movq	(%r14), %rsi
// 0x14062ef40  31c0                           xorl	%eax, %eax
// 0x14062ef42  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// Frames: F55 -> F56 -> F125 -> F126 -> F127 -> F53
// 0x14062ef50  4939c3                         cmpq	%rax, %r11
// Frames: F58 -> F125 -> F126 -> F127 -> F53
// 0x14062ef53  0f8437010000                   je	0x14062f090 <.text+0x62e090>
// Frames: F128 -> F126 -> F127 -> F53
// 0x14062ef59  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F129 -> F130 -> F128 -> F126 -> F127 -> F53
// 0x14062ef5d  493b3401                       cmpq	(%r9,%rax), %rsi
// 0x14062ef61  4889c8                         movq	%rcx, %rax
// Frames: F128 -> F126 -> F127 -> F53
// 0x14062ef64  75ea                           jne	0x14062ef50 <.text+0x62df50>
// Frames: F131 -> F132 -> F133 -> F134
// 0x14062ef66  4c895dd0                       movq	%r11, -0x30(%rbp)
// 0x14062ef6a  4c894d28                       movq	%r9, 0x28(%rbp)
// 0x14062ef6e  897d3c                         movl	%edi, 0x3c(%rbp)
// 0x14062ef71  488b4568                       movq	0x68(%rbp), %rax
// 0x14062ef75  8938                           movl	%edi, (%rax)
// 0x14062ef77  488b4560                       movq	0x60(%rbp), %rax
// Frames: F135 -> F136 -> F137 -> F133 -> F134
// 0x14062ef7b  488b7830                       movq	0x30(%rax), %rdi
// Frames: F138 -> F137 -> F133 -> F134
// 0x14062ef7f  488b4840                       movq	0x40(%rax), %rcx
// 0x14062ef83  48894dc8                       movq	%rcx, -0x38(%rbp)
// Frames: F139 -> F136 -> F137 -> F133 -> F134
// 0x14062ef87  483b7820                       cmpq	0x20(%rax), %rdi
// 0x14062ef8b  7510                           jne	0x14062ef9d <.text+0x62df9d>
// Frames: F140 -> F136 -> F137 -> F133 -> F134
// 0x14062ef8d  488b4dd8                       movq	-0x28(%rbp), %rcx
// 0x14062ef91  488d15d83d5402                 leaq	0x2543dd8(%rip), %rdx   # 0x142b72d70
// 0x14062ef98  e8133ef3ff                     callq	0x140562db0 <.text+0x561db0>
// Frames: F41 -> F42 -> F43 -> F141 -> F142 -> F136 -> F137 -> F133 -> F134
// 0x14062ef9d  488b5560                       movq	0x60(%rbp), %rdx
// 0x14062efa1  488b4228                       movq	0x28(%rdx), %rax
// Frames: F143 -> F142 -> F136 -> F137 -> F133 -> F134
// 0x14062efa5  488d0c7f                       leaq	(%rdi,%rdi,2), %rcx
// 0x14062efa9  4c8b45c8                       movq	-0x38(%rbp), %r8
// Frames: F144 -> F145 -> F136 -> F137 -> F133 -> F134
// 0x14062efad  4c8904c8                       movq	%r8, (%rax,%rcx,8)
// 0x14062efb1  488974c808                     movq	%rsi, 0x8(%rax,%rcx,8)
// 0x14062efb6  c644c81000                     movb	$0x0, 0x10(%rax,%rcx,8)
// Frames: F146 -> F136 -> F137 -> F133 -> F134
// 0x14062efbb  48ffc7                         incq	%rdi
// 0x14062efbe  48897a30                       movq	%rdi, 0x30(%rdx)
// Frames: F147 -> F137 -> F133 -> F134
// 0x14062efc2  48ff4240                       incq	0x40(%rdx)
// 0x14062efc6  8b4540                         movl	0x40(%rbp), %eax
// 0x14062efc9  488b4db8                       movq	-0x48(%rbp), %rcx
// Frames: F131 -> F132 -> F148
// 0x14062efcd  8901                           movl	%eax, (%rcx)
// Frames: F149
// 0x14062efcf  498b3e                         movq	(%r14), %rdi
// Frames: F150
// 0x14062efd2  410f104630                     movups	0x30(%r14), %xmm0
// 0x14062efd7  0f1145ec                       movups	%xmm0, -0x14(%rbp)
// 0x14062efdb  410f104624                     movups	0x24(%r14), %xmm0
// 0x14062efe0  0f2945e0                       movaps	%xmm0, -0x20(%rbp)
// Frames: F151
// 0x14062efe4  418b7650                       movl	0x50(%r14), %esi
// Frames: F152
// 0x14062efe8  498b4640                       movq	0x40(%r14), %rax
// 0x14062efec  48894518                       movq	%rax, 0x18(%rbp)
// 0x14062eff0  418b4648                       movl	0x48(%r14), %eax
// 0x14062eff4  894520                         movl	%eax, 0x20(%rbp)
// Frames: F153 -> F154 -> F155 -> F156 -> F157 -> F158 -> F159 -> F148
// 0x14062eff7  0fb6050b5c0003                 movzbl	0x3005c0b(%rip), %eax   # 0x143634c09
// Frames: F160 -> F155 -> F156 -> F157 -> F158 -> F159 -> F148
// 0x14062effe  b950000000                     movl	$0x50, %ecx
// 0x14062f003  ba08000000                     movl	$0x8, %edx
// 0x14062f008  e8034cfeff                     callq	0x140613c10 <.text+0x612c10>
// Frames: F157 -> F158 -> F159 -> F148
// 0x14062f00d  4885c0                         testq	%rax, %rax
// 0x14062f010  0f8456030000                   je	0x14062f36c <.text+0x62e36c>
// Frames: F158 -> F159 -> F148
// 0x14062f016  48c70001000000                 movq	$0x1, (%rax)
// 0x14062f01d  48c7400801000000               movq	$0x1, 0x8(%rax)
// 0x14062f025  c6401001                       movb	$0x1, 0x10(%rax)
// 0x14062f029  897014                         movl	%esi, 0x14(%rax)
// 0x14062f02c  488b4d18                       movq	0x18(%rbp), %rcx
// 0x14062f030  48894818                       movq	%rcx, 0x18(%rax)
// 0x14062f034  8b4d20                         movl	0x20(%rbp), %ecx
// 0x14062f037  894820                         movl	%ecx, 0x20(%rax)
// 0x14062f03a  0f2845e0                       movaps	-0x20(%rbp), %xmm0
// 0x14062f03e  0f114024                       movups	%xmm0, 0x24(%rax)
// 0x14062f042  0f1045ec                       movups	-0x14(%rbp), %xmm0
// 0x14062f046  0f114030                       movups	%xmm0, 0x30(%rax)
// 0x14062f04a  48897840                       movq	%rdi, 0x40(%rax)
// Frames: F148
// 0x14062f04e  488945e8                       movq	%rax, -0x18(%rbp)
// 0x14062f052  48c745e008000000               movq	$0x8, -0x20(%rbp)
// 0x14062f05a  488b4dc0                       movq	-0x40(%rbp), %rcx
// 0x14062f05e  488d55e0                       leaq	-0x20(%rbp), %rdx
// 0x14062f062  4c8d0507d73d02                 leaq	0x23dd707(%rip), %r8    # 0x142a0c770
// 0x14062f069  e8525e4500                     callq	0x140a84ec0 <.text+0xa83ec0>
// Frames: F53
// 0x14062f06e  4c8b7578                       movq	0x78(%rbp), %r14
// 0x14062f072  4c8b4d28                       movq	0x28(%rbp), %r9
// 0x14062f076  4c8b5dd0                       movq	-0x30(%rbp), %r11
// 0x14062f07a  8b7d3c                         movl	0x3c(%rbp), %edi
// 0x14062f07d  e9a7fcffff                     jmp	0x14062ed29 <.text+0x62dd29>
// 0x14062f082  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// Frames: F58 -> F125 -> F126 -> F127 -> F53
// 0x14062f090  4c8b7578                       movq	0x78(%rbp), %r14
// 0x14062f094  e990fcffff                     jmp	0x14062ed29 <.text+0x62dd29>
// Frames: F161 -> F22 -> F4
// 0x14062f099  488908                         movq	%rcx, (%rax)
// Frames: F36 -> F37 -> F38 -> F39 -> F162
// 0x14062f09c  488b0f                         movq	(%rdi), %rcx
// 0x14062f09f  4c8b7708                       movq	0x8(%rdi), %r14
// Frames: F41 -> F42 -> F43 -> F44 -> F12 -> F13 -> F45 -> F46 -> F47 -> F39 -> F162
// 0x14062f0a3  4c8bb9f8000000                 movq	0xf8(%rcx), %r15
// Frames: F12 -> F13 -> F45 -> F46 -> F47 -> F39 -> F162
// 0x14062f0aa  488b8100010000                 movq	0x100(%rcx), %rax
// Frames: F29 -> F30 -> F48 -> F45 -> F46 -> F47 -> F39 -> F162
// 0x14062f0b1  4d8d2487                       leaq	(%r15,%rax,4), %r12
// 0x14062f0b5  458b4d1c                       movl	0x1c(%r13), %r9d
// 0x14062f0b9  4d8b5500                       movq	(%r13), %r10
// 0x14062f0bd  4d8b5d10                       movq	0x10(%r13), %r11
// 0x14062f0c1  498d4220                       leaq	0x20(%r10), %rax
// 0x14062f0c5  48894560                       movq	%rax, 0x60(%rbp)
// 0x14062f0c9  48894d78                       movq	%rcx, 0x78(%rbp)
// 0x14062f0cd  80b90801000000                 cmpb	$0x0, 0x108(%rcx)
// 0x14062f0d4  44894d50                       movl	%r9d, 0x50(%rbp)
// 0x14062f0d8  4c895558                       movq	%r10, 0x58(%rbp)
// 0x14062f0dc  4c895d70                       movq	%r11, 0x70(%rbp)
// 0x14062f0e0  0f8429010000                   je	0x14062f20f <.text+0x62e20f>
// 0x14062f0e6  4531ed                         xorl	%r13d, %r13d
// 0x14062f0e9  31db                           xorl	%ebx, %ebx
// 0x14062f0eb  31d2                           xorl	%edx, %edx
// 0x14062f0ed  eb25                           jmp	0x14062f114 <.text+0x62e114>
// 0x14062f0ef  90                             nop
// Frames: F41 -> F42 -> F43 -> F141 -> F142 -> F136 -> F163 -> F133 -> F164
// 0x14062f0f0  48ffc3                         incq	%rbx
// 0x14062f0f3  498b4228                       movq	0x28(%r10), %rax
// Frames: F143 -> F142 -> F136 -> F163 -> F133 -> F164
// 0x14062f0f7  488d0c76                       leaq	(%rsi,%rsi,2), %rcx
// Frames: F144 -> F145 -> F136 -> F163 -> F133 -> F164
// 0x14062f0fb  48893cc8                       movq	%rdi, (%rax,%rcx,8)
// 0x14062f0ff  4c8944c808                     movq	%r8, 0x8(%rax,%rcx,8)
// 0x14062f104  c644c81000                     movb	$0x0, 0x10(%rax,%rcx,8)
// Frames: F146 -> F136 -> F163 -> F133 -> F164
// 0x14062f109  48ffc6                         incq	%rsi
// 0x14062f10c  49897230                       movq	%rsi, 0x30(%r10)
// Frames: F147 -> F163 -> F133 -> F164
// 0x14062f110  49ff4240                       incq	0x40(%r10)
// Frames: F52 -> F51 -> F162
// 0x14062f114  4c39eb                         cmpq	%r13, %rbx
// 0x14062f117  7576                           jne	0x14062f18f <.text+0x62e18f>
// 0x14062f119  0f1f8000000000                 nopl	(%rax)
// Frames: F55 -> F56 -> F106 -> F51 -> F162
// 0x14062f120  4d39e7                         cmpq	%r12, %r15
// Frames: F58 -> F106 -> F51 -> F162
// 0x14062f123  0f84d1000000                   je	0x14062f1fa <.text+0x62e1fa>
// Frames: F106 -> F51 -> F162
// 0x14062f129  418b0f                         movl	(%r15), %ecx
// Frames: F59 -> F60 -> F61 -> F62 -> F106 -> F51 -> F162
// 0x14062f12c  4983c704                       addq	$0x4, %r15
// Frames: F63 -> F18 -> F107 -> F108 -> F51 -> F162
// 0x14062f130  498b86a8010000                 movq	0x1a8(%r14), %rax
// 0x14062f137  488d0cc9                       leaq	(%rcx,%rcx,8), %rcx
// Frames: F66 -> F67 -> F109 -> F110 -> F51 -> F162
// 0x14062f13b  4c8b6cc810                     movq	0x10(%rax,%rcx,8), %r13
// Frames: F110 -> F51 -> F162
// 0x14062f140  4d85ed                         testq	%r13, %r13
// 0x14062f143  74db                           je	0x14062f120 <.text+0x62e120>
// Frames: F111 -> F112 -> F51 -> F162
// 0x14062f145  488d0cc8                       leaq	(%rax,%rcx,8), %rcx
// 0x14062f149  488b4578                       movq	0x78(%rbp), %rax
// 0x14062f14d  488b9010010000                 movq	0x110(%rax), %rdx
// Frames: F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F162
// 0x14062f154  488b4140                       movq	0x40(%rcx), %rax
// Frames: F63 -> F18 -> F75 -> F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F162
// 0x14062f158  4839d0                         cmpq	%rdx, %rax
// 0x14062f15b  7623                           jbe	0x14062f180 <.text+0x62e180>
// Frames: F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F162
// 0x14062f15d  4c8b4138                       movq	0x38(%rcx), %r8
// Frames: F81 -> F82 -> F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F162
// 0x14062f161  498b14d0                       movq	(%r8,%rdx,8), %rdx
// 0x14062f165  4885d2                         testq	%rdx, %rdx
// Frames: F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F162
// 0x14062f168  7416                           je	0x14062f180 <.text+0x62e180>
// Frames: F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F162
// 0x14062f16a  488b4918                       movq	0x18(%rcx), %rcx
// Frames: F84 -> F85 -> F86 -> F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F162
// 0x14062f16e  48f7d2                         notq	%rdx
// Frames: F87 -> F88 -> F89 -> F86 -> F90 -> F79 -> F111 -> F112 -> F51 -> F162
// 0x14062f171  488d1452                       leaq	(%rdx,%rdx,2), %rdx
// 0x14062f175  48c1e204                       shlq	$0x4, %rdx
// 0x14062f179  488b541110                     movq	0x10(%rcx,%rdx), %rdx
// 0x14062f17e  eb02                           jmp	0x14062f182 <.text+0x62e182>
// 0x14062f180  31d2                           xorl	%edx, %edx
// Frames: F63 -> F18 -> F93 -> F94 -> F114 -> F115 -> F97 -> F113 -> F111 -> F112 -> F51 -> F162
// 0x14062f182  488b4d78                       movq	0x78(%rbp), %rcx
// 0x14062f186  483b8118010000                 cmpq	0x118(%rcx), %rax
// 0x14062f18d  31db                           xorl	%ebx, %ebx
// Frames: F116 -> F117 -> F118 -> F119 -> F120 -> F121 -> F51 -> F162
// 0x14062f18f  89d8                           movl	%ebx, %eax
// Frames: F29 -> F165 -> F117 -> F118 -> F119 -> F120 -> F121 -> F51 -> F162
// 0x14062f191  486bc058                       imulq	$0x58, %rax, %rax
// Frames: F166
// 0x14062f195  4c8b0402                       movq	(%rdx,%rax), %r8
// Frames: F131 -> F167 -> F133 -> F164
// 0x14062f199  45890b                         movl	%r9d, (%r11)
// Frames: F135 -> F136 -> F163 -> F133 -> F164
// 0x14062f19c  498b7230                       movq	0x30(%r10), %rsi
// Frames: F138 -> F163 -> F133 -> F164
// 0x14062f1a0  498b7a40                       movq	0x40(%r10), %rdi
// Frames: F139 -> F136 -> F163 -> F133 -> F164
// 0x14062f1a4  493b7220                       cmpq	0x20(%r10), %rsi
// 0x14062f1a8  0f8542ffffff                   jne	0x14062f0f0 <.text+0x62e0f0>
// Frames: F140 -> F136 -> F163 -> F133 -> F164
// 0x14062f1ae  488b4d60                       movq	0x60(%rbp), %rcx
// 0x14062f1b2  48895548                       movq	%rdx, 0x48(%rbp)
// 0x14062f1b6  488d15b33b5402                 leaq	0x2543bb3(%rip), %rdx   # 0x142b72d70
// 0x14062f1bd  4c894568                       movq	%r8, 0x68(%rbp)
// 0x14062f1c1  e8ea3bf3ff                     callq	0x140562db0 <.text+0x561db0>
// 0x14062f1c6  4c8b4568                       movq	0x68(%rbp), %r8
// 0x14062f1ca  488b5548                       movq	0x48(%rbp), %rdx
// 0x14062f1ce  4c8b5d70                       movq	0x70(%rbp), %r11
// 0x14062f1d2  4c8b5558                       movq	0x58(%rbp), %r10
// 0x14062f1d6  448b4d50                       movl	0x50(%rbp), %r9d
// 0x14062f1da  e911ffffff                     jmp	0x14062f0f0 <.text+0x62e0f0>
// Frames: F168 -> F169 -> F170 -> F170 -> F171
// 0x14062f1df  488b55a0                       movq	-0x60(%rbp), %rdx
// Frames: F172 -> F168 -> F169 -> F170 -> F170 -> F171
// 0x14062f1e3  4885d2                         testq	%rdx, %rdx
// 0x14062f1e6  7412                           je	0x14062f1fa <.text+0x62e1fa>
// Frames: F173 -> F174 -> F168 -> F169 -> F170 -> F170 -> F171
// 0x14062f1e8  48c1e203                       shlq	$0x3, %rdx
// Frames: F175 -> F176 -> F177 -> F169 -> F170 -> F170 -> F171
// 0x14062f1ec  41b808000000                   movl	$0x8, %r8d
// 0x14062f1f2  4c89c9                         movq	%r9, %rcx
// 0x14062f1f5  e8264afeff                     callq	0x140613c20 <.text+0x612c20>
// Frames: F178
// 0x14062f1fa  90                             nop
// 0x14062f1fb  4881c408010000                 addq	$0x108, %rsp            # imm = 0x108
// 0x14062f202  5b                             popq	%rbx
// 0x14062f203  5f                             popq	%rdi
// 0x14062f204  5e                             popq	%rsi
// 0x14062f205  415c                           popq	%r12
// 0x14062f207  415d                           popq	%r13
// 0x14062f209  415e                           popq	%r14
// 0x14062f20b  415f                           popq	%r15
// 0x14062f20d  5d                             popq	%rbp
// 0x14062f20e  c3                             retq
// 0x14062f20f  ba08000000                     movl	$0x8, %edx
// 0x14062f214  4531ed                         xorl	%r13d, %r13d
// 0x14062f217  31f6                           xorl	%esi, %esi
// 0x14062f219  4531c0                         xorl	%r8d, %r8d
// 0x14062f21c  eb2a                           jmp	0x14062f248 <.text+0x62e248>
// 0x14062f21e  6690                           nop
// Frames: F41 -> F42 -> F43 -> F141 -> F142 -> F136 -> F163 -> F133 -> F164
// 0x14062f220  48ffc6                         incq	%rsi
// 0x14062f223  498b4228                       movq	0x28(%r10), %rax
// Frames: F143 -> F142 -> F136 -> F163 -> F133 -> F164
// 0x14062f227  488d0c5b                       leaq	(%rbx,%rbx,2), %rcx
// Frames: F144 -> F145 -> F136 -> F163 -> F133 -> F164
// 0x14062f22b  48893cc8                       movq	%rdi, (%rax,%rcx,8)
// 0x14062f22f  488b7d48                       movq	0x48(%rbp), %rdi
// 0x14062f233  48897cc808                     movq	%rdi, 0x8(%rax,%rcx,8)
// 0x14062f238  c644c81000                     movb	$0x0, 0x10(%rax,%rcx,8)
// Frames: F146 -> F136 -> F163 -> F133 -> F164
// 0x14062f23d  48ffc3                         incq	%rbx
// 0x14062f240  49895a30                       movq	%rbx, 0x30(%r10)
// Frames: F147 -> F163 -> F133 -> F164
// 0x14062f244  49ff4240                       incq	0x40(%r10)
// 0x14062f248  4c39ee                         cmpq	%r13, %rsi
// 0x14062f24b  0f85be000000                   jne	0x14062f30f <.text+0x62e30f>
// 0x14062f251  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F55 -> F56 -> F57 -> F51 -> F162
// 0x14062f260  4d39e7                         cmpq	%r12, %r15
// Frames: F58 -> F57 -> F51 -> F162
// 0x14062f263  7495                           je	0x14062f1fa <.text+0x62e1fa>
// Frames: F57 -> F51 -> F162
// 0x14062f265  418b0f                         movl	(%r15), %ecx
// Frames: F59 -> F60 -> F61 -> F62 -> F57 -> F51 -> F162
// 0x14062f268  4983c704                       addq	$0x4, %r15
// Frames: F63 -> F18 -> F64 -> F65 -> F51 -> F162
// 0x14062f26c  498b8600010000                 movq	0x100(%r14), %rax
// 0x14062f273  488d0c89                       leaq	(%rcx,%rcx,4), %rcx
// 0x14062f277  48c1e105                       shlq	$0x5, %rcx
// Frames: F66 -> F67 -> F68 -> F69 -> F51 -> F162
// 0x14062f27b  4c8b6c0858                     movq	0x58(%rax,%rcx), %r13
// Frames: F69 -> F51 -> F162
// 0x14062f280  4d85ed                         testq	%r13, %r13
// 0x14062f283  74db                           je	0x14062f260 <.text+0x62e260>
// Frames: F70 -> F71 -> F51 -> F162
// 0x14062f285  4801c8                         addq	%rcx, %rax
// 0x14062f288  8b8894000000                   movl	0x94(%rax), %ecx
// Frames: F63 -> F18 -> F72 -> F71 -> F51 -> F162
// 0x14062f28e  498b96a8010000                 movq	0x1a8(%r14), %rdx
// 0x14062f295  4c8d04c9                       leaq	(%rcx,%rcx,8), %r8
// 0x14062f299  488b4d78                       movq	0x78(%rbp), %rcx
// Frames: F73 -> F74 -> F51 -> F162
// 0x14062f29d  488b8910010000                 movq	0x110(%rcx), %rcx
// Frames: F63 -> F18 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F162
// 0x14062f2a4  4a394cc240                     cmpq	%rcx, 0x40(%rdx,%r8,8)
// 0x14062f2a9  763b                           jbe	0x14062f2e6 <.text+0x62e2e6>
// Frames: F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F162
// 0x14062f2ab  4a8d14c2                       leaq	(%rdx,%r8,8), %rdx
// 0x14062f2af  4c8b4238                       movq	0x38(%rdx), %r8
// Frames: F81 -> F82 -> F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F162
// 0x14062f2b3  498b0cc8                       movq	(%r8,%rcx,8), %rcx
// 0x14062f2b7  4885c9                         testq	%rcx, %rcx
// Frames: F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F162
// 0x14062f2ba  742a                           je	0x14062f2e6 <.text+0x62e2e6>
// Frames: F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F162
// 0x14062f2bc  488b5218                       movq	0x18(%rdx), %rdx
// Frames: F84 -> F85 -> F86 -> F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F162
// 0x14062f2c0  48f7d1                         notq	%rcx
// Frames: F87 -> F88 -> F89 -> F86 -> F90 -> F79 -> F80 -> F73 -> F74 -> F51 -> F162
// 0x14062f2c3  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x14062f2c7  48c1e104                       shlq	$0x4, %rcx
// 0x14062f2cb  4c8b440a10                     movq	0x10(%rdx,%rcx), %r8
// 0x14062f2d0  488b4d78                       movq	0x78(%rbp), %rcx
// Frames: F91 -> F92 -> F73 -> F74 -> F51 -> F162
// 0x14062f2d4  488b8918010000                 movq	0x118(%rcx), %rcx
// Frames: F63 -> F18 -> F93 -> F94 -> F95 -> F96 -> F97 -> F92 -> F73 -> F74 -> F51 -> F162
// 0x14062f2db  48398888000000                 cmpq	%rcx, 0x88(%rax)
// 0x14062f2e2  7719                           ja	0x14062f2fd <.text+0x62e2fd>
// 0x14062f2e4  eb23                           jmp	0x14062f309 <.text+0x62e309>
// Frames: F91 -> F92 -> F73 -> F74 -> F51 -> F162
// 0x14062f2e6  4531c0                         xorl	%r8d, %r8d
// 0x14062f2e9  488b4d78                       movq	0x78(%rbp), %rcx
// 0x14062f2ed  488b8918010000                 movq	0x118(%rcx), %rcx
// Frames: F63 -> F18 -> F93 -> F94 -> F95 -> F96 -> F97 -> F92 -> F73 -> F74 -> F51 -> F162
// 0x14062f2f4  48398888000000                 cmpq	%rcx, 0x88(%rax)
// 0x14062f2fb  760c                           jbe	0x14062f309 <.text+0x62e309>
// Frames: F94 -> F95 -> F96 -> F97 -> F92 -> F73 -> F74 -> F51 -> F162
// 0x14062f2fd  488b9080000000                 movq	0x80(%rax), %rdx
// Frames: F98 -> F82 -> F99 -> F93 -> F94 -> F95 -> F96 -> F97 -> F92 -> F73 -> F74 -> F51 -> F162
// 0x14062f304  48833cca00                     cmpq	$0x0, (%rdx,%rcx,8)
// Frames: F41 -> F42 -> F43 -> F44 -> F12 -> F13 -> F101 -> F102 -> F51 -> F162
// 0x14062f309  488b5050                       movq	0x50(%rax), %rdx
// 0x14062f30d  31f6                           xorl	%esi, %esi
// Frames: F103 -> F104 -> F51 -> F162
// 0x14062f30f  4889f0                         movq	%rsi, %rax
// 0x14062f312  48c1e004                       shlq	$0x4, %rax
// Frames: F103 -> F105 -> F51 -> F162
// 0x14062f316  8b440208                       movl	0x8(%rdx,%rax), %eax
// Frames: F29 -> F122 -> F117 -> F118 -> F119 -> F120 -> F179 -> F51 -> F162
// 0x14062f31a  486bc058                       imulq	$0x58, %rax, %rax
// Frames: F166
// 0x14062f31e  498b0400                       movq	(%r8,%rax), %rax
// 0x14062f322  48894548                       movq	%rax, 0x48(%rbp)
// Frames: F131 -> F167 -> F133 -> F164
// 0x14062f326  45890b                         movl	%r9d, (%r11)
// Frames: F135 -> F136 -> F163 -> F133 -> F164
// 0x14062f329  498b5a30                       movq	0x30(%r10), %rbx
// Frames: F138 -> F163 -> F133 -> F164
// 0x14062f32d  498b7a40                       movq	0x40(%r10), %rdi
// Frames: F139 -> F136 -> F163 -> F133 -> F164
// 0x14062f331  493b5a20                       cmpq	0x20(%r10), %rbx
// 0x14062f335  0f85e5feffff                   jne	0x14062f220 <.text+0x62e220>
// Frames: F140 -> F136 -> F163 -> F133 -> F164
// 0x14062f33b  488b4d60                       movq	0x60(%rbp), %rcx
// 0x14062f33f  48895540                       movq	%rdx, 0x40(%rbp)
// 0x14062f343  488d15263a5402                 leaq	0x2543a26(%rip), %rdx   # 0x142b72d70
// 0x14062f34a  4c894568                       movq	%r8, 0x68(%rbp)
// 0x14062f34e  e85d3af3ff                     callq	0x140562db0 <.text+0x561db0>
// 0x14062f353  488b5540                       movq	0x40(%rbp), %rdx
// 0x14062f357  4c8b4568                       movq	0x68(%rbp), %r8
// 0x14062f35b  4c8b5d70                       movq	0x70(%rbp), %r11
// 0x14062f35f  4c8b5558                       movq	0x58(%rbp), %r10
// 0x14062f363  448b4d50                       movl	0x50(%rbp), %r9d
// 0x14062f367  e9b4feffff                     jmp	0x14062f220 <.text+0x62e220>
// Frames: F180 -> F158 -> F159 -> F148
// 0x14062f36c  b908000000                     movl	$0x8, %ecx
// 0x14062f371  ba50000000                     movl	$0x50, %edx
// 0x14062f376  e8859c2a02                     callq	0x1428d9000 <.text+0x28d8000>
// 0x14062f37b  0f0b                           ud2
// 0x14062f37d  0f1f00                         nopl	(%rax)
// 0x14062f380  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14062f385  55                             pushq	%rbp
// 0x14062f386  4157                           pushq	%r15
// 0x14062f388  4156                           pushq	%r14
// 0x14062f38a  4155                           pushq	%r13
// 0x14062f38c  4154                           pushq	%r12
// 0x14062f38e  56                             pushq	%rsi
// 0x14062f38f  57                             pushq	%rdi
// 0x14062f390  53                             pushq	%rbx
// 0x14062f391  4883ec28                       subq	$0x28, %rsp
// 0x14062f395  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F168 -> F169 -> F170 -> F170 -> F171
// 0x14062f39c  488b55a0                       movq	-0x60(%rbp), %rdx
// Frames: F172 -> F168 -> F169 -> F170 -> F170 -> F171
// 0x14062f3a0  4885d2                         testq	%rdx, %rdx
// 0x14062f3a3  7413                           je	0x14062f3b8 <.text+0x62e3b8>
// Frames: F173 -> F174 -> F168 -> F169 -> F170 -> F170 -> F171
// 0x14062f3a5  48c1e203                       shlq	$0x3, %rdx
// Frames: F175 -> F176 -> F177 -> F169 -> F170 -> F170 -> F171
// 0x14062f3a9  41b808000000                   movl	$0x8, %r8d
// 0x14062f3af  488b4d28                       movq	0x28(%rbp), %rcx
// 0x14062f3b3  e86848feff                     callq	0x140613c20 <.text+0x612c20>
// Frames: F1
// 0x14062f3b8  90                             nop
// 0x14062f3b9  4883c428                       addq	$0x28, %rsp
// 0x14062f3bd  5b                             popq	%rbx
// 0x14062f3be  5f                             popq	%rdi
// 0x14062f3bf  5e                             popq	%rsi
// 0x14062f3c0  415c                           popq	%r12
// 0x14062f3c2  415d                           popq	%r13
// 0x14062f3c4  415e                           popq	%r14
// 0x14062f3c6  415f                           popq	%r15
// 0x14062f3c8  5d                             popq	%rbp
// 0x14062f3c9  c3                             retq
//
// RANGE rva_62f3d0_len_4ea: RVA 0x62f3d0, 1258 PDB bytes; 323 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_62f3d0_len_4ea.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 270:10664: system_roof::delete_roof::mark_roof_for_deletion
// Frames: F181
// 0x14062f3d0  55                             pushq	%rbp
// 0x14062f3d1  4157                           pushq	%r15
// 0x14062f3d3  4156                           pushq	%r14
// 0x14062f3d5  4155                           pushq	%r13
// 0x14062f3d7  4154                           pushq	%r12
// 0x14062f3d9  56                             pushq	%rsi
// 0x14062f3da  57                             pushq	%rdi
// 0x14062f3db  53                             pushq	%rbx
// 0x14062f3dc  4881ecc8000000                 subq	$0xc8, %rsp
// 0x14062f3e3  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14062f3eb  48c74540feffffff               movq	$-0x2, 0x40(%rbp)
// 0x14062f3f3  4c894d38                       movq	%r9, 0x38(%rbp)
// 0x14062f3f7  4c894508                       movq	%r8, 0x8(%rbp)
// Frames: F182 -> F183
// 0x14062f3fb  48c7451000000000               movq	$0x0, 0x10(%rbp)
// 0x14062f403  48c7451808000000               movq	$0x8, 0x18(%rbp)
// 0x14062f40b  48c7452000000000               movq	$0x0, 0x20(%rbp)
// Frames: F2 -> F3 -> F184
// 0x14062f413  4c8b21                         movq	(%rcx), %r12
// Frames: F5 -> F3 -> F184
// 0x14062f416  4c8b4108                       movq	0x8(%rcx), %r8
// Frames: F6 -> F7 -> F8 -> F3 -> F184
// 0x14062f41a  4d8b0c24                       movq	(%r12), %r9
// 0x14062f41e  4531ed                         xorl	%r13d, %r13d
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F184
// 0x14062f421  4c89c9                         movq	%r9, %rcx
// 0x14062f424  492b4818                       subq	0x18(%r8), %rcx
// 0x14062f428  490f42cd                       cmovbq	%r13, %rcx
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F184
// 0x14062f42c  4d2b4838                       subq	0x38(%r8), %r9
// 0x14062f430  4d0f42cd                       cmovbq	%r13, %r9
// Frames: F15 -> F16 -> F17 -> F18 -> F23 -> F7 -> F8 -> F3 -> F184
// 0x14062f434  488d0449                       leaq	(%rcx,%rcx,2), %rax
// 0x14062f438  48c1e003                       shlq	$0x3, %rax
// 0x14062f43c  49034008                       addq	0x8(%r8), %rax
// Frames: F12 -> F13 -> F23 -> F7 -> F8 -> F3 -> F184
// 0x14062f440  4d8b5010                       movq	0x10(%r8), %r10
// Frames: F15 -> F16 -> F17 -> F18 -> F23 -> F7 -> F8 -> F3 -> F184
// 0x14062f444  4929ca                         subq	%rcx, %r10
// 0x14062f447  4d0f42d5                       cmovbq	%r13, %r10
// 0x14062f44b  48895530                       movq	%rdx, 0x30(%rbp)
// 0x14062f44f  ba08000000                     movl	$0x8, %edx
// 0x14062f454  480f42c2                       cmovbq	%rdx, %rax
// Frames: F15 -> F16 -> F17 -> F18 -> F14 -> F7 -> F8 -> F3 -> F184
// 0x14062f458  4b8d0c49                       leaq	(%r9,%r9,2), %rcx
// 0x14062f45c  48c1e103                       shlq	$0x3, %rcx
// 0x14062f460  49034828                       addq	0x28(%r8), %rcx
// Frames: F12 -> F13 -> F14 -> F7 -> F8 -> F3 -> F184
// 0x14062f464  4d8b5830                       movq	0x30(%r8), %r11
// Frames: F15 -> F16 -> F17 -> F18 -> F14 -> F7 -> F8 -> F3 -> F184
// 0x14062f468  4d29cb                         subq	%r9, %r11
// 0x14062f46b  4d0f42dd                       cmovbq	%r13, %r11
// 0x14062f46f  480f42ca                       cmovbq	%rdx, %rcx
// Frames: F24 -> F7 -> F8 -> F3 -> F184
// 0x14062f473  498b5040                       movq	0x40(%r8), %rdx
// 0x14062f477  4c29da                         subq	%r11, %rdx
// 0x14062f47a  4c29d2                         subq	%r10, %rdx
// 0x14062f47d  49891424                       movq	%rdx, (%r12)
// Frames: F29 -> F30 -> F31 -> F32 -> F7 -> F8 -> F3 -> F184
// 0x14062f481  4b8d1452                       leaq	(%r10,%r10,2), %rdx
// 0x14062f485  488d1cd0                       leaq	(%rax,%rdx,8), %rbx
// 0x14062f489  4b8d145b                       leaq	(%r11,%r11,2), %rdx
// 0x14062f48d  4c8d34d1                       leaq	(%rcx,%rdx,8), %r14
// 0x14062f491  eb32                           jmp	0x14062f4c5 <.text+0x62e4c5>
// 0x14062f493  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F140 -> F185
// 0x14062f4a0  488d4d10                       leaq	0x10(%rbp), %rcx
// 0x14062f4a4  488d15ddd23d02                 leaq	0x23dd2dd(%rip), %rdx   # 0x142a0c788
// 0x14062f4ab  e8c033f3ff                     callq	0x140562870 <.text+0x561870>
// Frames: F41 -> F42 -> F43 -> F141 -> F142 -> F185
// 0x14062f4b0  488b4518                       movq	0x18(%rbp), %rax
// Frames: F144 -> F145 -> F185
// 0x14062f4b4  4e893ce8                       movq	%r15, (%rax,%r13,8)
// Frames: F146 -> F185
// 0x14062f4b8  49ffc5                         incq	%r13
// 0x14062f4bb  4c896d20                       movq	%r13, 0x20(%rbp)
// 0x14062f4bf  4889f9                         movq	%rdi, %rcx
// 0x14062f4c2  4889f0                         movq	%rsi, %rax
// Frames: F186 -> F187 -> F188 -> F189 -> F190 -> F184
// 0x14062f4c5  4885c0                         testq	%rax, %rax
// 0x14062f4c8  7426                           je	0x14062f4f0 <.text+0x62e4f0>
// 0x14062f4ca  4839d8                         cmpq	%rbx, %rax
// 0x14062f4cd  7421                           je	0x14062f4f0 <.text+0x62e4f0>
// Frames: F191 -> F190 -> F184
// 0x14062f4cf  488d7018                       leaq	0x18(%rax), %rsi
// 0x14062f4d3  4889cf                         movq	%rcx, %rdi
// 0x14062f4d6  49ff0424                       incq	(%r12)
// Frames: F185
// 0x14062f4da  4c8b7808                       movq	0x8(%rax), %r15
// Frames: F139 -> F185
// 0x14062f4de  4c3b6d10                       cmpq	0x10(%rbp), %r13
// 0x14062f4e2  75cc                           jne	0x14062f4b0 <.text+0x62e4b0>
// 0x14062f4e4  ebba                           jmp	0x14062f4a0 <.text+0x62e4a0>
// 0x14062f4e6  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F186 -> F192 -> F193 -> F188 -> F189 -> F190 -> F184
// 0x14062f4f0  4885c9                         testq	%rcx, %rcx
// 0x14062f4f3  741e                           je	0x14062f513 <.text+0x62e513>
// 0x14062f4f5  4c39f1                         cmpq	%r14, %rcx
// 0x14062f4f8  7419                           je	0x14062f513 <.text+0x62e513>
// Frames: F59 -> F60 -> F61 -> F62 -> F192 -> F193 -> F188 -> F189 -> F190 -> F184
// 0x14062f4fa  488d7918                       leaq	0x18(%rcx), %rdi
// 0x14062f4fe  31f6                           xorl	%esi, %esi
// 0x14062f500  4889c8                         movq	%rcx, %rax
// Frames: F191 -> F190 -> F184
// 0x14062f503  49ff0424                       incq	(%r12)
// Frames: F185
// 0x14062f507  4c8b7808                       movq	0x8(%rax), %r15
// Frames: F139 -> F185
// 0x14062f50b  4c3b6d10                       cmpq	0x10(%rbp), %r13
// 0x14062f50f  759f                           jne	0x14062f4b0 <.text+0x62e4b0>
// 0x14062f511  eb8d                           jmp	0x14062f4a0 <.text+0x62e4a0>
// Frames: F36 -> F37 -> F194 -> F195 -> F196
// 0x14062f513  4c8b4d30                       movq	0x30(%rbp), %r9
// 0x14062f517  498b09                         movq	(%r9), %rcx
// Frames: F41 -> F42 -> F43 -> F44 -> F12 -> F13 -> F45 -> F46 -> F197 -> F195 -> F196
// 0x14062f51a  4c8ba9f8000000                 movq	0xf8(%rcx), %r13
// Frames: F12 -> F13 -> F45 -> F46 -> F197 -> F195 -> F196
// 0x14062f521  488b8100010000                 movq	0x100(%rcx), %rax
// 0x14062f528  4c8b4508                       movq	0x8(%rbp), %r8
// 0x14062f52c  498b5008                       movq	0x8(%r8), %rdx
// 0x14062f530  48895528                       movq	%rdx, 0x28(%rbp)
// 0x14062f534  4885d2                         testq	%rdx, %rdx
// 0x14062f537  7508                           jne	0x14062f541 <.text+0x62e541>
// 0x14062f539  498b5010                       movq	0x10(%r8), %rdx
// 0x14062f53d  48895528                       movq	%rdx, 0x28(%rbp)
// 0x14062f541  4d8b7108                       movq	0x8(%r9), %r14
// 0x14062f545  488d1c8500000000               leaq	(,%rax,4), %rbx
// 0x14062f54d  4c01eb                         addq	%r13, %rbx
// 0x14062f550  48894df8                       movq	%rcx, -0x8(%rbp)
// 0x14062f554  0fb68108010000                 movzbl	0x108(%rcx), %eax
// 0x14062f55b  884508                         movb	%al, 0x8(%rbp)
// 0x14062f55e  488b4d38                       movq	0x38(%rbp), %rcx
// 0x14062f562  8b411c                         movl	0x1c(%rcx), %eax
// 0x14062f565  894504                         movl	%eax, 0x4(%rbp)
// 0x14062f568  488b01                         movq	(%rcx), %rax
// 0x14062f56b  488b4910                       movq	0x10(%rcx), %rcx
// 0x14062f56f  48894dd8                       movq	%rcx, -0x28(%rbp)
// 0x14062f573  48894530                       movq	%rax, 0x30(%rbp)
// 0x14062f577  4883c020                       addq	$0x20, %rax
// 0x14062f57b  488945e0                       movq	%rax, -0x20(%rbp)
// 0x14062f57f  b808000000                     movl	$0x8, %eax
// 0x14062f584  488945e8                       movq	%rax, -0x18(%rbp)
// 0x14062f588  48c7453800000000               movq	$0x0, 0x38(%rbp)
// 0x14062f590  488945f0                       movq	%rax, -0x10(%rbp)
// 0x14062f594  31ff                           xorl	%edi, %edi
// 0x14062f596  4531d2                         xorl	%r10d, %r10d
// 0x14062f599  eb3b                           jmp	0x14062f5d6 <.text+0x62e5d6>
// 0x14062f59b  0f1f440000                     nopl	(%rax,%rax)
// Frames: F41 -> F42 -> F43 -> F141 -> F142 -> F136 -> F163 -> F133 -> F198
// 0x14062f5a0  488b5530                       movq	0x30(%rbp), %rdx
// 0x14062f5a4  488b4228                       movq	0x28(%rdx), %rax
// Frames: F143 -> F142 -> F136 -> F163 -> F133 -> F198
// 0x14062f5a8  488d0cb6                       leaq	(%rsi,%rsi,4), %rcx
// Frames: F144 -> F145 -> F136 -> F163 -> F133 -> F198
// 0x14062f5ac  4c893cc8                       movq	%r15, (%rax,%rcx,8)
// 0x14062f5b0  c644c80801                     movb	$0x1, 0x8(%rax,%rcx,8)
// 0x14062f5b5  0f1045a9                       movups	-0x57(%rbp), %xmm0
// 0x14062f5b9  0f104db4                       movups	-0x4c(%rbp), %xmm1
// 0x14062f5bd  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// 0x14062f5c2  0f1144c809                     movups	%xmm0, 0x9(%rax,%rcx,8)
// Frames: F146 -> F136 -> F163 -> F133 -> F198
// 0x14062f5c7  48ffc6                         incq	%rsi
// 0x14062f5ca  48897230                       movq	%rsi, 0x30(%rdx)
// Frames: F147 -> F163 -> F133 -> F198
// 0x14062f5ce  48ff4240                       incq	0x40(%rdx)
// 0x14062f5d2  4c8b55d0                       movq	-0x30(%rbp), %r10
// Frames: F50 -> F51 -> F196
// 0x14062f5d6  807d0800                       cmpb	$0x0, 0x8(%rbp)
// 0x14062f5da  0f8480000000                   je	0x14062f660 <.text+0x62e660>
// Frames: F52 -> F51 -> F196
// 0x14062f5e0  4939fa                         cmpq	%rdi, %r10
// 0x14062f5e3  0f8515010000                   jne	0x14062f6fe <.text+0x62e6fe>
// 0x14062f5e9  0f1f8000000000                 nopl	(%rax)
// Frames: F55 -> F56 -> F106 -> F51 -> F196
// 0x14062f5f0  4939dd                         cmpq	%rbx, %r13
// Frames: F58 -> F106 -> F51 -> F196
// 0x14062f5f3  0f8442020000                   je	0x14062f83b <.text+0x62e83b>
// Frames: F106 -> F51 -> F196
// 0x14062f5f9  418b4d00                       movl	(%r13), %ecx
// Frames: F59 -> F60 -> F61 -> F62 -> F106 -> F51 -> F196
// 0x14062f5fd  4983c504                       addq	$0x4, %r13
// Frames: F63 -> F18 -> F107 -> F108 -> F51 -> F196
// 0x14062f601  498b86a8010000                 movq	0x1a8(%r14), %rax
// 0x14062f608  488d0cc9                       leaq	(%rcx,%rcx,8), %rcx
// Frames: F66 -> F67 -> F109 -> F110 -> F51 -> F196
// 0x14062f60c  488b7cc810                     movq	0x10(%rax,%rcx,8), %rdi
// Frames: F110 -> F51 -> F196
// 0x14062f611  4885ff                         testq	%rdi, %rdi
// 0x14062f614  74da                           je	0x14062f5f0 <.text+0x62e5f0>
// Frames: F112 -> F51 -> F196
// 0x14062f616  488d04c8                       leaq	(%rax,%rcx,8), %rax
// 0x14062f61a  488b4df8                       movq	-0x8(%rbp), %rcx
// 0x14062f61e  488b8910010000                 movq	0x110(%rcx), %rcx
// Frames: F63 -> F18 -> F75 -> F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F196
// 0x14062f625  48394840                       cmpq	%rcx, 0x40(%rax)
// 0x14062f629  0f86bc000000                   jbe	0x14062f6eb <.text+0x62e6eb>
// Frames: F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F196
// 0x14062f62f  488b5038                       movq	0x38(%rax), %rdx
// Frames: F81 -> F82 -> F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F196
// 0x14062f633  488b0cca                       movq	(%rdx,%rcx,8), %rcx
// 0x14062f637  4885c9                         testq	%rcx, %rcx
// Frames: F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F196
// 0x14062f63a  0f84ab000000                   je	0x14062f6eb <.text+0x62e6eb>
// Frames: F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F196
// 0x14062f640  488b5018                       movq	0x18(%rax), %rdx
// Frames: F84 -> F85 -> F86 -> F76 -> F77 -> F78 -> F79 -> F111 -> F112 -> F51 -> F196
// 0x14062f644  48f7d1                         notq	%rcx
// Frames: F87 -> F88 -> F89 -> F86 -> F90 -> F79 -> F111 -> F112 -> F51 -> F196
// 0x14062f647  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x14062f64b  48c1e104                       shlq	$0x4, %rcx
// 0x14062f64f  488b4c0a10                     movq	0x10(%rdx,%rcx), %rcx
// 0x14062f654  48894d38                       movq	%rcx, 0x38(%rbp)
// 0x14062f658  e996000000                     jmp	0x14062f6f3 <.text+0x62e6f3>
// 0x14062f65d  0f1f00                         nopl	(%rax)
// Frames: F54 -> F51 -> F196
// 0x14062f660  4939fa                         cmpq	%rdi, %r10
// 0x14062f663  0f85b5000000                   jne	0x14062f71e <.text+0x62e71e>
// 0x14062f669  0f1f8000000000                 nopl	(%rax)
// Frames: F55 -> F56 -> F57 -> F51 -> F196
// 0x14062f670  4939dd                         cmpq	%rbx, %r13
// Frames: F58 -> F57 -> F51 -> F196
// 0x14062f673  0f84c2010000                   je	0x14062f83b <.text+0x62e83b>
// Frames: F57 -> F51 -> F196
// 0x14062f679  418b4d00                       movl	(%r13), %ecx
// Frames: F59 -> F60 -> F61 -> F62 -> F57 -> F51 -> F196
// 0x14062f67d  4983c504                       addq	$0x4, %r13
// Frames: F63 -> F18 -> F64 -> F65 -> F51 -> F196
// 0x14062f681  498b8600010000                 movq	0x100(%r14), %rax
// 0x14062f688  488d0c89                       leaq	(%rcx,%rcx,4), %rcx
// 0x14062f68c  48c1e105                       shlq	$0x5, %rcx
// Frames: F66 -> F67 -> F68 -> F69 -> F51 -> F196
// 0x14062f690  488b7c0858                     movq	0x58(%rax,%rcx), %rdi
// Frames: F69 -> F51 -> F196
// 0x14062f695  4885ff                         testq	%rdi, %rdi
// 0x14062f698  74d6                           je	0x14062f670 <.text+0x62e670>
// Frames: F70 -> F71 -> F51 -> F196
// 0x14062f69a  4801c8                         addq	%rcx, %rax
// 0x14062f69d  8b8894000000                   movl	0x94(%rax), %ecx
// Frames: F63 -> F18 -> F72 -> F71 -> F51 -> F196
// 0x14062f6a3  498b96a8010000                 movq	0x1a8(%r14), %rdx
// 0x14062f6aa  4c8d04c9                       leaq	(%rcx,%rcx,8), %r8
// 0x14062f6ae  488b4df8                       movq	-0x8(%rbp), %rcx
// Frames: F74 -> F51 -> F196
// 0x14062f6b2  488b8910010000                 movq	0x110(%rcx), %rcx
// Frames: F63 -> F18 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F196
// 0x14062f6b9  4a394cc240                     cmpq	%rcx, 0x40(%rdx,%r8,8)
// 0x14062f6be  764b                           jbe	0x14062f70b <.text+0x62e70b>
// Frames: F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F196
// 0x14062f6c0  4a8d14c2                       leaq	(%rdx,%r8,8), %rdx
// 0x14062f6c4  4c8b4238                       movq	0x38(%rdx), %r8
// Frames: F81 -> F82 -> F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F196
// 0x14062f6c8  498b0cc8                       movq	(%r8,%rcx,8), %rcx
// 0x14062f6cc  4885c9                         testq	%rcx, %rcx
// Frames: F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F196
// 0x14062f6cf  743a                           je	0x14062f70b <.text+0x62e70b>
// Frames: F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F196
// 0x14062f6d1  488b5218                       movq	0x18(%rdx), %rdx
// Frames: F84 -> F85 -> F86 -> F76 -> F77 -> F78 -> F79 -> F80 -> F73 -> F74 -> F51 -> F196
// 0x14062f6d5  48f7d1                         notq	%rcx
// Frames: F87 -> F88 -> F89 -> F86 -> F90 -> F79 -> F80 -> F73 -> F74 -> F51 -> F196
// 0x14062f6d8  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x14062f6dc  48c1e104                       shlq	$0x4, %rcx
// 0x14062f6e0  488b4c0a10                     movq	0x10(%rdx,%rcx), %rcx
// 0x14062f6e5  48894d38                       movq	%rcx, 0x38(%rbp)
// 0x14062f6e9  eb28                           jmp	0x14062f713 <.text+0x62e713>
// 0x14062f6eb  48c7453800000000               movq	$0x0, 0x38(%rbp)
// Frames: F41 -> F42 -> F43 -> F44 -> F12 -> F13 -> F199 -> F200 -> F51 -> F196
// 0x14062f6f3  488b4008                       movq	0x8(%rax), %rax
// 0x14062f6f7  488945e8                       movq	%rax, -0x18(%rbp)
// 0x14062f6fb  4531d2                         xorl	%r10d, %r10d
// Frames: F201 -> F202 -> F203 -> F204 -> F51 -> F196
// 0x14062f6fe  488b45e8                       movq	-0x18(%rbp), %rax
// 0x14062f702  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F116 -> F117 -> F118 -> F119 -> F120 -> F121 -> F51 -> F196
// 0x14062f706  4489d1                         movl	%r10d, %ecx
// 0x14062f709  eb26                           jmp	0x14062f731 <.text+0x62e731>
// 0x14062f70b  48c7453800000000               movq	$0x0, 0x38(%rbp)
// Frames: F41 -> F42 -> F43 -> F44 -> F12 -> F13 -> F101 -> F102 -> F51 -> F196
// 0x14062f713  488b4050                       movq	0x50(%rax), %rax
// 0x14062f717  488945f0                       movq	%rax, -0x10(%rbp)
// 0x14062f71b  4531d2                         xorl	%r10d, %r10d
// Frames: F201 -> F202 -> F203 -> F205 -> F51 -> F196
// 0x14062f71e  4c89d1                         movq	%r10, %rcx
// 0x14062f721  48c1e104                       shlq	$0x4, %rcx
// 0x14062f725  488b55f0                       movq	-0x10(%rbp), %rdx
// 0x14062f729  488d040a                       leaq	(%rdx,%rcx), %rax
// Frames: F103 -> F105 -> F51 -> F196
// 0x14062f72d  8b4c0a08                       movl	0x8(%rdx,%rcx), %ecx
// Frames: F29 -> F122 -> F117 -> F118 -> F119 -> F120 -> F123 -> F51 -> F196
// 0x14062f731  4c6bf958                       imulq	$0x58, %rcx, %r15
// 0x14062f735  4c037d38                       addq	0x38(%rbp), %r15
// 0x14062f739  488b30                         movq	(%rax), %rsi
// Frames: F196
// 0x14062f73c  4889f0                         movq	%rsi, %rax
// 0x14062f73f  48c1e820                       shrq	$0x20, %rax
// 0x14062f743  0f84f2000000                   je	0x14062f83b <.text+0x62e83b>
// Frames: F206
// 0x14062f749  49ffc2                         incq	%r10
// 0x14062f74c  488b4518                       movq	0x18(%rbp), %rax
// 0x14062f750  488b4d20                       movq	0x20(%rbp), %rcx
// 0x14062f754  498b17                         movq	(%r15), %rdx
// Frames: F125 -> F126 -> F127 -> F206
// 0x14062f757  48c1e103                       shlq	$0x3, %rcx
// 0x14062f75b  4531c0                         xorl	%r8d, %r8d
// 0x14062f75e  6690                           nop
// Frames: F55 -> F56 -> F125 -> F126 -> F127 -> F206
// 0x14062f760  4c39c1                         cmpq	%r8, %rcx
// Frames: F58 -> F125 -> F126 -> F127 -> F206
// 0x14062f763  0f846dfeffff                   je	0x14062f5d6 <.text+0x62e5d6>
// Frames: F128 -> F126 -> F127 -> F206
// 0x14062f769  4d8d4808                       leaq	0x8(%r8), %r9
// Frames: F129 -> F130 -> F128 -> F126 -> F127 -> F206
// 0x14062f76d  4a3b1400                       cmpq	(%rax,%r8), %rdx
// 0x14062f771  4d89c8                         movq	%r9, %r8
// Frames: F128 -> F126 -> F127 -> F206
// 0x14062f774  75ea                           jne	0x14062f760 <.text+0x62e760>
// Frames: F207 -> F208 -> F209 -> F210 -> F211 -> F212 -> F213 -> F214 -> F215 -> F216
// 0x14062f776  4c8955d0                       movq	%r10, -0x30(%rbp)
// 0x14062f77a  488b4d28                       movq	0x28(%rbp), %rcx
// 0x14062f77e  488b01                         movq	(%rcx), %rax
// 0x14062f781  4c8b6110                       movq	0x10(%rcx), %r12
// Frames: F217 -> F208 -> F209 -> F210 -> F211 -> F212 -> F213 -> F214 -> F215 -> F216
// 0x14062f785  4c29e0                         subq	%r12, %rax
// Frames: F208 -> F209 -> F210 -> F211 -> F212 -> F213 -> F214 -> F215 -> F216
// 0x14062f788  4883f81f                       cmpq	$0x1f, %rax
// Frames: F209 -> F210 -> F211 -> F212 -> F213 -> F214 -> F215 -> F216
// 0x14062f78c  0f8683000000                   jbe	0x14062f815 <.text+0x62e815>
// Frames: F218
// 0x14062f792  488b4d28                       movq	0x28(%rbp), %rcx
// 0x14062f796  488b4108                       movq	0x8(%rcx), %rax
// 0x14062f79a  488d150ff87900                 leaq	0x79f80f(%rip), %rdx    # 0x140dcefb0 <.text+0xdcdfb0>
// 0x14062f7a1  4a891420                       movq	%rdx, (%rax,%r12)
// 0x14062f7a5  488d15d4027a00                 leaq	0x7a02d4(%rip), %rdx    # 0x140dcfa80 <.text+0xdcea80>
// 0x14062f7ac  4a89542008                     movq	%rdx, 0x8(%rax,%r12)
// 0x14062f7b1  4a89742010                     movq	%rsi, 0x10(%rax,%r12)
// 0x14062f7b6  42c644201800                   movb	$0x0, 0x18(%rax,%r12)
// 0x14062f7bc  4983c420                       addq	$0x20, %r12
// 0x14062f7c0  4c896110                       movq	%r12, 0x10(%rcx)
// 0x14062f7c4  488d4dc4                       leaq	-0x3c(%rbp), %rcx
// 0x14062f7c8  4c89fa                         movq	%r15, %rdx
// 0x14062f7cb  e8503a2b00                     callq	0x1408e3220 <.text+0x8e2220>
// Frames: F198
// 0x14062f7d0  8b45cc                         movl	-0x34(%rbp), %eax
// 0x14062f7d3  488d4db8                       leaq	-0x48(%rbp), %rcx
// 0x14062f7d7  894108                         movl	%eax, 0x8(%rcx)
// 0x14062f7da  488b45c4                       movq	-0x3c(%rbp), %rax
// 0x14062f7de  488901                         movq	%rax, (%rcx)
// 0x14062f7e1  8b4504                         movl	0x4(%rbp), %eax
// 0x14062f7e4  488b4dd8                       movq	-0x28(%rbp), %rcx
// Frames: F131 -> F167 -> F133 -> F198
// 0x14062f7e8  8901                           movl	%eax, (%rcx)
// 0x14062f7ea  488b4530                       movq	0x30(%rbp), %rax
// Frames: F135 -> F136 -> F163 -> F133 -> F198
// 0x14062f7ee  488b7030                       movq	0x30(%rax), %rsi
// Frames: F138 -> F163 -> F133 -> F198
// 0x14062f7f2  4c8b7840                       movq	0x40(%rax), %r15
// Frames: F139 -> F136 -> F163 -> F133 -> F198
// 0x14062f7f6  483b7020                       cmpq	0x20(%rax), %rsi
// 0x14062f7fa  0f85a0fdffff                   jne	0x14062f5a0 <.text+0x62e5a0>
// Frames: F140 -> F136 -> F163 -> F133 -> F198
// 0x14062f800  488b4de0                       movq	-0x20(%rbp), %rcx
// 0x14062f804  488d1565355402                 leaq	0x2543565(%rip), %rdx   # 0x142b72d70
// 0x14062f80b  e8a032f3ff                     callq	0x140562ab0 <.text+0x561ab0>
// 0x14062f810  e98bfdffff                     jmp	0x14062f5a0 <.text+0x62e5a0>
// Frames: F219 -> F210 -> F211 -> F212 -> F213 -> F214 -> F215 -> F216
// 0x14062f815  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x14062f81e  41b820000000                   movl	$0x20, %r8d
// 0x14062f824  41b901000000                   movl	$0x1, %r9d
// 0x14062f82a  488b4d28                       movq	0x28(%rbp), %rcx
// 0x14062f82e  4c89e2                         movq	%r12, %rdx
// 0x14062f831  e89ab42a02                     callq	0x1428dacd0 <.text+0x28d9cd0>
// 0x14062f836  e957ffffff                     jmp	0x14062f792 <.text+0x62e792>
// Frames: F168 -> F169 -> F170 -> F170 -> F220
// 0x14062f83b  488b5510                       movq	0x10(%rbp), %rdx
// Frames: F172 -> F168 -> F169 -> F170 -> F170 -> F220
// 0x14062f83f  4885d2                         testq	%rdx, %rdx
// 0x14062f842  7413                           je	0x14062f857 <.text+0x62e857>
// Frames: F168 -> F169 -> F170 -> F170 -> F220
// 0x14062f844  488b4d18                       movq	0x18(%rbp), %rcx
// Frames: F173 -> F174 -> F168 -> F169 -> F170 -> F170 -> F220
// 0x14062f848  48c1e203                       shlq	$0x3, %rdx
// Frames: F175 -> F176 -> F177 -> F169 -> F170 -> F170 -> F220
// 0x14062f84c  41b808000000                   movl	$0x8, %r8d
// 0x14062f852  e8c943feff                     callq	0x140613c20 <.text+0x612c20>
// Frames: F220
// 0x14062f857  90                             nop
// 0x14062f858  4881c4c8000000                 addq	$0xc8, %rsp
// 0x14062f85f  5b                             popq	%rbx
// 0x14062f860  5f                             popq	%rdi
// 0x14062f861  5e                             popq	%rsi
// 0x14062f862  415c                           popq	%r12
// 0x14062f864  415d                           popq	%r13
// 0x14062f866  415e                           popq	%r14
// 0x14062f868  415f                           popq	%r15
// 0x14062f86a  5d                             popq	%rbp
// 0x14062f86b  c3                             retq
// 0x14062f86c  0f1f4000                       nopl	(%rax)
// 0x14062f870  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14062f875  55                             pushq	%rbp
// 0x14062f876  4157                           pushq	%r15
// 0x14062f878  4156                           pushq	%r14
// 0x14062f87a  4155                           pushq	%r13
// 0x14062f87c  4154                           pushq	%r12
// 0x14062f87e  56                             pushq	%rsi
// 0x14062f87f  57                             pushq	%rdi
// 0x14062f880  53                             pushq	%rbx
// 0x14062f881  4883ec28                       subq	$0x28, %rsp
// 0x14062f885  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F168 -> F169 -> F170 -> F170 -> F220
// 0x14062f88c  488b5510                       movq	0x10(%rbp), %rdx
// Frames: F172 -> F168 -> F169 -> F170 -> F170 -> F220
// 0x14062f890  4885d2                         testq	%rdx, %rdx
// 0x14062f893  7413                           je	0x14062f8a8 <.text+0x62e8a8>
// Frames: F168 -> F169 -> F170 -> F170 -> F220
// 0x14062f895  488b4d18                       movq	0x18(%rbp), %rcx
// Frames: F173 -> F174 -> F168 -> F169 -> F170 -> F170 -> F220
// 0x14062f899  48c1e203                       shlq	$0x3, %rdx
// Frames: F175 -> F176 -> F177 -> F169 -> F170 -> F170 -> F220
// 0x14062f89d  41b808000000                   movl	$0x8, %r8d
// 0x14062f8a3  e87843feff                     callq	0x140613c20 <.text+0x612c20>
// Frames: F181
// 0x14062f8a8  90                             nop
// 0x14062f8a9  4883c428                       addq	$0x28, %rsp
// 0x14062f8ad  5b                             popq	%rbx
// 0x14062f8ae  5f                             popq	%rdi
// 0x14062f8af  5e                             popq	%rsi
// 0x14062f8b0  415c                           popq	%r12
// 0x14062f8b2  415d                           popq	%r13
// 0x14062f8b4  415e                           popq	%r14
// 0x14062f8b6  415f                           popq	%r15
// 0x14062f8b8  5d                             popq	%rbp
// 0x14062f8b9  c3                             retq
//
// RANGE rva_62f8c0_len_3f0: RVA 0x62f8c0, 1008 PDB bytes; 254 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_62f8c0_len_3f0.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 270:16556: system_roof::delete_roof::delete_roofs
// Frames: F221
// 0x14062f8c0  4157                           pushq	%r15
// 0x14062f8c2  4156                           pushq	%r14
// 0x14062f8c4  4155                           pushq	%r13
// 0x14062f8c6  4154                           pushq	%r12
// 0x14062f8c8  56                             pushq	%rsi
// 0x14062f8c9  57                             pushq	%rdi
// 0x14062f8ca  55                             pushq	%rbp
// 0x14062f8cb  53                             pushq	%rbx
// 0x14062f8cc  4881ec98000000                 subq	$0x98, %rsp
// 0x14062f8d3  0f29b42480000000               movaps	%xmm6, 0x80(%rsp)
// Frames: F36 -> F37 -> F194 -> F195 -> F222
// 0x14062f8db  488b19                         movq	(%rcx), %rbx
// 0x14062f8de  4c8b7108                       movq	0x8(%rcx), %r14
// Frames: F41 -> F42 -> F43 -> F44 -> F12 -> F13 -> F45 -> F46 -> F197 -> F195 -> F222
// 0x14062f8e2  4c8bbbf8000000                 movq	0xf8(%rbx), %r15
// Frames: F12 -> F13 -> F45 -> F46 -> F197 -> F195 -> F222
// 0x14062f8e9  488b8300010000                 movq	0x100(%rbx), %rax
// Frames: F29 -> F30 -> F48 -> F45 -> F46 -> F197 -> F195 -> F222
// 0x14062f8f0  4d8d2487                       leaq	(%r15,%rax,4), %r12
// 0x14062f8f4  488b02                         movq	(%rdx), %rax
// 0x14062f8f7  4889442430                     movq	%rax, 0x30(%rsp)
// 0x14062f8fc  488b6a08                       movq	0x8(%rdx), %rbp
// 0x14062f900  488b4218                       movq	0x18(%rdx), %rax
// 0x14062f904  4889442450                     movq	%rax, 0x50(%rsp)
// 0x14062f909  488b4210                       movq	0x10(%rdx), %rax
// 0x14062f90d  4889442448                     movq	%rax, 0x48(%rsp)
// 0x14062f912  80bb0801000000                 cmpb	$0x0, 0x108(%rbx)
// 0x14062f919  48896c2440                     movq	%rbp, 0x40(%rsp)
// 0x14062f91e  48895c2438                     movq	%rbx, 0x38(%rsp)
// 0x14062f923  0f8482010000                   je	0x14062faab <.text+0x62eaab>
// 0x14062f929  be08000000                     movl	$0x8, %esi
// 0x14062f92e  4531ed                         xorl	%r13d, %r13d
// 0x14062f931  0f57f6                         xorps	%xmm6, %xmm6
// 0x14062f934  488d1565ce3d02                 leaq	0x23dce65(%rip), %rdx   # 0x142a0c7a0
// 0x14062f93b  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x14062f940  31ff                           xorl	%edi, %edi
// 0x14062f942  4531c9                         xorl	%r9d, %r9d
// 0x14062f945  4531d2                         xorl	%r10d, %r10d
// 0x14062f948  4531db                         xorl	%r11d, %r11d
// Frames: F52 -> F51 -> F222
// 0x14062f94b  4c39ef                         cmpq	%r13, %rdi
// 0x14062f94e  7570                           jne	0x14062f9c0 <.text+0x62e9c0>
// Frames: F55 -> F56 -> F106 -> F51 -> F222
// 0x14062f950  4d39e7                         cmpq	%r12, %r15
// Frames: F58 -> F106 -> F51 -> F222
// 0x14062f953  0f843b030000                   je	0x14062fc94 <.text+0x62ec94>
// Frames: F106 -> F51 -> F222
// 0x14062f959  458b07                         movl	(%r15), %r8d
// Frames: F59 -> F60 -> F61 -> F62 -> F106 -> F51 -> F222
// 0x14062f95c  4983c704                       addq	$0x4, %r15
// Frames: F63 -> F18 -> F107 -> F108 -> F51 -> F222
// 0x14062f960  498b86a8010000                 movq	0x1a8(%r14), %rax
// 0x14062f967  4f8d04c0                       leaq	(%r8,%r8,8), %r8
// Frames: F66 -> F67 -> F109 -> F110 -> F51 -> F222
// 0x14062f96b  4e8b6cc010                     movq	0x10(%rax,%r8,8), %r13
// Frames: F110 -> F51 -> F222
// 0x14062f970  4d85ed                         testq	%r13, %r13
// 0x14062f973  74db                           je	0x14062f950 <.text+0x62e950>
// Frames: F76 -> F77 -> F78 -> F79 -> F223 -> F112 -> F51 -> F222
// 0x14062f975  4a8d04c0                       leaq	(%rax,%r8,8), %rax
// 0x14062f979  488b7038                       movq	0x38(%rax), %rsi
// 0x14062f97d  4c8b5040                       movq	0x40(%rax), %r10
// Frames: F91 -> F113 -> F223 -> F112 -> F51 -> F222
// 0x14062f981  4c8b8318010000                 movq	0x118(%rbx), %r8
// Frames: F63 -> F18 -> F93 -> F94 -> F114 -> F115 -> F97 -> F113 -> F223 -> F112 -> F51 -> F222
// 0x14062f988  4d39c2                         cmpq	%r8, %r10
// 0x14062f98b  7645                           jbe	0x14062f9d2 <.text+0x62e9d2>
// Frames: F99 -> F93 -> F94 -> F114 -> F115 -> F97 -> F113 -> F223 -> F112 -> F51 -> F222
// 0x14062f98d  4e8b04c6                       movq	(%rsi,%r8,8), %r8
// Frames: F98 -> F82 -> F99 -> F93 -> F94 -> F114 -> F115 -> F97 -> F113 -> F223 -> F112 -> F51 -> F222
// 0x14062f991  4d85c0                         testq	%r8, %r8
// Frames: F124 -> F223 -> F112 -> F51 -> F222
// 0x14062f994  743c                           je	0x14062f9d2 <.text+0x62e9d2>
// Frames: F76 -> F77 -> F78 -> F79 -> F224 -> F223 -> F112 -> F51 -> F222
// 0x14062f996  4c8b4818                       movq	0x18(%rax), %r9
// Frames: F84 -> F85 -> F86 -> F76 -> F77 -> F78 -> F79 -> F224 -> F223 -> F112 -> F51 -> F222
// 0x14062f99a  49f7d0                         notq	%r8
// Frames: F87 -> F88 -> F89 -> F86 -> F90 -> F79 -> F224 -> F223 -> F112 -> F51 -> F222
// 0x14062f99d  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x14062f9a1  49c1e004                       shlq	$0x4, %r8
// 0x14062f9a5  4f8b4c0110                     movq	0x10(%r9,%r8), %r9
// 0x14062f9aa  40b701                         movb	$0x1, %dil
// Frames: F223 -> F112 -> F51 -> F222
// 0x14062f9ad  4c8b8320010000                 movq	0x120(%rbx), %r8
// Frames: F63 -> F18 -> F75 -> F76 -> F77 -> F78 -> F79 -> F223 -> F112 -> F51 -> F222
// 0x14062f9b4  4d39c2                         cmpq	%r8, %r10
// 0x14062f9b7  7727                           ja	0x14062f9e0 <.text+0x62e9e0>
// 0x14062f9b9  4531db                         xorl	%r11d, %r11d
// 0x14062f9bc  eb4a                           jmp	0x14062fa08 <.text+0x62ea08>
// 0x14062f9be  6690                           nop
// Frames: F225
// 0x14062f9c0  4889f8                         movq	%rdi, %rax
// 0x14062f9c3  488d7801                       leaq	0x1(%rax), %rdi
// 0x14062f9c7  41f6c201                       testb	$0x1, %r10b
// 0x14062f9cb  7552                           jne	0x14062fa1f <.text+0x62ea1f>
// 0x14062f9cd  e979ffffff                     jmp	0x14062f94b <.text+0x62e94b>
// Frames: F223 -> F112 -> F51 -> F222
// 0x14062f9d2  31ff                           xorl	%edi, %edi
// 0x14062f9d4  4c8b8320010000                 movq	0x120(%rbx), %r8
// Frames: F63 -> F18 -> F75 -> F76 -> F77 -> F78 -> F79 -> F223 -> F112 -> F51 -> F222
// 0x14062f9db  4d39c2                         cmpq	%r8, %r10
// 0x14062f9de  76d9                           jbe	0x14062f9b9 <.text+0x62e9b9>
// Frames: F81 -> F82 -> F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F223 -> F112 -> F51 -> F222
// 0x14062f9e0  4d89ca                         movq	%r9, %r10
// 0x14062f9e3  4e8b0cc6                       movq	(%rsi,%r8,8), %r9
// 0x14062f9e7  4d85c9                         testq	%r9, %r9
// Frames: F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F223 -> F112 -> F51 -> F222
// 0x14062f9ea  7416                           je	0x14062fa02 <.text+0x62ea02>
// Frames: F76 -> F77 -> F78 -> F79 -> F223 -> F112 -> F51 -> F222
// 0x14062f9ec  4c8b4018                       movq	0x18(%rax), %r8
// Frames: F84 -> F85 -> F86 -> F76 -> F77 -> F78 -> F79 -> F223 -> F112 -> F51 -> F222
// 0x14062f9f0  49f7d1                         notq	%r9
// Frames: F87 -> F88 -> F89 -> F86 -> F90 -> F79 -> F223 -> F112 -> F51 -> F222
// 0x14062f9f3  4f8d0c49                       leaq	(%r9,%r9,2), %r9
// 0x14062f9f7  49c1e104                       shlq	$0x4, %r9
// 0x14062f9fb  4f8b5c0810                     movq	0x10(%r8,%r9), %r11
// 0x14062fa00  eb03                           jmp	0x14062fa05 <.text+0x62ea05>
// 0x14062fa02  4531db                         xorl	%r11d, %r11d
// 0x14062fa05  4d89d1                         movq	%r10, %r9
// Frames: F41 -> F42 -> F43 -> F44 -> F12 -> F13 -> F199 -> F200 -> F51 -> F222
// 0x14062fa08  488b7008                       movq	0x8(%rax), %rsi
// 0x14062fa0c  31c0                           xorl	%eax, %eax
// 0x14062fa0e  4189fa                         movl	%edi, %r10d
// 0x14062fa11  488d7801                       leaq	0x1(%rax), %rdi
// Frames: F225
// 0x14062fa15  41f6c201                       testb	$0x1, %r10b
// 0x14062fa19  0f842cffffff                   je	0x14062f94b <.text+0x62e94b>
// 0x14062fa1f  4d85c9                         testq	%r9, %r9
// 0x14062fa22  0f8423ffffff                   je	0x14062f94b <.text+0x62e94b>
// Frames: F116 -> F117 -> F118 -> F119 -> F226 -> F121 -> F51 -> F222
// 0x14062fa28  4189c0                         movl	%eax, %r8d
// Frames: F225
// 0x14062fa2b  f3430f1004c3                   movss	(%r11,%r8,8), %xmm0
// 0x14062fa31  0f2ec6                         ucomiss	%xmm6, %xmm0
// 0x14062fa34  0f8511ffffff                   jne	0x14062f94b <.text+0x62e94b>
// 0x14062fa3a  0f8a0bffffff                   jp	0x14062f94b <.text+0x62e94b>
// Frames: F227 -> F228
// 0x14062fa40  488b04c6                       movq	(%rsi,%rax,8), %rax
// 0x14062fa44  4889442478                     movq	%rax, 0x78(%rsp)
// 0x14062fa49  488b442430                     movq	0x30(%rsp), %rax
// 0x14062fa4e  4889442458                     movq	%rax, 0x58(%rsp)
// 0x14062fa53  48896c2460                     movq	%rbp, 0x60(%rsp)
// 0x14062fa58  488b442448                     movq	0x48(%rsp), %rax
// 0x14062fa5d  4889442468                     movq	%rax, 0x68(%rsp)
// 0x14062fa62  488b442450                     movq	0x50(%rsp), %rax
// 0x14062fa67  4889442470                     movq	%rax, 0x70(%rsp)
// 0x14062fa6c  4889742428                     movq	%rsi, 0x28(%rsp)
// 0x14062fa71  4889d6                         movq	%rdx, %rsi
// 0x14062fa74  4c89cd                         movq	%r9, %rbp
// 0x14062fa77  4489542420                     movl	%r10d, 0x20(%rsp)
// 0x14062fa7c  4c89db                         movq	%r11, %rbx
// Frames: F228
// 0x14062fa7f  e8ec4c6e00                     callq	0x140d14770 <.text+0xd13770>
// 0x14062fa84  4989db                         movq	%rbx, %r11
// 0x14062fa87  448b542420                     movl	0x20(%rsp), %r10d
// 0x14062fa8c  4989e9                         movq	%rbp, %r9
// 0x14062fa8f  4889f2                         movq	%rsi, %rdx
// 0x14062fa92  488b742428                     movq	0x28(%rsp), %rsi
// 0x14062fa97  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x14062fa9c  488b5c2438                     movq	0x38(%rsp), %rbx
// 0x14062faa1  488b6c2440                     movq	0x40(%rsp), %rbp
// Frames: F225
// 0x14062faa6  e9a0feffff                     jmp	0x14062f94b <.text+0x62e94b>
// 0x14062faab  41b908000000                   movl	$0x8, %r9d
// 0x14062fab1  4531ed                         xorl	%r13d, %r13d
// 0x14062fab4  0f57f6                         xorps	%xmm6, %xmm6
// 0x14062fab7  488d15e2cc3d02                 leaq	0x23dcce2(%rip), %rdx   # 0x142a0c7a0
// 0x14062fabe  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x14062fac3  31f6                           xorl	%esi, %esi
// 0x14062fac5  4531d2                         xorl	%r10d, %r10d
// 0x14062fac8  31ff                           xorl	%edi, %edi
// 0x14062faca  48c744242000000000             movq	$0x0, 0x20(%rsp)
// Frames: F54 -> F51 -> F222
// 0x14062fad3  4c39ee                         cmpq	%r13, %rsi
// 0x14062fad6  0f85a4000000                   jne	0x14062fb80 <.text+0x62eb80>
// 0x14062fadc  0f1f4000                       nopl	(%rax)
// Frames: F55 -> F56 -> F57 -> F51 -> F222
// 0x14062fae0  4d39e7                         cmpq	%r12, %r15
// Frames: F58 -> F57 -> F51 -> F222
// 0x14062fae3  0f84ab010000                   je	0x14062fc94 <.text+0x62ec94>
// Frames: F57 -> F51 -> F222
// 0x14062fae9  458b07                         movl	(%r15), %r8d
// Frames: F59 -> F60 -> F61 -> F62 -> F57 -> F51 -> F222
// 0x14062faec  4983c704                       addq	$0x4, %r15
// Frames: F63 -> F18 -> F64 -> F65 -> F51 -> F222
// 0x14062faf0  498b8600010000                 movq	0x100(%r14), %rax
// 0x14062faf7  4f8d0480                       leaq	(%r8,%r8,4), %r8
// 0x14062fafb  49c1e005                       shlq	$0x5, %r8
// Frames: F66 -> F67 -> F68 -> F69 -> F51 -> F222
// 0x14062faff  4e8b6c0058                     movq	0x58(%rax,%r8), %r13
// Frames: F69 -> F51 -> F222
// 0x14062fb04  4d85ed                         testq	%r13, %r13
// 0x14062fb07  74d7                           je	0x14062fae0 <.text+0x62eae0>
// Frames: F70 -> F71 -> F51 -> F222
// 0x14062fb09  4c01c0                         addq	%r8, %rax
// 0x14062fb0c  448b8894000000                 movl	0x94(%rax), %r9d
// Frames: F63 -> F18 -> F72 -> F71 -> F51 -> F222
// 0x14062fb13  4d8b86a8010000                 movq	0x1a8(%r14), %r8
// 0x14062fb1a  4f8d0cc9                       leaq	(%r9,%r9,8), %r9
// 0x14062fb1e  4f8d1cc8                       leaq	(%r8,%r9,8), %r11
// Frames: F76 -> F77 -> F78 -> F79 -> F80 -> F229 -> F74 -> F51 -> F222
// 0x14062fb22  4b8b74c838                     movq	0x38(%r8,%r9,8), %rsi
// 0x14062fb27  4f8b44c840                     movq	0x40(%r8,%r9,8), %r8
// Frames: F91 -> F92 -> F229 -> F74 -> F51 -> F222
// 0x14062fb2c  4c8b8b18010000                 movq	0x118(%rbx), %r9
// Frames: F63 -> F18 -> F93 -> F94 -> F95 -> F96 -> F97 -> F92 -> F229 -> F74 -> F51 -> F222
// 0x14062fb33  4c398888000000                 cmpq	%r9, 0x88(%rax)
// 0x14062fb3a  7656                           jbe	0x14062fb92 <.text+0x62eb92>
// Frames: F94 -> F95 -> F96 -> F97 -> F92 -> F229 -> F74 -> F51 -> F222
// 0x14062fb3c  4c89d7                         movq	%r10, %rdi
// 0x14062fb3f  4c8b9080000000                 movq	0x80(%rax), %r10
// Frames: F98 -> F82 -> F99 -> F93 -> F94 -> F95 -> F96 -> F97 -> F92 -> F229 -> F74 -> F51 -> F222
// 0x14062fb46  4b833cca00                     cmpq	$0x0, (%r10,%r9,8)
// Frames: F100 -> F229 -> F74 -> F51 -> F222
// 0x14062fb4b  7466                           je	0x14062fbb3 <.text+0x62ebb3>
// Frames: F63 -> F18 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F230 -> F229 -> F74 -> F51 -> F222
// 0x14062fb4d  40b701                         movb	$0x1, %dil
// 0x14062fb50  4d39c8                         cmpq	%r9, %r8
// 0x14062fb53  764d                           jbe	0x14062fba2 <.text+0x62eba2>
// Frames: F81 -> F82 -> F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F230 -> F229 -> F74 -> F51 -> F222
// 0x14062fb55  4e8b0cce                       movq	(%rsi,%r9,8), %r9
// 0x14062fb59  4d85c9                         testq	%r9, %r9
// Frames: F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F230 -> F229 -> F74 -> F51 -> F222
// 0x14062fb5c  7444                           je	0x14062fba2 <.text+0x62eba2>
// Frames: F76 -> F77 -> F78 -> F79 -> F80 -> F230 -> F229 -> F74 -> F51 -> F222
// 0x14062fb5e  4d8b5318                       movq	0x18(%r11), %r10
// Frames: F84 -> F85 -> F86 -> F76 -> F77 -> F78 -> F79 -> F80 -> F230 -> F229 -> F74 -> F51 -> F222
// 0x14062fb62  49f7d1                         notq	%r9
// Frames: F87 -> F88 -> F89 -> F86 -> F90 -> F79 -> F80 -> F230 -> F229 -> F74 -> F51 -> F222
// 0x14062fb65  4f8d0c49                       leaq	(%r9,%r9,2), %r9
// 0x14062fb69  49c1e104                       shlq	$0x4, %r9
// 0x14062fb6d  4f8b540a10                     movq	0x10(%r10,%r9), %r10
// Frames: F229 -> F74 -> F51 -> F222
// 0x14062fb72  4c8b8b20010000                 movq	0x120(%rbx), %r9
// Frames: F63 -> F18 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F229 -> F74 -> F51 -> F222
// 0x14062fb79  4d39c8                         cmpq	%r9, %r8
// 0x14062fb7c  7746                           ja	0x14062fbc4 <.text+0x62ebc4>
// 0x14062fb7e  eb68                           jmp	0x14062fbe8 <.text+0x62ebe8>
// Frames: F225
// 0x14062fb80  4889f0                         movq	%rsi, %rax
// 0x14062fb83  488d7001                       leaq	0x1(%rax), %rsi
// 0x14062fb87  40f6c701                       testb	$0x1, %dil
// 0x14062fb8b  7578                           jne	0x14062fc05 <.text+0x62ec05>
// 0x14062fb8d  e941ffffff                     jmp	0x14062fad3 <.text+0x62ead3>
// Frames: F229 -> F74 -> F51 -> F222
// 0x14062fb92  31ff                           xorl	%edi, %edi
// 0x14062fb94  4c8b8b20010000                 movq	0x120(%rbx), %r9
// Frames: F63 -> F18 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F229 -> F74 -> F51 -> F222
// 0x14062fb9b  4d39c8                         cmpq	%r9, %r8
// 0x14062fb9e  7724                           ja	0x14062fbc4 <.text+0x62ebc4>
// 0x14062fba0  eb46                           jmp	0x14062fbe8 <.text+0x62ebe8>
// Frames: F229 -> F74 -> F51 -> F222
// 0x14062fba2  4531d2                         xorl	%r10d, %r10d
// 0x14062fba5  4c8b8b20010000                 movq	0x120(%rbx), %r9
// Frames: F63 -> F18 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F229 -> F74 -> F51 -> F222
// 0x14062fbac  4d39c8                         cmpq	%r9, %r8
// 0x14062fbaf  7637                           jbe	0x14062fbe8 <.text+0x62ebe8>
// 0x14062fbb1  eb11                           jmp	0x14062fbc4 <.text+0x62ebc4>
// Frames: F229 -> F74 -> F51 -> F222
// 0x14062fbb3  4989fa                         movq	%rdi, %r10
// 0x14062fbb6  31ff                           xorl	%edi, %edi
// 0x14062fbb8  4c8b8b20010000                 movq	0x120(%rbx), %r9
// Frames: F63 -> F18 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F229 -> F74 -> F51 -> F222
// 0x14062fbbf  4d39c8                         cmpq	%r9, %r8
// 0x14062fbc2  7624                           jbe	0x14062fbe8 <.text+0x62ebe8>
// Frames: F81 -> F82 -> F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F229 -> F74 -> F51 -> F222
// 0x14062fbc4  4e8b04ce                       movq	(%rsi,%r9,8), %r8
// 0x14062fbc8  4d85c0                         testq	%r8, %r8
// Frames: F83 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F229 -> F74 -> F51 -> F222
// 0x14062fbcb  741b                           je	0x14062fbe8 <.text+0x62ebe8>
// Frames: F76 -> F77 -> F78 -> F79 -> F80 -> F229 -> F74 -> F51 -> F222
// 0x14062fbcd  4d8b4b18                       movq	0x18(%r11), %r9
// Frames: F84 -> F85 -> F86 -> F76 -> F77 -> F78 -> F79 -> F80 -> F229 -> F74 -> F51 -> F222
// 0x14062fbd1  49f7d0                         notq	%r8
// Frames: F87 -> F88 -> F89 -> F86 -> F90 -> F79 -> F80 -> F229 -> F74 -> F51 -> F222
// 0x14062fbd4  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x14062fbd8  49c1e004                       shlq	$0x4, %r8
// 0x14062fbdc  4f8b440110                     movq	0x10(%r9,%r8), %r8
// 0x14062fbe1  4c89442420                     movq	%r8, 0x20(%rsp)
// 0x14062fbe6  eb09                           jmp	0x14062fbf1 <.text+0x62ebf1>
// 0x14062fbe8  48c744242000000000             movq	$0x0, 0x20(%rsp)
// Frames: F41 -> F42 -> F43 -> F44 -> F12 -> F13 -> F101 -> F102 -> F51 -> F222
// 0x14062fbf1  4c8b4850                       movq	0x50(%rax), %r9
// 0x14062fbf5  31c0                           xorl	%eax, %eax
// 0x14062fbf7  488d7001                       leaq	0x1(%rax), %rsi
// Frames: F225
// 0x14062fbfb  40f6c701                       testb	$0x1, %dil
// 0x14062fbff  0f84cefeffff                   je	0x14062fad3 <.text+0x62ead3>
// 0x14062fc05  4d85d2                         testq	%r10, %r10
// 0x14062fc08  0f84c5feffff                   je	0x14062fad3 <.text+0x62ead3>
// Frames: F103 -> F105 -> F51 -> F222
// 0x14062fc0e  48c1e004                       shlq	$0x4, %rax
// 0x14062fc12  4d8d0401                       leaq	(%r9,%rax), %r8
// 0x14062fc16  458b4008                       movl	0x8(%r8), %r8d
// 0x14062fc1a  4c8b5c2420                     movq	0x20(%rsp), %r11
// Frames: F225
// 0x14062fc1f  f3430f1004c3                   movss	(%r11,%r8,8), %xmm0
// 0x14062fc25  0f2ec6                         ucomiss	%xmm6, %xmm0
// 0x14062fc28  0f85a5feffff                   jne	0x14062fad3 <.text+0x62ead3>
// 0x14062fc2e  0f8a9ffeffff                   jp	0x14062fad3 <.text+0x62ead3>
// Frames: F227 -> F228
// 0x14062fc34  498b0401                       movq	(%r9,%rax), %rax
// 0x14062fc38  4889442478                     movq	%rax, 0x78(%rsp)
// 0x14062fc3d  488b442430                     movq	0x30(%rsp), %rax
// 0x14062fc42  4889442458                     movq	%rax, 0x58(%rsp)
// 0x14062fc47  48896c2460                     movq	%rbp, 0x60(%rsp)
// 0x14062fc4c  488b442448                     movq	0x48(%rsp), %rax
// 0x14062fc51  4889442468                     movq	%rax, 0x68(%rsp)
// 0x14062fc56  488b442450                     movq	0x50(%rsp), %rax
// 0x14062fc5b  4889442470                     movq	%rax, 0x70(%rsp)
// 0x14062fc60  897c2428                       movl	%edi, 0x28(%rsp)
// 0x14062fc64  4c89d7                         movq	%r10, %rdi
// 0x14062fc67  4c89cd                         movq	%r9, %rbp
// Frames: F228
// 0x14062fc6a  e8014b6e00                     callq	0x140d14770 <.text+0xd13770>
// 0x14062fc6f  4989e9                         movq	%rbp, %r9
// 0x14062fc72  4989fa                         movq	%rdi, %r10
// 0x14062fc75  8b7c2428                       movl	0x28(%rsp), %edi
// 0x14062fc79  488b5c2438                     movq	0x38(%rsp), %rbx
// 0x14062fc7e  488b6c2440                     movq	0x40(%rsp), %rbp
// 0x14062fc83  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x14062fc88  488d1511cb3d02                 leaq	0x23dcb11(%rip), %rdx   # 0x142a0c7a0
// Frames: F225
// 0x14062fc8f  e93ffeffff                     jmp	0x14062fad3 <.text+0x62ead3>
// Frames: F231
// 0x14062fc94  0f28b42480000000               movaps	0x80(%rsp), %xmm6
// 0x14062fc9c  4881c498000000                 addq	$0x98, %rsp
// 0x14062fca3  5b                             popq	%rbx
// 0x14062fca4  5d                             popq	%rbp
// 0x14062fca5  5f                             popq	%rdi
// 0x14062fca6  5e                             popq	%rsi
// 0x14062fca7  415c                           popq	%r12
// 0x14062fca9  415d                           popq	%r13
// 0x14062fcab  415e                           popq	%r14
// 0x14062fcad  415f                           popq	%r15
// 0x14062fcaf  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/delete_roof.rs:12:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:1235:0 bevy_ecs::system::system_param::impl$19::deref_mut
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs:28:0 bevy_ecs::event::reader::EventReader::read
// F4: crates/systems/roof/src/delete_roof.rs:20:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:377:0 bevy_ecs::system::system_param::impl$19::deref_mut
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:55:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F7: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs:89:0 bevy_ecs::event::event_cursor::EventCursor::read_with_id
// F8: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs:79:0 bevy_ecs::system::system_param::impl$19::deref_mut
// F9: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:1860:0 core::num::impl$11::saturating_sub
// F10: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:57:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F11: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:60:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F12: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1575:0 alloc::vec::Vec::as_slice
// F13: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 alloc::vec::impl$8::deref
// F14: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:62:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F15: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:659:0 core::num::impl$11::checked_sub
// F16: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:366:0 core::slice::index::impl$4::get
// F17: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:538:0 core::slice::index::impl$7::get
// F18: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 core::slice::impl$0::get
// F19: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1451:0 core::option::Option::and_then
// F20: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:179:0 core::iter::adapters::chain::impl$1::last
// F21: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:116:0 bevy_ecs::event::iterators::impl$3::last
// F22: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:30:0 bevy_ecs::event::iterators::impl$0::last
// F23: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:61:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:67:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F25: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:178:0 core::iter::adapters::chain::impl$1::last
// F26: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:64:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F27: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:2487:0 core::option::impl$39::branch
// F28: crates/systems/roof/src/delete_roof.rs:32:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F29: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 core::ptr::mut_ptr::impl$0::add
// F30: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F31: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1051:0 core::slice::impl$0::iter
// F32: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:69:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F33: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:71:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F34: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3424:0 alloc::vec::impl$15::from_iter
// F35: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/traits/iterator.rs:1971:0 core::iter::traits::iterator::Iterator::collect
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:640:0 bevy_ecs::system::query::Query::copy_unsafe
// F37: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:622:0 bevy_ecs::system::query::Query::reborrow_unsafe
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:545:0 bevy_ecs::system::query::Query::as_readonly
// F39: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:671:0 bevy_ecs::system::query::Query::iter
// F40: crates/systems/roof/src/delete_roof.rs:34:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F41: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:508:0 alloc::raw_vec::RawVecInner::non_null
// F42: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:503:0 alloc::raw_vec::RawVecInner::ptr
// F43: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:282:0 alloc::raw_vec::RawVec::ptr
// F44: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1671:0 alloc::vec::Vec::as_ptr
// F45: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2372:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F46: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:52:0 bevy_ecs::query::iter::QueryIter::new
// F47: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:2579:0 bevy_ecs::system::query::Query::as_readonly
// F48: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1051:0 alloc::vec::impl$8::deref
// F49: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2373:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F50: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2464:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F51: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:889:0 bevy_ecs::query::iter::impl$1::next
// F52: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2467:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F53: crates/systems/roof/src/delete_roof.rs:35:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2505:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F55: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:1623:0 core::ptr::non_null::impl$14::eq
// F56: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$182::next
// F57: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2506:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F58: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:25:0 core::slice::iter::impl$182::next
// F59: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:617:0 core::ptr::non_null::NonNull::add
// F60: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:108:0 core::slice::iter::Iter::post_inc_start
// F61: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:443:0 core::slice::iter::impl$186::next_unchecked
// F62: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:165:0 core::slice::iter::impl$182::next
// F63: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:228:0 core::slice::index::impl$2::get
// F64: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:896:0 bevy_ecs::archetype::Archetypes::get
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2507:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F66: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2681:0 alloc::vec::Vec::len
// F67: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2706:0 alloc::vec::Vec::is_empty
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:616:0 bevy_ecs::archetype::Archetype::is_empty
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2508:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:461:0 bevy_ecs::archetype::Archetype::table_id
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2511:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:722:0 bevy_ecs::archetype::Archetype::table_id
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$2::set_archetype
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2515:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:60:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:396:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:476:0 bevy_ecs::storage::table::Table::get_column
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:367:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1181:0 bevy_ecs::query::fetch::impl$26::set_table
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1169:0 bevy_ecs::query::fetch::impl$26::set_archetype
// F81: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:707:0 core::option::Option::as_ref
// F82: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F83: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::slice::impl$0::get
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:398:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F86: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1119:0 core::option::Option::map
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs:304:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F90: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F91: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1211:0 bevy_ecs::query::fetch::impl$26::matches_component_set
// F92: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1805:0 bevy_ecs::query::fetch::impl$41::set_archetype
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:51:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
// F94: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:389:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
// F95: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:622:0 bevy_ecs::archetype::Archetype::contains
// F96: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1805:0 bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
// F97: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1214:0 bevy_ecs::query::fetch::impl$26::matches_component_set
// F98: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:608:0 core::option::Option::is_some
// F99: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:631:0 core::slice::impl$0::get
// F100: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1806:0 bevy_ecs::query::fetch::impl$41::set_archetype
// F101: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:467:0 bevy_ecs::archetype::Archetype::entities
// F102: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2528:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F103: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:326:0 bevy_ecs::archetype::ArchetypeEntity::table_row
// F104: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2540:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F105: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2555:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F106: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2468:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F107: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:722:0 bevy_ecs::storage::table::Tables::get
// F108: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2469:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:616:0 bevy_ecs::storage::table::Table::is_empty
// F110: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2470:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F111: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs:199:0 bevy_ecs::query::world_query::impl$2::set_table
// F112: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2476:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F113: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1816:0 bevy_ecs::query::fetch::impl$41::set_table
// F114: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:497:0 bevy_ecs::storage::table::Table::has_column
// F115: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1816:0 bevy_ecs::query::fetch::impl$41::set_table::closure$0
// F116: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:129:0 bevy_ecs::storage::table::TableRow::as_usize
// F117: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1239:0 bevy_ecs::query::fetch::impl$27::fetch::closure$0
// F118: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2474:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F119: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1234:0 bevy_ecs::query::fetch::impl$27::fetch
// F120: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2090:0 bevy_ecs::query::fetch::impl$61::fetch
// F121: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2498:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F122: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs:532:0 bevy_ptr::ThinSlicePtr::get
// F123: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:0:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F124: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1817:0 bevy_ecs::query::fetch::impl$41::set_table
// F125: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:284:0 core::slice::iter::impl$182::any
// F126: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/cmp.rs:237:0 core::slice::cmp::impl$15::slice_contains
// F127: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:2555:0 core::slice::impl$0::contains
// F128: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:285:0 core::slice::iter::impl$182::any
// F129: crates/country-core/src/resources/walls/mod.rs:71:0 country_core::resources::walls::impl$9::eq
// F130: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/cmp.rs:237:0 core::slice::cmp::impl$15::slice_contains::closure$0
// F131: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F132: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F133: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs:75:0 bevy_ecs::event::writer::EventWriter::write
// F134: crates/systems/roof/src/delete_roof.rs:36:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F135: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2419:0 alloc::vec::Vec::push
// F136: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:139:0 bevy_ecs::event::collections::Events::send_with_caller
// F137: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:125:0 bevy_ecs::change_detection::impl$38::deref_mut
// F138: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:130:0 bevy_ecs::event::collections::Events::send_with_caller
// F139: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2422:0 alloc::vec::Vec::push
// F140: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2423:0 alloc::vec::Vec::push
// F141: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1734:0 alloc::vec::Vec::as_mut_ptr
// F142: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2426:0 alloc::vec::Vec::push
// F143: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 alloc::vec::Vec::as_mut_ptr
// F144: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1582:0 core::ptr::write
// F145: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2427:0 alloc::vec::Vec::push
// F146: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2428:0 alloc::vec::Vec::push
// F147: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:140:0 bevy_ecs::event::collections::Events::send_with_caller
// F148: crates/systems/roof/src/delete_roof.rs:41:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F149: crates/systems/roof/src/delete_roof.rs:42:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F150: crates/systems/roof/src/delete_roof.rs:43:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F151: crates/systems/roof/src/delete_roof.rs:44:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F152: crates/systems/roof/src/delete_roof.rs:45:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F153: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1752:0 core::ptr::read_volatile
// F154: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:94:0 alloc::alloc::alloc
// F155: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:192:0 alloc::alloc::Global::alloc_impl
// F156: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:254:0 alloc::alloc::impl$1::allocate
// F157: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:349:0 alloc::alloc::exchange_malloc
// F158: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/boxed.rs:274:0 alloc::boxed::impl$0::new
// F159: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/sync.rs:389:0 alloc::sync::Arc::new
// F160: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:96:0 alloc::alloc::alloc
// F161: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:117:0 bevy_ecs::event::iterators::impl$3::last
// F162: crates/systems/roof/src/delete_roof.rs:25:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F163: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:125:0 bevy_ecs::event::collections::Events::send
// F164: crates/systems/roof/src/delete_roof.rs:26:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F165: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs:532:0 bevy_ecs::storage::table::TableRow::as_usize
// F166: crates/systems/roof/src/delete_roof.rs:28:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F167: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::event::collections::Events::send
// F168: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:751:0 alloc::raw_vec::RawVecInner::deallocate
// F169: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:402:0 alloc::raw_vec::impl$3::drop
// F170: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:523:0 core::ptr::drop_in_place
// F171: crates/systems/roof/src/delete_roof.rs:49:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F172: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:523:0 alloc::raw_vec::RawVecInner::current_memory
// F173: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:945:0 core::num::impl$11::unchecked_mul
// F174: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:531:0 alloc::raw_vec::RawVecInner::current_memory
// F175: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:116:0 alloc::alloc::dealloc
// F176: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:269:0 alloc::alloc::impl$1::deallocate
// F177: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:753:0 alloc::raw_vec::RawVecInner::deallocate
// F178: crates/systems/roof/src/delete_roof.rs:50:0 system_roof::delete_roof::process_invalid_roof_notifications::h5b5fb13702ec0ac5
// F179: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2552:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F180: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/alloc.rs:351:0 alloc::alloc::exchange_malloc
// F181: crates/systems/roof/src/delete_roof.rs:52:0 system_roof::delete_roof::mark_roof_for_deletion::hef04bfec97e73858
// F182: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:423:0 alloc::vec::Vec::new
// F183: crates/systems/roof/src/delete_roof.rs:58:0 system_roof::delete_roof::mark_roof_for_deletion::hef04bfec97e73858
// F184: crates/systems/roof/src/delete_roof.rs:59:0 system_roof::delete_roof::mark_roof_for_deletion::hef04bfec97e73858
// F185: crates/systems/roof/src/delete_roof.rs:60:0 system_roof::delete_roof::mark_roof_for_deletion::hef04bfec97e73858
// F186: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:729:0 core::option::Option::as_mut
// F187: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:333:0 core::iter::adapters::chain::and_then_or_clear
// F188: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:84:0 core::iter::adapters::chain::impl$1::next
// F189: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:89:0 bevy_ecs::event::iterators::impl$3::next
// F190: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:15:0 bevy_ecs::event::iterators::impl$0::next
// F191: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:95:0 bevy_ecs::event::iterators::impl$3::next
// F192: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:84:0 core::iter::adapters::chain::impl$1::next::closure$0
// F193: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1552:0 core::option::Option::or_else
// F194: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:604:0 bevy_ecs::system::query::Query::reborrow
// F195: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:702:0 bevy_ecs::system::query::Query::iter_mut
// F196: crates/systems/roof/src/delete_roof.rs:63:0 system_roof::delete_roof::mark_roof_for_deletion::hef04bfec97e73858
// F197: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:2579:0 bevy_ecs::system::query::Query::reborrow
// F198: crates/systems/roof/src/delete_roof.rs:66:0 system_roof::delete_roof::mark_roof_for_deletion::hef04bfec97e73858
// F199: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:212:0 bevy_ecs::storage::table::Table::entities
// F200: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2479:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F201: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:90:0 core::slice::index::get_noubcheck
// F202: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:256:0 core::slice::index::impl$2::get_unchecked
// F203: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:663:0 core::slice::impl$0::get_unchecked
// F204: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2486:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F205: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2536:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F206: crates/systems/roof/src/delete_roof.rs:64:0 system_roof::delete_roof::mark_roof_for_deletion::hef04bfec97e73858
// F207: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:513:0 alloc::raw_vec::RawVecInner::capacity
// F208: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:623:0 alloc::raw_vec::RawVecInner::needs_to_grow
// F209: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:558:0 alloc::raw_vec::RawVecInner::reserve
// F210: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:327:0 alloc::raw_vec::RawVec::reserve
// F211: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1272:0 alloc::vec::Vec::reserve
// F212: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:0:0 bevy_ecs::system::commands::Commands::queue_internal
// F213: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:554:0 bevy_ecs::system::commands::Commands::queue
// F214: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:1802:0 bevy_ecs::system::commands::EntityCommands::queue
// F215: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:1348:0 bevy_ecs::system::commands::EntityCommands::insert
// F216: crates/systems/roof/src/delete_roof.rs:65:0 system_roof::delete_roof::mark_roof_for_deletion::hef04bfec97e73858
// F217: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:2022:0 alloc::raw_vec::RawVecInner::capacity
// F218: crates/systems/roof/src/delete_roof.rs:68:0 system_roof::delete_roof::mark_roof_for_deletion::hef04bfec97e73858
// F219: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:559:0 alloc::raw_vec::RawVecInner::reserve
// F220: crates/systems/roof/src/delete_roof.rs:72:0 system_roof::delete_roof::mark_roof_for_deletion::hef04bfec97e73858
// F221: crates/systems/roof/src/delete_roof.rs:74:0 system_roof::delete_roof::delete_roofs::hf7fb8bba8bd56e3c
// F222: crates/systems/roof/src/delete_roof.rs:83:0 system_roof::delete_roof::delete_roofs::hf7fb8bba8bd56e3c
// F223: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs:199:0 bevy_ecs::query::world_query::impl$4::set_table
// F224: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1820:0 bevy_ecs::query::fetch::impl$41::set_table
// F225: crates/systems/roof/src/delete_roof.rs:87:0 system_roof::delete_roof::delete_roofs::hf7fb8bba8bd56e3c
// F226: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2090:0 bevy_ecs::query::fetch::impl$65::fetch
// F227: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs:408:0 bevy_ecs::system::commands::Commands::entity
// F228: crates/systems/roof/src/delete_roof.rs:88:0 system_roof::delete_roof::delete_roofs::hf7fb8bba8bd56e3c
// F229: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$4::set_archetype
// F230: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1809:0 bevy_ecs::query::fetch::impl$41::set_archetype
// F231: crates/systems/roof/src/delete_roof.rs:91:0 system_roof::delete_roof::delete_roofs::hf7fb8bba8bd56e3c
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/delete_roof.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 47f0bcfcae132fe5c9745695f335e7974f44da1c708a16aac8316706f2e4bde0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: delete_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 270
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::delete_roof::process_invalid_roof_notifications
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":270,"record_offset":260,"rva":6482816,"va":5375191936}
// 编译实例 RVAs: 0x62eb80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-111613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375191936,"raw_hex":"e8034cfeff","size":5,"soft_fail":false,"target_va":5375081488,"va":5375193096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-112090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375191936,"raw_hex":"e8264afeff","size":5,"soft_fail":false,"target_va":5375081504,"va":5375193589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7993819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375191936,"raw_hex":"e8dbf97900","size":5,"soft_fail":false,"target_va":5383186064,"va":5375192240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36347013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375191936,"raw_hex":"e8859c2a02","size":5,"soft_fail":false,"target_va":5411540992,"va":5375193974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-836077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375191936,"raw_hex":"e8133ef3ff","size":5,"soft_fail":false,"target_va":5374356912,"va":5375192984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4546130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375191936,"raw_hex":"e8525e4500","size":5,"soft_fail":false,"target_va":5379739328,"va":5375193193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":270,"record_offset":6096,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":270,"record_offset":6040,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":270,"record_offset":1004,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":270,"record_offset":440,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":270,"record_offset":5716,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":270,"record_offset":2380,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":270,"record_offset":1972,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":270,"record_offset":1916,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":10196,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":270,"record_offset":5492,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":6208,"parent_record_offset":6184,"ancestor_record_offset":260,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":6232,"parent_record_offset":6208,"ancestor_record_offset":260,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":10380,"parent_record_offset":10356,"ancestor_record_offset":260,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":6152,"parent_record_offset":6124,"ancestor_record_offset":260,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":6184,"parent_record_offset":6152,"ancestor_record_offset":260,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":10356,"parent_record_offset":10268,"ancestor_record_offset":260,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":270,"record_offset":6124,"parent_record_offset":6096,"ancestor_record_offset":260,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":2212,"parent_record_offset":2188,"ancestor_record_offset":260,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":10296,"parent_record_offset":10268,"ancestor_record_offset":260,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":10268,"parent_record_offset":10244,"ancestor_record_offset":260,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":2260,"parent_record_offset":2236,"ancestor_record_offset":260,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":260,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":10244,"parent_record_offset":10220,"ancestor_record_offset":260,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":5876,"parent_record_offset":5836,"ancestor_record_offset":260,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":2188,"parent_record_offset":2164,"ancestor_record_offset":260,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":260,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":260,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":260,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":5836,"parent_record_offset":5800,"ancestor_record_offset":260,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":1940,"parent_record_offset":1916,"ancestor_record_offset":260,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":652,"parent_record_offset":552,"ancestor_record_offset":260,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":3528,"parent_record_offset":3500,"ancestor_record_offset":260,"inlinee":5673,"source_line":621,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":3772,"parent_record_offset":2416,"ancestor_record_offset":260,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":2744,"parent_record_offset":2416,"ancestor_record_offset":260,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":2828,"parent_record_offset":2416,"ancestor_record_offset":260,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":3968,"parent_record_offset":2416,"ancestor_record_offset":260,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":2660,"parent_record_offset":2416,"ancestor_record_offset":260,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":270,"record_offset":5772,"parent_record_offset":5740,"ancestor_record_offset":260,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":270,"record_offset":5740,"parent_record_offset":5716,"ancestor_record_offset":260,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":270,"record_offset":6756,"parent_record_offset":6716,"ancestor_record_offset":260,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":270,"record_offset":5800,"parent_record_offset":5740,"ancestor_record_offset":260,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":270,"record_offset":520,"parent_record_offset":472,"ancestor_record_offset":260,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":270,"record_offset":552,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":270,"record_offset":1040,"parent_record_offset":1004,"ancestor_record_offset":260,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":5224,"parent_record_offset":5200,"ancestor_record_offset":260,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::matches_component_set
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":3468,"parent_record_offset":3440,"ancestor_record_offset":260,"inlinee":5720,"source_line":1210,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":2944,"parent_record_offset":2920,"ancestor_record_offset":260,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":260,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":260,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":260,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":3440,"parent_record_offset":2920,"ancestor_record_offset":260,"inlinee":17008,"source_line":1799,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":3500,"parent_record_offset":3468,"ancestor_record_offset":260,"inlinee":17010,"source_line":1805,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":4840,"parent_record_offset":4344,"ancestor_record_offset":260,"inlinee":17014,"source_line":1815,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":4904,"parent_record_offset":4872,"ancestor_record_offset":260,"inlinee":17016,"source_line":1816,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":5176,"parent_record_offset":2416,"ancestor_record_offset":260,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":270,"record_offset":2080,"parent_record_offset":1996,"ancestor_record_offset":260,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":270,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":260,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":270,"record_offset":2416,"parent_record_offset":2380,"ancestor_record_offset":260,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":270,"record_offset":2920,"parent_record_offset":2416,"ancestor_record_offset":260,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":270,"record_offset":4344,"parent_record_offset":2416,"ancestor_record_offset":260,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":270,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":260,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":3584,"parent_record_offset":3556,"ancestor_record_offset":260,"inlinee":5675,"source_line":49,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":260,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":3556,"parent_record_offset":3528,"ancestor_record_offset":260,"inlinee":5674,"source_line":388,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":260,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":260,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":3020,"parent_record_offset":2992,"ancestor_record_offset":260,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":260,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":4932,"parent_record_offset":4904,"ancestor_record_offset":260,"inlinee":17017,"source_line":496,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":4260,"parent_record_offset":2416,"ancestor_record_offset":260,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":260,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":4176,"parent_record_offset":2416,"ancestor_record_offset":260,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":270,"record_offset":3364,"parent_record_offset":3340,"ancestor_record_offset":260,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":3340,"parent_record_offset":3316,"ancestor_record_offset":260,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":270,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":260,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":270,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":260,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":270,"record_offset":2024,"parent_record_offset":1996,"ancestor_record_offset":260,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":270,"record_offset":472,"parent_record_offset":440,"ancestor_record_offset":260,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":270,"record_offset":5412,"parent_record_offset":5388,"ancestor_record_offset":260,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":270,"record_offset":1080,"parent_record_offset":1040,"ancestor_record_offset":260,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":270,"record_offset":780,"parent_record_offset":756,"ancestor_record_offset":260,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":270,"record_offset":596,"parent_record_offset":552,"ancestor_record_offset":260,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":270,"record_offset":10324,"parent_record_offset":10296,"ancestor_record_offset":260,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":270,"record_offset":3160,"parent_record_offset":3096,"ancestor_record_offset":260,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":270,"record_offset":1108,"parent_record_offset":1080,"ancestor_record_offset":260,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":270,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":260,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":270,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":260,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":270,"record_offset":3224,"parent_record_offset":3044,"ancestor_record_offset":260,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":270,"record_offset":1168,"parent_record_offset":1040,"ancestor_record_offset":260,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":10220,"parent_record_offset":10196,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":270,"record_offset":1808,"parent_record_offset":1784,"ancestor_record_offset":260,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":270,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":260,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":270,"record_offset":2544,"parent_record_offset":2508,"ancestor_record_offset":260,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":6260,"parent_record_offset":6232,"ancestor_record_offset":260,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":5996,"parent_record_offset":5836,"ancestor_record_offset":260,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":270,"record_offset":5520,"parent_record_offset":5492,"ancestor_record_offset":260,"inlinee":12925,"source_line":236,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":270,"record_offset":5648,"parent_record_offset":5548,"ancestor_record_offset":260,"inlinee":12927,"source_line":237,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":270,"record_offset":708,"parent_record_offset":552,"ancestor_record_offset":260,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":270,"record_offset":1760,"parent_record_offset":1292,"ancestor_record_offset":260,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":270,"record_offset":2708,"parent_record_offset":2684,"ancestor_record_offset":260,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":270,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":260,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":270,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":260,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":270,"record_offset":1784,"parent_record_offset":1760,"ancestor_record_offset":260,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":270,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":260,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":270,"record_offset":5548,"parent_record_offset":5520,"ancestor_record_offset":260,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":270,"record_offset":2508,"parent_record_offset":2416,"ancestor_record_offset":260,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":270,"record_offset":2572,"parent_record_offset":2508,"ancestor_record_offset":260,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":270,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":260,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":270,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":260,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::delete_roof::mark_roof_for_deletion
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":270,"record_offset":10664,"rva":6484944,"va":5375194064}
// 编译实例 RVAs: 0x62f3d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-113719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375194064,"raw_hex":"e8c943feff","size":5,"soft_fail":false,"target_va":5375081504,"va":5375195218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-838720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375194064,"raw_hex":"e8c033f3ff","size":5,"soft_fail":false,"target_va":5374355568,"va":5375194283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36353178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375194064,"raw_hex":"e89ab42a02","size":5,"soft_fail":false,"target_va":5411548368,"va":5375195185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2832976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375194064,"raw_hex":"e8503a2b00","size":5,"soft_fail":false,"target_va":5378028064,"va":5375195083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":10832,"parent_record_offset":10664,"ancestor_record_offset":10664,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":11568,"parent_record_offset":10664,"ancestor_record_offset":10664,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":270,"record_offset":11756,"parent_record_offset":10664,"ancestor_record_offset":10664,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":270,"record_offset":10860,"parent_record_offset":10664,"ancestor_record_offset":10664,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":270,"record_offset":12460,"parent_record_offset":10664,"ancestor_record_offset":10664,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":270,"record_offset":12812,"parent_record_offset":10664,"ancestor_record_offset":10664,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":270,"record_offset":15784,"parent_record_offset":10664,"ancestor_record_offset":10664,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":270,"record_offset":12116,"parent_record_offset":10664,"ancestor_record_offset":10664,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":16080,"parent_record_offset":10664,"ancestor_record_offset":10664,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":270,"record_offset":15580,"parent_record_offset":10664,"ancestor_record_offset":10664,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":16268,"parent_record_offset":16244,"ancestor_record_offset":10664,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":16244,"parent_record_offset":16152,"ancestor_record_offset":10664,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":11636,"parent_record_offset":11612,"ancestor_record_offset":10664,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":15928,"parent_record_offset":15900,"ancestor_record_offset":10664,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":16012,"parent_record_offset":15988,"ancestor_record_offset":10664,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":16180,"parent_record_offset":16152,"ancestor_record_offset":10664,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":16152,"parent_record_offset":16128,"ancestor_record_offset":10664,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":15988,"parent_record_offset":15956,"ancestor_record_offset":10664,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":11684,"parent_record_offset":11660,"ancestor_record_offset":10664,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":11660,"parent_record_offset":11636,"ancestor_record_offset":10664,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":15956,"parent_record_offset":15928,"ancestor_record_offset":10664,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":16128,"parent_record_offset":16104,"ancestor_record_offset":10664,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":11612,"parent_record_offset":11568,"ancestor_record_offset":10664,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":12324,"parent_record_offset":12300,"ancestor_record_offset":10664,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":11196,"parent_record_offset":11172,"ancestor_record_offset":10664,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":13184,"parent_record_offset":13160,"ancestor_record_offset":10664,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":13208,"parent_record_offset":13184,"ancestor_record_offset":10664,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":12576,"parent_record_offset":12532,"ancestor_record_offset":10664,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":15900,"parent_record_offset":15868,"ancestor_record_offset":10664,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":11172,"parent_record_offset":10948,"ancestor_record_offset":10664,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":15096,"parent_record_offset":12836,"ancestor_record_offset":10664,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":13976,"parent_record_offset":12836,"ancestor_record_offset":10664,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":14060,"parent_record_offset":12836,"ancestor_record_offset":10664,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":15376,"parent_record_offset":12836,"ancestor_record_offset":10664,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":13892,"parent_record_offset":12836,"ancestor_record_offset":10664,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":270,"record_offset":12776,"parent_record_offset":12488,"ancestor_record_offset":10664,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":270,"record_offset":12488,"parent_record_offset":12460,"ancestor_record_offset":10664,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":270,"record_offset":12532,"parent_record_offset":12488,"ancestor_record_offset":10664,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":270,"record_offset":10924,"parent_record_offset":10884,"ancestor_record_offset":10664,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":270,"record_offset":10948,"parent_record_offset":10924,"ancestor_record_offset":10664,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":270,"record_offset":11784,"parent_record_offset":11756,"ancestor_record_offset":10664,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":15004,"parent_record_offset":14980,"ancestor_record_offset":10664,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":14176,"parent_record_offset":14152,"ancestor_record_offset":10664,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":13268,"parent_record_offset":13244,"ancestor_record_offset":10664,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":14980,"parent_record_offset":14956,"ancestor_record_offset":10664,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":15028,"parent_record_offset":15004,"ancestor_record_offset":10664,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":14956,"parent_record_offset":12836,"ancestor_record_offset":10664,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":270,"record_offset":12228,"parent_record_offset":12140,"ancestor_record_offset":10664,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":270,"record_offset":12252,"parent_record_offset":12228,"ancestor_record_offset":10664,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":270,"record_offset":12836,"parent_record_offset":12812,"ancestor_record_offset":10664,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":270,"record_offset":14152,"parent_record_offset":12836,"ancestor_record_offset":10664,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":270,"record_offset":13244,"parent_record_offset":12836,"ancestor_record_offset":10664,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":270,"record_offset":13688,"parent_record_offset":13664,"ancestor_record_offset":10664,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":13368,"parent_record_offset":13344,"ancestor_record_offset":10664,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":13344,"parent_record_offset":13320,"ancestor_record_offset":10664,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":13548,"parent_record_offset":13524,"ancestor_record_offset":10664,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":14676,"parent_record_offset":12836,"ancestor_record_offset":10664,"inlinee":11681,"source_line":211,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":13320,"parent_record_offset":13292,"ancestor_record_offset":10664,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":13292,"parent_record_offset":13268,"ancestor_record_offset":10664,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":13160,"parent_record_offset":12836,"ancestor_record_offset":10664,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":15052,"parent_record_offset":15028,"ancestor_record_offset":10664,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":13076,"parent_record_offset":12836,"ancestor_record_offset":10664,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":270,"record_offset":13664,"parent_record_offset":13640,"ancestor_record_offset":10664,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":13640,"parent_record_offset":13616,"ancestor_record_offset":10664,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":270,"record_offset":15840,"parent_record_offset":15812,"ancestor_record_offset":10664,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":270,"record_offset":15868,"parent_record_offset":15840,"ancestor_record_offset":10664,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":270,"record_offset":15812,"parent_record_offset":15784,"ancestor_record_offset":10664,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":270,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":10664,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":270,"record_offset":12140,"parent_record_offset":12116,"ancestor_record_offset":10664,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":270,"record_offset":12172,"parent_record_offset":12140,"ancestor_record_offset":10664,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":270,"record_offset":10884,"parent_record_offset":10860,"ancestor_record_offset":10664,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":270,"record_offset":15500,"parent_record_offset":15476,"ancestor_record_offset":10664,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":270,"record_offset":11852,"parent_record_offset":11824,"ancestor_record_offset":10664,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":270,"record_offset":11824,"parent_record_offset":11784,"ancestor_record_offset":10664,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":270,"record_offset":11932,"parent_record_offset":11908,"ancestor_record_offset":10664,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":270,"record_offset":11128,"parent_record_offset":11100,"ancestor_record_offset":10664,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":270,"record_offset":10988,"parent_record_offset":10948,"ancestor_record_offset":10664,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":270,"record_offset":16212,"parent_record_offset":16180,"ancestor_record_offset":10664,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":270,"record_offset":13460,"parent_record_offset":13396,"ancestor_record_offset":10664,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":270,"record_offset":11876,"parent_record_offset":11852,"ancestor_record_offset":10664,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":270,"record_offset":13484,"parent_record_offset":13460,"ancestor_record_offset":10664,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":270,"record_offset":13524,"parent_record_offset":13344,"ancestor_record_offset":10664,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":270,"record_offset":11908,"parent_record_offset":11824,"ancestor_record_offset":10664,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":16104,"parent_record_offset":16080,"ancestor_record_offset":10664,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":270,"record_offset":11460,"parent_record_offset":11436,"ancestor_record_offset":10664,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":270,"record_offset":12056,"parent_record_offset":12032,"ancestor_record_offset":10664,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":270,"record_offset":12960,"parent_record_offset":12924,"ancestor_record_offset":10664,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":11724,"parent_record_offset":11568,"ancestor_record_offset":10664,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":270,"record_offset":15604,"parent_record_offset":15580,"ancestor_record_offset":10664,"inlinee":12925,"source_line":236,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":270,"record_offset":15716,"parent_record_offset":15628,"ancestor_record_offset":10664,"inlinee":12927,"source_line":237,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":270,"record_offset":11044,"parent_record_offset":10948,"ancestor_record_offset":10664,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":270,"record_offset":14872,"parent_record_offset":12836,"ancestor_record_offset":10664,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":270,"record_offset":11412,"parent_record_offset":10948,"ancestor_record_offset":10664,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":270,"record_offset":14920,"parent_record_offset":14896,"ancestor_record_offset":10664,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":270,"record_offset":13124,"parent_record_offset":13100,"ancestor_record_offset":10664,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":270,"record_offset":14896,"parent_record_offset":14872,"ancestor_record_offset":10664,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":270,"record_offset":11100,"parent_record_offset":11072,"ancestor_record_offset":10664,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":270,"record_offset":11072,"parent_record_offset":11044,"ancestor_record_offset":10664,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":270,"record_offset":11436,"parent_record_offset":11412,"ancestor_record_offset":10664,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":270,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":10664,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":270,"record_offset":15628,"parent_record_offset":15604,"ancestor_record_offset":10664,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":270,"record_offset":11984,"parent_record_offset":11932,"ancestor_record_offset":10664,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":270,"record_offset":12008,"parent_record_offset":11984,"ancestor_record_offset":10664,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":270,"record_offset":15740,"parent_record_offset":15716,"ancestor_record_offset":10664,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":270,"record_offset":13572,"parent_record_offset":13548,"ancestor_record_offset":10664,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::delete_roof::delete_roofs
// 记录形式: compiled_procedure; instances=1
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":270,"record_offset":16556,"rva":6486208,"va":5375195328}
// 编译实例 RVAs: 0x62f8c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7228652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375195328,"raw_hex":"e8ec4c6e00","size":5,"soft_fail":false,"target_va":5382424432,"va":5375195775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":270,"record_offset":17092,"parent_record_offset":16556,"ancestor_record_offset":16556,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":270,"record_offset":21316,"parent_record_offset":16556,"ancestor_record_offset":16556,"inlinee":13699,"source_line":407,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":270,"record_offset":16688,"parent_record_offset":16556,"ancestor_record_offset":16556,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":16924,"parent_record_offset":16900,"ancestor_record_offset":16556,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":16972,"parent_record_offset":16948,"ancestor_record_offset":16556,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":16948,"parent_record_offset":16924,"ancestor_record_offset":16556,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":16900,"parent_record_offset":16876,"ancestor_record_offset":16556,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":16876,"parent_record_offset":16844,"ancestor_record_offset":16556,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":17484,"parent_record_offset":17460,"ancestor_record_offset":16556,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":17508,"parent_record_offset":17484,"ancestor_record_offset":16556,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":16844,"parent_record_offset":16820,"ancestor_record_offset":16556,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":19788,"parent_record_offset":19764,"ancestor_record_offset":16556,"inlinee":5673,"source_line":621,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":21084,"parent_record_offset":17140,"ancestor_record_offset":16556,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":19360,"parent_record_offset":17140,"ancestor_record_offset":16556,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":19444,"parent_record_offset":17140,"ancestor_record_offset":16556,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":21280,"parent_record_offset":17140,"ancestor_record_offset":16556,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":270,"record_offset":19276,"parent_record_offset":17140,"ancestor_record_offset":16556,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":19032,"parent_record_offset":19008,"ancestor_record_offset":16556,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":17716,"parent_record_offset":17684,"ancestor_record_offset":16556,"inlinee":5720,"source_line":1210,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":19564,"parent_record_offset":19536,"ancestor_record_offset":16556,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":17572,"parent_record_offset":17544,"ancestor_record_offset":16556,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":19008,"parent_record_offset":18984,"ancestor_record_offset":16556,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":19056,"parent_record_offset":19032,"ancestor_record_offset":16556,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":19704,"parent_record_offset":19536,"ancestor_record_offset":16556,"inlinee":17008,"source_line":1799,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":19764,"parent_record_offset":19736,"ancestor_record_offset":16556,"inlinee":17010,"source_line":1805,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":17684,"parent_record_offset":17544,"ancestor_record_offset":16556,"inlinee":17014,"source_line":1815,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":17744,"parent_record_offset":17716,"ancestor_record_offset":16556,"inlinee":17016,"source_line":1816,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$65::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":270,"record_offset":18984,"parent_record_offset":17140,"ancestor_record_offset":16556,"inlinee":17024,"source_line":2083,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":270,"record_offset":16796,"parent_record_offset":16712,"ancestor_record_offset":16556,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":270,"record_offset":16820,"parent_record_offset":16796,"ancestor_record_offset":16556,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":270,"record_offset":17140,"parent_record_offset":17092,"ancestor_record_offset":16556,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":270,"record_offset":19536,"parent_record_offset":17140,"ancestor_record_offset":16556,"inlinee":17025,"source_line":182,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":270,"record_offset":17544,"parent_record_offset":17140,"ancestor_record_offset":16556,"inlinee":17022,"source_line":195,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":270,"record_offset":18244,"parent_record_offset":18220,"ancestor_record_offset":16556,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":17816,"parent_record_offset":17792,"ancestor_record_offset":16556,"inlinee":5675,"source_line":49,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":18412,"parent_record_offset":18384,"ancestor_record_offset":16556,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":17792,"parent_record_offset":17768,"ancestor_record_offset":16556,"inlinee":5674,"source_line":388,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":17644,"parent_record_offset":17620,"ancestor_record_offset":16556,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":18104,"parent_record_offset":18080,"ancestor_record_offset":16556,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":18788,"parent_record_offset":17140,"ancestor_record_offset":16556,"inlinee":11681,"source_line":211,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":17620,"parent_record_offset":17596,"ancestor_record_offset":16556,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":17596,"parent_record_offset":17572,"ancestor_record_offset":16556,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":17768,"parent_record_offset":17744,"ancestor_record_offset":16556,"inlinee":17017,"source_line":496,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":17460,"parent_record_offset":17140,"ancestor_record_offset":16556,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":19080,"parent_record_offset":19056,"ancestor_record_offset":16556,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":17376,"parent_record_offset":17140,"ancestor_record_offset":16556,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":270,"record_offset":18220,"parent_record_offset":18196,"ancestor_record_offset":16556,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":270,"record_offset":18196,"parent_record_offset":18172,"ancestor_record_offset":16556,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":270,"record_offset":16764,"parent_record_offset":16740,"ancestor_record_offset":16556,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":270,"record_offset":16712,"parent_record_offset":16688,"ancestor_record_offset":16556,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":270,"record_offset":16740,"parent_record_offset":16712,"ancestor_record_offset":16556,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":270,"record_offset":17900,"parent_record_offset":17840,"ancestor_record_offset":16556,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":270,"record_offset":18532,"parent_record_offset":18508,"ancestor_record_offset":16556,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":270,"record_offset":17924,"parent_record_offset":17900,"ancestor_record_offset":16556,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":270,"record_offset":18080,"parent_record_offset":18056,"ancestor_record_offset":16556,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":270,"record_offset":17040,"parent_record_offset":17016,"ancestor_record_offset":16556,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":270,"record_offset":17336,"parent_record_offset":17312,"ancestor_record_offset":16556,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":270,"record_offset":17260,"parent_record_offset":17224,"ancestor_record_offset":16556,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":270,"record_offset":17400,"parent_record_offset":17376,"ancestor_record_offset":16556,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":270,"record_offset":17424,"parent_record_offset":17400,"ancestor_record_offset":16556,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":270,"record_offset":17016,"parent_record_offset":16844,"ancestor_record_offset":16556,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":270,"record_offset":17312,"parent_record_offset":17288,"ancestor_record_offset":16556,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":270,"record_offset":17224,"parent_record_offset":17140,"ancestor_record_offset":16556,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":270,"record_offset":17288,"parent_record_offset":17224,"ancestor_record_offset":16556,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":270,"record_offset":18128,"parent_record_offset":18104,"ancestor_record_offset":16556,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00270.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_roof::delete_roof::delete_roofs
// 信息 ID: 074ee2aaa9c938570e1b3a0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 22f506c314bfeff5269f4bf0: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands)> > @ 0xdde907 -> 0x62f8c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands)> > > @ 0x1a27e9d -> 0x62f8c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands)> > > @ 0x1a2dbee -> 0x62f8c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::delete_roof::mark_roof_for_deletion
// 信息 ID: 17a3ee03a6a720ec37b41013
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 634f59edb43c20e01e8dc3ba: void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> > @ 0xddeac6 -> 0x62f3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> > > @ 0x1a2972b -> 0x62f3d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::delete_roof::process_invalid_roof_notifications
// 信息 ID: 926b17d66819260371ef2a58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 966a6da9871e5ffa21dca45d: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>)> > > @ 0x437934 -> 0x62eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>)> > @ 0x4e613c -> 0x62eb80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
