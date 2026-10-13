// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: assemble_roof_tiles.rs.assembly.json.gz

// FUNCTION system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$2
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 2444:7836 -> host 2444:268 / rva_1f3d850_len_b65; exact_symbolizer_function_frame
//   Function-frame RVA span [0x1f3df95, 0x1f3dfa2), 3 instructions
//
// FUNCTION system_roof::visual::assemble_roof_tiles::assemble_roof_tiles
// Candidate interfaces: f302eca862f77fde05a2893b
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 2444:268 -> rva_1f3d850_len_b65
//
// FUNCTION system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 7; complete records in enriched/assembly sidecars.
// Compiled procedure 3785:30460 -> rva_282f570_len_3c3
// Compiled procedure 3786:8700 -> rva_2830c70_len_3c3
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1f3d850_len_b65: RVA 0x1f3d850, 2917 PDB bytes; 731 instructions
// Evidence: _assembly/roof-v1/C/pack-001/rva_1f3d850_len_b65.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2444:268: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles
// Frames: F1
// 0x141f3d850  55                             pushq	%rbp
// 0x141f3d851  4157                           pushq	%r15
// 0x141f3d853  4156                           pushq	%r14
// 0x141f3d855  4155                           pushq	%r13
// 0x141f3d857  4154                           pushq	%r12
// 0x141f3d859  56                             pushq	%rsi
// 0x141f3d85a  57                             pushq	%rdi
// 0x141f3d85b  53                             pushq	%rbx
// 0x141f3d85c  4881ec88010000                 subq	$0x188, %rsp            # imm = 0x188
// 0x141f3d863  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x141f3d86b  660f7fb5f0000000               movdqa	%xmm6, 0xf0(%rbp)
// 0x141f3d873  48c785e8000000feffffff         movq	$-0x2, 0xe8(%rbp)
// 0x141f3d87e  4d89cd                         movq	%r9, %r13
// 0x141f3d881  4c89c6                         movq	%r8, %rsi
// 0x141f3d884  4889d3                         movq	%rdx, %rbx
// 0x141f3d887  4989ce                         movq	%rcx, %r14
// Frames: F2
// 0x141f3d88a  e831e8f4fe                     callq	0x140e8c0c0 <.text+0xe8b0c0>
// 0x141f3d88f  84c0                           testb	%al, %al
// 0x141f3d891  0f84ff000000                   je	0x141f3d996 <.text+0x1f3c996>
// Frames: F3 -> F2
// 0x141f3d897  bf3d000000                     movl	$0x3d, %edi
// 0x141f3d89c  b93d000000                     movl	$0x3d, %ecx
// 0x141f3d8a1  e81a0b0000                     callq	0x141f3e3c0 <.text+0x1f3d3c0>
// 0x141f3d8a6  4c8d25ceb94d01                 leaq	0x14db9ce(%rip), %r12   # 0x14341927b
// 0x141f3d8ad  4883f801                       cmpq	$0x1, %rax
// 0x141f3d8b1  7539                           jne	0x141f3d8ec <.text+0x1f3c8ec>
// Frames: F4 -> F5 -> F6 -> F7 -> F8 -> F2
// 0x141f3d8b3  4989d1                         movq	%rdx, %r9
// 0x141f3d8b6  4883fa3d                       cmpq	$0x3d, %rdx
// 0x141f3d8ba  0f87ee080000                   ja	0x141f3e1ae <.text+0x1f3d1ae>
// Frames: F8 -> F2
// 0x141f3d8c0  4c89c9                         movq	%r9, %rcx
// 0x141f3d8c3  e8f80a0000                     callq	0x141f3e3c0 <.text+0x1f3d3c0>
// 0x141f3d8c8  a801                           testb	$0x1, %al
// 0x141f3d8ca  7420                           je	0x141f3d8ec <.text+0x1f3c8ec>
// Frames: F9 -> F2
// 0x141f3d8cc  4c8d4202                       leaq	0x2(%rdx), %r8
// Frames: F4 -> F10 -> F11 -> F7 -> F9 -> F2
// 0x141f3d8d0  4983f83d                       cmpq	$0x3d, %r8
// 0x141f3d8d4  0f87f4080000                   ja	0x141f3e1ce <.text+0x1f3d1ce>
// Frames: F12 -> F13 -> F14 -> F11 -> F7 -> F9 -> F2
// 0x141f3d8da  bf3b000000                     movl	$0x3b, %edi
// 0x141f3d8df  4829d7                         subq	%rdx, %rdi
// Frames: F15 -> F16 -> F13 -> F14 -> F11 -> F7 -> F9 -> F2
// 0x141f3d8e2  4c8d2592b94d01                 leaq	0x14db992(%rip), %r12   # 0x14341927b
// 0x141f3d8e9  4d01c4                         addq	%r8, %r12
// 0x141f3d8ec  4c89ade0000000                 movq	%r13, 0xe0(%rbp)
// 0x141f3d8f3  4c8d0571b84d01                 leaq	0x14db871(%rip), %r8    # 0x14341916b
// 0x141f3d8fa  4c8d2d35b84d01                 leaq	0x14db835(%rip), %r13   # 0x143419136
// 0x141f3d901  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F17 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d910  410fbe40ff                     movsbl	-0x1(%r8), %eax
// 0x141f3d915  85c0                           testl	%eax, %eax
// 0x141f3d917  7817                           js	0x141f3d930 <.text+0x1f3c930>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d919  49ffc8                         decq	%r8
// 0x141f3d91c  83f85c                         cmpl	$0x5c, %eax
// 0x141f3d91f  7563                           jne	0x141f3d984 <.text+0x1f3c984>
// 0x141f3d921  eb7a                           jmp	0x141f3d99d <.text+0x1f3c99d>
// 0x141f3d923  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F30 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d930  410fb648fe                     movzbl	-0x2(%r8), %ecx
// Frames: F31 -> F32 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d935  80f9c0                         cmpb	$-0x40, %cl
// Frames: F32 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d938  7d23                           jge	0x141f3d95d <.text+0x1f3c95d>
// Frames: F33 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d93a  410fb650fd                     movzbl	-0x3(%r8), %edx
// Frames: F31 -> F34 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d93f  80fac0                         cmpb	$-0x40, %dl
// Frames: F34 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d942  7d22                           jge	0x141f3d966 <.text+0x1f3c966>
// Frames: F35 -> F36 -> F37 -> F38 -> F39 -> F40 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d944  450fb648fc                     movzbl	-0x4(%r8), %r9d
// 0x141f3d949  4983c0fc                       addq	$-0x4, %r8
// Frames: F41 -> F42 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d94d  4183e107                       andl	$0x7, %r9d
// Frames: F43 -> F44 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d951  41c1e106                       shll	$0x6, %r9d
// 0x141f3d955  83e23f                         andl	$0x3f, %edx
// 0x141f3d958  4409ca                         orl	%r9d, %edx
// 0x141f3d95b  eb10                           jmp	0x141f3d96d <.text+0x1f3c96d>
// Frames: F32 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d95d  4983c0fe                       addq	$-0x2, %r8
// 0x141f3d961  83e11f                         andl	$0x1f, %ecx
// 0x141f3d964  eb0f                           jmp	0x141f3d975 <.text+0x1f3c975>
// 0x141f3d966  4983c0fd                       addq	$-0x3, %r8
// 0x141f3d96a  83e20f                         andl	$0xf, %edx
// Frames: F43 -> F45 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d96d  c1e206                         shll	$0x6, %edx
// 0x141f3d970  83e13f                         andl	$0x3f, %ecx
// 0x141f3d973  09d1                           orl	%edx, %ecx
// Frames: F43 -> F46 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d975  c1e106                         shll	$0x6, %ecx
// 0x141f3d978  243f                           andb	$0x3f, %al
// 0x141f3d97a  0fb6c0                         movzbl	%al, %eax
// 0x141f3d97d  09c8                           orl	%ecx, %eax
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d97f  83f85c                         cmpl	$0x5c, %eax
// 0x141f3d982  7419                           je	0x141f3d99d <.text+0x1f3c99d>
// 0x141f3d984  83f82f                         cmpl	$0x2f, %eax
// 0x141f3d987  7414                           je	0x141f3d99d <.text+0x1f3c99d>
// Frames: F47 -> F48 -> F49 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d989  4d39e8                         cmpq	%r13, %r8
// Frames: F50 -> F49 -> F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F2
// 0x141f3d98c  7582                           jne	0x141f3d910 <.text+0x1f3c910>
// 0x141f3d98e  41bf35000000                   movl	$0x35, %r15d
// 0x141f3d994  eb2a                           jmp	0x141f3d9c0 <.text+0x1f3c9c0>
// 0x141f3d996  31c0                           xorl	%eax, %eax
// 0x141f3d998  e991000000                     jmp	0x141f3da2e <.text+0x1f3ca2e>
// Frames: F51 -> F2
// 0x141f3d99d  4c8d2d92b74d01                 leaq	0x14db792(%rip), %r13   # 0x143419136
// 0x141f3d9a4  4d29e8                         subq	%r13, %r8
// 0x141f3d9a7  49ffc0                         incq	%r8
// Frames: F4 -> F10 -> F11 -> F7 -> F51 -> F2
// 0x141f3d9aa  4983f835                       cmpq	$0x35, %r8
// 0x141f3d9ae  0f87d7070000                   ja	0x141f3e18b <.text+0x1f3d18b>
// Frames: F12 -> F13 -> F14 -> F11 -> F7 -> F51 -> F2
// 0x141f3d9b4  41bf35000000                   movl	$0x35, %r15d
// 0x141f3d9ba  4d29c7                         subq	%r8, %r15
// Frames: F15 -> F16 -> F13 -> F14 -> F11 -> F7 -> F51 -> F2
// 0x141f3d9bd  4d01c5                         addq	%r8, %r13
// Frames: F52 -> F53 -> F54 -> F55 -> F56 -> F57 -> F2
// 0x141f3d9c0  e8bb0bf5fe                     callq	0x140e8e580 <.text+0xe8d580>
// Frames: F58 -> F52 -> F53 -> F54 -> F55 -> F56 -> F57 -> F2
// 0x141f3d9c5  80786801                       cmpb	$0x1, 0x68(%rax)
// 0x141f3d9c9  0f8530070000                   jne	0x141f3e0ff <.text+0x1f3d0ff>
// Frames: F59 -> F60 -> F61 -> F62 -> F63 -> F55 -> F56 -> F57 -> F2
// 0x141f3d9cf  48833800                       cmpq	$0x0, (%rax)
// 0x141f3d9d3  0f851a070000                   jne	0x141f3e0f3 <.text+0x1f3d0f3>
// Frames: F64 -> F65 -> F66 -> F60 -> F61 -> F62 -> F63 -> F55 -> F56 -> F57 -> F2
// 0x141f3d9d9  48c700ffffffff                 movq	$-0x1, (%rax)
// Frames: F67 -> F68 -> F61 -> F62 -> F63 -> F55 -> F56 -> F57 -> F2
// 0x141f3d9e0  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F69 -> F62 -> F63 -> F55 -> F56 -> F57 -> F2
// 0x141f3d9e4  4c897c2420                     movq	%r15, 0x20(%rsp)
// 0x141f3d9e9  48c744243000000000             movq	$0x0, 0x30(%rsp)
// 0x141f3d9f2  48c744242801000000             movq	$0x1, 0x28(%rsp)
// 0x141f3d9fb  4c89e2                         movq	%r12, %rdx
// 0x141f3d9fe  4989f8                         movq	%rdi, %r8
// 0x141f3da01  4d89e9                         movq	%r13, %r9
// 0x141f3da04  488985d0000000                 movq	%rax, 0xd0(%rbp)
// 0x141f3da0b  e880eaf4fe                     callq	0x140e8c490 <.text+0xe8b490>
// 0x141f3da10  90                             nop
// Frames: F64 -> F65 -> F70 -> F71 -> F71 -> F62 -> F63 -> F55 -> F56 -> F57 -> F2
// 0x141f3da11  488b8dd0000000                 movq	0xd0(%rbp), %rcx
// 0x141f3da18  48ff01                         incq	(%rcx)
// Frames: F2
// 0x141f3da1b  488985a8000000                 movq	%rax, 0xa8(%rbp)
// 0x141f3da22  b801000000                     movl	$0x1, %eax
// 0x141f3da27  4c8bade0000000                 movq	0xe0(%rbp), %r13
// 0x141f3da2e  488985a0000000                 movq	%rax, 0xa0(%rbp)
// Frames: F72 -> F73 -> F74
// 0x141f3da35  4d8b0e                         movq	(%r14), %r9
// Frames: F75 -> F73 -> F74
// 0x141f3da38  498b4e08                       movq	0x8(%r14), %rcx
// Frames: F76 -> F77 -> F78 -> F73 -> F74
// 0x141f3da3c  498b11                         movq	(%r9), %rdx
// 0x141f3da3f  4531d2                         xorl	%r10d, %r10d
// Frames: F79 -> F80 -> F77 -> F78 -> F73 -> F74
// 0x141f3da42  4989d0                         movq	%rdx, %r8
// 0x141f3da45  4c2b4118                       subq	0x18(%rcx), %r8
// 0x141f3da49  4d0f42c2                       cmovbq	%r10, %r8
// Frames: F79 -> F81 -> F77 -> F78 -> F73 -> F74
// 0x141f3da4d  482b5138                       subq	0x38(%rcx), %rdx
// 0x141f3da51  488b8578010000                 movq	0x178(%rbp), %rax
// 0x141f3da58  490f42d2                       cmovbq	%r10, %rdx
// Frames: F82 -> F83 -> F84 -> F85 -> F86 -> F77 -> F78 -> F73 -> F74
// 0x141f3da5c  4d89c6                         movq	%r8, %r14
// 0x141f3da5f  49c1e605                       shlq	$0x5, %r14
// 0x141f3da63  4c037108                       addq	0x8(%rcx), %r14
// Frames: F87 -> F88 -> F86 -> F77 -> F78 -> F73 -> F74
// 0x141f3da67  4c8b5910                       movq	0x10(%rcx), %r11
// Frames: F87 -> F88 -> F89 -> F77 -> F78 -> F73 -> F74
// 0x141f3da6b  488b7930                       movq	0x30(%rcx), %rdi
// Frames: F82 -> F83 -> F84 -> F85 -> F86 -> F77 -> F78 -> F73 -> F74
// 0x141f3da6f  4d29c3                         subq	%r8, %r11
// 0x141f3da72  4d0f42da                       cmovbq	%r10, %r11
// 0x141f3da76  41b808000000                   movl	$0x8, %r8d
// 0x141f3da7c  4d0f42f0                       cmovbq	%r8, %r14
// Frames: F82 -> F83 -> F84 -> F85 -> F89 -> F77 -> F78 -> F73 -> F74
// 0x141f3da80  4989d7                         movq	%rdx, %r15
// 0x141f3da83  49c1e705                       shlq	$0x5, %r15
// 0x141f3da87  4c037928                       addq	0x28(%rcx), %r15
// 0x141f3da8b  4829d7                         subq	%rdx, %rdi
// 0x141f3da8e  4d0f42f8                       cmovbq	%r8, %r15
// 0x141f3da92  490f42fa                       cmovbq	%r10, %rdi
// 0x141f3da96  488b9570010000                 movq	0x170(%rbp), %rdx
// Frames: F90 -> F77 -> F78 -> F73 -> F74
// 0x141f3da9d  488b4940                       movq	0x40(%rcx), %rcx
// 0x141f3daa1  4829f9                         subq	%rdi, %rcx
// 0x141f3daa4  4c29d9                         subq	%r11, %rcx
// 0x141f3daa7  4c894d60                       movq	%r9, 0x60(%rbp)
// 0x141f3daab  498909                         movq	%rcx, (%r9)
// Frames: F91 -> F92 -> F93 -> F94 -> F77 -> F78 -> F73 -> F74
// 0x141f3daae  49c1e305                       shlq	$0x5, %r11
// 0x141f3dab2  4d01f3                         addq	%r14, %r11
// 0x141f3dab5  4c895d70                       movq	%r11, 0x70(%rbp)
// 0x141f3dab9  48c1e705                       shlq	$0x5, %rdi
// 0x141f3dabd  4c89bdb0000000                 movq	%r15, 0xb0(%rbp)
// 0x141f3dac4  4c01ff                         addq	%r15, %rdi
// 0x141f3dac7  48897d78                       movq	%rdi, 0x78(%rbp)
// 0x141f3dacb  488b0e                         movq	(%rsi), %rcx
// 0x141f3dace  48894d68                       movq	%rcx, 0x68(%rbp)
// 0x141f3dad2  488b3b                         movq	(%rbx), %rdi
// 0x141f3dad5  488b4b08                       movq	0x8(%rbx), %rcx
// 0x141f3dad9  48898de0000000                 movq	%rcx, 0xe0(%rbp)
// 0x141f3dae0  8b4b14                         movl	0x14(%rbx), %ecx
// 0x141f3dae3  898ddc000000                   movl	%ecx, 0xdc(%rbp)
// 0x141f3dae9  488b0a                         movq	(%rdx), %rcx
// 0x141f3daec  48898d80000000                 movq	%rcx, 0x80(%rbp)
// 0x141f3daf3  498b4d00                       movq	(%r13), %rcx
// 0x141f3daf7  488b00                         movq	(%rax), %rax
// 0x141f3dafa  48898598000000                 movq	%rax, 0x98(%rbp)
// 0x141f3db01  660f76f6                       pcmpeqd	%xmm6, %xmm6
// 0x141f3db05  48898d88000000                 movq	%rcx, 0x88(%rbp)
// 0x141f3db0c  488d4120                       leaq	0x20(%rcx), %rax
// 0x141f3db10  48898590000000                 movq	%rax, 0x90(%rbp)
// 0x141f3db17  eb12                           jmp	0x141f3db2b <.text+0x1f3cb2b>
// 0x141f3db19  0f1f8000000000                 nopl	(%rax)
// 0x141f3db20  488b85d0000000                 movq	0xd0(%rbp), %rax
// 0x141f3db27  4e8d3cb8                       leaq	(%rax,%r15,4), %r15
// Frames: F95 -> F96 -> F97 -> F98 -> F99 -> F74
// 0x141f3db2b  4d85f6                         testq	%r14, %r14
// 0x141f3db2e  7410                           je	0x141f3db40 <.text+0x1f3cb40>
// 0x141f3db30  4c3b7570                       cmpq	0x70(%rbp), %r14
// 0x141f3db34  740a                           je	0x141f3db40 <.text+0x1f3cb40>
// 0x141f3db36  498d4620                       leaq	0x20(%r14), %rax
// 0x141f3db3a  4c89f6                         movq	%r14, %rsi
// 0x141f3db3d  eb2b                           jmp	0x141f3db6a <.text+0x1f3cb6a>
// 0x141f3db3f  90                             nop
// Frames: F95 -> F100 -> F101 -> F97 -> F98 -> F99 -> F74
// 0x141f3db40  488b85b0000000                 movq	0xb0(%rbp), %rax
// 0x141f3db47  4885c0                         testq	%rax, %rax
// 0x141f3db4a  0f84be040000                   je	0x141f3e00e <.text+0x1f3d00e>
// 0x141f3db50  483b4578                       cmpq	0x78(%rbp), %rax
// 0x141f3db54  0f84b4040000                   je	0x141f3e00e <.text+0x1f3d00e>
// Frames: F102 -> F103 -> F100 -> F101 -> F97 -> F98 -> F99 -> F74
// 0x141f3db5a  4889c6                         movq	%rax, %rsi
// 0x141f3db5d  488d4820                       leaq	0x20(%rax), %rcx
// 0x141f3db61  31c0                           xorl	%eax, %eax
// 0x141f3db63  48898db0000000                 movq	%rcx, 0xb0(%rbp)
// Frames: F74
// 0x141f3db6a  4989c6                         movq	%rax, %r14
// 0x141f3db6d  488b4560                       movq	0x60(%rbp), %rax
// Frames: F104 -> F99 -> F74
// 0x141f3db71  48ff00                         incq	(%rax)
// Frames: F105 -> F106 -> F107
// 0x141f3db74  488b4d68                       movq	0x68(%rbp), %rcx
// 0x141f3db78  488d1521bd4d01                 leaq	0x14dbd21(%rip), %rdx   # 0x1434198a0
// 0x141f3db7f  e8fc460000                     callq	0x141f42280 <.text+0x1f41280>
// 0x141f3db84  90                             nop
// 0x141f3db85  4885c0                         testq	%rax, %rax
// 0x141f3db88  0f84ee050000                   je	0x141f3e17c <.text+0x1f3d17c>
// Frames: F108 -> F106 -> F107
// 0x141f3db8e  488b08                         movq	(%rax), %rcx
// 0x141f3db91  488b4008                       movq	0x8(%rax), %rax
// Frames: F106 -> F107
// 0x141f3db95  ff5018                         callq	*0x18(%rax)
// 0x141f3db98  90                             nop
// Frames: F107
// 0x141f3db99  4889c3                         movq	%rax, %rbx
// Frames: F109 -> F107
// 0x141f3db9c  4885c0                         testq	%rax, %rax
// 0x141f3db9f  0f84d7050000                   je	0x141f3e17c <.text+0x1f3d17c>
// Frames: F110
// 0x141f3dba5  488b4e08                       movq	0x8(%rsi), %rcx
// 0x141f3dba9  89c8                           movl	%ecx, %eax
// 0x141f3dbab  f7d0                           notl	%eax
// 0x141f3dbad  488b95e0000000                 movq	0xe0(%rbp), %rdx
// Frames: F111 -> F85 -> F112 -> F113 -> F114 -> F115 -> F110
// 0x141f3dbb4  483b4210                       cmpq	0x10(%rdx), %rax
// Frames: F116 -> F117 -> F113 -> F114 -> F115 -> F110
// 0x141f3dbb8  0f8375050000                   jae	0x141f3e133 <.text+0x1f3d133>
// Frames: F118 -> F119 -> F120 -> F117 -> F113 -> F114 -> F115 -> F110
// 0x141f3dbbe  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x141f3dbc2  48c1e003                       shlq	$0x3, %rax
// 0x141f3dbc6  488b95e0000000                 movq	0xe0(%rbp), %rdx
// 0x141f3dbcd  48034208                       addq	0x8(%rdx), %rax
// 0x141f3dbd1  4889ca                         movq	%rcx, %rdx
// 0x141f3dbd4  48c1ea20                       shrq	$0x20, %rdx
// 0x141f3dbd8  395010                         cmpl	%edx, 0x10(%rax)
// Frames: F121 -> F117 -> F113 -> F114 -> F115 -> F110
// 0x141f3dbdb  0f8552050000                   jne	0x141f3e133 <.text+0x1f3d133>
// Frames: F122 -> F123 -> F115 -> F110
// 0x141f3dbe1  833800                         cmpl	$0x0, (%rax)
// 0x141f3dbe4  0f8449050000                   je	0x141f3e133 <.text+0x1f3d133>
// Frames: F124 -> F125 -> F115 -> F110
// 0x141f3dbea  4c8b7808                       movq	0x8(%rax), %r15
// 0x141f3dbee  4489fa                         movl	%r15d, %edx
// Frames: F126 -> F125 -> F115 -> F110
// 0x141f3dbf1  483b5738                       cmpq	0x38(%rdi), %rdx
// Frames: F127 -> F128 -> F125 -> F115 -> F110
// 0x141f3dbf5  0f834a050000                   jae	0x141f3e145 <.text+0x1f3d145>
// Frames: F129 -> F130 -> F131 -> F127 -> F128 -> F125 -> F115 -> F110
// 0x141f3dbfb  4c8b4728                       movq	0x28(%rdi), %r8
// Frames: F91 -> F129 -> F130 -> F131 -> F127 -> F128 -> F125 -> F115 -> F110
// 0x141f3dbff  c1ea06                         shrl	$0x6, %edx
// Frames: F132 -> F130 -> F131 -> F127 -> F128 -> F125 -> F115 -> F110
// 0x141f3dc02  498b14d0                       movq	(%r8,%rdx,8), %rdx
// Frames: F130 -> F131 -> F127 -> F128 -> F125 -> F115 -> F110
// 0x141f3dc06  4c0fa3fa                       btq	%r15, %rdx
// Frames: F133 -> F115 -> F110
// 0x141f3dc0a  0f8335050000                   jae	0x141f3e145 <.text+0x1f3d145>
// Frames: F134 -> F135 -> F136 -> F137 -> F87 -> F88 -> F138 -> F139 -> F115 -> F110
// 0x141f3dc10  49c1ef20                       shrq	$0x20, %r15
// 0x141f3dc14  488b8de0000000                 movq	0xe0(%rbp), %rcx
// 0x141f3dc1b  4c8b89a0010000                 movq	0x1a0(%rcx), %r9
// Frames: F140 -> F141 -> F138 -> F139 -> F115 -> F110
// 0x141f3dc22  4f8d14ff                       leaq	(%r15,%r15,8), %r10
// Frames: F142 -> F115 -> F110
// 0x141f3dc26  4c8b8718010000                 movq	0x118(%rdi), %r8
// Frames: F143 -> F144 -> F145 -> F146 -> F147 -> F142 -> F115 -> F110
// 0x141f3dc2d  4b8b4cd118                     movq	0x18(%r9,%r10,8), %rcx
// 0x141f3dc32  4b8b54d138                     movq	0x38(%r9,%r10,8), %rdx
// Frames: F111 -> F85 -> F148 -> F143 -> F144 -> F149 -> F150 -> F151 -> F147 -> F142 -> F115 -> F110
// 0x141f3dc37  4f3b44d140                     cmpq	0x40(%r9,%r10,8), %r8
// 0x141f3dc3c  7322                           jae	0x141f3dc60 <.text+0x1f3cc60>
// Frames: F152 -> F53 -> F153 -> F148 -> F143 -> F144 -> F149 -> F150 -> F151 -> F147 -> F142 -> F115 -> F110
// 0x141f3dc3e  4e8b04c2                       movq	(%rdx,%r8,8), %r8
// 0x141f3dc42  4d85c0                         testq	%r8, %r8
// 0x141f3dc45  7419                           je	0x141f3dc60 <.text+0x1f3cc60>
// Frames: F154 -> F155 -> F156 -> F143 -> F144 -> F149 -> F150 -> F151 -> F147 -> F142 -> F115 -> F110
// 0x141f3dc47  49f7d0                         notq	%r8
// Frames: F157 -> F158 -> F155 -> F156 -> F143 -> F144 -> F149 -> F150 -> F151 -> F147 -> F142 -> F115 -> F110
// 0x141f3dc4a  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x141f3dc4e  49c1e004                       shlq	$0x4, %r8
// Frames: F159 -> F160 -> F161 -> F156 -> F162 -> F150 -> F151 -> F147 -> F142 -> F115 -> F110
// 0x141f3dc52  4e8b6c0110                     movq	0x10(%rcx,%r8), %r13
// 0x141f3dc57  eb0a                           jmp	0x141f3dc63 <.text+0x1f3cc63>
// 0x141f3dc59  0f1f8000000000                 nopl	(%rax)
// 0x141f3dc60  4531ed                         xorl	%r13d, %r13d
// Frames: F163 -> F164 -> F165 -> F166 -> F167 -> F168 -> F169 -> F115 -> F110
// 0x141f3dc63  448b7804                       movl	0x4(%rax), %r15d
// 0x141f3dc67  488b8710010000                 movq	0x110(%rdi), %rax
// 0x141f3dc6e  488b04c2                       movq	(%rdx,%rax,8), %rax
// 0x141f3dc72  48f7d0                         notq	%rax
// 0x141f3dc75  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x141f3dc79  48c1e004                       shlq	$0x4, %rax
// 0x141f3dc7d  488b540110                     movq	0x10(%rcx,%rax), %rdx
// 0x141f3dc82  4c8b440120                     movq	0x20(%rcx,%rax), %r8
// 0x141f3dc87  4c8985d0000000                 movq	%r8, 0xd0(%rbp)
// 0x141f3dc8e  4c8b640128                     movq	0x28(%rcx,%rax), %r12
// 0x141f3dc93  41f7d7                         notl	%r15d
// Frames: F91 -> F170 -> F165 -> F166 -> F167 -> F168 -> F169 -> F115 -> F110
// 0x141f3dc96  4c89f8                         movq	%r15, %rax
// 0x141f3dc99  48c1e004                       shlq	$0x4, %rax
// 0x141f3dc9d  488d3402                       leaq	(%rdx,%rax), %rsi
// Frames: F171 -> F172 -> F173
// 0x141f3dca1  48833c0200                     cmpq	$0x0, (%rdx,%rax)
// Frames: F173
// 0x141f3dca6  0f85bd000000                   jne	0x141f3dd69 <.text+0x1f3cd69>
// Frames: F174 -> F175 -> F176 -> F177 -> F178 -> F179 -> F180 -> F181
// 0x141f3dcac  e85f560cfe                     callq	0x140003310 <.text+0x2310>
// Frames: F182 -> F175 -> F176 -> F177 -> F178 -> F179 -> F180 -> F181
// 0x141f3dcb1  b920000000                     movl	$0x20, %ecx
// 0x141f3dcb6  ba08000000                     movl	$0x8, %edx
// 0x141f3dcbb  e8207c76fe                     callq	0x1406a58e0 <.text+0x6a48e0>
// Frames: F178 -> F179 -> F180 -> F181
// 0x141f3dcc0  4885c0                         testq	%rax, %rax
// 0x141f3dcc3  0f8415040000                   je	0x141f3e0de <.text+0x1f3d0de>
// Frames: F183 -> F180 -> F181
// 0x141f3dcc9  4889c2                         movq	%rax, %rdx
// 0x141f3dccc  48c70000000000                 movq	$0x0, (%rax)
// 0x141f3dcd3  48c7400804000000               movq	$0x4, 0x8(%rax)
// 0x141f3dcdb  48c7401000000000               movq	$0x0, 0x10(%rax)
// 0x141f3dce3  c6401800                       movb	$0x0, 0x18(%rax)
// Frames: F184 -> F185 -> F186 -> F187 -> F181
// 0x141f3dce7  4c8b4348                       movq	0x48(%rbx), %r8
// 0x141f3dceb  eb05                           jmp	0x141f3dcf2 <.text+0x1f3ccf2>
// 0x141f3dced  0f1f00                         nopl	(%rax)
// Frames: F188 -> F189 -> F190 -> F187 -> F181
// 0x141f3dcf0  f390                           pause
// Frames: F191 -> F192 -> F193 -> F187 -> F181
// 0x141f3dcf2  498b4008                       movq	0x8(%r8), %rax
// 0x141f3dcf6  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F194 -> F187 -> F181
// 0x141f3dd00  4883f8ff                       cmpq	$-0x1, %rax
// 0x141f3dd04  74ea                           je	0x141f3dcf0 <.text+0x1f3ccf0>
// Frames: F195 -> F187 -> F181
// 0x141f3dd06  4885c0                         testq	%rax, %rax
// 0x141f3dd09  0f8866030000                   js	0x141f3e075 <.text+0x1f3d075>
// Frames: F196 -> F187 -> F181
// 0x141f3dd0f  488d4801                       leaq	0x1(%rax), %rcx
// Frames: F197 -> F198 -> F196 -> F187 -> F181
// 0x141f3dd13  f0                             lock
// 0x141f3dd14  490fb14808                     cmpxchgq	%rcx, 0x8(%r8)
// Frames: F196 -> F187 -> F181
// 0x141f3dd19  75e5                           jne	0x141f3dd00 <.text+0x1f3cd00>
// Frames: F199 -> F200 -> F181
// 0x141f3dd1b  4c8985c0000000                 movq	%r8, 0xc0(%rbp)
// 0x141f3dd22  488995b8000000                 movq	%rdx, 0xb8(%rbp)
// 0x141f3dd29  8b85dc000000                   movl	0xdc(%rbp), %eax
// 0x141f3dd2f  438904bc                       movl	%eax, (%r12,%r15,4)
// Frames: F71 -> F181
// 0x141f3dd33  48833e00                       cmpq	$0x0, (%rsi)
// 0x141f3dd37  4889b5c8000000                 movq	%rsi, 0xc8(%rbp)
// 0x141f3dd3e  740d                           je	0x141f3dd4d <.text+0x1f3cd4d>
// 0x141f3dd40  488b8dc8000000                 movq	0xc8(%rbp), %rcx
// 0x141f3dd47  e8841908ff                     callq	0x140fbf6d0 <.text+0xfbe6d0>
// 0x141f3dd4c  90                             nop
// Frames: F181
// 0x141f3dd4d  488b85c0000000                 movq	0xc0(%rbp), %rax
// 0x141f3dd54  488bb5c8000000                 movq	0xc8(%rbp), %rsi
// 0x141f3dd5b  488906                         movq	%rax, (%rsi)
// 0x141f3dd5e  488b85b8000000                 movq	0xb8(%rbp), %rax
// 0x141f3dd65  48894608                       movq	%rax, 0x8(%rsi)
// Frames: F199 -> F200 -> F201
// 0x141f3dd69  8b85dc000000                   movl	0xdc(%rbp), %eax
// 0x141f3dd6f  438904bc                       movl	%eax, (%r12,%r15,4)
// Frames: F95 -> F201
// 0x141f3dd73  48833e00                       cmpq	$0x0, (%rsi)
// Frames: F109 -> F201
// 0x141f3dd77  0f842b030000                   je	0x141f3e0a8 <.text+0x1f3d0a8>
// Frames: F202 -> F203
// 0x141f3dd7d  488b7608                       movq	0x8(%rsi), %rsi
// Frames: F109 -> F202 -> F203
// 0x141f3dd81  4885f6                         testq	%rsi, %rsi
// 0x141f3dd84  0f8430030000                   je	0x141f3e0ba <.text+0x1f3d0ba>
// Frames: F204 -> F203
// 0x141f3dd8a  4c8d6618                       leaq	0x18(%rsi), %r12
// Frames: F197 -> F205 -> F206 -> F204 -> F203
// 0x141f3dd8e  31c0                           xorl	%eax, %eax
// 0x141f3dd90  b101                           movb	$0x1, %cl
// 0x141f3dd92  f0                             lock
// 0x141f3dd93  0fb04e18                       cmpxchgb	%cl, 0x18(%rsi)
// Frames: F207 -> F204 -> F203
// 0x141f3dd97  0f855d020000                   jne	0x141f3dffa <.text+0x1f3cffa>
// Frames: F208 -> F209 -> F204 -> F203
// 0x141f3dd9d  4c89e1                         movq	%r12, %rcx
// 0x141f3dda0  e82b39f5fe                     callq	0x140e916d0 <.text+0xe906d0>
// 0x141f3dda5  90                             nop
// Frames: F210 -> F203
// 0x141f3dda6  488b4610                       movq	0x10(%rsi), %rax
// Frames: F211 -> F212 -> F203
// 0x141f3ddaa  4885c0                         testq	%rax, %rax
// Frames: F213 -> F214 -> F203
// 0x141f3ddad  7421                           je	0x141f3ddd0 <.text+0x1f3cdd0>
// 0x141f3ddaf  488b4e08                       movq	0x8(%rsi), %rcx
// 0x141f3ddb3  488d0481                       leaq	(%rcx,%rax,4), %rax
// 0x141f3ddb7  4883c0fc                       addq	$-0x4, %rax
// 0x141f3ddbb  4885c0                         testq	%rax, %rax
// 0x141f3ddbe  7410                           je	0x141f3ddd0 <.text+0x1f3cdd0>
// Frames: F215 -> F214 -> F203
// 0x141f3ddc0  8b08                           movl	(%rax), %ecx
// Frames: F216 -> F203
// 0x141f3ddc2  488b4310                       movq	0x10(%rbx), %rax
// Frames: F217 -> F216 -> F203
// 0x141f3ddc6  eb18                           jmp	0x141f3dde0 <.text+0x1f3cde0>
// 0x141f3ddc8  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F216 -> F203
// 0x141f3ddd0  488b4310                       movq	0x10(%rbx), %rax
// Frames: F82 -> F218 -> F219 -> F216 -> F203
// 0x141f3ddd4  4885c0                         testq	%rax, %rax
// Frames: F109 -> F218 -> F219 -> F216 -> F203
// 0x141f3ddd7  0f84ef020000                   je	0x141f3e0cc <.text+0x1f3d0cc>
// Frames: F218 -> F219 -> F216 -> F203
// 0x141f3dddd  8d48ff                         leal	-0x1(%rax), %ecx
// Frames: F220 -> F203
// 0x141f3dde0  488b5308                       movq	0x8(%rbx), %rdx
// Frames: F221 -> F203
// 0x141f3dde4  f30f6f4318                     movdqu	0x18(%rbx), %xmm0
// Frames: F222 -> F203
// 0x141f3dde9  f30f6f4b28                     movdqu	0x28(%rbx), %xmm1
// Frames: F223 -> F203
// 0x141f3ddee  4c8d4378                       leaq	0x78(%rbx), %r8
// Frames: F224 -> F203
// 0x141f3ddf2  4c8d4b58                       leaq	0x58(%rbx), %r9
// Frames: F225 -> F203
// 0x141f3ddf6  f30f6f5338                     movdqu	0x38(%rbx), %xmm2
// Frames: F226 -> F203
// 0x141f3ddfb  894d48                         movl	%ecx, 0x48(%rbp)
// 0x141f3ddfe  488955f0                       movq	%rdx, -0x10(%rbp)
// 0x141f3de02  488945f8                       movq	%rax, -0x8(%rbp)
// 0x141f3de06  f30f7f4500                     movdqu	%xmm0, (%rbp)
// 0x141f3de0b  48897510                       movq	%rsi, 0x10(%rbp)
// 0x141f3de0f  f30f7f4d18                     movdqu	%xmm1, 0x18(%rbp)
// 0x141f3de14  4c894528                       movq	%r8, 0x28(%rbp)
// 0x141f3de18  4c894d30                       movq	%r9, 0x30(%rbp)
// 0x141f3de1c  f30f7f5538                     movdqu	%xmm2, 0x38(%rbp)
// Frames: F227
// 0x141f3de21  488d4df0                       leaq	-0x10(%rbp), %rcx
// 0x141f3de25  e8563c0000                     callq	0x141f41a80 <.text+0x1f40a80>
// 0x141f3de2a  90                             nop
// Frames: F228
// 0x141f3de2b  496bc758                       imulq	$0x58, %r15, %rax
// 0x141f3de2f  4901c5                         addq	%rax, %r13
// 0x141f3de32  4c89e9                         movq	%r13, %rcx
// 0x141f3de35  e856b0cafe                     callq	0x140be8e90 <.text+0xbe7e90>
// 0x141f3de3a  90                             nop
// 0x141f3de3b  84c0                           testb	%al, %al
// 0x141f3de3d  0f85dd000000                   jne	0x141f3df20 <.text+0x1f3cf20>
// Frames: F229
// 0x141f3de43  498b5500                       movq	(%r13), %rdx
// 0x141f3de47  488b8d80000000                 movq	0x80(%rbp), %rcx
// 0x141f3de4e  4c8d0563b34d01                 leaq	0x14db363(%rip), %r8    # 0x1434191b8
// 0x141f3de55  e81634c6fe                     callq	0x140ba1270 <.text+0xba0270>
// 0x141f3de5a  90                             nop
// Frames: F230 -> F229
// 0x141f3de5b  4885c0                         testq	%rax, %rax
// 0x141f3de5e  0f84bc000000                   je	0x141f3df20 <.text+0x1f3cf20>
// Frames: F156 -> F229
// 0x141f3de64  8b9874020000                   movl	0x274(%rax), %ebx
// 0x141f3de6a  488bb588000000                 movq	0x88(%rbp), %rsi
// Frames: F231 -> F232 -> F233
// 0x141f3de71  48837e1800                     cmpq	$0x0, 0x18(%rsi)
// 0x141f3de76  0f84d3000000                   je	0x141f3df4f <.text+0x1f3cf4f>
// Frames: F234 -> F235 -> F232 -> F233
// 0x141f3de7c  488b8d90000000                 movq	0x90(%rbp), %rcx
// 0x141f3de83  4c89ea                         movq	%r13, %rdx
// 0x141f3de86  e875d907ff                     callq	0x140fbb800 <.text+0xfba800>
// Frames: F236 -> F237 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3de8b  4989c0                         movq	%rax, %r8
// 0x141f3de8e  49c1e839                       shrq	$0x39, %r8
// 0x141f3de92  488b0e                         movq	(%rsi), %rcx
// Frames: F241 -> F242 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3de95  488b5608                       movq	0x8(%rsi), %rdx
// 0x141f3de99  66410f6ec0                     movd	%r8d, %xmm0
// 0x141f3de9e  660f60c0                       punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
// 0x141f3dea2  f20f70c000                     pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
// 0x141f3dea7  660f70c044                     pshufd	$0x44, %xmm0, %xmm0     # xmm0 = xmm0[0,1,0,1]
// 0x141f3deac  4531c0                         xorl	%r8d, %r8d
// Frames: F243 -> F244 -> F245 -> F246 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3deaf  4821d0                         andq	%rdx, %rax
// 0x141f3deb2  f30f6f0c01                     movdqu	(%rcx,%rax), %xmm1
// Frames: F247 -> F248 -> F249 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3deb7  660f6fd1                       movdqa	%xmm1, %xmm2
// 0x141f3debb  660f74d0                       pcmpeqb	%xmm0, %xmm2
// 0x141f3debf  66440fd7ca                     pmovmskb	%xmm2, %r9d
// Frames: F250 -> F251 -> F249 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3dec4  4585c9                         testl	%r9d, %r9d
// 0x141f3dec7  7437                           je	0x141f3df00 <.text+0x1f3cf00>
// 0x141f3dec9  4d8b5500                       movq	(%r13), %r10
// Frames: F252 -> F253 -> F254 -> F251 -> F249 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3decd  f3410fbcf1                     tzcntl	%r9d, %esi
// Frames: F255 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3ded2  4801c6                         addq	%rax, %rsi
// 0x141f3ded5  4821d6                         andq	%rdx, %rsi
// Frames: F256 -> F257 -> F258 -> F259 -> F260 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3ded8  48c1e605                       shlq	$0x5, %rsi
// 0x141f3dedc  4989cb                         movq	%rcx, %r11
// 0x141f3dedf  4929f3                         subq	%rsi, %r11
// Frames: F261 -> F262 -> F263 -> F264 -> F259 -> F260 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3dee2  4d3b53e0                       cmpq	-0x20(%r11), %r10
// Frames: F265 -> F260 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3dee6  0f84a9000000                   je	0x141f3df95 <.text+0x1f3cf95>
// Frames: F266 -> F252 -> F253 -> F254 -> F251 -> F249 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3deec  458d59ff                       leal	-0x1(%r9), %r11d
// Frames: F250 -> F251 -> F249 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3def0  664521cb                       andw	%r9w, %r11w
// 0x141f3def4  4589d9                         movl	%r11d, %r9d
// 0x141f3def7  75d4                           jne	0x141f3decd <.text+0x1f3cecd>
// 0x141f3def9  0f1f8000000000                 nopl	(%rax)
// Frames: F247 -> F248 -> F267 -> F268 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3df00  660f74ce                       pcmpeqb	%xmm6, %xmm1
// 0x141f3df04  66440fd7c9                     pmovmskb	%xmm1, %r9d
// Frames: F269 -> F268 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3df09  4585c9                         testl	%r9d, %r9d
// 0x141f3df0c  7541                           jne	0x141f3df4f <.text+0x1f3cf4f>
// Frames: F270 -> F271 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3df0e  4c01c0                         addq	%r8, %rax
// 0x141f3df11  4883c010                       addq	$0x10, %rax
// Frames: F272 -> F271 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3df15  4983c010                       addq	$0x10, %r8
// Frames: F273 -> F238 -> F239 -> F240 -> F232 -> F233
// 0x141f3df19  eb94                           jmp	0x141f3deaf <.text+0x1f3ceaf>
// 0x141f3df1b  0f1f440000                     nopl	(%rax,%rax)
// Frames: F71 -> F274
// 0x141f3df20  488b5d10                       movq	0x10(%rbp), %rbx
// Frames: F275 -> F71 -> F274
// 0x141f3df24  4883c318                       addq	$0x18, %rbx
// Frames: F276 -> F277 -> F275 -> F71 -> F274
// 0x141f3df28  4889d9                         movq	%rbx, %rcx
// 0x141f3df2b  e8a039f5fe                     callq	0x140e918d0 <.text+0xe908d0>
// 0x141f3df30  90                             nop
// Frames: F278 -> F279 -> F280 -> F275 -> F71 -> F274
// 0x141f3df31  b001                           movb	$0x1, %al
// 0x141f3df33  31c9                           xorl	%ecx, %ecx
// 0x141f3df35  f0                             lock
// 0x141f3df36  0fb00b                         cmpxchgb	%cl, (%rbx)
// Frames: F281 -> F275 -> F71 -> F274
// 0x141f3df39  0f84e1fbffff                   je	0x141f3db20 <.text+0x1f3cb20>
// Frames: F282 -> F275 -> F71 -> F274
// 0x141f3df3f  4889d9                         movq	%rbx, %rcx
// 0x141f3df42  31d2                           xorl	%edx, %edx
// 0x141f3df44  e84785e600                     callq	0x142da6490 <.text+0x2da5490>
// 0x141f3df49  90                             nop
// 0x141f3df4a  e9d1fbffff                     jmp	0x141f3db20 <.text+0x1f3cb20>
// Frames: F283
// 0x141f3df4f  4531c9                         xorl	%r9d, %r9d
// 0x141f3df52  498d450c                       leaq	0xc(%r13), %rax
// 0x141f3df56  41807d0800                     cmpb	$0x0, 0x8(%r13)
// 0x141f3df5b  744c                           je	0x141f3dfa9 <.text+0x1f3cfa9>
// Frames: F284
// 0x141f3df5d  488b4810                       movq	0x10(%rax), %rcx
// 0x141f3df61  48894dd0                       movq	%rcx, -0x30(%rbp)
// 0x141f3df65  f30f6f00                       movdqu	(%rax), %xmm0
// 0x141f3df69  660f7f45c0                     movdqa	%xmm0, -0x40(%rbp)
// Frames: F285
// 0x141f3df6e  488d45f0                       leaq	-0x10(%rbp), %rax
// 0x141f3df72  4889442428                     movq	%rax, 0x28(%rsp)
// 0x141f3df77  488b8598000000                 movq	0x98(%rbp), %rax
// 0x141f3df7e  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141f3df83  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x141f3df87  4c89ea                         movq	%r13, %rdx
// 0x141f3df8a  4189d8                         movl	%ebx, %r8d
// 0x141f3df8d  e82edb8e00                     callq	0x14282bac0 <.text+0x282aac0>
// 0x141f3df92  90                             nop
// 0x141f3df93  eb36                           jmp	0x141f3dfcb <.text+0x1f3cfcb>
// Frames: F286 -> F287 -> F288 -> F233
// 0x141f3df95  49837bf800                     cmpq	$0x0, -0x8(%r11)
// 0x141f3df9a  410f95c1                       setne	%r9b
// 0x141f3df9e  498d450c                       leaq	0xc(%r13), %rax
// Frames: F283
// 0x141f3dfa2  41807d0800                     cmpb	$0x0, 0x8(%r13)
// 0x141f3dfa7  75b4                           jne	0x141f3df5d <.text+0x1f3cf5d>
// Frames: F289
// 0x141f3dfa9  f30f6f00                       movdqu	(%rax), %xmm0
// 0x141f3dfad  660f7f45e0                     movdqa	%xmm0, -0x20(%rbp)
// Frames: F290
// 0x141f3dfb2  488d45f0                       leaq	-0x10(%rbp), %rax
// 0x141f3dfb6  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141f3dfbb  488d4de0                       leaq	-0x20(%rbp), %rcx
// 0x141f3dfbf  4c89ea                         movq	%r13, %rdx
// 0x141f3dfc2  4189d8                         movl	%ebx, %r8d
// 0x141f3dfc5  e8261a8f00                     callq	0x14282f9f0 <.text+0x282e9f0>
// 0x141f3dfca  90                             nop
// Frames: F71 -> F274
// 0x141f3dfcb  488b5d10                       movq	0x10(%rbp), %rbx
// Frames: F275 -> F71 -> F274
// 0x141f3dfcf  4883c318                       addq	$0x18, %rbx
// Frames: F276 -> F277 -> F275 -> F71 -> F274
// 0x141f3dfd3  4889d9                         movq	%rbx, %rcx
// 0x141f3dfd6  e8f538f5fe                     callq	0x140e918d0 <.text+0xe908d0>
// 0x141f3dfdb  90                             nop
// Frames: F278 -> F279 -> F280 -> F275 -> F71 -> F274
// 0x141f3dfdc  b001                           movb	$0x1, %al
// 0x141f3dfde  31c9                           xorl	%ecx, %ecx
// 0x141f3dfe0  f0                             lock
// 0x141f3dfe1  0fb00b                         cmpxchgb	%cl, (%rbx)
// Frames: F281 -> F275 -> F71 -> F274
// 0x141f3dfe4  0f8436fbffff                   je	0x141f3db20 <.text+0x1f3cb20>
// Frames: F282 -> F275 -> F71 -> F274
// 0x141f3dfea  4889d9                         movq	%rbx, %rcx
// 0x141f3dfed  31d2                           xorl	%edx, %edx
// 0x141f3dfef  e89c84e600                     callq	0x142da6490 <.text+0x2da5490>
// 0x141f3dff4  90                             nop
// 0x141f3dff5  e926fbffff                     jmp	0x141f3db20 <.text+0x1f3cb20>
// Frames: F291 -> F204 -> F203
// 0x141f3dffa  4c89e1                         movq	%r12, %rcx
// 0x141f3dffd  41b800ca9a3b                   movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
// 0x141f3e003  e83887e600                     callq	0x142da6740 <.text+0x2da5740>
// 0x141f3e008  90                             nop
// 0x141f3e009  e98ffdffff                     jmp	0x141f3dd9d <.text+0x1f3cd9d>
// Frames: F71 -> F292
// 0x141f3e00e  4883bda000000000               cmpq	$0x0, 0xa0(%rbp)
// 0x141f3e016  7442                           je	0x141f3e05a <.text+0x1f3d05a>
// Frames: F52 -> F53 -> F54 -> F55 -> F56 -> F293 -> F71 -> F71 -> F292
// 0x141f3e018  e86305f5fe                     callq	0x140e8e580 <.text+0xe8d580>
// Frames: F58 -> F52 -> F53 -> F54 -> F55 -> F56 -> F293 -> F71 -> F71 -> F292
// 0x141f3e01d  80786801                       cmpb	$0x1, 0x68(%rax)
// 0x141f3e021  0f85ed000000                   jne	0x141f3e114 <.text+0x1f3d114>
// Frames: F59 -> F60 -> F61 -> F62 -> F63 -> F55 -> F56 -> F293 -> F71 -> F71 -> F292
// 0x141f3e027  48833800                       cmpq	$0x0, (%rax)
// 0x141f3e02b  0f85c2000000                   jne	0x141f3e0f3 <.text+0x1f3d0f3>
// Frames: F64 -> F65 -> F66 -> F60 -> F61 -> F62 -> F63 -> F55 -> F56 -> F293 -> F71 -> F71 -> F292
// 0x141f3e031  48c700ffffffff                 movq	$-0x1, (%rax)
// Frames: F67 -> F68 -> F61 -> F62 -> F63 -> F55 -> F56 -> F293 -> F71 -> F71 -> F292
// 0x141f3e038  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F294 -> F62 -> F63 -> F55 -> F56 -> F293 -> F71 -> F71 -> F292
// 0x141f3e03c  488b95a8000000                 movq	0xa8(%rbp), %rdx
// 0x141f3e043  488985e0000000                 movq	%rax, 0xe0(%rbp)
// 0x141f3e04a  e8a1e8f4fe                     callq	0x140e8c8f0 <.text+0xe8b8f0>
// 0x141f3e04f  90                             nop
// Frames: F64 -> F65 -> F70 -> F71 -> F71 -> F62 -> F63 -> F55 -> F56 -> F293 -> F71 -> F71 -> F292
// 0x141f3e050  488b85e0000000                 movq	0xe0(%rbp), %rax
// 0x141f3e057  48ff00                         incq	(%rax)
// Frames: F292
// 0x141f3e05a  0f28b5f0000000                 movaps	0xf0(%rbp), %xmm6
// 0x141f3e061  4881c488010000                 addq	$0x188, %rsp            # imm = 0x188
// 0x141f3e068  5b                             popq	%rbx
// 0x141f3e069  5f                             popq	%rdi
// 0x141f3e06a  5e                             popq	%rsi
// 0x141f3e06b  415c                           popq	%r12
// 0x141f3e06d  415d                           popq	%r13
// 0x141f3e06f  415e                           popq	%r14
// 0x141f3e071  415f                           popq	%r15
// 0x141f3e073  5d                             popq	%rbp
// 0x141f3e074  c3                             retq
// Frames: F295 -> F195 -> F187 -> F181
// 0x141f3e075  488d05242e4d01                 leaq	0x14d2e24(%rip), %rax   # 0x143410ea0
// 0x141f3e07c  488945f0                       movq	%rax, -0x10(%rbp)
// 0x141f3e080  488d0559474dfe                 leaq	-0x1b2b8a7(%rip), %rax  # 0x1404127e0 <.text+0x4117e0>
// 0x141f3e087  488945f8                       movq	%rax, -0x8(%rbp)
// 0x141f3e08b  488d0d1e2e4d01                 leaq	0x14d2e1e(%rip), %rcx   # 0x143410eb0
// 0x141f3e092  4c8d059f2e4d01                 leaq	0x14d2e9f(%rip), %r8    # 0x143410f38
// 0x141f3e099  488d55f0                       leaq	-0x10(%rbp), %rdx
// 0x141f3e09d  e8fe37e700                     callq	0x142db18a0 <.text+0x2db08a0>
// 0x141f3e0a2  90                             nop
// 0x141f3e0a3  e9e1000000                     jmp	0x141f3e189 <.text+0x1f3d189>
// Frames: F296 -> F201
// 0x141f3e0a8  488d0df1b04d01                 leaq	0x14db0f1(%rip), %rcx   # 0x1434191a0
// 0x141f3e0af  e86c35e700                     callq	0x142db1620 <.text+0x2db0620>
// 0x141f3e0b4  90                             nop
// 0x141f3e0b5  e9cf000000                     jmp	0x141f3e189 <.text+0x1f3d189>
// Frames: F296 -> F202 -> F203
// 0x141f3e0ba  488d0d2fb94d01                 leaq	0x14db92f(%rip), %rcx   # 0x1434199f0
// 0x141f3e0c1  e85a35e700                     callq	0x142db1620 <.text+0x2db0620>
// 0x141f3e0c6  90                             nop
// 0x141f3e0c7  e9bd000000                     jmp	0x141f3e189 <.text+0x1f3d189>
// Frames: F296 -> F218 -> F219 -> F216 -> F203
// 0x141f3e0cc  488d0d35b94d01                 leaq	0x14db935(%rip), %rcx   # 0x143419a08
// 0x141f3e0d3  e84835e700                     callq	0x142db1620 <.text+0x2db0620>
// 0x141f3e0d8  90                             nop
// 0x141f3e0d9  e9ab000000                     jmp	0x141f3e189 <.text+0x1f3d189>
// Frames: F297 -> F179 -> F180 -> F181
// 0x141f3e0de  b908000000                     movl	$0x8, %ecx
// 0x141f3e0e3  ba20000000                     movl	$0x20, %edx
// 0x141f3e0e8  e8f431e700                     callq	0x142db12e1 <.text+0x2db02e1>
// 0x141f3e0ed  90                             nop
// 0x141f3e0ee  e996000000                     jmp	0x141f3e189 <.text+0x1f3d189>
// 0x141f3e0f3  488d0d56a16401                 leaq	0x164a156(%rip), %rcx   # 0x143588250
// 0x141f3e0fa  e86134e700                     callq	0x142db1560 <.text+0x2db0560>
// Frames: F298 -> F52 -> F53 -> F54 -> F55 -> F56 -> F57 -> F2
// 0x141f3e0ff  4889c1                         movq	%rax, %rcx
// 0x141f3e102  31d2                           xorl	%edx, %edx
// 0x141f3e104  e82749ed00                     callq	0x142e12a30 <.text+0x2e11a30>
// Frames: F299 -> F54 -> F55 -> F56 -> F57 -> F2
// 0x141f3e109  4885c0                         testq	%rax, %rax
// 0x141f3e10c  0f85bdf8ffff                   jne	0x141f3d9cf <.text+0x1f3c9cf>
// 0x141f3e112  eb13                           jmp	0x141f3e127 <.text+0x1f3d127>
// Frames: F298 -> F52 -> F53 -> F54 -> F55 -> F56 -> F293 -> F71 -> F71 -> F292
// 0x141f3e114  4889c1                         movq	%rax, %rcx
// 0x141f3e117  31d2                           xorl	%edx, %edx
// 0x141f3e119  e81249ed00                     callq	0x142e12a30 <.text+0x2e11a30>
// Frames: F299 -> F54 -> F55 -> F56 -> F293 -> F71 -> F71 -> F292
// 0x141f3e11e  4885c0                         testq	%rax, %rax
// 0x141f3e121  0f8500ffffff                   jne	0x141f3e027 <.text+0x1f3d027>
// 0x141f3e127  488d0db2a06401                 leaq	0x164a0b2(%rip), %rcx   # 0x1435881e0
// 0x141f3e12e  e82d2ce700                     callq	0x142db0d60 <.text+0x2dafd60>
// Frames: F300 -> F301 -> F115 -> F110
// 0x141f3e133  48b800000000ffffffff           movabsq	$-0x100000000, %rax # imm = 0xFFFFFFFF00000000
// 0x141f3e13d  4921c7                         andq	%rax, %r15
// 0x141f3e140  49ffc7                         incq	%r15
// 0x141f3e143  eb04                           jmp	0x141f3e149 <.text+0x1f3d149>
// Frames: F302 -> F115 -> F110
// 0x141f3e145  49c1e720                       shlq	$0x20, %r15
// Frames: F303 -> F110
// 0x141f3e149  4c897df0                       movq	%r15, -0x10(%rbp)
// 0x141f3e14d  48894df8                       movq	%rcx, -0x8(%rbp)
// 0x141f3e151  488d0530b04d01                 leaq	0x14db030(%rip), %rax   # 0x143419188
// 0x141f3e158  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141f3e15d  488d0decb04d01                 leaq	0x14db0ec(%rip), %rcx   # 0x143419250
// 0x141f3e164  4c8d0dc5b04d01                 leaq	0x14db0c5(%rip), %r9    # 0x143419230
// 0x141f3e16b  4c8d45f0                       leaq	-0x10(%rbp), %r8
// 0x141f3e16f  ba2b000000                     movl	$0x2b, %edx
// 0x141f3e174  e8c734e700                     callq	0x142db1640 <.text+0x2db0640>
// 0x141f3e179  90                             nop
// 0x141f3e17a  eb0d                           jmp	0x141f3e189 <.text+0x1f3d189>
// Frames: F296 -> F107
// 0x141f3e17c  488d0dedaf4d01                 leaq	0x14dafed(%rip), %rcx   # 0x143419170
// 0x141f3e183  e89834e700                     callq	0x142db1620 <.text+0x2db0620>
// 0x141f3e188  90                             nop
// 0x141f3e189  0f0b                           ud2
// Frames: F304 -> F7 -> F51 -> F2
// 0x141f3e18b  488d058eb14d01                 leaq	0x14db18e(%rip), %rax   # 0x143419320
// 0x141f3e192  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141f3e197  488d0d98af4d01                 leaq	0x14daf98(%rip), %rcx   # 0x143419136
// 0x141f3e19e  ba35000000                     movl	$0x35, %edx
// 0x141f3e1a3  41b935000000                   movl	$0x35, %r9d
// 0x141f3e1a9  e89233e700                     callq	0x142db1540 <.text+0x2db0540>
// Frames: F305 -> F7 -> F8 -> F2
// 0x141f3e1ae  488d058bb14d01                 leaq	0x14db18b(%rip), %rax   # 0x143419340
// 0x141f3e1b5  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141f3e1ba  488d0dbab04d01                 leaq	0x14db0ba(%rip), %rcx   # 0x14341927b
// 0x141f3e1c1  ba3d000000                     movl	$0x3d, %edx
// 0x141f3e1c6  4531c0                         xorl	%r8d, %r8d
// 0x141f3e1c9  e87233e700                     callq	0x142db1540 <.text+0x2db0540>
// Frames: F304 -> F7 -> F9 -> F2
// 0x141f3e1ce  488d0583b14d01                 leaq	0x14db183(%rip), %rax   # 0x143419358
// 0x141f3e1d5  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141f3e1da  488d0d9ab04d01                 leaq	0x14db09a(%rip), %rcx   # 0x14341927b
// 0x141f3e1e1  ba3d000000                     movl	$0x3d, %edx
// 0x141f3e1e6  41b93d000000                   movl	$0x3d, %r9d
// 0x141f3e1ec  e84f33e700                     callq	0x142db1540 <.text+0x2db0540>
// 0x141f3e1f1  cc                             int3
// 0x141f3e1f2  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x141f3e200  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f3e205  55                             pushq	%rbp
// 0x141f3e206  4157                           pushq	%r15
// 0x141f3e208  4156                           pushq	%r14
// 0x141f3e20a  4155                           pushq	%r13
// 0x141f3e20c  4154                           pushq	%r12
// 0x141f3e20e  56                             pushq	%rsi
// 0x141f3e20f  57                             pushq	%rdi
// 0x141f3e210  53                             pushq	%rbx
// 0x141f3e211  4883ec48                       subq	$0x48, %rsp
// 0x141f3e215  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141f3e21c  660f7f742430                   movdqa	%xmm6, 0x30(%rsp)
// 0x141f3e222  488b85e0000000                 movq	0xe0(%rbp), %rax
// Frames: F64 -> F65 -> F70 -> F71 -> F71 -> F62 -> F63 -> F55 -> F56 -> F293 -> F71 -> F71 -> F292
// 0x141f3e229  48ff00                         incq	(%rax)
// 0x141f3e22c  0f28742430                     movaps	0x30(%rsp), %xmm6
// Frames: F62 -> F63 -> F55 -> F56 -> F293 -> F71 -> F71 -> F292
// 0x141f3e231  4883c448                       addq	$0x48, %rsp
// 0x141f3e235  5b                             popq	%rbx
// 0x141f3e236  5f                             popq	%rdi
// 0x141f3e237  5e                             popq	%rsi
// 0x141f3e238  415c                           popq	%r12
// 0x141f3e23a  415d                           popq	%r13
// 0x141f3e23c  415e                           popq	%r14
// 0x141f3e23e  415f                           popq	%r15
// 0x141f3e240  5d                             popq	%rbp
// 0x141f3e241  c3                             retq
// 0x141f3e242  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x141f3e250  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f3e255  55                             pushq	%rbp
// 0x141f3e256  4157                           pushq	%r15
// 0x141f3e258  4156                           pushq	%r14
// 0x141f3e25a  4155                           pushq	%r13
// 0x141f3e25c  4154                           pushq	%r12
// 0x141f3e25e  56                             pushq	%rsi
// 0x141f3e25f  57                             pushq	%rdi
// 0x141f3e260  53                             pushq	%rbx
// 0x141f3e261  4883ec48                       subq	$0x48, %rsp
// 0x141f3e265  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141f3e26c  660f7f742430                   movdqa	%xmm6, 0x30(%rsp)
// 0x141f3e272  488b85d0000000                 movq	0xd0(%rbp), %rax
// Frames: F64 -> F65 -> F70 -> F71 -> F71 -> F62 -> F63 -> F55 -> F56 -> F57 -> F2
// 0x141f3e279  48ff00                         incq	(%rax)
// 0x141f3e27c  0f28742430                     movaps	0x30(%rsp), %xmm6
// Frames: F62 -> F63 -> F55 -> F56 -> F57 -> F2
// 0x141f3e281  4883c448                       addq	$0x48, %rsp
// 0x141f3e285  5b                             popq	%rbx
// 0x141f3e286  5f                             popq	%rdi
// 0x141f3e287  5e                             popq	%rsi
// 0x141f3e288  415c                           popq	%r12
// 0x141f3e28a  415d                           popq	%r13
// 0x141f3e28c  415e                           popq	%r14
// 0x141f3e28e  415f                           popq	%r15
// 0x141f3e290  5d                             popq	%rbp
// 0x141f3e291  c3                             retq
// 0x141f3e292  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x141f3e2a0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f3e2a5  55                             pushq	%rbp
// 0x141f3e2a6  4157                           pushq	%r15
// 0x141f3e2a8  4156                           pushq	%r14
// 0x141f3e2aa  4155                           pushq	%r13
// 0x141f3e2ac  4154                           pushq	%r12
// 0x141f3e2ae  56                             pushq	%rsi
// 0x141f3e2af  57                             pushq	%rdi
// 0x141f3e2b0  53                             pushq	%rbx
// 0x141f3e2b1  4883ec48                       subq	$0x48, %rsp
// 0x141f3e2b5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141f3e2bc  660f7f742430                   movdqa	%xmm6, 0x30(%rsp)
// 0x141f3e2c2  488b85c0000000                 movq	0xc0(%rbp), %rax
// 0x141f3e2c9  488b8dc8000000                 movq	0xc8(%rbp), %rcx
// Frames: F181
// 0x141f3e2d0  488901                         movq	%rax, (%rcx)
// 0x141f3e2d3  488b85b8000000                 movq	0xb8(%rbp), %rax
// 0x141f3e2da  48894108                       movq	%rax, 0x8(%rcx)
// 0x141f3e2de  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x141f3e2e3  4883c448                       addq	$0x48, %rsp
// 0x141f3e2e7  5b                             popq	%rbx
// 0x141f3e2e8  5f                             popq	%rdi
// 0x141f3e2e9  5e                             popq	%rsi
// 0x141f3e2ea  415c                           popq	%r12
// 0x141f3e2ec  415d                           popq	%r13
// 0x141f3e2ee  415e                           popq	%r14
// 0x141f3e2f0  415f                           popq	%r15
// 0x141f3e2f2  5d                             popq	%rbp
// 0x141f3e2f3  c3                             retq
// 0x141f3e2f4  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// 0x141f3e300  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f3e305  55                             pushq	%rbp
// 0x141f3e306  4157                           pushq	%r15
// 0x141f3e308  4156                           pushq	%r14
// 0x141f3e30a  4155                           pushq	%r13
// 0x141f3e30c  4154                           pushq	%r12
// 0x141f3e30e  56                             pushq	%rsi
// 0x141f3e30f  57                             pushq	%rdi
// 0x141f3e310  53                             pushq	%rbx
// 0x141f3e311  4883ec48                       subq	$0x48, %rsp
// 0x141f3e315  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141f3e31c  660f7f742430                   movdqa	%xmm6, 0x30(%rsp)
// 0x141f3e322  488d4df0                       leaq	-0x10(%rbp), %rcx
// Frames: F274
// 0x141f3e326  e8051415fe                     callq	0x14008f730 <.text+0x8e730>
// 0x141f3e32b  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x141f3e330  4883c448                       addq	$0x48, %rsp
// 0x141f3e334  5b                             popq	%rbx
// 0x141f3e335  5f                             popq	%rdi
// 0x141f3e336  5e                             popq	%rsi
// 0x141f3e337  415c                           popq	%r12
// 0x141f3e339  415d                           popq	%r13
// 0x141f3e33b  415e                           popq	%r14
// 0x141f3e33d  415f                           popq	%r15
// 0x141f3e33f  5d                             popq	%rbp
// 0x141f3e340  c3                             retq
// 0x141f3e341  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// 0x141f3e350  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x141f3e355  55                             pushq	%rbp
// 0x141f3e356  4157                           pushq	%r15
// 0x141f3e358  4156                           pushq	%r14
// 0x141f3e35a  4155                           pushq	%r13
// 0x141f3e35c  4154                           pushq	%r12
// 0x141f3e35e  56                             pushq	%rsi
// 0x141f3e35f  57                             pushq	%rdi
// 0x141f3e360  53                             pushq	%rbx
// 0x141f3e361  4883ec48                       subq	$0x48, %rsp
// 0x141f3e365  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x141f3e36c  660f7f742430                   movdqa	%xmm6, 0x30(%rsp)
// Frames: F71 -> F292
// 0x141f3e372  4883bda000000000               cmpq	$0x0, 0xa0(%rbp)
// 0x141f3e37a  7423                           je	0x141f3e39f <.text+0x1f3d39f>
// 0x141f3e37c  488d85a8000000                 leaq	0xa8(%rbp), %rax
// 0x141f3e383  48894558                       movq	%rax, 0x58(%rbp)
// 0x141f3e387  488d4558                       leaq	0x58(%rbp), %rax
// 0x141f3e38b  48894550                       movq	%rax, 0x50(%rbp)
// Frames: F56 -> F293 -> F71 -> F71 -> F292
// 0x141f3e38f  488d0d2aaf4d01                 leaq	0x14daf2a(%rip), %rcx   # 0x1434192c0
// 0x141f3e396  488d5550                       leaq	0x50(%rbp), %rdx
// 0x141f3e39a  e891298e00                     callq	0x142820d30 <.text+0x281fd30>
// Frames: F1
// 0x141f3e39f  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x141f3e3a4  4883c448                       addq	$0x48, %rsp
// 0x141f3e3a8  5b                             popq	%rbx
// 0x141f3e3a9  5f                             popq	%rdi
// 0x141f3e3aa  5e                             popq	%rsi
// 0x141f3e3ab  415c                           popq	%r12
// 0x141f3e3ad  415d                           popq	%r13
// 0x141f3e3af  415e                           popq	%r14
// 0x141f3e3b1  415f                           popq	%r15
// 0x141f3e3b3  5d                             popq	%rbp
// 0x141f3e3b4  c3                             retq
//
// RANGE rva_282f570_len_3c3: RVA 0x282f570, 963 PDB bytes; 221 instructions
// Evidence: _assembly/roof-v1/C/pack-003/rva_282f570_len_3c3.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3785:30460: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// Frames: F306
// 0x14282f570  55                             pushq	%rbp
// 0x14282f571  56                             pushq	%rsi
// 0x14282f572  4881ecc8000000                 subq	$0xc8, %rsp
// 0x14282f579  488dac2480000000               leaq	0x80(%rsp), %rbp
// Frames: F307
// 0x14282f581  48c74540feffffff               movq	$-0x2, 0x40(%rbp)
// 0x14282f589  e8021e71ff                     callq	0x141f41390 <.text+0x1f40390>
// 0x14282f58e  84c0                           testb	%al, %al
// 0x14282f590  0f84e8010000                   je	0x14282f77e <.text+0x282e77e>
// Frames: F308 -> F309 -> F310 -> F311 -> F312
// 0x14282f596  8b0534594601                   movl	0x1465934(%rip), %eax   # 0x143c94ed0
// Frames: F311 -> F312
// 0x14282f59c  85c0                           testl	%eax, %eax
// 0x14282f59e  0f85e4010000                   jne	0x14282f788 <.text+0x282e788>
// Frames: F312
// 0x14282f5a4  488b151d9c4601                 movq	0x1469c1d(%rip), %rdx   # 0x143c991c8
// 0x14282f5ab  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x14282f5af  e8dcb48cfd                     callq	0x1400faa90 <.text+0xf9a90>
// Frames: F313 -> F312
// 0x14282f5b4  837db001                       cmpl	$0x1, -0x50(%rbp)
// 0x14282f5b8  0f8400020000                   je	0x14282f7be <.text+0x282e7be>
// Frames: F314 -> F312
// 0x14282f5be  488b45b8                       movq	-0x48(%rbp), %rax
// 0x14282f5c2  48894518                       movq	%rax, 0x18(%rbp)
// 0x14282f5c6  0fb645c0                       movzbl	-0x40(%rbp), %eax
// 0x14282f5ca  88453f                         movb	%al, 0x3f(%rbp)
// 0x14282f5cd  e83e3d7dfd                     callq	0x140003310 <.text+0x2310>
// Frames: F182 -> F175 -> F176 -> F177 -> F315 -> F316 -> F317 -> F318 -> F319 -> F320 -> F321 -> F322 -> F323 -> F324 -> F312
// 0x14282f5d2  b90f000000                     movl	$0xf, %ecx
// 0x14282f5d7  ba01000000                     movl	$0x1, %edx
// 0x14282f5dc  e8ff62e7fd                     callq	0x1406a58e0 <.text+0x6a48e0>
// Frames: F325 -> F316 -> F317 -> F318 -> F319 -> F320 -> F321 -> F322 -> F323 -> F324 -> F312
// 0x14282f5e1  4885c0                         testq	%rax, %rax
// 0x14282f5e4  0f845f020000                   je	0x14282f849 <.text+0x282e849>
// Frames: F243 -> F326 -> F327 -> F320 -> F321 -> F322 -> F323 -> F324 -> F312
// 0x14282f5ea  48b97061746821202829           movabsq	$0x2928202168746170, %rcx # imm = 0x2928202168746170
// 0x14282f5f4  48894807                       movq	%rcx, 0x7(%rax)
// 0x14282f5f8  48b96d6f64756c655f70           movabsq	$0x705f656c75646f6d, %rcx # imm = 0x705F656C75646F6D
// 0x14282f602  488908                         movq	%rcx, (%rax)
// Frames: F328 -> F323 -> F324 -> F312
// 0x14282f605  48c745b00f000000               movq	$0xf, -0x50(%rbp)
// 0x14282f60d  488945b8                       movq	%rax, -0x48(%rbp)
// 0x14282f611  48c745c00f000000               movq	$0xf, -0x40(%rbp)
// Frames: F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x14282f619  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x14282f622  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x14282f626  ba0f000000                     movl	$0xf, %edx
// 0x14282f62b  41b817000000                   movl	$0x17, %r8d
// 0x14282f631  41b901000000                   movl	$0x1, %r9d
// 0x14282f637  e8d4065400                     callq	0x142d6fd10 <.text+0x2d6ed10>
// 0x14282f63c  90                             nop
// Frames: F134 -> F135 -> F136 -> F337 -> F338 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x14282f63d  488b45b8                       movq	-0x48(%rbp), %rax
// Frames: F339 -> F340 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x14282f641  488b55c0                       movq	-0x40(%rbp), %rdx
// Frames: F341 -> F338 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x14282f645  0f10055f9bd500                 movups	0xd59b5f(%rip), %xmm0   # 0x1435891ab
// 0x14282f64c  0f110410                       movups	%xmm0, (%rax,%rdx)
// 0x14282f650  48b9203a3a205761726e           movabsq	$0x6e726157203a3a20, %rcx # imm = 0x6E726157203A3A20
// 0x14282f65a  48894c100f                     movq	%rcx, 0xf(%rax,%rdx)
// Frames: F342 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x14282f65f  4883c217                       addq	$0x17, %rdx
// Frames: F343 -> F312
// 0x14282f663  48895530                       movq	%rdx, 0x30(%rbp)
// 0x14282f667  0f1045b0                       movups	-0x50(%rbp), %xmm0
// 0x14282f66b  0f294520                       movaps	%xmm0, 0x20(%rbp)
// Frames: F344 -> F345 -> F346 -> F330 -> F331 -> F332 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x14282f66f  488b4520                       movq	0x20(%rbp), %rax
// Frames: F347 -> F345 -> F346 -> F330 -> F331 -> F332 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x14282f673  4829d0                         subq	%rdx, %rax
// Frames: F345 -> F346 -> F330 -> F331 -> F332 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x14282f676  4883f819                       cmpq	$0x19, %rax
// Frames: F346 -> F330 -> F331 -> F332 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x14282f67a  0f8678010000                   jbe	0x14282f7f8 <.text+0x282e7f8>
// Frames: F134 -> F135 -> F136 -> F337 -> F338 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x14282f680  488b4528                       movq	0x28(%rbp), %rax
// Frames: F341 -> F338 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x14282f684  0f1005419bd500                 movups	0xd59b41(%rip), %xmm0   # 0x1435891cc
// 0x14282f68b  0f1144100a                     movups	%xmm0, 0xa(%rax,%rdx)
// 0x14282f690  0f10052b9bd500                 movups	0xd59b2b(%rip), %xmm0   # 0x1435891c2
// 0x14282f697  0f110410                       movups	%xmm0, (%rax,%rdx)
// Frames: F342 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x14282f69b  4883c21a                       addq	$0x1a, %rdx
// 0x14282f69f  48895530                       movq	%rdx, 0x30(%rbp)
// Frames: F343 -> F312
// 0x14282f6a3  488955c0                       movq	%rdx, -0x40(%rbp)
// 0x14282f6a7  0f284520                       movaps	0x20(%rbp), %xmm0
// 0x14282f6ab  0f2945b0                       movaps	%xmm0, -0x50(%rbp)
// 0x14282f6af  488b4518                       movq	0x18(%rbp), %rax
// Frames: F67 -> F348 -> F312
// 0x14282f6b3  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F349 -> F312
// 0x14282f6b7  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x14282f6bb  e870846dff                     callq	0x141f07b30 <.text+0x1f06b30>
// 0x14282f6c0  90                             nop
// Frames: F312
// 0x14282f6c1  84c0                           testb	%al, %al
// 0x14282f6c3  0f8589000000                   jne	0x14282f752 <.text+0x282e752>
// Frames: F191 -> F192 -> F350 -> F312
// 0x14282f6c9  488b05c0f74501                 movq	0x145f7c0(%rip), %rax   # 0x143c8ee90
// 0x14282f6d0  488b00                         movq	(%rax), %rax
// Frames: F351 -> F352 -> F312
// 0x14282f6d3  4883f802                       cmpq	$0x2, %rax
// Frames: F312
// 0x14282f6d7  7279                           jb	0x14282f752 <.text+0x282e752>
// Frames: F353 -> F354 -> F355 -> F356 -> F312
// 0x14282f6d9  48c745b000000000               movq	$0x0, -0x50(%rbp)
// 0x14282f6e1  488d05f49ad500                 leaq	0xd59af4(%rip), %rax    # 0x1435891dc
// 0x14282f6e8  488945b8                       movq	%rax, -0x48(%rbp)
// 0x14282f6ec  48c745c028000000               movq	$0x28, -0x40(%rbp)
// 0x14282f6f4  48c745c800000000               movq	$0x0, -0x38(%rbp)
// 0x14282f6fc  488d0d459ad500                 leaq	0xd59a45(%rip), %rcx    # 0x143589148
// 0x14282f703  48894dd0                       movq	%rcx, -0x30(%rbp)
// 0x14282f707  48c745d835000000               movq	$0x35, -0x28(%rbp)
// 0x14282f70f  48c745e002000000               movq	$0x2, -0x20(%rbp)
// 0x14282f717  488945e8                       movq	%rax, -0x18(%rbp)
// 0x14282f71b  48c745f028000000               movq	$0x28, -0x10(%rbp)
// 0x14282f723  48b8010000002e000000           movabsq	$0x2e00000001, %rax # imm = 0x2E00000001
// 0x14282f72d  488945f8                       movq	%rax, -0x8(%rbp)
// 0x14282f731  488d058a9ad500                 leaq	0xd59a8a(%rip), %rax    # 0x1435891c2
// 0x14282f738  48894500                       movq	%rax, (%rbp)
// 0x14282f73c  48c7450835000000               movq	$0x35, 0x8(%rbp)
// Frames: F355 -> F356 -> F312
// 0x14282f744  488d4d3e                       leaq	0x3e(%rbp), %rcx
// 0x14282f748  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x14282f74c  e82fa76ffe                     callq	0x140f29e80 <.text+0xf28e80>
// 0x14282f751  90                             nop
// Frames: F357 -> F358 -> F71 -> F312
// 0x14282f752  807d3f00                       cmpb	$0x0, 0x3f(%rbp)
// 0x14282f756  488b7518                       movq	0x18(%rbp), %rsi
// 0x14282f75a  7516                           jne	0x14282f772 <.text+0x282e772>
// Frames: F191 -> F192 -> F359 -> F360 -> F361 -> F357 -> F358 -> F71 -> F312
// 0x14282f75c  488b05d5f74501                 movq	0x145f7d5(%rip), %rax   # 0x143c8ef38
// 0x14282f763  488b00                         movq	(%rax), %rax
// Frames: F359 -> F360 -> F361 -> F357 -> F358 -> F71 -> F312
// 0x14282f766  48d1e0                         shlq	%rax
// 0x14282f769  4885c0                         testq	%rax, %rax
// 0x14282f76c  0f85c1000000                   jne	0x14282f833 <.text+0x282e833>
// Frames: F362 -> F363 -> F364 -> F365 -> F71 -> F312
// 0x14282f772  31c0                           xorl	%eax, %eax
// 0x14282f774  8606                           xchgb	%al, (%rsi)
// Frames: F364 -> F365 -> F71 -> F312
// 0x14282f776  3c02                           cmpb	$0x2, %al
// 0x14282f778  0f84a2000000                   je	0x14282f820 <.text+0x282e820>
// Frames: F366
// 0x14282f77e  4881c4c8000000                 addq	$0xc8, %rsp
// 0x14282f785  5e                             popq	%rsi
// 0x14282f786  5d                             popq	%rbp
// 0x14282f787  c3                             retq
// Frames: F367 -> F312
// 0x14282f788  c6452001                       movb	$0x1, 0x20(%rbp)
// 0x14282f78c  488d4520                       leaq	0x20(%rbp), %rax
// Frames: F368 -> F312
// 0x14282f790  488945b0                       movq	%rax, -0x50(%rbp)
// 0x14282f794  488d05e599d500                 leaq	0xd599e5(%rip), %rax    # 0x143589180
// 0x14282f79b  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14282f7a0  488d0d29574601                 leaq	0x1465729(%rip), %rcx   # 0x143c94ed0
// 0x14282f7a7  4c8d0d5a9ad500                 leaq	0xd59a5a(%rip), %r9     # 0x143589208
// 0x14282f7ae  4c8d45b0                       leaq	-0x50(%rbp), %r8
// 0x14282f7b2  31d2                           xorl	%edx, %edx
// 0x14282f7b4  e8770d5800                     callq	0x142db0530 <.text+0x2daf530>
// 0x14282f7b9  e9e6fdffff                     jmp	0x14282f5a4 <.text+0x282e5a4>
// Frames: F369 -> F312
// 0x14282f7be  488b45b8                       movq	-0x48(%rbp), %rax
// 0x14282f7c2  0fb64dc0                       movzbl	-0x40(%rbp), %ecx
// 0x14282f7c6  48894520                       movq	%rax, 0x20(%rbp)
// 0x14282f7ca  884d28                         movb	%cl, 0x28(%rbp)
// 0x14282f7cd  488d05ac99d500                 leaq	0xd599ac(%rip), %rax    # 0x143589180
// 0x14282f7d4  4889442420                     movq	%rax, 0x20(%rsp)
// 0x14282f7d9  488d0db899d500                 leaq	0xd599b8(%rip), %rcx    # 0x143589198
// 0x14282f7e0  4c8d0dd19ad500                 leaq	0xd59ad1(%rip), %r9     # 0x1435892b8
// 0x14282f7e7  4c8d4520                       leaq	0x20(%rbp), %r8
// 0x14282f7eb  ba13000000                     movl	$0x13, %edx
// 0x14282f7f0  e84b1e5800                     callq	0x142db1640 <.text+0x2db0640>
// 0x14282f7f5  90                             nop
// 0x14282f7f6  eb61                           jmp	0x14282f859 <.text+0x282e859>
// Frames: F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x14282f7f8  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x14282f801  488d4d20                       leaq	0x20(%rbp), %rcx
// 0x14282f805  41b81a000000                   movl	$0x1a, %r8d
// 0x14282f80b  41b901000000                   movl	$0x1, %r9d
// 0x14282f811  e8fa045400                     callq	0x142d6fd10 <.text+0x2d6ed10>
// 0x14282f816  90                             nop
// Frames: F339 -> F340 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x14282f817  488b5530                       movq	0x30(%rbp), %rdx
// 0x14282f81b  e960feffff                     jmp	0x14282f680 <.text+0x282e680>
// Frames: F370 -> F365 -> F71 -> F312
// 0x14282f820  4889f1                         movq	%rsi, %rcx
// 0x14282f823  e848095800                     callq	0x142db0170 <.text+0x2daf170>
// 0x14282f828  90                             nop
// Frames: F366
// 0x14282f829  4881c4c8000000                 addq	$0xc8, %rsp
// 0x14282f830  5e                             popq	%rsi
// 0x14282f831  5d                             popq	%rbp
// 0x14282f832  c3                             retq
// Frames: F371 -> F360 -> F361 -> F357 -> F358 -> F71 -> F312
// 0x14282f833  e878165800                     callq	0x142db0eb0 <.text+0x2dafeb0>
// Frames: F357 -> F358 -> F71 -> F312
// 0x14282f838  84c0                           testb	%al, %al
// 0x14282f83a  0f8532ffffff                   jne	0x14282f772 <.text+0x282e772>
// Frames: F372 -> F373 -> F374 -> F358 -> F71 -> F312
// 0x14282f840  c6460101                       movb	$0x1, 0x1(%rsi)
// 0x14282f844  e929ffffff                     jmp	0x14282f772 <.text+0x282e772>
// Frames: F375 -> F317 -> F318 -> F319 -> F320 -> F321 -> F322 -> F323 -> F324 -> F312
// 0x14282f849  b901000000                     movl	$0x1, %ecx
// 0x14282f84e  ba0f000000                     movl	$0xf, %edx
// 0x14282f853  e8a21a5800                     callq	0x142db12fa <.text+0x2db02fa>
// 0x14282f858  90                             nop
// 0x14282f859  0f0b                           ud2
// 0x14282f85b  0f1f440000                     nopl	(%rax,%rax)
// 0x14282f860  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14282f865  55                             pushq	%rbp
// 0x14282f866  56                             pushq	%rsi
// 0x14282f867  4883ec28                       subq	$0x28, %rsp
// 0x14282f86b  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F376 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x14282f872  488b5520                       movq	0x20(%rbp), %rdx
// Frames: F379 -> F376 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x14282f876  4885d2                         testq	%rdx, %rdx
// 0x14282f879  7410                           je	0x14282f88b <.text+0x282e88b>
// Frames: F376 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x14282f87b  488b4d28                       movq	0x28(%rbp), %rcx
// Frames: F380 -> F381 -> F382 -> F383 -> F384 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x14282f87f  41b801000000                   movl	$0x1, %r8d
// 0x14282f885  e86660e7fd                     callq	0x1406a58f0 <.text+0x6a48f0>
// 0x14282f88a  90                             nop
// 0x14282f88b  4883c428                       addq	$0x28, %rsp
// 0x14282f88f  5e                             popq	%rsi
// 0x14282f890  5d                             popq	%rbp
// 0x14282f891  c3                             retq
// 0x14282f892  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x14282f8a0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14282f8a5  55                             pushq	%rbp
// 0x14282f8a6  56                             pushq	%rsi
// 0x14282f8a7  4883ec28                       subq	$0x28, %rsp
// 0x14282f8ab  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F376 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x14282f8b2  488b55b0                       movq	-0x50(%rbp), %rdx
// Frames: F379 -> F376 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x14282f8b6  4885d2                         testq	%rdx, %rdx
// 0x14282f8b9  7410                           je	0x14282f8cb <.text+0x282e8cb>
// Frames: F376 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x14282f8bb  488b4db8                       movq	-0x48(%rbp), %rcx
// Frames: F380 -> F381 -> F382 -> F383 -> F384 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x14282f8bf  41b801000000                   movl	$0x1, %r8d
// 0x14282f8c5  e82660e7fd                     callq	0x1406a58f0 <.text+0x6a48f0>
// 0x14282f8ca  90                             nop
// 0x14282f8cb  4883c428                       addq	$0x28, %rsp
// 0x14282f8cf  5e                             popq	%rsi
// 0x14282f8d0  5d                             popq	%rbp
// 0x14282f8d1  c3                             retq
// 0x14282f8d2  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x14282f8e0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14282f8e5  55                             pushq	%rbp
// 0x14282f8e6  56                             pushq	%rsi
// 0x14282f8e7  4883ec28                       subq	$0x28, %rsp
// 0x14282f8eb  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14282f8f2  488b4d18                       movq	0x18(%rbp), %rcx
// 0x14282f8f6  0fb6553f                       movzbl	0x3f(%rbp), %edx
// Frames: F312
// 0x14282f8fa  e8812285fd                     callq	0x140081b80 <.text+0x80b80>
// 0x14282f8ff  90                             nop
// Frames: F306
// 0x14282f900  4883c428                       addq	$0x28, %rsp
// 0x14282f904  5e                             popq	%rsi
// 0x14282f905  5d                             popq	%rbp
// 0x14282f906  c3                             retq
// 0x14282f907  660f1f840000000000             nopw	(%rax,%rax)
// 0x14282f910  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14282f915  55                             pushq	%rbp
// 0x14282f916  56                             pushq	%rsi
// 0x14282f917  4883ec28                       subq	$0x28, %rsp
// 0x14282f91b  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14282f922  488d4d20                       leaq	0x20(%rbp), %rcx
// Frames: F369 -> F312
// 0x14282f926  e8b58285fd                     callq	0x140087be0 <.text+0x86be0>
// 0x14282f92b  90                             nop
// Frames: F385 -> F312
// 0x14282f92c  4883c428                       addq	$0x28, %rsp
// 0x14282f930  5e                             popq	%rsi
// 0x14282f931  5d                             popq	%rbp
// 0x14282f932  c3                             retq
//
// RANGE rva_2830c70_len_3c3: RVA 0x2830c70, 963 PDB bytes; 221 instructions
// Evidence: _assembly/roof-v1/C/pack-003/rva_2830c70_len_3c3.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3786:8700: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// Frames: F306
// 0x142830c70  55                             pushq	%rbp
// 0x142830c71  56                             pushq	%rsi
// 0x142830c72  4881ecc8000000                 subq	$0xc8, %rsp
// 0x142830c79  488dac2480000000               leaq	0x80(%rsp), %rbp
// Frames: F307
// 0x142830c81  48c74540feffffff               movq	$-0x2, 0x40(%rbp)
// 0x142830c89  e8020771ff                     callq	0x141f41390 <.text+0x1f40390>
// 0x142830c8e  84c0                           testb	%al, %al
// 0x142830c90  0f84e8010000                   je	0x142830e7e <.text+0x282fe7e>
// Frames: F308 -> F309 -> F310 -> F311 -> F312
// 0x142830c96  8b0534424601                   movl	0x1464234(%rip), %eax   # 0x143c94ed0
// Frames: F311 -> F312
// 0x142830c9c  85c0                           testl	%eax, %eax
// 0x142830c9e  0f85e4010000                   jne	0x142830e88 <.text+0x282fe88>
// Frames: F312
// 0x142830ca4  488b151d854601                 movq	0x146851d(%rip), %rdx   # 0x143c991c8
// 0x142830cab  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x142830caf  e8dc9d8cfd                     callq	0x1400faa90 <.text+0xf9a90>
// Frames: F313 -> F312
// 0x142830cb4  837db001                       cmpl	$0x1, -0x50(%rbp)
// 0x142830cb8  0f8400020000                   je	0x142830ebe <.text+0x282febe>
// Frames: F314 -> F312
// 0x142830cbe  488b45b8                       movq	-0x48(%rbp), %rax
// 0x142830cc2  48894518                       movq	%rax, 0x18(%rbp)
// 0x142830cc6  0fb645c0                       movzbl	-0x40(%rbp), %eax
// 0x142830cca  88453f                         movb	%al, 0x3f(%rbp)
// 0x142830ccd  e83e267dfd                     callq	0x140003310 <.text+0x2310>
// Frames: F182 -> F175 -> F176 -> F177 -> F315 -> F316 -> F317 -> F318 -> F319 -> F320 -> F321 -> F322 -> F323 -> F324 -> F312
// 0x142830cd2  b90f000000                     movl	$0xf, %ecx
// 0x142830cd7  ba01000000                     movl	$0x1, %edx
// 0x142830cdc  e8ff4be7fd                     callq	0x1406a58e0 <.text+0x6a48e0>
// Frames: F325 -> F316 -> F317 -> F318 -> F319 -> F320 -> F321 -> F322 -> F323 -> F324 -> F312
// 0x142830ce1  4885c0                         testq	%rax, %rax
// 0x142830ce4  0f845f020000                   je	0x142830f49 <.text+0x282ff49>
// Frames: F243 -> F326 -> F327 -> F320 -> F321 -> F322 -> F323 -> F324 -> F312
// 0x142830cea  48b97061746821202829           movabsq	$0x2928202168746170, %rcx # imm = 0x2928202168746170
// 0x142830cf4  48894807                       movq	%rcx, 0x7(%rax)
// 0x142830cf8  48b96d6f64756c655f70           movabsq	$0x705f656c75646f6d, %rcx # imm = 0x705F656C75646F6D
// 0x142830d02  488908                         movq	%rcx, (%rax)
// Frames: F328 -> F323 -> F324 -> F312
// 0x142830d05  48c745b00f000000               movq	$0xf, -0x50(%rbp)
// 0x142830d0d  488945b8                       movq	%rax, -0x48(%rbp)
// 0x142830d11  48c745c00f000000               movq	$0xf, -0x40(%rbp)
// Frames: F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x142830d19  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x142830d22  488d4db0                       leaq	-0x50(%rbp), %rcx
// 0x142830d26  ba0f000000                     movl	$0xf, %edx
// 0x142830d2b  41b817000000                   movl	$0x17, %r8d
// 0x142830d31  41b901000000                   movl	$0x1, %r9d
// 0x142830d37  e8d4ef5300                     callq	0x142d6fd10 <.text+0x2d6ed10>
// 0x142830d3c  90                             nop
// Frames: F134 -> F135 -> F136 -> F337 -> F338 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x142830d3d  488b45b8                       movq	-0x48(%rbp), %rax
// Frames: F339 -> F340 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x142830d41  488b55c0                       movq	-0x40(%rbp), %rdx
// Frames: F341 -> F338 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x142830d45  0f10051787d500                 movups	0xd58717(%rip), %xmm0   # 0x143589463
// 0x142830d4c  0f110410                       movups	%xmm0, (%rax,%rdx)
// 0x142830d50  48b9203a3a205761726e           movabsq	$0x6e726157203a3a20, %rcx # imm = 0x6E726157203A3A20
// 0x142830d5a  48894c100f                     movq	%rcx, 0xf(%rax,%rdx)
// Frames: F342 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x142830d5f  4883c217                       addq	$0x17, %rdx
// Frames: F343 -> F312
// 0x142830d63  48895530                       movq	%rdx, 0x30(%rbp)
// 0x142830d67  0f1045b0                       movups	-0x50(%rbp), %xmm0
// 0x142830d6b  0f294520                       movaps	%xmm0, 0x20(%rbp)
// Frames: F344 -> F345 -> F346 -> F330 -> F331 -> F332 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x142830d6f  488b4520                       movq	0x20(%rbp), %rax
// Frames: F347 -> F345 -> F346 -> F330 -> F331 -> F332 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x142830d73  4829d0                         subq	%rdx, %rax
// Frames: F345 -> F346 -> F330 -> F331 -> F332 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x142830d76  4883f819                       cmpq	$0x19, %rax
// Frames: F346 -> F330 -> F331 -> F332 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x142830d7a  0f8678010000                   jbe	0x142830ef8 <.text+0x282fef8>
// Frames: F134 -> F135 -> F136 -> F337 -> F338 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x142830d80  488b4528                       movq	0x28(%rbp), %rax
// Frames: F341 -> F338 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x142830d84  0f1005f986d500                 movups	0xd586f9(%rip), %xmm0   # 0x143589484
// 0x142830d8b  0f1144100a                     movups	%xmm0, 0xa(%rax,%rdx)
// 0x142830d90  0f1005e386d500                 movups	0xd586e3(%rip), %xmm0   # 0x14358947a
// 0x142830d97  0f110410                       movups	%xmm0, (%rax,%rdx)
// Frames: F342 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x142830d9b  4883c21a                       addq	$0x1a, %rdx
// 0x142830d9f  48895530                       movq	%rdx, 0x30(%rbp)
// Frames: F343 -> F312
// 0x142830da3  488955c0                       movq	%rdx, -0x40(%rbp)
// 0x142830da7  0f284520                       movaps	0x20(%rbp), %xmm0
// 0x142830dab  0f2945b0                       movaps	%xmm0, -0x50(%rbp)
// 0x142830daf  488b4518                       movq	0x18(%rbp), %rax
// Frames: F67 -> F348 -> F312
// 0x142830db3  488d4808                       leaq	0x8(%rax), %rcx
// Frames: F349 -> F312
// 0x142830db7  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x142830dbb  e8706d6dff                     callq	0x141f07b30 <.text+0x1f06b30>
// 0x142830dc0  90                             nop
// Frames: F312
// 0x142830dc1  84c0                           testb	%al, %al
// 0x142830dc3  0f8589000000                   jne	0x142830e52 <.text+0x282fe52>
// Frames: F191 -> F192 -> F350 -> F312
// 0x142830dc9  488b05c0e04501                 movq	0x145e0c0(%rip), %rax   # 0x143c8ee90
// 0x142830dd0  488b00                         movq	(%rax), %rax
// Frames: F351 -> F352 -> F312
// 0x142830dd3  4883f802                       cmpq	$0x2, %rax
// Frames: F312
// 0x142830dd7  7279                           jb	0x142830e52 <.text+0x282fe52>
// Frames: F353 -> F354 -> F355 -> F356 -> F312
// 0x142830dd9  48c745b000000000               movq	$0x0, -0x50(%rbp)
// 0x142830de1  488d05ac86d500                 leaq	0xd586ac(%rip), %rax    # 0x143589494
// 0x142830de8  488945b8                       movq	%rax, -0x48(%rbp)
// 0x142830dec  48c745c028000000               movq	$0x28, -0x40(%rbp)
// 0x142830df4  48c745c800000000               movq	$0x0, -0x38(%rbp)
// 0x142830dfc  488d0dfd85d500                 leaq	0xd585fd(%rip), %rcx    # 0x143589400
// 0x142830e03  48894dd0                       movq	%rcx, -0x30(%rbp)
// 0x142830e07  48c745d835000000               movq	$0x35, -0x28(%rbp)
// 0x142830e0f  48c745e002000000               movq	$0x2, -0x20(%rbp)
// 0x142830e17  488945e8                       movq	%rax, -0x18(%rbp)
// 0x142830e1b  48c745f028000000               movq	$0x28, -0x10(%rbp)
// 0x142830e23  48b8010000002e000000           movabsq	$0x2e00000001, %rax # imm = 0x2E00000001
// 0x142830e2d  488945f8                       movq	%rax, -0x8(%rbp)
// 0x142830e31  488d054286d500                 leaq	0xd58642(%rip), %rax    # 0x14358947a
// 0x142830e38  48894500                       movq	%rax, (%rbp)
// 0x142830e3c  48c7450835000000               movq	$0x35, 0x8(%rbp)
// Frames: F355 -> F356 -> F312
// 0x142830e44  488d4d3e                       leaq	0x3e(%rbp), %rcx
// 0x142830e48  488d55b0                       leaq	-0x50(%rbp), %rdx
// 0x142830e4c  e82f906ffe                     callq	0x140f29e80 <.text+0xf28e80>
// 0x142830e51  90                             nop
// Frames: F357 -> F358 -> F71 -> F312
// 0x142830e52  807d3f00                       cmpb	$0x0, 0x3f(%rbp)
// 0x142830e56  488b7518                       movq	0x18(%rbp), %rsi
// 0x142830e5a  7516                           jne	0x142830e72 <.text+0x282fe72>
// Frames: F191 -> F192 -> F359 -> F360 -> F361 -> F357 -> F358 -> F71 -> F312
// 0x142830e5c  488b05d5e04501                 movq	0x145e0d5(%rip), %rax   # 0x143c8ef38
// 0x142830e63  488b00                         movq	(%rax), %rax
// Frames: F359 -> F360 -> F361 -> F357 -> F358 -> F71 -> F312
// 0x142830e66  48d1e0                         shlq	%rax
// 0x142830e69  4885c0                         testq	%rax, %rax
// 0x142830e6c  0f85c1000000                   jne	0x142830f33 <.text+0x282ff33>
// Frames: F362 -> F363 -> F364 -> F365 -> F71 -> F312
// 0x142830e72  31c0                           xorl	%eax, %eax
// 0x142830e74  8606                           xchgb	%al, (%rsi)
// Frames: F364 -> F365 -> F71 -> F312
// 0x142830e76  3c02                           cmpb	$0x2, %al
// 0x142830e78  0f84a2000000                   je	0x142830f20 <.text+0x282ff20>
// Frames: F366
// 0x142830e7e  4881c4c8000000                 addq	$0xc8, %rsp
// 0x142830e85  5e                             popq	%rsi
// 0x142830e86  5d                             popq	%rbp
// 0x142830e87  c3                             retq
// Frames: F367 -> F312
// 0x142830e88  c6452001                       movb	$0x1, 0x20(%rbp)
// 0x142830e8c  488d4520                       leaq	0x20(%rbp), %rax
// Frames: F368 -> F312
// 0x142830e90  488945b0                       movq	%rax, -0x50(%rbp)
// 0x142830e94  488d059d85d500                 leaq	0xd5859d(%rip), %rax    # 0x143589438
// 0x142830e9b  4889442420                     movq	%rax, 0x20(%rsp)
// 0x142830ea0  488d0d29404601                 leaq	0x1464029(%rip), %rcx   # 0x143c94ed0
// 0x142830ea7  4c8d0d1286d500                 leaq	0xd58612(%rip), %r9     # 0x1435894c0
// 0x142830eae  4c8d45b0                       leaq	-0x50(%rbp), %r8
// 0x142830eb2  31d2                           xorl	%edx, %edx
// 0x142830eb4  e877f65700                     callq	0x142db0530 <.text+0x2daf530>
// 0x142830eb9  e9e6fdffff                     jmp	0x142830ca4 <.text+0x282fca4>
// Frames: F369 -> F312
// 0x142830ebe  488b45b8                       movq	-0x48(%rbp), %rax
// 0x142830ec2  0fb64dc0                       movzbl	-0x40(%rbp), %ecx
// 0x142830ec6  48894520                       movq	%rax, 0x20(%rbp)
// 0x142830eca  884d28                         movb	%cl, 0x28(%rbp)
// 0x142830ecd  488d056485d500                 leaq	0xd58564(%rip), %rax    # 0x143589438
// 0x142830ed4  4889442420                     movq	%rax, 0x20(%rsp)
// 0x142830ed9  488d0d7085d500                 leaq	0xd58570(%rip), %rcx    # 0x143589450
// 0x142830ee0  4c8d0d8986d500                 leaq	0xd58689(%rip), %r9     # 0x143589570
// 0x142830ee7  4c8d4520                       leaq	0x20(%rbp), %r8
// 0x142830eeb  ba13000000                     movl	$0x13, %edx
// 0x142830ef0  e84b075800                     callq	0x142db1640 <.text+0x2db0640>
// 0x142830ef5  90                             nop
// 0x142830ef6  eb61                           jmp	0x142830f59 <.text+0x282ff59>
// Frames: F329 -> F330 -> F331 -> F332 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x142830ef8  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x142830f01  488d4d20                       leaq	0x20(%rbp), %rcx
// 0x142830f05  41b81a000000                   movl	$0x1a, %r8d
// 0x142830f0b  41b901000000                   movl	$0x1, %r9d
// 0x142830f11  e8faed5300                     callq	0x142d6fd10 <.text+0x2d6ed10>
// 0x142830f16  90                             nop
// Frames: F339 -> F340 -> F333 -> F334 -> F335 -> F336 -> F312
// 0x142830f17  488b5530                       movq	0x30(%rbp), %rdx
// 0x142830f1b  e960feffff                     jmp	0x142830d80 <.text+0x282fd80>
// Frames: F370 -> F365 -> F71 -> F312
// 0x142830f20  4889f1                         movq	%rsi, %rcx
// 0x142830f23  e848f25700                     callq	0x142db0170 <.text+0x2daf170>
// 0x142830f28  90                             nop
// Frames: F366
// 0x142830f29  4881c4c8000000                 addq	$0xc8, %rsp
// 0x142830f30  5e                             popq	%rsi
// 0x142830f31  5d                             popq	%rbp
// 0x142830f32  c3                             retq
// Frames: F371 -> F360 -> F361 -> F357 -> F358 -> F71 -> F312
// 0x142830f33  e878ff5700                     callq	0x142db0eb0 <.text+0x2dafeb0>
// Frames: F357 -> F358 -> F71 -> F312
// 0x142830f38  84c0                           testb	%al, %al
// 0x142830f3a  0f8532ffffff                   jne	0x142830e72 <.text+0x282fe72>
// Frames: F372 -> F373 -> F374 -> F358 -> F71 -> F312
// 0x142830f40  c6460101                       movb	$0x1, 0x1(%rsi)
// 0x142830f44  e929ffffff                     jmp	0x142830e72 <.text+0x282fe72>
// Frames: F375 -> F317 -> F318 -> F319 -> F320 -> F321 -> F322 -> F323 -> F324 -> F312
// 0x142830f49  b901000000                     movl	$0x1, %ecx
// 0x142830f4e  ba0f000000                     movl	$0xf, %edx
// 0x142830f53  e8a2035800                     callq	0x142db12fa <.text+0x2db02fa>
// 0x142830f58  90                             nop
// 0x142830f59  0f0b                           ud2
// 0x142830f5b  0f1f440000                     nopl	(%rax,%rax)
// 0x142830f60  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x142830f65  55                             pushq	%rbp
// 0x142830f66  56                             pushq	%rsi
// 0x142830f67  4883ec28                       subq	$0x28, %rsp
// 0x142830f6b  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F376 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x142830f72  488b5520                       movq	0x20(%rbp), %rdx
// Frames: F379 -> F376 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x142830f76  4885d2                         testq	%rdx, %rdx
// 0x142830f79  7410                           je	0x142830f8b <.text+0x282ff8b>
// Frames: F376 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x142830f7b  488b4d28                       movq	0x28(%rbp), %rcx
// Frames: F380 -> F381 -> F382 -> F383 -> F384 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x142830f7f  41b801000000                   movl	$0x1, %r8d
// 0x142830f85  e86649e7fd                     callq	0x1406a58f0 <.text+0x6a48f0>
// 0x142830f8a  90                             nop
// 0x142830f8b  4883c428                       addq	$0x28, %rsp
// 0x142830f8f  5e                             popq	%rsi
// 0x142830f90  5d                             popq	%rbp
// 0x142830f91  c3                             retq
// 0x142830f92  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x142830fa0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x142830fa5  55                             pushq	%rbp
// 0x142830fa6  56                             pushq	%rsi
// 0x142830fa7  4883ec28                       subq	$0x28, %rsp
// 0x142830fab  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F376 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x142830fb2  488b55b0                       movq	-0x50(%rbp), %rdx
// Frames: F379 -> F376 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x142830fb6  4885d2                         testq	%rdx, %rdx
// 0x142830fb9  7410                           je	0x142830fcb <.text+0x282ffcb>
// Frames: F376 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x142830fbb  488b4db8                       movq	-0x48(%rbp), %rcx
// Frames: F380 -> F381 -> F382 -> F383 -> F384 -> F377 -> F71 -> F71 -> F71 -> F378 -> F312
// 0x142830fbf  41b801000000                   movl	$0x1, %r8d
// 0x142830fc5  e82649e7fd                     callq	0x1406a58f0 <.text+0x6a48f0>
// 0x142830fca  90                             nop
// 0x142830fcb  4883c428                       addq	$0x28, %rsp
// 0x142830fcf  5e                             popq	%rsi
// 0x142830fd0  5d                             popq	%rbp
// 0x142830fd1  c3                             retq
// 0x142830fd2  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// 0x142830fe0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x142830fe5  55                             pushq	%rbp
// 0x142830fe6  56                             pushq	%rsi
// 0x142830fe7  4883ec28                       subq	$0x28, %rsp
// 0x142830feb  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x142830ff2  488b4d18                       movq	0x18(%rbp), %rcx
// 0x142830ff6  0fb6553f                       movzbl	0x3f(%rbp), %edx
// Frames: F312
// 0x142830ffa  e8810b85fd                     callq	0x140081b80 <.text+0x80b80>
// 0x142830fff  90                             nop
// Frames: F306
// 0x142831000  4883c428                       addq	$0x28, %rsp
// 0x142831004  5e                             popq	%rsi
// 0x142831005  5d                             popq	%rbp
// 0x142831006  c3                             retq
// 0x142831007  660f1f840000000000             nopw	(%rax,%rax)
// 0x142831010  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x142831015  55                             pushq	%rbp
// 0x142831016  56                             pushq	%rsi
// 0x142831017  4883ec28                       subq	$0x28, %rsp
// 0x14283101b  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x142831022  488d4d20                       leaq	0x20(%rbp), %rcx
// Frames: F369 -> F312
// 0x142831026  e8b56b85fd                     callq	0x140087be0 <.text+0x86be0>
// 0x14283102b  90                             nop
// Frames: F385 -> F312
// 0x14283102c  4883c428                       addq	$0x28, %rsp
// 0x142831030  5e                             popq	%rsi
// 0x142831031  5d                             popq	%rbp
// 0x142831032  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/visual/assemble_roof_tiles.rs:10:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F2: crates/systems/roof/src/visual/assemble_roof_tiles.rs:20:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:627:0 puffin::clean_function_name
// F4: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/mod.rs:382:0 core::str::impl$0::is_char_boundary
// F5: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:439:0 core::str::traits::impl$10::get
// F6: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:470:0 core::str::traits::impl$10::index
// F7: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:63:0 core::str::traits::impl$4::index
// F8: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:628:0 puffin::clean_function_name
// F9: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:630:0 puffin::clean_function_name
// F10: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:508:0 core::str::traits::impl$11::get
// F11: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:541:0 core::str::traits::impl$11::index
// F12: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:211:0 core::str::traits::impl$7::get_unchecked
// F13: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:530:0 core::str::traits::impl$11::get_unchecked
// F14: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:511:0 core::str::traits::impl$11::get
// F15: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:863:0 core::ptr::const_ptr::impl$0::add
// F16: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:212:0 core::str::traits::impl$7::get_unchecked
// F17: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:84:0 core::str::validations::next_code_point_reverse
// F18: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/iter.rs:128:0 core::str::iter::impl$2::next_back
// F19: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/iter.rs:216:0 core::str::iter::impl$6::next_back
// F20: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:714:0 core::str::pattern::impl$11::next_back
// F21: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:317:0 core::str::pattern::ReverseSearcher::next_match_back
// F22: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:799:0 core::str::pattern::impl$22::next_match_back
// F23: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/mod.rs:1531:0 core::str::impl$0::rfind
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:662:0 puffin::short_file_name
// F25: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:332:0 core::slice::iter::impl$171::any
// F26: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/cmp.rs:417:0 core::slice::cmp::impl$49::slice_contains
// F27: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:2593:0 core::slice::impl$0::contains
// F28: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:660:0 core::str::pattern::impl$8::matches
// F29: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:717:0 core::str::pattern::impl$11::next_back
// F30: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:93:0 core::str::validations::next_code_point_reverse
// F31: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:24:0 core::str::validations::utf8_is_cont_byte
// F32: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:95:0 core::str::validations::next_code_point_reverse
// F33: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:98:0 core::str::validations::next_code_point_reverse
// F34: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:100:0 core::str::validations::next_code_point_reverse
// F35: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:583:0 core::ptr::non_null::NonNull::offset
// F36: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:739:0 core::ptr::non_null::NonNull::sub
// F37: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:132:0 core::slice::iter::Iter::pre_dec_end
// F38: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:84:0 core::slice::iter::Iter::next_back_unchecked
// F39: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:447:0 core::slice::iter::impl$172::next_back
// F40: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:103:0 core::str::validations::next_code_point_reverse
// F41: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:11:0 core::str::validations::utf8_first_byte
// F42: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:104:0 core::str::validations::next_code_point_reverse
// F43: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:17:0 core::str::validations::utf8_acc_cont_byte
// F44: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:105:0 core::str::validations::next_code_point_reverse
// F45: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:107:0 core::str::validations::next_code_point_reverse
// F46: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:109:0 core::str::validations::next_code_point_reverse
// F47: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:1720:0 core::ptr::non_null::impl$16::eq
// F48: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$172::next_back
// F49: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/validations.rs:83:0 core::str::validations::next_code_point_reverse
// F50: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:25:0 core::slice::iter::impl$172::next_back
// F51: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:664:0 puffin::short_file_name
// F52: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/thread_local/native/mod.rs:97:0 puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
// F53: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F54: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:512:0 std::thread::local::LocalKey::try_with
// F55: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:477:0 std::thread::local::LocalKey::with
// F56: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:429:0 puffin::ThreadProfiler::call
// F57: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:591:0 puffin::ProfilerScope::new
// F58: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/thread_local/native/lazy.rs:59:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F59: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:2066:0 core::cell::BorrowRefMut::new
// F60: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1256:0 core::cell::RefCell::try_borrow_mut
// F61: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1222:0 core::cell::RefCell::borrow_mut
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:429:0 puffin::impl$6::call::closure$0
// F63: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:513:0 std::thread::local::LocalKey::try_with
// F64: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/mem/mod.rs:902:0 core::mem::replace
// F65: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:513:0 core::cell::Cell::replace
// F66: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:2068:0 core::cell::BorrowRefMut::new
// F67: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:2447:0 core::cell::UnsafeCell::get
// F68: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1264:0 core::cell::RefCell::try_borrow_mut
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:592:0 puffin::impl$9::new::closure$0
// F70: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:2055:0 core::cell::impl$52::drop
// F71: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:805:0 core::ptr::drop_in_place
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:1065:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs:28:0 bevy_ecs::message::message_reader::MessageReader::read
// F74: crates/systems/roof/src/visual/assemble_roof_tiles.rs:22:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:377:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:55:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs:93:0 bevy_ecs::message::message_cursor::MessageCursor::read_with_id
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs:80:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F79: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:2345:0 core::num::impl$11::saturating_sub
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:57:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:60:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F82: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:923:0 core::num::impl$11::checked_sub
// F83: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:368:0 core::slice::index::impl$4::get
// F84: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:546:0 core::slice::index::impl$7::get
// F85: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 core::slice::impl$0::get
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:61:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F87: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1815:0 alloc::vec::Vec::as_slice
// F88: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3729:0 alloc::vec::impl$9::deref
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:62:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F90: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:67:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F91: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 core::ptr::mut_ptr::impl$0::add
// F92: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F93: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1041:0 core::slice::impl$0::iter
// F94: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:69:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F95: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:767:0 core::option::Option::as_mut
// F96: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:332:0 core::iter::adapters::chain::and_then_or_clear
// F97: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:0 core::iter::adapters::chain::impl$1::next
// F98: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:89:0 bevy_ecs::message::iterators::impl$3::next
// F99: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:15:0 bevy_ecs::message::iterators::impl$0::next
// F100: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:0 core::iter::adapters::chain::impl$1::next::closure$0
// F101: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1656:0 core::option::Option::or_else
// F102: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:659:0 core::ptr::non_null::NonNull::add
// F103: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:185:0 core::slice::iter::impl$171::next
// F104: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:95:0 bevy_ecs::message::iterators::impl$3::next
// F105: crates/country-core/src/asset_libraries/ssbo_library.rs:248:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F106: crates/country-core/src/asset_libraries/ssbo_library.rs:284:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
// F107: crates/systems/roof/src/visual/assemble_roof_tiles.rs:23:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F108: crates/country-core/src/asset_libraries/ssbo_library.rs:250:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
// F109: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:0 core::option::Option::unwrap
// F110: crates/systems/roof/src/visual/assemble_roof_tiles.rs:25:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F111: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:219:0 core::slice::index::impl$2::get
// F112: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:957:0 bevy_ecs::entity::Entities::get_id_location
// F113: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:949:0 bevy_ecs::entity::Entities::get
// F114: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1546:0 bevy_ecs::system::query::Query::get_inner
// F115: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1525:0 bevy_ecs::system::query::Query::get_mut
// F116: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1583:0 core::option::Option::filter
// F117: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:958:0 bevy_ecs::entity::Entities::get_id_location
// F118: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:214:0 bevy_ecs::entity::impl$37::eq
// F119: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:958:0 bevy_ecs::entity::impl$18::get_id_location::closure$0
// F120: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1584:0 core::option::Option::filter
// F121: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1589:0 core::option::Option::filter
// F122: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1338:0 core::option::Option::ok_or
// F123: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1547:0 bevy_ecs::system::query::Query::get_inner
// F124: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:123:0 bevy_ecs::archetype::ArchetypeId::index
// F125: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1551:0 bevy_ecs::system::query::Query::get_inner
// F126: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:393:0 bevy_ecs::archetype::ArchetypeId::index
// F127: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/bool.rs:66:0 core::bool::impl$0::then
// F128: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 bevy_ecs::archetype::ArchetypeId::index
// F129: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F130: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F131: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F132: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/internal_macros.rs:28:0 fixedbitset::FixedBitSet::get_unchecked
// F133: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1548:0 bevy_ecs::system::query::Query::get_inner
// F134: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:612:0 alloc::raw_vec::RawVecInner::non_null
// F135: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:607:0 alloc::raw_vec::RawVecInner::ptr
// F136: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:295:0 alloc::raw_vec::RawVec::ptr
// F137: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1916:0 alloc::vec::Vec::as_ptr
// F138: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:753:0 bevy_ecs::storage::table::Tables::get
// F139: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1580:0 bevy_ecs::system::query::Query::get_inner
// F140: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:221:0 core::slice::index::impl$2::get
// F141: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 alloc::vec::impl$9::deref
// F142: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1582:0 bevy_ecs::system::query::Query::get_inner
// F143: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:406:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F144: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:490:0 bevy_ecs::storage::table::Table::get_column
// F145: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1932:0 bevy_ecs::query::fetch::impl$48::set_table
// F146: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1921:0 bevy_ecs::query::fetch::impl$48::set_archetype
// F147: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$2::set_archetype
// F148: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F149: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:381:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F150: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1552:0 bevy_ecs::query::fetch::impl$36::set_table
// F151: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1539:0 bevy_ecs::query::fetch::impl$36::set_archetype
// F152: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:745:0 core::option::Option::as_ref
// F153: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1546:0 core::slice::impl$0::get
// F154: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F155: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:408:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F156: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:0 core::option::Option::map
// F157: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:253:0 core::slice::index::impl$2::get_unchecked
// F158: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 nonmax::NonMaxUsize::get
// F159: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F160: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs:298:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F161: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:382:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F162: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:382:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F163: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxU32::get
// F164: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:116:0 bevy_ecs::storage::table::TableRow::index
// F165: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2000:0 bevy_ecs::query::fetch::impl$49::fetch::closure$0
// F166: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2966:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F167: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1993:0 bevy_ecs::query::fetch::impl$49::fetch
// F168: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2529:0 bevy_ecs::query::fetch::impl$89::fetch
// F169: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1591:0 bevy_ecs::system::query::Query::get_inner
// F170: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs:1079:0 bevy_ecs::storage::table::TableRow::index
// F171: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:636:0 core::option::Option::is_some
// F172: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:685:0 core::option::Option::is_none
// F173: crates/systems/roof/src/visual/assemble_roof_tiles.rs:27:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F174: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:93:0 alloc::alloc::alloc
// F175: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:190:0 alloc::alloc::Global::alloc_impl_runtime
// F176: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:312:0 alloc::alloc::Global::alloc_impl
// F177: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:429:0 alloc::alloc::impl$1::allocate
// F178: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:248:0 alloc::boxed::box_new_uninit
// F179: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:286:0 alloc::boxed::impl$0::new
// F180: crates/country-core/src/render/sparse_instance_buffer.rs:159:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
// F181: crates/systems/roof/src/visual/assemble_roof_tiles.rs:28:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F182: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:95:0 alloc::alloc::alloc
// F183: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:289:0 alloc::boxed::impl$0::new
// F184: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:444:0 core::ptr::non_null::NonNull::as_ref
// F185: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2104:0 alloc::sync::Arc::inner
// F186: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1919:0 alloc::sync::Arc::downgrade
// F187: crates/country-core/src/render/sparse_instance_buffer.rs:167:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
// F188: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:25:0 core::core_arch::x86::sse2::_mm_pause
// F189: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/hint.rs:287:0 core::hint::spin_loop
// F190: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1924:0 alloc::sync::Arc::downgrade
// F191: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3890:0 core::sync::atomic::atomic_load
// F192: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:2844:0 core::sync::atomic::AtomicUsize::load
// F193: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:0:0 alloc::sync::Arc::downgrade
// F194: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1923:0 alloc::sync::Arc::downgrade
// F195: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1930:0 alloc::sync::Arc::downgrade
// F196: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1939:0 alloc::sync::Arc::downgrade
// F197: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:4040:0 core::sync::atomic::atomic_compare_exchange_weak
// F198: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3104:0 core::sync::atomic::AtomicUsize::compare_exchange_weak
// F199: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$57::set_changed
// F200: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$58::deref_mut
// F201: crates/systems/roof/src/visual/assemble_roof_tiles.rs:31:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F202: crates/country-core/src/render/sparse_instance_buffer.rs:204:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F203: crates/systems/roof/src/visual/assemble_roof_tiles.rs:32:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F204: crates/country-core/src/render/sparse_instance_buffer.rs:205:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F205: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3104:0 core::sync::atomic::AtomicU8::compare_exchange_weak
// F206: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:69:0 parking_lot::raw_mutex::impl$0::lock
// F207: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:67:0 parking_lot::raw_mutex::impl$0::lock
// F208: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs:1114:0 parking_lot_core::parking_lot::deadlock::acquire_resource
// F209: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:74:0 parking_lot::raw_mutex::impl$0::lock
// F210: crates/country-core/src/render/sparse_instance_buffer.rs:208:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F211: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:282:0 core::slice::impl$0::last
// F212: crates/country-core/src/render/sparse_instance_buffer.rs:211:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F213: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:2125:0 core::option::Option::copied
// F214: crates/country-core/src/render/sparse_instance_buffer.rs:212:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F215: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:2126:0 core::option::Option::copied
// F216: crates/country-core/src/render/sparse_instance_buffer.rs:213:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F217: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1065:0 core::option::Option::unwrap_or_else
// F218: crates/country-core/src/render/sparse_instance_buffer.rs:213:0 country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
// F219: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1067:0 core::option::Option::unwrap_or_else
// F220: crates/country-core/src/render/sparse_instance_buffer.rs:214:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F221: crates/country-core/src/render/sparse_instance_buffer.rs:215:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F222: crates/country-core/src/render/sparse_instance_buffer.rs:217:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F223: crates/country-core/src/render/sparse_instance_buffer.rs:218:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F224: crates/country-core/src/render/sparse_instance_buffer.rs:219:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F225: crates/country-core/src/render/sparse_instance_buffer.rs:220:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F226: crates/country-core/src/render/sparse_instance_buffer.rs:207:0 country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// F227: crates/systems/roof/src/visual/assemble_roof_tiles.rs:33:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F228: crates/systems/roof/src/visual/assemble_roof_tiles.rs:35:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F229: crates/systems/roof/src/visual/assemble_roof_tiles.rs:40:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F230: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1164:0 core::option::Option::map
// F231: /rust/deps/hashbrown-0.16.1/src/map.rs:1309:0 hashbrown::map::HashMap::get
// F232: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:1002:0 std::collections::hash::map::HashMap::get
// F233: crates/systems/roof/src/visual/assemble_roof_tiles.rs:51:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F234: /rust/deps/hashbrown-0.16.1/src/map.rs:258:0 hashbrown::map::make_hash
// F235: /rust/deps/hashbrown-0.16.1/src/map.rs:1310:0 hashbrown::map::HashMap::get
// F236: /rust/deps/hashbrown-0.16.1/src/control/tag.rs:48:0 hashbrown::control::tag::Tag::full
// F237: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:2050:0 hashbrown::raw::RawTableInner::find_inner
// F238: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:1236:0 hashbrown::raw::RawTable::find
// F239: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:1251:0 hashbrown::raw::RawTable::get
// F240: /rust/deps/hashbrown-0.16.1/src/map.rs:1311:0 hashbrown::map::HashMap::get
// F241: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:2492:0 hashbrown::raw::RawTableInner::probe_seq
// F242: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:2051:0 hashbrown::raw::RawTableInner::find_inner
// F243: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:547:0 core::ptr::copy_nonoverlapping
// F244: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1335:0 core::core_arch::x86::sse2::_mm_loadu_si128
// F245: /rust/deps/hashbrown-0.16.1/src/control/group/sse2.rs:52:0 hashbrown::control::group::sse2::Group::load
// F246: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:2068:0 hashbrown::raw::RawTableInner::find_inner
// F247: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1563:0 core::core_arch::x86::sse2::_mm_movemask_epi8
// F248: /rust/deps/hashbrown-0.16.1/src/control/group/sse2.rs:87:0 hashbrown::control::group::sse2::Group::match_tag
// F249: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:2070:0 hashbrown::raw::RawTableInner::find_inner
// F250: /rust/deps/hashbrown-0.16.1/src/control/bitmask.rs:50:0 hashbrown::control::bitmask::BitMask::lowest_set_bit
// F251: /rust/deps/hashbrown-0.16.1/src/control/bitmask.rs:113:0 hashbrown::control::bitmask::impl$2::next
// F252: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/nonzero.rs:641:0 core::num::nonzero::NonZero::trailing_zeros
// F253: /rust/deps/hashbrown-0.16.1/src/control/bitmask.rs:80:0 hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
// F254: /rust/deps/hashbrown-0.16.1/src/control/bitmask.rs:51:0 hashbrown::control::bitmask::BitMask::lowest_set_bit
// F255: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:2073:0 hashbrown::raw::RawTableInner::find_inner
// F256: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:1072:0 core::ptr::mut_ptr::impl$0::sub
// F257: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:339:0 hashbrown::raw::Bucket::from_base_index
// F258: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:812:0 hashbrown::raw::RawTable::bucket
// F259: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:1236:0 hashbrown::raw::impl$8::find::closure$0
// F260: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:2075:0 hashbrown::raw::RawTableInner::find_inner
// F261: crates/country-core/src/resources/walls/mod.rs:84:0 country_core::resources::walls::impl$10::eq
// F262: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:2123:0 core::cmp::impls::impl$9::eq
// F263: /rust/deps/hashbrown-0.16.1/src/lib.rs:174:0 hashbrown::impl$0::equivalent
// F264: /rust/deps/hashbrown-0.16.1/src/map.rs:224:0 hashbrown::map::equivalent_key::closure$0
// F265: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics/mod.rs:434:0 core::intrinsics::likely
// F266: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/nonzero.rs:499:0 core::num::nonzero::NonZero::get
// F267: /rust/deps/hashbrown-0.16.1/src/control/group/sse2.rs:95:0 hashbrown::control::group::sse2::Group::match_empty
// F268: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:2080:0 hashbrown::raw::RawTableInner::find_inner
// F269: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics/mod.rs:434:0 hashbrown::control::group::sse2::Group::match_empty
// F270: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:91:0 hashbrown::raw::ProbeSeq::move_next
// F271: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:2084:0 hashbrown::raw::RawTableInner::find_inner
// F272: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:90:0 hashbrown::raw::ProbeSeq::move_next
// F273: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:2053:0 hashbrown::raw::RawTableInner::find_inner
// F274: crates/systems/roof/src/visual/assemble_roof_tiles.rs:69:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F275: crates/country-core/src/render/sparse_instance_buffer.rs:599:0 country_core::render::sparse_instance_buffer::impl$5::drop
// F276: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs:1130:0 parking_lot_core::parking_lot::deadlock::release_resource
// F277: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:101:0 parking_lot::raw_mutex::impl$0::unlock
// F278: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3984:0 core::sync::atomic::atomic_compare_exchange
// F279: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3039:0 core::sync::atomic::AtomicU8::compare_exchange
// F280: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:104:0 parking_lot::raw_mutex::impl$0::unlock
// F281: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:102:0 parking_lot::raw_mutex::impl$0::unlock
// F282: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:109:0 parking_lot::raw_mutex::impl$0::unlock
// F283: crates/systems/roof/src/visual/assemble_roof_tiles.rs:54:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F284: crates/systems/roof/src/visual/assemble_roof_tiles.rs:58:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F285: crates/systems/roof/src/visual/assemble_roof_tiles.rs:59:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F286: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3022:0 alloc::vec::Vec::is_empty
// F287: crates/systems/roof/src/visual/assemble_roof_tiles.rs:51:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$2
// F288: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:664:0 core::option::Option::is_some_and
// F289: crates/systems/roof/src/visual/assemble_roof_tiles.rs:55:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F290: crates/systems/roof/src/visual/assemble_roof_tiles.rs:56:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F291: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:72:0 parking_lot::raw_mutex::impl$0::lock
// F292: crates/systems/roof/src/visual/assemble_roof_tiles.rs:70:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::hf948b88b9fbdcc4d
// F293: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:602:0 puffin::impl$10::drop
// F294: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs:602:0 puffin::impl$10::drop::closure$0
// F295: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:259:0 core::panicking::panic_display
// F296: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1016:0 core::option::Option::unwrap
// F297: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:250:0 alloc::boxed::box_new_uninit
// F298: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/thread_local/native/lazy.rs:62:0 std::sys::thread_local::native::lazy::Storage::get_or_init
// F299: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:269:0 core::ptr::const_ptr::impl$0::as_ref
// F300: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:2189:0 core::result::impl$28::from_residual
// F301: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1543:0 bevy_ecs::system::query::Query::get_inner
// F302: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1553:0 bevy_ecs::system::query::Query::get_inner
// F303: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1233:0 core::result::Result::unwrap
// F304: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:543:0 core::str::traits::impl$11::index
// F305: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:472:0 core::str::traits::impl$10::index
// F306: crates/systems/roof/src/visual/assemble_roof_tiles.rs:44:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// F307: crates/systems/roof/src/visual/assemble_roof_tiles.rs:45:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// F308: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3891:0 core::sync::atomic::atomic_load
// F309: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:2844:0 core::sync::atomic::AtomicU32::load
// F310: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/sync/once/futex.rs:82:0 std::sys::sync::once::futex::Once::is_completed
// F311: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/once.rs:154:0 std::sync::once::Once::call_once
// F312: crates/systems/roof/src/visual/assemble_roof_tiles.rs:46:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// F313: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1183:0 core::result::Result::expect
// F314: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:0 core::result::Result::expect
// F315: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:464:0 alloc::raw_vec::RawVecInner::try_allocate_in
// F316: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:433:0 alloc::raw_vec::RawVecInner::with_capacity_in
// F317: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:0 alloc::raw_vec::RawVec::with_capacity_in
// F318: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:965:0 alloc::vec::Vec::with_capacity_in
// F319: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:448:0 alloc::slice::impl$0::to_vec_in::impl$1::to_vec
// F320: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:400:0 alloc::slice::impl$0::to_vec_in
// F321: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:376:0 alloc::slice::impl$0::to_vec
// F322: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:842:0 alloc::slice::impl$9::to_owned
// F323: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/str.rs:211:0 alloc::str::impl$4::to_owned
// F324: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:3114:0 alloc::string::impl$48::from
// F325: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:468:0 alloc::raw_vec::RawVecInner::try_allocate_in
// F326: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:1252:0 core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
// F327: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:454:0 alloc::slice::impl$0::to_vec_in::impl$1::to_vec
// F328: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:1014:0 alloc::slice::impl$9::to_owned
// F329: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:673:0 alloc::raw_vec::RawVecInner::reserve
// F330: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:340:0 alloc::raw_vec::RawVec::reserve
// F331: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1446:0 alloc::vec::Vec::reserve
// F332: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2879:0 alloc::vec::Vec::append_elements
// F333: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_extend.rs:56:0 alloc::vec::spec_extend::impl$4::spec_extend
// F334: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3500:0 alloc::vec::Vec::extend_from_slice
// F335: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:1102:0 alloc::string::String::push_str
// F336: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2776:0 alloc::string::impl$31::add
// F337: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2000:0 alloc::vec::Vec::as_mut_ptr
// F338: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2883:0 alloc::vec::Vec::append_elements
// F339: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2997:0 alloc::vec::Vec::len
// F340: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2880:0 alloc::vec::Vec::append_elements
// F341: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:547:0 alloc::vec::Vec::as_mut_ptr
// F342: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2886:0 alloc::vec::Vec::append_elements
// F343: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2777:0 alloc::string::impl$31::add
// F344: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:617:0 alloc::raw_vec::RawVecInner::capacity
// F345: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:765:0 alloc::raw_vec::RawVecInner::needs_to_grow
// F346: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:671:0 alloc::raw_vec::RawVecInner::reserve
// F347: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:2494:0 alloc::raw_vec::RawVecInner::capacity
// F348: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/poison/mutex.rs:736:0 std::sync::poison::mutex::impl$13::deref_mut
// F349: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/collections/btree/set.rs:902:0 alloc::collections::btree::set::BTreeSet::insert
// F350: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs:1328:0 log::max_level
// F351: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:662:0 core::option::Option::is_some_and
// F352: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:1429:0 core::cmp::PartialOrd::le
// F353: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs:766:0 log::impl$55::clone
// F354: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs:1034:0 log::RecordBuilder::build
// F355: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs:81:0 log::__private_api::log_impl
// F356: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs:94:0 log::__private_api::log
// F357: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/poison.rs:129:0 std::sync::poison::Flag::done
// F358: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/poison/mutex.rs:745:0 std::sync::poison::mutex::impl$14::drop
// F359: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:464:0 std::panicking::panic_count::count_is_zero
// F360: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:616:0 std::panicking::panicking
// F361: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/functions.rs:217:0 std::thread::functions::panicking
// F362: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3908:0 core::sync::atomic::atomic_swap
// F363: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:2901:0 core::sync::atomic::AtomicU8::swap
// F364: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/sync/mutex/futex.rs:90:0 std::sys::sync::mutex::futex::Mutex::unlock
// F365: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/poison/mutex.rs:746:0 std::sync::poison::mutex::impl$14::drop
// F366: crates/systems/roof/src/visual/assemble_roof_tiles.rs:48:0 system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// F367: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/once.rs:158:0 std::sync::once::Once::call_once
// F368: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/once.rs:159:0 std::sync::once::Once::call_once
// F369: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1185:0 core::result::Result::expect
// F370: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/sync/mutex/futex.rs:95:0 std::sys::sync::mutex::futex::Mutex::unlock
// F371: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:476:0 std::panicking::panic_count::count_is_zero
// F372: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3875:0 core::sync::atomic::atomic_store
// F373: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:759:0 core::sync::atomic::AtomicBool::store
// F374: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/poison.rs:130:0 std::sync::poison::Flag::done
// F375: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:441:0 alloc::raw_vec::RawVecInner::with_capacity_in
// F376: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:872:0 alloc::raw_vec::RawVecInner::deallocate
// F377: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:423:0 alloc::raw_vec::impl$3::drop
// F378: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2778:0 alloc::string::impl$31::add
// F379: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:632:0 alloc::raw_vec::RawVecInner::current_memory
// F380: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:115:0 alloc::alloc::dealloc
// F381: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:209:0 alloc::alloc::Global::deallocate_impl_runtime
// F382: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:324:0 alloc::alloc::Global::deallocate_impl
// F383: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:442:0 alloc::alloc::impl$1::deallocate
// F384: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:874:0 alloc::raw_vec::RawVecInner::deallocate
// F385: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1179:0 core::result::Result::expect
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/visual/assemble_roof_tiles.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 987d63e6d9d760401f0c9e621d3a6d8ac809c19fb2e6cba764aff6551d9a5621
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: assemble_roof_tiles.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2444, 3785, 3786
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
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
// 首条位置: {"module":2444,"record_offset":268,"rva":32757840,"va":5401466960}
// 编译实例 RVAs: 0x1f3d850
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25789408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e8207c76fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5401468091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32745889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e85f560cfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5401468076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008f730
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32173051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e8051415fe","size":5,"soft_fail":false,"target_va":5369296688,"va":5401469734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbb800
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16262795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e875d907ff","size":5,"soft_fail":false,"target_va":5385205760,"va":5401468550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbf6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16246396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e8841908ff","size":5,"soft_fail":false,"target_va":5385221840,"va":5401468231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141f41a80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e8563c0000","size":5,"soft_fail":false,"target_va":5401483904,"va":5401468453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141f42280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e8fc460000","size":5,"soft_fail":false,"target_va":5401485952,"va":5401467775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14282bac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9362222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e82edb8e00","size":5,"soft_fail":false,"target_va":5410831040,"va":5401468813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14282f9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9378342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e8261a8f00","size":5,"soft_fail":false,"target_va":5410847216,"va":5401468869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15151604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e8f431e700","size":5,"soft_fail":false,"target_va":5416620769,"va":5401469160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15152225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e86134e700","size":5,"soft_fail":false,"target_va":5416621408,"va":5401469178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t15152492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e86c35e700","size":5,"soft_fail":false,"target_va":5416621600,"va":5401469103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15153150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e8fe37e700","size":5,"soft_fail":false,"target_va":5416622240,"va":5401469085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15152327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e8c734e700","size":5,"soft_fail":false,"target_va":5416621632,"va":5401469300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e81a0b0000","size":5,"soft_fail":false,"target_va":5401469888,"va":5401467041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t15152018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e89233e700","size":5,"soft_fail":false,"target_va":5416621376,"va":5401469353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20271018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e856b0cafe","size":5,"soft_fail":false,"target_va":5381197456,"va":5401468469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20564970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e81634c6fe","size":5,"soft_fail":false,"target_va":5380903536,"va":5401468501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15107896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e83887e600","size":5,"soft_fail":false,"target_va":5416576832,"va":5401468931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15107399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e84785e600","size":5,"soft_fail":false,"target_va":5416576144,"va":5401468740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17483477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e82b39f5fe","size":5,"soft_fail":false,"target_va":5383984848,"va":5401468320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17483360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e8a039f5fe","size":5,"soft_fail":false,"target_va":5383985360,"va":5401468715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17503616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e880eaf4fe","size":5,"soft_fail":false,"target_va":5383963792,"va":5401467403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17495109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e8bb0bf5fe","size":5,"soft_fail":false,"target_va":5383972224,"va":5401467328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17504095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e8a1e8f4fe","size":5,"soft_fail":false,"target_va":5383964912,"va":5401469002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17504207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e831e8f4fe","size":5,"soft_fail":false,"target_va":5383962816,"va":5401467018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t15550759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e82749ed00","size":5,"soft_fail":false,"target_va":5417019952,"va":5401469188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9316753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e891298e00","size":5,"soft_fail":false,"target_va":5410786608,"va":5401469850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15150125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401466960,"raw_hex":"e82d2ce700","size":5,"soft_fail":false,"target_va":5416619360,"va":5401469230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5401466960,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5401467797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2444,"record_offset":6032,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2444,"record_offset":3392,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":2444,"record_offset":2692,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2444,"record_offset":3780,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11799,"source_line":1524,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":6172,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":5432,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":7812,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":6668,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":3744,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2444,"record_offset":6088,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2444,"record_offset":8952,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2444,"record_offset":3684,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2444,"record_offset":5488,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":26135,"source_line":155,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2444,"record_offset":6236,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11663,"source_line":196,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2444,"record_offset":1884,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2444,"record_offset":484,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2444,"record_offset":872,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2444,"record_offset":6700,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2444,"record_offset":5620,"parent_record_offset":5596,"ancestor_record_offset":268,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2444,"record_offset":5644,"parent_record_offset":5620,"ancestor_record_offset":268,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2444,"record_offset":5668,"parent_record_offset":5644,"ancestor_record_offset":268,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2444,"record_offset":5596,"parent_record_offset":5564,"ancestor_record_offset":268,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2444,"record_offset":5564,"parent_record_offset":5528,"ancestor_record_offset":268,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2444,"record_offset":5528,"parent_record_offset":5488,"ancestor_record_offset":268,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2444,"record_offset":4400,"parent_record_offset":4376,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2444,"record_offset":4448,"parent_record_offset":4424,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2444,"record_offset":4424,"parent_record_offset":4400,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::downgrade
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2444,"record_offset":5720,"parent_record_offset":5488,"ancestor_record_offset":268,"inlinee":12983,"source_line":1913,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2444,"record_offset":5772,"parent_record_offset":5720,"ancestor_record_offset":268,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2444,"record_offset":4376,"parent_record_offset":4352,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2444,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2444,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":268,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2444,"record_offset":3012,"parent_record_offset":2780,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2444,"record_offset":4108,"parent_record_offset":3812,"ancestor_record_offset":268,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2444,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":268,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2444,"record_offset":3872,"parent_record_offset":3812,"ancestor_record_offset":268,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2444,"record_offset":3896,"parent_record_offset":3872,"ancestor_record_offset":268,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2444,"record_offset":4012,"parent_record_offset":3980,"ancestor_record_offset":268,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2444,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":268,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2444,"record_offset":2780,"parent_record_offset":2756,"ancestor_record_offset":268,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2444,"record_offset":3420,"parent_record_offset":3392,"ancestor_record_offset":268,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":2444,"record_offset":2756,"parent_record_offset":2716,"ancestor_record_offset":268,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2444,"record_offset":5240,"parent_record_offset":5216,"ancestor_record_offset":268,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2444,"record_offset":4664,"parent_record_offset":4528,"ancestor_record_offset":268,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2444,"record_offset":4688,"parent_record_offset":4664,"ancestor_record_offset":268,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2444,"record_offset":4552,"parent_record_offset":4528,"ancestor_record_offset":268,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2444,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":268,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2444,"record_offset":5216,"parent_record_offset":5192,"ancestor_record_offset":268,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2444,"record_offset":5264,"parent_record_offset":5240,"ancestor_record_offset":268,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2444,"record_offset":5192,"parent_record_offset":3812,"ancestor_record_offset":268,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2444,"record_offset":4528,"parent_record_offset":3812,"ancestor_record_offset":268,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":2444,"record_offset":5136,"parent_record_offset":5112,"ancestor_record_offset":268,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2444,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":268,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2444,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":268,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2444,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":268,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2444,"record_offset":4600,"parent_record_offset":4576,"ancestor_record_offset":268,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2444,"record_offset":4712,"parent_record_offset":4688,"ancestor_record_offset":268,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2444,"record_offset":5288,"parent_record_offset":5264,"ancestor_record_offset":268,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2444,"record_offset":4296,"parent_record_offset":3812,"ancestor_record_offset":268,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2444,"record_offset":5112,"parent_record_offset":5088,"ancestor_record_offset":268,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2444,"record_offset":5088,"parent_record_offset":5064,"ancestor_record_offset":268,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2444,"record_offset":3812,"parent_record_offset":3780,"ancestor_record_offset":268,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":2444,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":268,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":2444,"record_offset":4144,"parent_record_offset":4108,"ancestor_record_offset":268,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2444,"record_offset":2212,"parent_record_offset":2184,"ancestor_record_offset":268,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2444,"record_offset":2240,"parent_record_offset":2212,"ancestor_record_offset":268,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2444,"record_offset":2160,"parent_record_offset":2132,"ancestor_record_offset":268,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2444,"record_offset":2184,"parent_record_offset":2160,"ancestor_record_offset":268,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2444,"record_offset":2300,"parent_record_offset":2184,"ancestor_record_offset":268,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2444,"record_offset":2412,"parent_record_offset":2388,"ancestor_record_offset":268,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2444,"record_offset":7396,"parent_record_offset":7372,"ancestor_record_offset":268,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2444,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":268,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2444,"record_offset":7048,"parent_record_offset":7024,"ancestor_record_offset":268,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_pause
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2444,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":268,"inlinee":6415,"source_line":22,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::spin_loop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2444,"record_offset":5828,"parent_record_offset":5720,"ancestor_record_offset":268,"inlinee":6414,"source_line":270,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2444,"record_offset":7464,"parent_record_offset":6828,"ancestor_record_offset":268,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":2444,"record_offset":3476,"parent_record_offset":3452,"ancestor_record_offset":268,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":2444,"record_offset":3452,"parent_record_offset":3420,"ancestor_record_offset":268,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":2444,"record_offset":3556,"parent_record_offset":3532,"ancestor_record_offset":268,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2444,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":268,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2444,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":268,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2444,"record_offset":2828,"parent_record_offset":2780,"ancestor_record_offset":268,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2444,"record_offset":7196,"parent_record_offset":7168,"ancestor_record_offset":268,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2444,"record_offset":7168,"parent_record_offset":7140,"ancestor_record_offset":268,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2444,"record_offset":2032,"parent_record_offset":1992,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":3500,"parent_record_offset":3476,"ancestor_record_offset":268,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":4896,"parent_record_offset":4872,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":6500,"parent_record_offset":6236,"ancestor_record_offset":268,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":3980,"parent_record_offset":3896,"ancestor_record_offset":268,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":5456,"parent_record_offset":5432,"ancestor_record_offset":268,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":4936,"parent_record_offset":4764,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":4080,"parent_record_offset":3812,"ancestor_record_offset":268,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":3532,"parent_record_offset":3452,"ancestor_record_offset":268,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":6312,"parent_record_offset":6236,"ancestor_record_offset":268,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2444,"record_offset":6532,"parent_record_offset":6236,"ancestor_record_offset":268,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::panicking::panic_display
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panicking.rs
//     evidence={"module":2444,"record_offset":5996,"parent_record_offset":5720,"ancestor_record_offset":268,"inlinee":4970,"source_line":258,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2444,"record_offset":820,"parent_record_offset":792,"ancestor_record_offset":268,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2444,"record_offset":2648,"parent_record_offset":1992,"ancestor_record_offset":268,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2444,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":268,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2444,"record_offset":2364,"parent_record_offset":2132,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2444,"record_offset":3284,"parent_record_offset":3260,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2444,"record_offset":7312,"parent_record_offset":7288,"ancestor_record_offset":268,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2444,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2444,"record_offset":5796,"parent_record_offset":5772,"ancestor_record_offset":268,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2444,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":268,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2444,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":268,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2444,"record_offset":1476,"parent_record_offset":1448,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2444,"record_offset":5396,"parent_record_offset":3812,"ancestor_record_offset":268,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2444,"record_offset":1576,"parent_record_offset":1548,"ancestor_record_offset":268,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2444,"record_offset":1548,"parent_record_offset":1520,"ancestor_record_offset":268,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2444,"record_offset":2884,"parent_record_offset":2780,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2444,"record_offset":3236,"parent_record_offset":2780,"ancestor_record_offset":268,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2444,"record_offset":6472,"parent_record_offset":6236,"ancestor_record_offset":268,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2444,"record_offset":3948,"parent_record_offset":3924,"ancestor_record_offset":268,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2444,"record_offset":5016,"parent_record_offset":4984,"ancestor_record_offset":268,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2444,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":268,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2444,"record_offset":2912,"parent_record_offset":2884,"ancestor_record_offset":268,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2444,"record_offset":3260,"parent_record_offset":3236,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2444,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":268,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2444,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":268,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2444,"record_offset":1604,"parent_record_offset":1576,"ancestor_record_offset":268,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2444,"record_offset":3608,"parent_record_offset":3556,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2444,"record_offset":1196,"parent_record_offset":1080,"ancestor_record_offset":268,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2444,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":268,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2444,"record_offset":908,"parent_record_offset":872,"ancestor_record_offset":268,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2444,"record_offset":1048,"parent_record_offset":1016,"ancestor_record_offset":268,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2444,"record_offset":1016,"parent_record_offset":980,"ancestor_record_offset":268,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2444,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":268,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2444,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":268,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2444,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":268,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2444,"record_offset":1520,"parent_record_offset":980,"ancestor_record_offset":268,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2444,"record_offset":588,"parent_record_offset":556,"ancestor_record_offset":268,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2444,"record_offset":556,"parent_record_offset":528,"ancestor_record_offset":268,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2444,"record_offset":712,"parent_record_offset":680,"ancestor_record_offset":268,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2444,"record_offset":768,"parent_record_offset":712,"ancestor_record_offset":268,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2444,"record_offset":680,"parent_record_offset":652,"ancestor_record_offset":268,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2444,"record_offset":528,"parent_record_offset":484,"ancestor_record_offset":268,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2444,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":268,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2444,"record_offset":1080,"parent_record_offset":1048,"ancestor_record_offset":268,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2444,"record_offset":1364,"parent_record_offset":1080,"ancestor_record_offset":268,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2444,"record_offset":1336,"parent_record_offset":1080,"ancestor_record_offset":268,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2444,"record_offset":1140,"parent_record_offset":1080,"ancestor_record_offset":268,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2444,"record_offset":7744,"parent_record_offset":7684,"ancestor_record_offset":268,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2444,"record_offset":6384,"parent_record_offset":6348,"ancestor_record_offset":268,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2444,"record_offset":5940,"parent_record_offset":5720,"ancestor_record_offset":268,"inlinee":4713,"source_line":3097,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2444,"record_offset":5884,"parent_record_offset":5720,"ancestor_record_offset":268,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2444,"record_offset":7768,"parent_record_offset":7744,"ancestor_record_offset":268,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2444,"record_offset":5964,"parent_record_offset":5940,"ancestor_record_offset":268,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2444,"record_offset":5908,"parent_record_offset":5884,"ancestor_record_offset":268,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2444,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":268,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2444,"record_offset":6564,"parent_record_offset":6532,"ancestor_record_offset":268,"inlinee":11665,"source_line":213,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2444,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":268,"inlinee":4972,"source_line":598,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":2444,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":268,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2444,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":268,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2444,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":268,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2444,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":268,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2444,"record_offset":7108,"parent_record_offset":7080,"ancestor_record_offset":268,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2444,"record_offset":7140,"parent_record_offset":7108,"ancestor_record_offset":268,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2444,"record_offset":7080,"parent_record_offset":6828,"ancestor_record_offset":268,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2444,"record_offset":6940,"parent_record_offset":6828,"ancestor_record_offset":268,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2444,"record_offset":7492,"parent_record_offset":6828,"ancestor_record_offset":268,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2444,"record_offset":7024,"parent_record_offset":6828,"ancestor_record_offset":268,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2444,"record_offset":6884,"parent_record_offset":6828,"ancestor_record_offset":268,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2444,"record_offset":7372,"parent_record_offset":7348,"ancestor_record_offset":268,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2444,"record_offset":6724,"parent_record_offset":6700,"ancestor_record_offset":268,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2444,"record_offset":7348,"parent_record_offset":7240,"ancestor_record_offset":268,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2444,"record_offset":6752,"parent_record_offset":6724,"ancestor_record_offset":268,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2444,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":268,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2444,"record_offset":7584,"parent_record_offset":6828,"ancestor_record_offset":268,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2444,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":268,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2444,"record_offset":6804,"parent_record_offset":6780,"ancestor_record_offset":268,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2444,"record_offset":6780,"parent_record_offset":6724,"ancestor_record_offset":268,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2444,"record_offset":6828,"parent_record_offset":6804,"ancestor_record_offset":268,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2444,"record_offset":6912,"parent_record_offset":6828,"ancestor_record_offset":268,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2444,"record_offset":7240,"parent_record_offset":6828,"ancestor_record_offset":268,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2444,"record_offset":5320,"parent_record_offset":5288,"ancestor_record_offset":268,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2444,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":268,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":2444,"record_offset":6348,"parent_record_offset":6236,"ancestor_record_offset":268,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":2444,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":268,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":2444,"record_offset":6440,"parent_record_offset":6348,"ancestor_record_offset":268,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":2444,"record_offset":7716,"parent_record_offset":7684,"ancestor_record_offset":268,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2444,"record_offset":1920,"parent_record_offset":1884,"ancestor_record_offset":268,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2444,"record_offset":8136,"parent_record_offset":8104,"ancestor_record_offset":268,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2444,"record_offset":8588,"parent_record_offset":8384,"ancestor_record_offset":268,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2444,"record_offset":2060,"parent_record_offset":2032,"ancestor_record_offset":268,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2444,"record_offset":2132,"parent_record_offset":1992,"ancestor_record_offset":268,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2444,"record_offset":2336,"parent_record_offset":2132,"ancestor_record_offset":268,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2444,"record_offset":2088,"parent_record_offset":2060,"ancestor_record_offset":268,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2444,"record_offset":1992,"parent_record_offset":1956,"ancestor_record_offset":268,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2444,"record_offset":1956,"parent_record_offset":1920,"ancestor_record_offset":268,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/visual/assemble_roof_tiles.rs
//     evidence={"module":2444,"record_offset":7836,"parent_record_offset":7812,"ancestor_record_offset":268,"inlinee":105133,"source_line":51,"archive":"_pdb/C/modules/02444.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// 记录形式: compiled_procedure; instances=2
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3785,"record_offset":30460,"rva":42136944,"va":5410846064}
// 编译实例 RVAs: 0x282f570, 0x2830c70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5769544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e848095800","size":5,"soft_fail":false,"target_va":5416616304,"va":5410846755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::once::futex::Once>::call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5770615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e8770d5800","size":5,"soft_fail":false,"target_va":5416617264,"va":5410846644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-26237137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e82fa76ffe","size":5,"soft_fail":false,"target_va":5384609408,"va":5410846540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-35167489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e8ff62e7fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5410846172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-35168154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e86660e7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5410846853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42123970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e83e3d7dfd","size":5,"soft_fail":false,"target_va":5368722192,"va":5410846157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=2
//     target_aliases=47 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-41606527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e8812285fd","size":5,"soft_fail":false,"target_va":5369240448,"va":5410846970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=2
//     target_aliases=122 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-41581899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e8b58285fd","size":5,"soft_fail":false,"target_va":5369265120,"va":5410847014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400faa90
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-41110308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e8dcb48cfd","size":5,"soft_fail":false,"target_va":5369735824,"va":5410846127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141f41390
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9363966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e8021e71ff","size":5,"soft_fail":false,"target_va":5401482128,"va":5410846089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: assemble_roof_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5506772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e8d4065400","size":5,"soft_fail":false,"target_va":5416353040,"va":5410846263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::BTreeMap$LT$K$C$V$C$A$GT$::insert
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9599888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e870846dff","size":5,"soft_fail":false,"target_va":5401246512,"va":5410846395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5773986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e8a21a5800","size":5,"soft_fail":false,"target_va":5416620794,"va":5410846803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5774923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e84b1e5800","size":5,"soft_fail":false,"target_va":5416621632,"va":5410846704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5772920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410846064,"raw_hex":"e878165800","size":5,"soft_fail":false,"target_va":5416619696,"va":5410846771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::btree::set::BTreeSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\set.rs
//     evidence={"module":3785,"record_offset":32856,"parent_record_offset":30460,"ancestor_record_offset":30460,"inlinee":15173,"source_line":898,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3785,"record_offset":31332,"parent_record_offset":30460,"ancestor_record_offset":30460,"inlinee":21760,"source_line":2775,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$48::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3785,"record_offset":30816,"parent_record_offset":30460,"ancestor_record_offset":30460,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3785,"record_offset":32968,"parent_record_offset":30460,"ancestor_record_offset":30460,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3785,"record_offset":33136,"parent_record_offset":30460,"ancestor_record_offset":30460,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3785,"record_offset":30772,"parent_record_offset":30460,"ancestor_record_offset":30460,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3785,"record_offset":33024,"parent_record_offset":30460,"ancestor_record_offset":30460,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3785,"record_offset":32884,"parent_record_offset":30460,"ancestor_record_offset":30460,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_direct] std::sync::once::Once::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":3785,"record_offset":30652,"parent_record_offset":30460,"ancestor_record_offset":30460,"inlinee":21517,"source_line":149,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":3785,"record_offset":32800,"parent_record_offset":30460,"ancestor_record_offset":30460,"inlinee":12499,"source_line":735,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3785,"record_offset":31148,"parent_record_offset":31124,"ancestor_record_offset":30460,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3785,"record_offset":31172,"parent_record_offset":31148,"ancestor_record_offset":30460,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3785,"record_offset":31904,"parent_record_offset":31880,"ancestor_record_offset":30460,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3785,"record_offset":31928,"parent_record_offset":31904,"ancestor_record_offset":30460,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3785,"record_offset":31196,"parent_record_offset":31172,"ancestor_record_offset":30460,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3785,"record_offset":31952,"parent_record_offset":31928,"ancestor_record_offset":30460,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3785,"record_offset":31124,"parent_record_offset":31096,"ancestor_record_offset":30460,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3785,"record_offset":31880,"parent_record_offset":31824,"ancestor_record_offset":30460,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3785,"record_offset":31596,"parent_record_offset":31560,"ancestor_record_offset":30460,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3785,"record_offset":31500,"parent_record_offset":31476,"ancestor_record_offset":30460,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3785,"record_offset":31036,"parent_record_offset":31008,"ancestor_record_offset":30460,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3785,"record_offset":32296,"parent_record_offset":32272,"ancestor_record_offset":30460,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3785,"record_offset":31852,"parent_record_offset":31824,"ancestor_record_offset":30460,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3785,"record_offset":31824,"parent_record_offset":31800,"ancestor_record_offset":30460,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3785,"record_offset":32272,"parent_record_offset":32240,"ancestor_record_offset":30460,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3785,"record_offset":31644,"parent_record_offset":31620,"ancestor_record_offset":30460,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3785,"record_offset":31620,"parent_record_offset":31596,"ancestor_record_offset":30460,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3785,"record_offset":31524,"parent_record_offset":31500,"ancestor_record_offset":30460,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3785,"record_offset":31096,"parent_record_offset":31064,"ancestor_record_offset":30460,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3785,"record_offset":31064,"parent_record_offset":31036,"ancestor_record_offset":30460,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3785,"record_offset":31800,"parent_record_offset":31776,"ancestor_record_offset":30460,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3785,"record_offset":30916,"parent_record_offset":30872,"ancestor_record_offset":30460,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3785,"record_offset":30944,"parent_record_offset":30916,"ancestor_record_offset":30460,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3785,"record_offset":30972,"parent_record_offset":30944,"ancestor_record_offset":30460,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3785,"record_offset":30872,"parent_record_offset":30844,"ancestor_record_offset":30460,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":3785,"record_offset":30844,"parent_record_offset":30816,"ancestor_record_offset":30460,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3785,"record_offset":31368,"parent_record_offset":31332,"ancestor_record_offset":30460,"inlinee":6765,"source_line":1101,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3785,"record_offset":31440,"parent_record_offset":31416,"ancestor_record_offset":30460,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3785,"record_offset":31560,"parent_record_offset":31440,"ancestor_record_offset":30460,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3785,"record_offset":31392,"parent_record_offset":31368,"ancestor_record_offset":30460,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3785,"record_offset":31684,"parent_record_offset":31440,"ancestor_record_offset":30460,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3785,"record_offset":31476,"parent_record_offset":31440,"ancestor_record_offset":30460,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3785,"record_offset":31008,"parent_record_offset":30972,"ancestor_record_offset":30460,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3785,"record_offset":31416,"parent_record_offset":31392,"ancestor_record_offset":30460,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3785,"record_offset":32824,"parent_record_offset":32800,"ancestor_record_offset":30460,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3785,"record_offset":32992,"parent_record_offset":32968,"ancestor_record_offset":30460,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3785,"record_offset":31252,"parent_record_offset":30972,"ancestor_record_offset":30460,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3785,"record_offset":31276,"parent_record_offset":31252,"ancestor_record_offset":30460,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3785,"record_offset":31728,"parent_record_offset":31332,"ancestor_record_offset":30460,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3785,"record_offset":33392,"parent_record_offset":33204,"ancestor_record_offset":30460,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3785,"record_offset":30708,"parent_record_offset":30684,"ancestor_record_offset":30460,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3785,"record_offset":33484,"parent_record_offset":33452,"ancestor_record_offset":30460,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3785,"record_offset":32908,"parent_record_offset":32884,"ancestor_record_offset":30460,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3785,"record_offset":30732,"parent_record_offset":30708,"ancestor_record_offset":30460,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3785,"record_offset":33416,"parent_record_offset":33392,"ancestor_record_offset":30460,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3785,"record_offset":33508,"parent_record_offset":33484,"ancestor_record_offset":30460,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3785,"record_offset":33072,"parent_record_offset":33048,"ancestor_record_offset":30460,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3785,"record_offset":33048,"parent_record_offset":33024,"ancestor_record_offset":30460,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3785,"record_offset":33096,"parent_record_offset":33072,"ancestor_record_offset":30460,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":3785,"record_offset":33292,"parent_record_offset":33264,"ancestor_record_offset":30460,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":3785,"record_offset":33264,"parent_record_offset":33236,"ancestor_record_offset":30460,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":3785,"record_offset":33204,"parent_record_offset":33168,"ancestor_record_offset":30460,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":3785,"record_offset":33168,"parent_record_offset":33136,"ancestor_record_offset":30460,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":3785,"record_offset":33452,"parent_record_offset":33168,"ancestor_record_offset":30460,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":3785,"record_offset":30684,"parent_record_offset":30652,"ancestor_record_offset":30460,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":3785,"record_offset":33236,"parent_record_offset":33204,"ancestor_record_offset":30460,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/03785.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19AAD
// 首条位置: {"module":2444,"inlinee":105133}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2444,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":7836,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02444.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles
// 信息 ID: 2a26f8bea0f9092d085195fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f302eca862f77fde05a2893b: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0xfd5492 -> 0x1f3d850 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x1f2d59e -> 0x1f3d850 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$1
// 信息 ID: 6425f749e7946c03b04fc772
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_roof::visual::assemble_roof_tiles::assemble_roof_tiles", "captures": "unknown"}
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1 <- core::iter::adapters::map::impl$2::fold<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::vec::impl$21::extend_trusted::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1 <- core::iter::adapters::map::impl$2::fold<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::vec::impl$21::extend_trusted::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1 <- system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1 <- system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 5 条。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282c6c2 -> 0x282f570 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282ce8c -> 0x282f570 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282d066 -> 0x282f570 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282e447 -> 0x282f570 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282e6c7 -> 0x282f570 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x283049a -> 0x2830c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x2830709 -> 0x2830c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$2
// 信息 ID: af72cb665a7f1acbd0dbe9a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_roof::visual::assemble_roof_tiles::assemble_roof_tiles", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles; module=2444, inline_record=7836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
