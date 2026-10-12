// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: delete_roof.rs.assembly.json.gz

// FUNCTION system_roof::delete_roof::delete_roofs
// Candidate interfaces: a036a1adbc56c746e85782c5
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 268:268 -> rva_6bb650_len_539
//
// FUNCTION system_roof::delete_roof::mark_roof_for_deletion
// Candidate interfaces: 2def2f6ddc709910cdfe3344
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 268:7168 -> rva_6bbb90_len_4ca
//
// FUNCTION system_roof::delete_roof::process_invalid_roof_notifications
// Candidate interfaces: c84d717d4ec3c7d6f672c3d8
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 268:13128 -> rva_6bc060_len_7aa
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_6bb650_len_539: RVA 0x6bb650, 1337 PDB bytes; 311 instructions
// Evidence: _assembly/roof-v1/C/pack-000/rva_6bb650_len_539.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 268:268: system_roof::delete_roof::delete_roofs
// Frames: F1
// 0x1406bb650  4157                           pushq	%r15
// 0x1406bb652  4156                           pushq	%r14
// 0x1406bb654  4155                           pushq	%r13
// 0x1406bb656  4154                           pushq	%r12
// 0x1406bb658  56                             pushq	%rsi
// 0x1406bb659  57                             pushq	%rdi
// 0x1406bb65a  55                             pushq	%rbp
// 0x1406bb65b  53                             pushq	%rbx
// 0x1406bb65c  4881ecf8000000                 subq	$0xf8, %rsp
// 0x1406bb663  0f29bc24e0000000               movaps	%xmm7, 0xe0(%rsp)
// 0x1406bb66b  0f29b424d0000000               movaps	%xmm6, 0xd0(%rsp)
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7
// 0x1406bb673  4c8b19                         movq	(%rcx), %r11
// 0x1406bb676  488b6908                       movq	0x8(%rcx), %rbp
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F15 -> F16 -> F6 -> F7
// 0x1406bb67a  4d8ba3f8000000                 movq	0xf8(%r11), %r12
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F6 -> F7
// 0x1406bb681  498b8300010000                 movq	0x100(%r11), %rax
// Frames: F17 -> F18 -> F19 -> F14 -> F15 -> F16 -> F6 -> F7
// 0x1406bb688  498d3c84                       leaq	(%r12,%rax,4), %rdi
// Frames: F20 -> F15 -> F16 -> F6 -> F7
// 0x1406bb68c  450fb6ab08010000               movzbl	0x108(%r11), %r13d
// 0x1406bb694  418b401c                       movl	0x1c(%r8), %eax
// 0x1406bb698  8944243c                       movl	%eax, 0x3c(%rsp)
// 0x1406bb69c  498b00                         movq	(%r8), %rax
// 0x1406bb69f  498b4810                       movq	0x10(%r8), %rcx
// 0x1406bb6a3  48894c2478                     movq	%rcx, 0x78(%rsp)
// 0x1406bb6a8  488b4a18                       movq	0x18(%rdx), %rcx
// 0x1406bb6ac  48894c2470                     movq	%rcx, 0x70(%rsp)
// 0x1406bb6b1  488b4a10                       movq	0x10(%rdx), %rcx
// 0x1406bb6b5  48894c2468                     movq	%rcx, 0x68(%rsp)
// 0x1406bb6ba  0f1032                         movups	(%rdx), %xmm6
// 0x1406bb6bd  4889442450                     movq	%rax, 0x50(%rsp)
// 0x1406bb6c2  4883c020                       addq	$0x20, %rax
// 0x1406bb6c6  4889442460                     movq	%rax, 0x60(%rsp)
// 0x1406bb6cb  4531ff                         xorl	%r15d, %r15d
// 0x1406bb6ce  b808000000                     movl	$0x8, %eax
// 0x1406bb6d3  4889442448                     movq	%rax, 0x48(%rsp)
// 0x1406bb6d8  0f57ff                         xorps	%xmm7, %xmm7
// 0x1406bb6db  4889442440                     movq	%rax, 0x40(%rsp)
// 0x1406bb6e0  48c744243000000000             movq	$0x0, 0x30(%rsp)
// 0x1406bb6e9  48c744242800000000             movq	$0x0, 0x28(%rsp)
// 0x1406bb6f2  4531c9                         xorl	%r9d, %r9d
// 0x1406bb6f5  4531f6                         xorl	%r14d, %r14d
// 0x1406bb6f8  31f6                           xorl	%esi, %esi
// 0x1406bb6fa  44886c2427                     movb	%r13b, 0x27(%rsp)
// 0x1406bb6ff  4c895c2458                     movq	%r11, 0x58(%rsp)
// Frames: F21 -> F22 -> F7
// 0x1406bb704  4584ed                         testb	%r13b, %r13b
// 0x1406bb707  0f8483000000                   je	0x1406bb790 <.text+0x6ba790>
// Frames: F23 -> F22 -> F7
// 0x1406bb70d  4439f6                         cmpl	%r14d, %esi
// 0x1406bb710  0f851a010000                   jne	0x1406bb830 <.text+0x6ba830>
// 0x1406bb716  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F24 -> F25 -> F26 -> F22 -> F7
// 0x1406bb720  4939fc                         cmpq	%rdi, %r12
// Frames: F25 -> F26 -> F22 -> F7
// 0x1406bb723  0f843c040000                   je	0x1406bbb65 <.text+0x6bab65>
// Frames: F26 -> F22 -> F7
// 0x1406bb729  418b0c24                       movl	(%r12), %ecx
// Frames: F27 -> F28 -> F26 -> F22 -> F7
// 0x1406bb72d  4983c404                       addq	$0x4, %r12
// Frames: F29 -> F30 -> F31 -> F32 -> F22 -> F7
// 0x1406bb731  488b85a0010000                 movq	0x1a0(%rbp), %rax
// 0x1406bb738  488d0cc9                       leaq	(%rcx,%rcx,8), %rcx
// Frames: F33 -> F34 -> F35 -> F36 -> F22 -> F7
// 0x1406bb73c  4c8b74c810                     movq	0x10(%rax,%rcx,8), %r14
// Frames: F36 -> F22 -> F7
// 0x1406bb741  4d85f6                         testq	%r14, %r14
// 0x1406bb744  74da                           je	0x1406bb720 <.text+0x6ba720>
// Frames: F37 -> F38 -> F22 -> F7
// 0x1406bb746  488d14c8                       leaq	(%rax,%rcx,8), %rdx
// 0x1406bb74a  498b8310010000                 movq	0x110(%r11), %rax
// Frames: F39 -> F40 -> F41 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bb751  4c8b4240                       movq	0x40(%rdx), %r8
// Frames: F29 -> F30 -> F43 -> F39 -> F40 -> F41 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bb755  4c39c0                         cmpq	%r8, %rax
// 0x1406bb758  0f8348020000                   jae	0x1406bb9a6 <.text+0x6ba9a6>
// Frames: F39 -> F40 -> F41 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bb75e  488b4a38                       movq	0x38(%rdx), %rcx
// Frames: F44 -> F45 -> F46 -> F43 -> F39 -> F40 -> F41 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bb762  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bb766  4885c0                         testq	%rax, %rax
// 0x1406bb769  0f8437020000                   je	0x1406bb9a6 <.text+0x6ba9a6>
// Frames: F39 -> F40 -> F41 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bb76f  488b4a18                       movq	0x18(%rdx), %rcx
// Frames: F47 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bb773  48f7d0                         notq	%rax
// Frames: F50 -> F51 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bb776  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1406bb77a  48c1e004                       shlq	$0x4, %rax
// Frames: F52 -> F53 -> F54 -> F49 -> F55 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bb77e  488b440110                     movq	0x10(%rcx,%rax), %rax
// 0x1406bb783  e920020000                     jmp	0x1406bb9a8 <.text+0x6ba9a8>
// 0x1406bb788  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F56 -> F22 -> F7
// 0x1406bb790  4439f6                         cmpl	%r14d, %esi
// 0x1406bb793  0f85a7000000                   jne	0x1406bb840 <.text+0x6ba840>
// 0x1406bb799  0f1f8000000000                 nopl	(%rax)
// Frames: F24 -> F25 -> F57 -> F22 -> F7
// 0x1406bb7a0  4939fc                         cmpq	%rdi, %r12
// Frames: F25 -> F57 -> F22 -> F7
// 0x1406bb7a3  0f84bc030000                   je	0x1406bbb65 <.text+0x6bab65>
// Frames: F57 -> F22 -> F7
// 0x1406bb7a9  418b0c24                       movl	(%r12), %ecx
// Frames: F27 -> F28 -> F57 -> F22 -> F7
// 0x1406bb7ad  4983c404                       addq	$0x4, %r12
// Frames: F29 -> F30 -> F58 -> F59 -> F22 -> F7
// 0x1406bb7b1  488b8500010000                 movq	0x100(%rbp), %rax
// 0x1406bb7b8  488d0c89                       leaq	(%rcx,%rcx,4), %rcx
// 0x1406bb7bc  48c1e105                       shlq	$0x5, %rcx
// Frames: F33 -> F34 -> F60 -> F61 -> F22 -> F7
// 0x1406bb7c0  4c8b740858                     movq	0x58(%rax,%rcx), %r14
// Frames: F61 -> F22 -> F7
// 0x1406bb7c5  4d85f6                         testq	%r14, %r14
// 0x1406bb7c8  74d6                           je	0x1406bb7a0 <.text+0x6ba7a0>
// Frames: F62 -> F63 -> F22 -> F7
// 0x1406bb7ca  4801c8                         addq	%rcx, %rax
// 0x1406bb7cd  8b8894000000                   movl	0x94(%rax), %ecx
// Frames: F29 -> F30 -> F64 -> F63 -> F22 -> F7
// 0x1406bb7d3  4c8b85a0010000                 movq	0x1a0(%rbp), %r8
// 0x1406bb7da  4c8d0cc9                       leaq	(%rcx,%rcx,8), %r9
// 0x1406bb7de  4b8d14c8                       leaq	(%r8,%r9,8), %rdx
// Frames: F65 -> F66 -> F22 -> F7
// 0x1406bb7e2  498b8b10010000                 movq	0x110(%r11), %rcx
// Frames: F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bb7e9  4f8b44c840                     movq	0x40(%r8,%r9,8), %r8
// Frames: F29 -> F30 -> F43 -> F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bb7ee  4c39c1                         cmpq	%r8, %rcx
// 0x1406bb7f1  0f83f5010000                   jae	0x1406bb9ec <.text+0x6ba9ec>
// Frames: F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bb7f7  4c8b4a38                       movq	0x38(%rdx), %r9
// Frames: F44 -> F45 -> F46 -> F43 -> F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bb7fb  498b0cc9                       movq	(%r9,%rcx,8), %rcx
// 0x1406bb7ff  4885c9                         testq	%rcx, %rcx
// 0x1406bb802  0f84e4010000                   je	0x1406bb9ec <.text+0x6ba9ec>
// Frames: F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bb808  4c8b4a18                       movq	0x18(%rdx), %r9
// Frames: F47 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bb80c  48f7d1                         notq	%rcx
// Frames: F50 -> F51 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bb80f  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1406bb813  48c1e104                       shlq	$0x4, %rcx
// Frames: F52 -> F53 -> F54 -> F49 -> F55 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bb817  498b4c0910                     movq	0x10(%r9,%rcx), %rcx
// 0x1406bb81c  48894c2428                     movq	%rcx, 0x28(%rsp)
// 0x1406bb821  e9cf010000                     jmp	0x1406bb9f5 <.text+0x6ba9f5>
// 0x1406bb826  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F68 -> F22 -> F7
// 0x1406bb830  89f2                           movl	%esi, %edx
// 0x1406bb832  488b442448                     movq	0x48(%rsp), %rax
// Frames: F69 -> F22 -> F7
// 0x1406bb837  488b1cd0                       movq	(%rax,%rdx,8), %rbx
// Frames: F17 -> F70 -> F71 -> F72 -> F73 -> F74 -> F69 -> F22 -> F7
// 0x1406bb83b  eb18                           jmp	0x1406bb855 <.text+0x6ba855>
// 0x1406bb83d  0f1f00                         nopl	(%rax)
// Frames: F75 -> F22 -> F7
// 0x1406bb840  89f0                           movl	%esi, %eax
// Frames: F50 -> F76 -> F75 -> F22 -> F7
// 0x1406bb842  48c1e004                       shlq	$0x4, %rax
// 0x1406bb846  488b4c2440                     movq	0x40(%rsp), %rcx
// Frames: F77 -> F78 -> F22 -> F7
// 0x1406bb84b  488b1c01                       movq	(%rcx,%rax), %rbx
// Frames: F79 -> F80 -> F22 -> F7
// 0x1406bb84f  8b540108                       movl	0x8(%rcx,%rax), %edx
// Frames: F81 -> F82 -> F71 -> F72 -> F73 -> F74 -> F83 -> F22 -> F7
// 0x1406bb853  f7d2                           notl	%edx
// Frames: F17 -> F70 -> F71 -> F72 -> F73 -> F74 -> F84 -> F22 -> F7
// 0x1406bb855  486bc258                       imulq	$0x58, %rdx, %rax
// 0x1406bb859  4803442428                     addq	0x28(%rsp), %rax
// 0x1406bb85e  b101                           movb	$0x1, %cl
// Frames: F85 -> F86 -> F74 -> F84 -> F22 -> F7
// 0x1406bb860  f644243001                     testb	$0x1, 0x30(%rsp)
// 0x1406bb865  7409                           je	0x1406bb870 <.text+0x6ba870>
// 0x1406bb867  4d89f8                         movq	%r15, %r8
// 0x1406bb86a  eb09                           jmp	0x1406bb875 <.text+0x6ba875>
// 0x1406bb86c  0f1f4000                       nopl	(%rax)
// 0x1406bb870  31c9                           xorl	%ecx, %ecx
// 0x1406bb872  4531c0                         xorl	%r8d, %r8d
// Frames: F17 -> F70 -> F71 -> F72 -> F73 -> F74 -> F84 -> F22 -> F7
// 0x1406bb875  498d14d1                       leaq	(%r9,%rdx,8), %rdx
// 0x1406bb879  48894c2430                     movq	%rcx, 0x30(%rsp)
// Frames: F7
// 0x1406bb87e  85db                           testl	%ebx, %ebx
// 0x1406bb880  0f84df020000                   je	0x1406bbb65 <.text+0x6bab65>
// Frames: F87 -> F88
// 0x1406bb886  ffc6                           incl	%esi
// 0x1406bb888  4d85c0                         testq	%r8, %r8
// Frames: F88
// 0x1406bb88b  0f8473feffff                   je	0x1406bb704 <.text+0x6ba704>
// 0x1406bb891  f30f1002                       movss	(%rdx), %xmm0
// 0x1406bb895  0f2ec7                         ucomiss	%xmm7, %xmm0
// 0x1406bb898  0f8566feffff                   jne	0x1406bb704 <.text+0x6ba704>
// 0x1406bb89e  0f8a60feffff                   jp	0x1406bb704 <.text+0x6ba704>
// Frames: F89 -> F90 -> F91
// 0x1406bb8a4  4989dd                         movq	%rbx, %r13
// 0x1406bb8a7  4c898c2480000000               movq	%r9, 0x80(%rsp)
// 0x1406bb8af  8b4c243c                       movl	0x3c(%rsp), %ecx
// 0x1406bb8b3  488b542478                     movq	0x78(%rsp), %rdx
// 0x1406bb8b8  890a                           movl	%ecx, (%rdx)
// Frames: F91
// 0x1406bb8ba  488b00                         movq	(%rax), %rax
// Frames: F92 -> F91
// 0x1406bb8bd  48c78424c000000001000000       movq	$0x1, 0xc0(%rsp)
// 0x1406bb8c9  48898424c8000000               movq	%rax, 0xc8(%rsp)
// 0x1406bb8d1  488b4c2460                     movq	0x60(%rsp), %rcx
// 0x1406bb8d6  488d9c24c0000000               leaq	0xc0(%rsp), %rbx
// Frames: F93 -> F94 -> F95 -> F96 -> F92 -> F91
// 0x1406bb8de  4889da                         movq	%rbx, %rdx
// 0x1406bb8e1  e87a009000                     callq	0x140fbb960 <.text+0xfba960>
// Frames: F97 -> F95 -> F96 -> F92 -> F91
// 0x1406bb8e6  488d8c2488000000               leaq	0x88(%rsp), %rcx
// 0x1406bb8ee  488b542450                     movq	0x50(%rsp), %rdx
// 0x1406bb8f3  4989c0                         movq	%rax, %r8
// 0x1406bb8f6  4989d9                         movq	%rbx, %r9
// 0x1406bb8f9  e8928b5e00                     callq	0x140ca4490 <.text+0xca3490>
// Frames: F95 -> F96 -> F92 -> F91
// 0x1406bb8fe  4883bc248800000004             cmpq	$0x4, 0x88(%rsp)
// 0x1406bb907  b802000000                     movl	$0x2, %eax
// 0x1406bb90c  7407                           je	0x1406bb915 <.text+0x6ba915>
// 0x1406bb90e  8b8424a0000000                 movl	0xa0(%rsp), %eax
// 0x1406bb915  488b8c2498000000               movq	0x98(%rsp), %rcx
// Frames: F92 -> F91
// 0x1406bb91d  48898c24b0000000               movq	%rcx, 0xb0(%rsp)
// 0x1406bb925  888424b8000000                 movb	%al, 0xb8(%rsp)
// 0x1406bb92c  4c89ea                         movq	%r13, %rdx
// 0x1406bb92f  741f                           je	0x1406bb950 <.text+0x6ba950>
// Frames: F98 -> F99 -> F100 -> F101 -> F101 -> F101 -> F92 -> F91
// 0x1406bb931  f0                             lock
// 0x1406bb932  48ff09                         decq	(%rcx)
// Frames: F100 -> F101 -> F101 -> F101 -> F92 -> F91
// 0x1406bb935  7510                           jne	0x1406bb947 <.text+0x6ba947>
// Frames: F102 -> F101 -> F101 -> F101 -> F92 -> F91
// 0x1406bb937  488d8c24b0000000               leaq	0xb0(%rsp), %rcx
// 0x1406bb93f  e8ac685d00                     callq	0x140c921f0 <.text+0xc911f0>
// 0x1406bb944  4c89ea                         movq	%r13, %rdx
// Frames: F103 -> F91
// 0x1406bb947  488b442450                     movq	0x50(%rsp), %rax
// 0x1406bb94c  c6403801                       movb	$0x1, 0x38(%rax)
// Frames: F104 -> F105
// 0x1406bb950  4889942488000000               movq	%rdx, 0x88(%rsp)
// 0x1406bb958  0f11b42490000000               movups	%xmm6, 0x90(%rsp)
// 0x1406bb960  488b442468                     movq	0x68(%rsp), %rax
// 0x1406bb965  48898424a0000000               movq	%rax, 0xa0(%rsp)
// 0x1406bb96d  488b442470                     movq	0x70(%rsp), %rax
// 0x1406bb972  48898424a8000000               movq	%rax, 0xa8(%rsp)
// Frames: F105
// 0x1406bb97a  488d8c2488000000               leaq	0x88(%rsp), %rcx
// 0x1406bb982  488d1537648c02                 leaq	0x28c6437(%rip), %rdx   # 0x142f81dc0
// 0x1406bb989  e812fd8300                     callq	0x140efb6a0 <.text+0xefa6a0>
// 0x1406bb98e  4c8b5c2458                     movq	0x58(%rsp), %r11
// 0x1406bb993  440fb66c2427                   movzbl	0x27(%rsp), %r13d
// 0x1406bb999  4c8b8c2480000000               movq	0x80(%rsp), %r9
// Frames: F88
// 0x1406bb9a1  e95efdffff                     jmp	0x1406bb704 <.text+0x6ba704>
// 0x1406bb9a6  31c0                           xorl	%eax, %eax
// Frames: F106 -> F107 -> F37 -> F38 -> F22 -> F7
// 0x1406bb9a8  4d8b9318010000                 movq	0x118(%r11), %r10
// 0x1406bb9af  4531c9                         xorl	%r9d, %r9d
// 0x1406bb9b2  31c9                           xorl	%ecx, %ecx
// Frames: F29 -> F30 -> F108 -> F109 -> F110 -> F111 -> F112 -> F107 -> F37 -> F38 -> F22 -> F7
// 0x1406bb9b4  4d39c2                         cmpq	%r8, %r10
// 0x1406bb9b7  0f83e2000000                   jae	0x1406bba9f <.text+0x6baa9f>
// Frames: F109 -> F110 -> F111 -> F112 -> F107 -> F37 -> F38 -> F22 -> F7
// 0x1406bb9bd  488b4a38                       movq	0x38(%rdx), %rcx
// Frames: F87 -> F45 -> F113 -> F108 -> F109 -> F110 -> F111 -> F112 -> F107 -> F37 -> F38 -> F22 -> F7
// 0x1406bb9c1  4a8b0cd1                       movq	(%rcx,%r10,8), %rcx
// 0x1406bb9c5  4885c9                         testq	%rcx, %rcx
// Frames: F114 -> F37 -> F38 -> F22 -> F7
// 0x1406bb9c8  0f84cc000000                   je	0x1406bba9a <.text+0x6baa9a>
// Frames: F39 -> F40 -> F41 -> F42 -> F115 -> F37 -> F38 -> F22 -> F7
// 0x1406bb9ce  4c8b4a18                       movq	0x18(%rdx), %r9
// Frames: F47 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F115 -> F37 -> F38 -> F22 -> F7
// 0x1406bb9d2  48f7d1                         notq	%rcx
// Frames: F50 -> F51 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F115 -> F37 -> F38 -> F22 -> F7
// 0x1406bb9d5  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1406bb9d9  48c1e104                       shlq	$0x4, %rcx
// Frames: F52 -> F53 -> F54 -> F49 -> F55 -> F42 -> F115 -> F37 -> F38 -> F22 -> F7
// 0x1406bb9dd  4d8b7c0910                     movq	0x10(%r9,%rcx), %r15
// 0x1406bb9e2  b101                           movb	$0x1, %cl
// 0x1406bb9e4  41b101                         movb	$0x1, %r9b
// 0x1406bb9e7  e9b3000000                     jmp	0x1406bba9f <.text+0x6baa9f>
// 0x1406bb9ec  48c744242800000000             movq	$0x0, 0x28(%rsp)
// Frames: F106 -> F116 -> F65 -> F66 -> F22 -> F7
// 0x1406bb9f5  4d8b9318010000                 movq	0x118(%r11), %r10
// 0x1406bb9fc  4531c9                         xorl	%r9d, %r9d
// 0x1406bb9ff  31c9                           xorl	%ecx, %ecx
// Frames: F29 -> F30 -> F108 -> F109 -> F117 -> F118 -> F112 -> F116 -> F65 -> F66 -> F22 -> F7
// 0x1406bba01  4c3b9088000000                 cmpq	0x88(%rax), %r10
// 0x1406bba08  7356                           jae	0x1406bba60 <.text+0x6baa60>
// Frames: F109 -> F117 -> F118 -> F112 -> F116 -> F65 -> F66 -> F22 -> F7
// 0x1406bba0a  488b8880000000                 movq	0x80(%rax), %rcx
// Frames: F87 -> F45 -> F113 -> F108 -> F109 -> F117 -> F118 -> F112 -> F116 -> F65 -> F66 -> F22 -> F7
// 0x1406bba11  4a833cd100                     cmpq	$0x0, (%rcx,%r10,8)
// Frames: F119 -> F65 -> F66 -> F22 -> F7
// 0x1406bba16  0f84fd000000                   je	0x1406bbb19 <.text+0x6bab19>
// Frames: F29 -> F30 -> F43 -> F39 -> F40 -> F41 -> F42 -> F67 -> F120 -> F65 -> F66 -> F22 -> F7
// 0x1406bba1c  4d39c2                         cmpq	%r8, %r10
// 0x1406bba1f  7337                           jae	0x1406bba58 <.text+0x6baa58>
// Frames: F39 -> F40 -> F41 -> F42 -> F67 -> F120 -> F65 -> F66 -> F22 -> F7
// 0x1406bba21  488b4a38                       movq	0x38(%rdx), %rcx
// Frames: F44 -> F45 -> F46 -> F43 -> F39 -> F40 -> F41 -> F42 -> F67 -> F120 -> F65 -> F66 -> F22 -> F7
// 0x1406bba25  4a8b0cd1                       movq	(%rcx,%r10,8), %rcx
// 0x1406bba29  4885c9                         testq	%rcx, %rcx
// 0x1406bba2c  742a                           je	0x1406bba58 <.text+0x6baa58>
// Frames: F39 -> F40 -> F41 -> F42 -> F67 -> F120 -> F65 -> F66 -> F22 -> F7
// 0x1406bba2e  4c8b4a18                       movq	0x18(%rdx), %r9
// Frames: F47 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F67 -> F120 -> F65 -> F66 -> F22 -> F7
// 0x1406bba32  48f7d1                         notq	%rcx
// Frames: F50 -> F51 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F67 -> F120 -> F65 -> F66 -> F22 -> F7
// 0x1406bba35  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1406bba39  48c1e104                       shlq	$0x4, %rcx
// Frames: F52 -> F53 -> F54 -> F49 -> F55 -> F42 -> F67 -> F120 -> F65 -> F66 -> F22 -> F7
// 0x1406bba3d  4d8b7c0910                     movq	0x10(%r9,%rcx), %r15
// 0x1406bba42  b101                           movb	$0x1, %cl
// 0x1406bba44  41b101                         movb	$0x1, %r9b
// Frames: F65 -> F66 -> F22 -> F7
// 0x1406bba47  4d8b9320010000                 movq	0x120(%r11), %r10
// Frames: F29 -> F30 -> F43 -> F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bba4e  4d39c2                         cmpq	%r8, %r10
// 0x1406bba51  721d                           jb	0x1406bba70 <.text+0x6baa70>
// 0x1406bba53  e9d6000000                     jmp	0x1406bbb2e <.text+0x6bab2e>
// 0x1406bba58  b101                           movb	$0x1, %cl
// 0x1406bba5a  41b101                         movb	$0x1, %r9b
// 0x1406bba5d  4531ff                         xorl	%r15d, %r15d
// Frames: F65 -> F66 -> F22 -> F7
// 0x1406bba60  4d8b9320010000                 movq	0x120(%r11), %r10
// Frames: F29 -> F30 -> F43 -> F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bba67  4d39c2                         cmpq	%r8, %r10
// 0x1406bba6a  0f83be000000                   jae	0x1406bbb2e <.text+0x6bab2e>
// Frames: F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bba70  4c8b4238                       movq	0x38(%rdx), %r8
// Frames: F44 -> F45 -> F46 -> F43 -> F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bba74  4f8b04d0                       movq	(%r8,%r10,8), %r8
// 0x1406bba78  4d85c0                         testq	%r8, %r8
// 0x1406bba7b  0f84ad000000                   je	0x1406bbb2e <.text+0x6bab2e>
// Frames: F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bba81  488b5218                       movq	0x18(%rdx), %rdx
// Frames: F47 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bba85  49f7d0                         notq	%r8
// Frames: F50 -> F51 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bba88  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x1406bba8c  49c1e004                       shlq	$0x4, %r8
// Frames: F52 -> F53 -> F54 -> F49 -> F55 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bba90  4e8b540210                     movq	0x10(%rdx,%r8), %r10
// 0x1406bba95  e997000000                     jmp	0x1406bbb31 <.text+0x6bab31>
// 0x1406bba9a  31c9                           xorl	%ecx, %ecx
// 0x1406bba9c  4531c9                         xorl	%r9d, %r9d
// Frames: F37 -> F38 -> F22 -> F7
// 0x1406bba9f  4c897c2430                     movq	%r15, 0x30(%rsp)
// 0x1406bbaa4  4d8b9320010000                 movq	0x120(%r11), %r10
// 0x1406bbaab  4589ef                         movl	%r13d, %r15d
// Frames: F29 -> F30 -> F43 -> F39 -> F40 -> F41 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bbaae  4d39c2                         cmpq	%r8, %r10
// 0x1406bbab1  7323                           jae	0x1406bbad6 <.text+0x6baad6>
// Frames: F39 -> F40 -> F41 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bbab3  4c8b4238                       movq	0x38(%rdx), %r8
// Frames: F44 -> F45 -> F46 -> F43 -> F39 -> F40 -> F41 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bbab7  4f8b04d0                       movq	(%r8,%r10,8), %r8
// 0x1406bbabb  4d85c0                         testq	%r8, %r8
// 0x1406bbabe  7416                           je	0x1406bbad6 <.text+0x6baad6>
// Frames: F39 -> F40 -> F41 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bbac0  4c8b5218                       movq	0x18(%rdx), %r10
// Frames: F47 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bbac4  49f7d0                         notq	%r8
// Frames: F50 -> F51 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bbac7  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x1406bbacb  49c1e004                       shlq	$0x4, %r8
// Frames: F52 -> F53 -> F54 -> F49 -> F55 -> F42 -> F37 -> F38 -> F22 -> F7
// 0x1406bbacf  4f8b6c0210                     movq	0x10(%r10,%r8), %r13
// 0x1406bbad4  eb03                           jmp	0x1406bbad9 <.text+0x6baad9>
// 0x1406bbad6  4531ed                         xorl	%r13d, %r13d
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F121 -> F122 -> F22 -> F7
// 0x1406bbad9  488b5208                       movq	0x8(%rdx), %rdx
// 0x1406bbadd  4889542448                     movq	%rdx, 0x48(%rsp)
// Frames: F69 -> F22 -> F7
// 0x1406bbae2  488b1a                         movq	(%rdx), %rbx
// 0x1406bbae5  31d2                           xorl	%edx, %edx
// 0x1406bbae7  41ba00000000                   movl	$0x0, %r10d
// 0x1406bbaed  4889442428                     movq	%rax, 0x28(%rsp)
// 0x1406bbaf2  be00000000                     movl	$0x0, %esi
// 0x1406bbaf7  41b800000000                   movl	$0x0, %r8d
// Frames: F85 -> F123 -> F74 -> F69 -> F22 -> F7
// 0x1406bbafd  4584c9                         testb	%r9b, %r9b
// 0x1406bbb00  4d89e9                         movq	%r13, %r9
// 0x1406bbb03  4589fd                         movl	%r15d, %r13d
// 0x1406bbb06  4c8b7c2430                     movq	0x30(%rsp), %r15
// 0x1406bbb0b  0f8464fdffff                   je	0x1406bb875 <.text+0x6ba875>
// 0x1406bbb11  4489d6                         movl	%r10d, %esi
// 0x1406bbb14  e94efdffff                     jmp	0x1406bb867 <.text+0x6ba867>
// Frames: F65 -> F66 -> F22 -> F7
// 0x1406bbb19  31c9                           xorl	%ecx, %ecx
// 0x1406bbb1b  4531c9                         xorl	%r9d, %r9d
// 0x1406bbb1e  4d8b9320010000                 movq	0x120(%r11), %r10
// Frames: F29 -> F30 -> F43 -> F39 -> F40 -> F41 -> F42 -> F67 -> F65 -> F66 -> F22 -> F7
// 0x1406bbb25  4d39c2                         cmpq	%r8, %r10
// 0x1406bbb28  0f8242ffffff                   jb	0x1406bba70 <.text+0x6baa70>
// 0x1406bbb2e  4531d2                         xorl	%r10d, %r10d
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F124 -> F125 -> F22 -> F7
// 0x1406bbb31  488b4050                       movq	0x50(%rax), %rax
// Frames: F77 -> F78 -> F22 -> F7
// 0x1406bbb35  488b18                         movq	(%rax), %rbx
// 0x1406bbb38  4889442440                     movq	%rax, 0x40(%rsp)
// Frames: F79 -> F80 -> F22 -> F7
// 0x1406bbb3d  8b5008                         movl	0x8(%rax), %edx
// Frames: F81 -> F82 -> F71 -> F72 -> F73 -> F74 -> F83 -> F22 -> F7
// 0x1406bbb40  f7d2                           notl	%edx
// Frames: F17 -> F126 -> F71 -> F72 -> F73 -> F74 -> F83 -> F22 -> F7
// 0x1406bbb42  486bc258                       imulq	$0x58, %rdx, %rax
// 0x1406bbb46  4803442428                     addq	0x28(%rsp), %rax
// 0x1406bbb4b  31f6                           xorl	%esi, %esi
// Frames: F85 -> F123 -> F74 -> F83 -> F22 -> F7
// 0x1406bbb4d  4584c9                         testb	%r9b, %r9b
// 0x1406bbb50  7408                           je	0x1406bbb5a <.text+0x6bab5a>
// 0x1406bbb52  4d89d1                         movq	%r10, %r9
// 0x1406bbb55  e90dfdffff                     jmp	0x1406bb867 <.text+0x6ba867>
// 0x1406bbb5a  4531c0                         xorl	%r8d, %r8d
// 0x1406bbb5d  4d89d1                         movq	%r10, %r9
// 0x1406bbb60  e910fdffff                     jmp	0x1406bb875 <.text+0x6ba875>
// Frames: F127
// 0x1406bbb65  0f28b424d0000000               movaps	0xd0(%rsp), %xmm6
// 0x1406bbb6d  0f28bc24e0000000               movaps	0xe0(%rsp), %xmm7
// 0x1406bbb75  4881c4f8000000                 addq	$0xf8, %rsp
// 0x1406bbb7c  5b                             popq	%rbx
// 0x1406bbb7d  5d                             popq	%rbp
// 0x1406bbb7e  5f                             popq	%rdi
// 0x1406bbb7f  5e                             popq	%rsi
// 0x1406bbb80  415c                           popq	%r12
// 0x1406bbb82  415d                           popq	%r13
// 0x1406bbb84  415e                           popq	%r14
// 0x1406bbb86  415f                           popq	%r15
// 0x1406bbb88  c3                             retq
//
// RANGE rva_6bbb90_len_4ca: RVA 0x6bbb90, 1226 PDB bytes; 322 instructions
// Evidence: _assembly/roof-v1/C/pack-000/rva_6bbb90_len_4ca.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 268:7168: system_roof::delete_roof::mark_roof_for_deletion
// Frames: F128
// 0x1406bbb90  55                             pushq	%rbp
// 0x1406bbb91  4157                           pushq	%r15
// 0x1406bbb93  4156                           pushq	%r14
// 0x1406bbb95  4155                           pushq	%r13
// 0x1406bbb97  4154                           pushq	%r12
// 0x1406bbb99  56                             pushq	%rsi
// 0x1406bbb9a  57                             pushq	%rdi
// 0x1406bbb9b  53                             pushq	%rbx
// 0x1406bbb9c  4881ecc8000000                 subq	$0xc8, %rsp
// 0x1406bbba3  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x1406bbbab  48c74540feffffff               movq	$-0x2, 0x40(%rbp)
// 0x1406bbbb3  4c894d38                       movq	%r9, 0x38(%rbp)
// Frames: F129 -> F130
// 0x1406bbbb7  48c7451000000000               movq	$0x0, 0x10(%rbp)
// 0x1406bbbbf  48c7451808000000               movq	$0x8, 0x18(%rbp)
// 0x1406bbbc7  48c7452000000000               movq	$0x0, 0x20(%rbp)
// Frames: F131 -> F132 -> F133
// 0x1406bbbcf  4c8b39                         movq	(%rcx), %r15
// Frames: F134 -> F132 -> F133
// 0x1406bbbd2  4c8b4908                       movq	0x8(%rcx), %r9
// Frames: F135 -> F136 -> F137 -> F132 -> F133
// 0x1406bbbd6  4d8b17                         movq	(%r15), %r10
// 0x1406bbbd9  4531e4                         xorl	%r12d, %r12d
// Frames: F138 -> F139 -> F136 -> F137 -> F132 -> F133
// 0x1406bbbdc  4c89d1                         movq	%r10, %rcx
// 0x1406bbbdf  492b4918                       subq	0x18(%r9), %rcx
// 0x1406bbbe3  490f42cc                       cmovbq	%r12, %rcx
// Frames: F138 -> F140 -> F136 -> F137 -> F132 -> F133
// 0x1406bbbe7  4d2b5138                       subq	0x38(%r9), %r10
// 0x1406bbbeb  4d0f42d4                       cmovbq	%r12, %r10
// Frames: F141 -> F142 -> F143 -> F30 -> F144 -> F136 -> F137 -> F132 -> F133
// 0x1406bbbef  488d0449                       leaq	(%rcx,%rcx,2), %rax
// 0x1406bbbf3  48c1e003                       shlq	$0x3, %rax
// 0x1406bbbf7  49034108                       addq	0x8(%r9), %rax
// Frames: F12 -> F13 -> F144 -> F136 -> F137 -> F132 -> F133
// 0x1406bbbfb  4d8b5910                       movq	0x10(%r9), %r11
// Frames: F141 -> F142 -> F143 -> F30 -> F144 -> F136 -> F137 -> F132 -> F133
// 0x1406bbbff  4929cb                         subq	%rcx, %r11
// 0x1406bbc02  4d0f42dc                       cmovbq	%r12, %r11
// 0x1406bbc06  4c894508                       movq	%r8, 0x8(%rbp)
// 0x1406bbc0a  41b808000000                   movl	$0x8, %r8d
// 0x1406bbc10  490f42c0                       cmovbq	%r8, %rax
// Frames: F141 -> F142 -> F143 -> F30 -> F145 -> F136 -> F137 -> F132 -> F133
// 0x1406bbc14  4b8d0c52                       leaq	(%r10,%r10,2), %rcx
// 0x1406bbc18  48c1e103                       shlq	$0x3, %rcx
// 0x1406bbc1c  49034928                       addq	0x28(%r9), %rcx
// Frames: F12 -> F13 -> F145 -> F136 -> F137 -> F132 -> F133
// 0x1406bbc20  498b7130                       movq	0x30(%r9), %rsi
// Frames: F141 -> F142 -> F143 -> F30 -> F145 -> F136 -> F137 -> F132 -> F133
// 0x1406bbc24  4c29d6                         subq	%r10, %rsi
// 0x1406bbc27  490f42c8                       cmovbq	%r8, %rcx
// 0x1406bbc2b  48895530                       movq	%rdx, 0x30(%rbp)
// 0x1406bbc2f  490f42f4                       cmovbq	%r12, %rsi
// Frames: F146 -> F136 -> F137 -> F132 -> F133
// 0x1406bbc33  498b5140                       movq	0x40(%r9), %rdx
// 0x1406bbc37  4829f2                         subq	%rsi, %rdx
// 0x1406bbc3a  4c29da                         subq	%r11, %rdx
// 0x1406bbc3d  498917                         movq	%rdx, (%r15)
// Frames: F17 -> F18 -> F147 -> F148 -> F136 -> F137 -> F132 -> F133
// 0x1406bbc40  4b8d145b                       leaq	(%r11,%r11,2), %rdx
// 0x1406bbc44  4c8d2cd0                       leaq	(%rax,%rdx,8), %r13
// 0x1406bbc48  488d1476                       leaq	(%rsi,%rsi,2), %rdx
// 0x1406bbc4c  488d1cd1                       leaq	(%rcx,%rdx,8), %rbx
// 0x1406bbc50  eb2d                           jmp	0x1406bbc7f <.text+0x6bac7f>
// 0x1406bbc52  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// Frames: F149 -> F150 -> F151
// 0x1406bbc60  488d4d10                       leaq	0x10(%rbp), %rcx
// 0x1406bbc64  e897376b02                     callq	0x142d6f400 <.text+0x2d6e400>
// 0x1406bbc69  90                             nop
// Frames: F8 -> F9 -> F10 -> F152 -> F153 -> F150 -> F151
// 0x1406bbc6a  488b4518                       movq	0x18(%rbp), %rax
// Frames: F154 -> F155 -> F150 -> F151
// 0x1406bbc6e  4a8934e0                       movq	%rsi, (%rax,%r12,8)
// Frames: F156 -> F150 -> F151
// 0x1406bbc72  49ffc4                         incq	%r12
// 0x1406bbc75  4c896520                       movq	%r12, 0x20(%rbp)
// 0x1406bbc79  4889f9                         movq	%rdi, %rcx
// 0x1406bbc7c  4c89f0                         movq	%r14, %rax
// Frames: F157 -> F158 -> F159 -> F160 -> F161 -> F133
// 0x1406bbc7f  4885c0                         testq	%rax, %rax
// 0x1406bbc82  741c                           je	0x1406bbca0 <.text+0x6baca0>
// 0x1406bbc84  4c39e8                         cmpq	%r13, %rax
// 0x1406bbc87  7417                           je	0x1406bbca0 <.text+0x6baca0>
// Frames: F162 -> F161 -> F133
// 0x1406bbc89  4c8d7018                       leaq	0x18(%rax), %r14
// 0x1406bbc8d  4889cf                         movq	%rcx, %rdi
// 0x1406bbc90  49ff07                         incq	(%r15)
// Frames: F151
// 0x1406bbc93  488b7008                       movq	0x8(%rax), %rsi
// Frames: F163 -> F150 -> F151
// 0x1406bbc97  4c3b6510                       cmpq	0x10(%rbp), %r12
// 0x1406bbc9b  75cd                           jne	0x1406bbc6a <.text+0x6bac6a>
// 0x1406bbc9d  ebc1                           jmp	0x1406bbc60 <.text+0x6bac60>
// 0x1406bbc9f  90                             nop
// Frames: F157 -> F164 -> F165 -> F159 -> F160 -> F161 -> F133
// 0x1406bbca0  4885c9                         testq	%rcx, %rcx
// 0x1406bbca3  741e                           je	0x1406bbcc3 <.text+0x6bacc3>
// 0x1406bbca5  4839d9                         cmpq	%rbx, %rcx
// 0x1406bbca8  7419                           je	0x1406bbcc3 <.text+0x6bacc3>
// Frames: F27 -> F28 -> F164 -> F165 -> F159 -> F160 -> F161 -> F133
// 0x1406bbcaa  488d7918                       leaq	0x18(%rcx), %rdi
// 0x1406bbcae  4531f6                         xorl	%r14d, %r14d
// 0x1406bbcb1  4889c8                         movq	%rcx, %rax
// Frames: F162 -> F161 -> F133
// 0x1406bbcb4  49ff07                         incq	(%r15)
// Frames: F151
// 0x1406bbcb7  488b7008                       movq	0x8(%rax), %rsi
// Frames: F163 -> F150 -> F151
// 0x1406bbcbb  4c3b6510                       cmpq	0x10(%rbp), %r12
// 0x1406bbcbf  75a9                           jne	0x1406bbc6a <.text+0x6bac6a>
// 0x1406bbcc1  eb9d                           jmp	0x1406bbc60 <.text+0x6bac60>
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F166
// 0x1406bbcc3  4c8b4d30                       movq	0x30(%rbp), %r9
// 0x1406bbcc7  498b09                         movq	(%r9), %rcx
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F15 -> F16 -> F6 -> F166
// 0x1406bbcca  4c8ba9f8000000                 movq	0xf8(%rcx), %r13
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F6 -> F166
// 0x1406bbcd1  488b8100010000                 movq	0x100(%rcx), %rax
// 0x1406bbcd8  4c8b4508                       movq	0x8(%rbp), %r8
// 0x1406bbcdc  498b5008                       movq	0x8(%r8), %rdx
// 0x1406bbce0  48895528                       movq	%rdx, 0x28(%rbp)
// 0x1406bbce4  4885d2                         testq	%rdx, %rdx
// 0x1406bbce7  7508                           jne	0x1406bbcf1 <.text+0x6bacf1>
// 0x1406bbce9  498b5010                       movq	0x10(%r8), %rdx
// 0x1406bbced  48895528                       movq	%rdx, 0x28(%rbp)
// 0x1406bbcf1  498b5908                       movq	0x8(%r9), %rbx
// 0x1406bbcf5  4c8d248500000000               leaq	(,%rax,4), %r12
// 0x1406bbcfd  4d01ec                         addq	%r13, %r12
// 0x1406bbd00  48894df8                       movq	%rcx, -0x8(%rbp)
// 0x1406bbd04  0fb68108010000                 movzbl	0x108(%rcx), %eax
// 0x1406bbd0b  884508                         movb	%al, 0x8(%rbp)
// 0x1406bbd0e  488b4d38                       movq	0x38(%rbp), %rcx
// 0x1406bbd12  8b411c                         movl	0x1c(%rcx), %eax
// 0x1406bbd15  894504                         movl	%eax, 0x4(%rbp)
// 0x1406bbd18  488b01                         movq	(%rcx), %rax
// 0x1406bbd1b  488b4910                       movq	0x10(%rcx), %rcx
// 0x1406bbd1f  48894dd8                       movq	%rcx, -0x28(%rbp)
// 0x1406bbd23  48894530                       movq	%rax, 0x30(%rbp)
// 0x1406bbd27  4883c020                       addq	$0x20, %rax
// 0x1406bbd2b  488945e0                       movq	%rax, -0x20(%rbp)
// 0x1406bbd2f  31ff                           xorl	%edi, %edi
// 0x1406bbd31  b808000000                     movl	$0x8, %eax
// 0x1406bbd36  488945e8                       movq	%rax, -0x18(%rbp)
// 0x1406bbd3a  488945f0                       movq	%rax, -0x10(%rbp)
// 0x1406bbd3e  48c7453800000000               movq	$0x0, 0x38(%rbp)
// 0x1406bbd46  4531c9                         xorl	%r9d, %r9d
// 0x1406bbd49  eb3b                           jmp	0x1406bbd86 <.text+0x6bad86>
// 0x1406bbd4b  0f1f440000                     nopl	(%rax,%rax)
// Frames: F8 -> F9 -> F10 -> F152 -> F153 -> F150 -> F167 -> F168 -> F169 -> F170
// 0x1406bbd50  488b5530                       movq	0x30(%rbp), %rdx
// 0x1406bbd54  488b4228                       movq	0x28(%rdx), %rax
// Frames: F171 -> F153 -> F150 -> F167 -> F168 -> F169 -> F170
// 0x1406bbd58  488d0cb6                       leaq	(%rsi,%rsi,4), %rcx
// Frames: F154 -> F155 -> F150 -> F167 -> F168 -> F169 -> F170
// 0x1406bbd5c  4c8934c8                       movq	%r14, (%rax,%rcx,8)
// 0x1406bbd60  c644c80801                     movb	$0x1, 0x8(%rax,%rcx,8)
// 0x1406bbd65  0f1045b1                       movups	-0x4f(%rbp), %xmm0
// 0x1406bbd69  0f104dbc                       movups	-0x44(%rbp), %xmm1
// 0x1406bbd6d  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// 0x1406bbd72  0f1144c809                     movups	%xmm0, 0x9(%rax,%rcx,8)
// Frames: F156 -> F150 -> F167 -> F168 -> F169 -> F170
// 0x1406bbd77  48ffc6                         incq	%rsi
// 0x1406bbd7a  48897230                       movq	%rsi, 0x30(%rdx)
// Frames: F172 -> F168 -> F169 -> F170
// 0x1406bbd7e  48ff4240                       incq	0x40(%rdx)
// 0x1406bbd82  448b4d00                       movl	(%rbp), %r9d
// Frames: F21 -> F22 -> F166
// 0x1406bbd86  807d0800                       cmpb	$0x0, 0x8(%rbp)
// 0x1406bbd8a  0f8480000000                   je	0x1406bbe10 <.text+0x6bae10>
// Frames: F23 -> F22 -> F166
// 0x1406bbd90  4139f9                         cmpl	%edi, %r9d
// 0x1406bbd93  0f8515010000                   jne	0x1406bbeae <.text+0x6baeae>
// 0x1406bbd99  0f1f8000000000                 nopl	(%rax)
// Frames: F24 -> F25 -> F26 -> F22 -> F166
// 0x1406bbda0  4d39e5                         cmpq	%r12, %r13
// Frames: F25 -> F26 -> F22 -> F166
// 0x1406bbda3  0f842b020000                   je	0x1406bbfd4 <.text+0x6bafd4>
// Frames: F26 -> F22 -> F166
// 0x1406bbda9  418b4d00                       movl	(%r13), %ecx
// Frames: F27 -> F28 -> F26 -> F22 -> F166
// 0x1406bbdad  4983c504                       addq	$0x4, %r13
// Frames: F29 -> F30 -> F31 -> F32 -> F22 -> F166
// 0x1406bbdb1  488b83a0010000                 movq	0x1a0(%rbx), %rax
// 0x1406bbdb8  488d0cc9                       leaq	(%rcx,%rcx,8), %rcx
// Frames: F33 -> F34 -> F35 -> F36 -> F22 -> F166
// 0x1406bbdbc  488b7cc810                     movq	0x10(%rax,%rcx,8), %rdi
// Frames: F36 -> F22 -> F166
// 0x1406bbdc1  4885ff                         testq	%rdi, %rdi
// 0x1406bbdc4  74da                           je	0x1406bbda0 <.text+0x6bada0>
// Frames: F38 -> F22 -> F166
// 0x1406bbdc6  488d04c8                       leaq	(%rax,%rcx,8), %rax
// 0x1406bbdca  488b4df8                       movq	-0x8(%rbp), %rcx
// 0x1406bbdce  488b8910010000                 movq	0x110(%rcx), %rcx
// Frames: F29 -> F30 -> F43 -> F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F166
// 0x1406bbdd5  483b4840                       cmpq	0x40(%rax), %rcx
// 0x1406bbdd9  0f83bc000000                   jae	0x1406bbe9b <.text+0x6bae9b>
// Frames: F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F166
// 0x1406bbddf  488b5038                       movq	0x38(%rax), %rdx
// Frames: F44 -> F45 -> F46 -> F43 -> F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F166
// 0x1406bbde3  488b0cca                       movq	(%rdx,%rcx,8), %rcx
// 0x1406bbde7  4885c9                         testq	%rcx, %rcx
// 0x1406bbdea  0f84ab000000                   je	0x1406bbe9b <.text+0x6bae9b>
// Frames: F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F166
// 0x1406bbdf0  488b5018                       movq	0x18(%rax), %rdx
// Frames: F47 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F166
// 0x1406bbdf4  48f7d1                         notq	%rcx
// Frames: F50 -> F51 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F166
// 0x1406bbdf7  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1406bbdfb  48c1e104                       shlq	$0x4, %rcx
// Frames: F52 -> F53 -> F54 -> F49 -> F55 -> F42 -> F173 -> F38 -> F22 -> F166
// 0x1406bbdff  488b4c0a10                     movq	0x10(%rdx,%rcx), %rcx
// 0x1406bbe04  48894d38                       movq	%rcx, 0x38(%rbp)
// 0x1406bbe08  e996000000                     jmp	0x1406bbea3 <.text+0x6baea3>
// 0x1406bbe0d  0f1f00                         nopl	(%rax)
// Frames: F56 -> F22 -> F166
// 0x1406bbe10  4139f9                         cmpl	%edi, %r9d
// 0x1406bbe13  0f85b5000000                   jne	0x1406bbece <.text+0x6baece>
// 0x1406bbe19  0f1f8000000000                 nopl	(%rax)
// Frames: F24 -> F25 -> F57 -> F22 -> F166
// 0x1406bbe20  4d39e5                         cmpq	%r12, %r13
// Frames: F25 -> F57 -> F22 -> F166
// 0x1406bbe23  0f84ab010000                   je	0x1406bbfd4 <.text+0x6bafd4>
// Frames: F57 -> F22 -> F166
// 0x1406bbe29  418b4d00                       movl	(%r13), %ecx
// Frames: F27 -> F28 -> F57 -> F22 -> F166
// 0x1406bbe2d  4983c504                       addq	$0x4, %r13
// Frames: F29 -> F30 -> F58 -> F59 -> F22 -> F166
// 0x1406bbe31  488b8300010000                 movq	0x100(%rbx), %rax
// 0x1406bbe38  488d0c89                       leaq	(%rcx,%rcx,4), %rcx
// 0x1406bbe3c  48c1e105                       shlq	$0x5, %rcx
// Frames: F33 -> F34 -> F60 -> F61 -> F22 -> F166
// 0x1406bbe40  488b7c0858                     movq	0x58(%rax,%rcx), %rdi
// Frames: F61 -> F22 -> F166
// 0x1406bbe45  4885ff                         testq	%rdi, %rdi
// 0x1406bbe48  74d6                           je	0x1406bbe20 <.text+0x6bae20>
// Frames: F62 -> F63 -> F22 -> F166
// 0x1406bbe4a  4801c8                         addq	%rcx, %rax
// 0x1406bbe4d  8b8894000000                   movl	0x94(%rax), %ecx
// Frames: F29 -> F30 -> F64 -> F63 -> F22 -> F166
// 0x1406bbe53  488b93a0010000                 movq	0x1a0(%rbx), %rdx
// 0x1406bbe5a  4c8d04c9                       leaq	(%rcx,%rcx,8), %r8
// 0x1406bbe5e  488b4df8                       movq	-0x8(%rbp), %rcx
// Frames: F66 -> F22 -> F166
// 0x1406bbe62  488b8910010000                 movq	0x110(%rcx), %rcx
// Frames: F29 -> F30 -> F43 -> F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F166
// 0x1406bbe69  4a3b4cc240                     cmpq	0x40(%rdx,%r8,8), %rcx
// 0x1406bbe6e  734b                           jae	0x1406bbebb <.text+0x6baebb>
// Frames: F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F166
// 0x1406bbe70  4a8d14c2                       leaq	(%rdx,%r8,8), %rdx
// 0x1406bbe74  4c8b4238                       movq	0x38(%rdx), %r8
// Frames: F44 -> F45 -> F46 -> F43 -> F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F166
// 0x1406bbe78  498b0cc8                       movq	(%r8,%rcx,8), %rcx
// 0x1406bbe7c  4885c9                         testq	%rcx, %rcx
// 0x1406bbe7f  743a                           je	0x1406bbebb <.text+0x6baebb>
// Frames: F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F166
// 0x1406bbe81  488b5218                       movq	0x18(%rdx), %rdx
// Frames: F47 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F166
// 0x1406bbe85  48f7d1                         notq	%rcx
// Frames: F50 -> F51 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F166
// 0x1406bbe88  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1406bbe8c  48c1e104                       shlq	$0x4, %rcx
// Frames: F52 -> F53 -> F54 -> F49 -> F55 -> F42 -> F67 -> F174 -> F66 -> F22 -> F166
// 0x1406bbe90  488b4c0a10                     movq	0x10(%rdx,%rcx), %rcx
// 0x1406bbe95  48894d38                       movq	%rcx, 0x38(%rbp)
// 0x1406bbe99  eb28                           jmp	0x1406bbec3 <.text+0x6baec3>
// 0x1406bbe9b  48c7453800000000               movq	$0x0, 0x38(%rbp)
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F121 -> F122 -> F22 -> F166
// 0x1406bbea3  488b4008                       movq	0x8(%rax), %rax
// 0x1406bbea7  488945e8                       movq	%rax, -0x18(%rbp)
// 0x1406bbeab  4531c9                         xorl	%r9d, %r9d
// Frames: F68 -> F22 -> F166
// 0x1406bbeae  4489c9                         movl	%r9d, %ecx
// 0x1406bbeb1  488b45e8                       movq	-0x18(%rbp), %rax
// Frames: F50 -> F76 -> F68 -> F22 -> F166
// 0x1406bbeb5  488d04c8                       leaq	(%rax,%rcx,8), %rax
// 0x1406bbeb9  eb28                           jmp	0x1406bbee3 <.text+0x6baee3>
// 0x1406bbebb  48c7453800000000               movq	$0x0, 0x38(%rbp)
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F124 -> F125 -> F22 -> F166
// 0x1406bbec3  488b4050                       movq	0x50(%rax), %rax
// 0x1406bbec7  488945f0                       movq	%rax, -0x10(%rbp)
// 0x1406bbecb  4531c9                         xorl	%r9d, %r9d
// Frames: F75 -> F22 -> F166
// 0x1406bbece  4489c9                         movl	%r9d, %ecx
// Frames: F50 -> F76 -> F75 -> F22 -> F166
// 0x1406bbed1  48c1e104                       shlq	$0x4, %rcx
// 0x1406bbed5  488b55f0                       movq	-0x10(%rbp), %rdx
// 0x1406bbed9  488d040a                       leaq	(%rdx,%rcx), %rax
// Frames: F79 -> F80 -> F22 -> F166
// 0x1406bbedd  8b4c0a08                       movl	0x8(%rdx,%rcx), %ecx
// Frames: F81 -> F82 -> F71 -> F72 -> F73 -> F175 -> F83 -> F22 -> F166
// 0x1406bbee1  f7d1                           notl	%ecx
// Frames: F17 -> F70 -> F71 -> F72 -> F73 -> F175 -> F84 -> F22 -> F166
// 0x1406bbee3  4c6bf158                       imulq	$0x58, %rcx, %r14
// 0x1406bbee7  4c037538                       addq	0x38(%rbp), %r14
// 0x1406bbeeb  488b30                         movq	(%rax), %rsi
// Frames: F166
// 0x1406bbeee  85f6                           testl	%esi, %esi
// 0x1406bbef0  0f84de000000                   je	0x1406bbfd4 <.text+0x6bafd4>
// Frames: F176
// 0x1406bbef6  41ffc1                         incl	%r9d
// 0x1406bbef9  488b4518                       movq	0x18(%rbp), %rax
// 0x1406bbefd  488b4d20                       movq	0x20(%rbp), %rcx
// 0x1406bbf01  498b16                         movq	(%r14), %rdx
// Frames: F177 -> F178 -> F179 -> F176
// 0x1406bbf04  48c1e103                       shlq	$0x3, %rcx
// 0x1406bbf08  4531c0                         xorl	%r8d, %r8d
// 0x1406bbf0b  0f1f440000                     nopl	(%rax,%rax)
// Frames: F24 -> F25 -> F177 -> F178 -> F179 -> F176
// 0x1406bbf10  4c39c1                         cmpq	%r8, %rcx
// Frames: F25 -> F177 -> F178 -> F179 -> F176
// 0x1406bbf13  0f846dfeffff                   je	0x1406bbd86 <.text+0x6bad86>
// Frames: F180 -> F181 -> F182 -> F178 -> F179 -> F176
// 0x1406bbf19  4a391400                       cmpq	%rdx, (%rax,%r8)
// Frames: F182 -> F178 -> F179 -> F176
// 0x1406bbf1d  4d8d4008                       leaq	0x8(%r8), %r8
// 0x1406bbf21  75ed                           jne	0x1406bbf10 <.text+0x6baf10>
// Frames: F183 -> F184 -> F185 -> F186 -> F187 -> F188 -> F189 -> F190 -> F191 -> F192
// 0x1406bbf23  44894d00                       movl	%r9d, (%rbp)
// 0x1406bbf27  488b4d28                       movq	0x28(%rbp), %rcx
// 0x1406bbf2b  488b01                         movq	(%rcx), %rax
// 0x1406bbf2e  4c8b7910                       movq	0x10(%rcx), %r15
// Frames: F193 -> F184 -> F185 -> F186 -> F187 -> F188 -> F189 -> F190 -> F191 -> F192
// 0x1406bbf32  4c29f8                         subq	%r15, %rax
// Frames: F184 -> F185 -> F186 -> F187 -> F188 -> F189 -> F190 -> F191 -> F192
// 0x1406bbf35  4883f817                       cmpq	$0x17, %rax
// Frames: F185 -> F186 -> F187 -> F188 -> F189 -> F190 -> F191 -> F192
// 0x1406bbf39  7672                           jbe	0x1406bbfad <.text+0x6bafad>
// Frames: F194
// 0x1406bbf3b  488b4d28                       movq	0x28(%rbp), %rcx
// 0x1406bbf3f  488b4108                       movq	0x8(%rcx), %rax
// 0x1406bbf43  488d15260a9000                 leaq	0x900a26(%rip), %rdx    # 0x140fbc970 <.text+0xfbb970>
// 0x1406bbf4a  4a891438                       movq	%rdx, (%rax,%r15)
// 0x1406bbf4e  4a89743808                     movq	%rsi, 0x8(%rax,%r15)
// 0x1406bbf53  42c644381000                   movb	$0x0, 0x10(%rax,%r15)
// 0x1406bbf59  4983c718                       addq	$0x18, %r15
// 0x1406bbf5d  4c897910                       movq	%r15, 0x10(%rcx)
// 0x1406bbf61  488d4dcc                       leaq	-0x34(%rbp), %rcx
// 0x1406bbf65  4c89f2                         movq	%r14, %rdx
// 0x1406bbf68  e863cf5200                     callq	0x140be8ed0 <.text+0xbe7ed0>
// 0x1406bbf6d  90                             nop
// Frames: F170
// 0x1406bbf6e  8b45d4                         movl	-0x2c(%rbp), %eax
// 0x1406bbf71  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x1406bbf75  894108                         movl	%eax, 0x8(%rcx)
// 0x1406bbf78  488b45cc                       movq	-0x34(%rbp), %rax
// 0x1406bbf7c  488901                         movq	%rax, (%rcx)
// 0x1406bbf7f  8b4504                         movl	0x4(%rbp), %eax
// 0x1406bbf82  488b4dd8                       movq	-0x28(%rbp), %rcx
// Frames: F89 -> F195 -> F169 -> F170
// 0x1406bbf86  8901                           movl	%eax, (%rcx)
// 0x1406bbf88  488b4530                       movq	0x30(%rbp), %rax
// Frames: F196 -> F150 -> F167 -> F168 -> F169 -> F170
// 0x1406bbf8c  488b7030                       movq	0x30(%rax), %rsi
// Frames: F197 -> F168 -> F169 -> F170
// 0x1406bbf90  4c8b7040                       movq	0x40(%rax), %r14
// Frames: F163 -> F150 -> F167 -> F168 -> F169 -> F170
// 0x1406bbf94  483b7020                       cmpq	0x20(%rax), %rsi
// 0x1406bbf98  0f85b2fdffff                   jne	0x1406bbd50 <.text+0x6bad50>
// Frames: F149 -> F150 -> F167 -> F168 -> F169 -> F170
// 0x1406bbf9e  488b4de0                       movq	-0x20(%rbp), %rcx
// 0x1406bbfa2  e879336b02                     callq	0x142d6f320 <.text+0x2d6e320>
// 0x1406bbfa7  90                             nop
// 0x1406bbfa8  e9a3fdffff                     jmp	0x1406bbd50 <.text+0x6bad50>
// Frames: F198 -> F186 -> F187 -> F188 -> F189 -> F190 -> F191 -> F192
// 0x1406bbfad  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x1406bbfb6  41b818000000                   movl	$0x18, %r8d
// 0x1406bbfbc  41b901000000                   movl	$0x1, %r9d
// 0x1406bbfc2  488b4d28                       movq	0x28(%rbp), %rcx
// 0x1406bbfc6  4c89fa                         movq	%r15, %rdx
// 0x1406bbfc9  e8423d6b02                     callq	0x142d6fd10 <.text+0x2d6ed10>
// 0x1406bbfce  90                             nop
// 0x1406bbfcf  e967ffffff                     jmp	0x1406bbf3b <.text+0x6baf3b>
// Frames: F199 -> F200 -> F101 -> F101 -> F201
// 0x1406bbfd4  488b5510                       movq	0x10(%rbp), %rdx
// Frames: F202 -> F199 -> F200 -> F101 -> F101 -> F201
// 0x1406bbfd8  4885d2                         testq	%rdx, %rdx
// 0x1406bbfdb  7414                           je	0x1406bbff1 <.text+0x6baff1>
// Frames: F199 -> F200 -> F101 -> F101 -> F201
// 0x1406bbfdd  488b4d18                       movq	0x18(%rbp), %rcx
// Frames: F203 -> F204 -> F199 -> F200 -> F101 -> F101 -> F201
// 0x1406bbfe1  48c1e203                       shlq	$0x3, %rdx
// Frames: F205 -> F206 -> F207 -> F208 -> F209 -> F200 -> F101 -> F101 -> F201
// 0x1406bbfe5  41b808000000                   movl	$0x8, %r8d
// 0x1406bbfeb  e80099feff                     callq	0x1406a58f0 <.text+0x6a48f0>
// 0x1406bbff0  90                             nop
// Frames: F201
// 0x1406bbff1  4881c4c8000000                 addq	$0xc8, %rsp
// 0x1406bbff8  5b                             popq	%rbx
// 0x1406bbff9  5f                             popq	%rdi
// 0x1406bbffa  5e                             popq	%rsi
// 0x1406bbffb  415c                           popq	%r12
// 0x1406bbffd  415d                           popq	%r13
// 0x1406bbfff  415e                           popq	%r14
// 0x1406bc001  415f                           popq	%r15
// 0x1406bc003  5d                             popq	%rbp
// 0x1406bc004  c3                             retq
// 0x1406bc005  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// 0x1406bc010  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406bc015  55                             pushq	%rbp
// 0x1406bc016  4157                           pushq	%r15
// 0x1406bc018  4156                           pushq	%r14
// 0x1406bc01a  4155                           pushq	%r13
// 0x1406bc01c  4154                           pushq	%r12
// 0x1406bc01e  56                             pushq	%rsi
// 0x1406bc01f  57                             pushq	%rdi
// 0x1406bc020  53                             pushq	%rbx
// 0x1406bc021  4883ec28                       subq	$0x28, %rsp
// 0x1406bc025  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F199 -> F200 -> F101 -> F101 -> F201
// 0x1406bc02c  488b5510                       movq	0x10(%rbp), %rdx
// Frames: F202 -> F199 -> F200 -> F101 -> F101 -> F201
// 0x1406bc030  4885d2                         testq	%rdx, %rdx
// 0x1406bc033  7414                           je	0x1406bc049 <.text+0x6bb049>
// Frames: F199 -> F200 -> F101 -> F101 -> F201
// 0x1406bc035  488b4d18                       movq	0x18(%rbp), %rcx
// Frames: F203 -> F204 -> F199 -> F200 -> F101 -> F101 -> F201
// 0x1406bc039  48c1e203                       shlq	$0x3, %rdx
// Frames: F205 -> F206 -> F207 -> F208 -> F209 -> F200 -> F101 -> F101 -> F201
// 0x1406bc03d  41b808000000                   movl	$0x8, %r8d
// 0x1406bc043  e8a898feff                     callq	0x1406a58f0 <.text+0x6a48f0>
// 0x1406bc048  90                             nop
// Frames: F128
// 0x1406bc049  4883c428                       addq	$0x28, %rsp
// 0x1406bc04d  5b                             popq	%rbx
// 0x1406bc04e  5f                             popq	%rdi
// 0x1406bc04f  5e                             popq	%rsi
// 0x1406bc050  415c                           popq	%r12
// 0x1406bc052  415d                           popq	%r13
// 0x1406bc054  415e                           popq	%r14
// 0x1406bc056  415f                           popq	%r15
// 0x1406bc058  5d                             popq	%rbp
// 0x1406bc059  c3                             retq
//
// RANGE rva_6bc060_len_7aa: RVA 0x6bc060, 1962 PDB bytes; 499 instructions
// Evidence: _assembly/roof-v1/C/pack-000/rva_6bc060_len_7aa.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 268:13128: system_roof::delete_roof::process_invalid_roof_notifications
// Frames: F210
// 0x1406bc060  55                             pushq	%rbp
// 0x1406bc061  4157                           pushq	%r15
// 0x1406bc063  4156                           pushq	%r14
// 0x1406bc065  4155                           pushq	%r13
// 0x1406bc067  4154                           pushq	%r12
// 0x1406bc069  56                             pushq	%rsi
// 0x1406bc06a  57                             pushq	%rdi
// 0x1406bc06b  53                             pushq	%rbx
// 0x1406bc06c  4881ecf8000000                 subq	$0xf8, %rsp
// 0x1406bc073  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x1406bc07b  48c74570feffffff               movq	$-0x2, 0x70(%rbp)
// 0x1406bc083  4c89cb                         movq	%r9, %rbx
// 0x1406bc086  4889cf                         movq	%rcx, %rdi
// 0x1406bc089  4c8ba5e0000000                 movq	0xe0(%rbp), %r12
// Frames: F131 -> F132 -> F211
// 0x1406bc090  498b00                         movq	(%r8), %rax
// Frames: F134 -> F132 -> F211
// 0x1406bc093  498b4808                       movq	0x8(%r8), %rcx
// Frames: F135 -> F136 -> F137 -> F132 -> F211
// 0x1406bc097  4c8b00                         movq	(%rax), %r8
// 0x1406bc09a  4531ff                         xorl	%r15d, %r15d
// Frames: F138 -> F139 -> F136 -> F137 -> F132 -> F211
// 0x1406bc09d  4d89c1                         movq	%r8, %r9
// 0x1406bc0a0  4c2b4918                       subq	0x18(%rcx), %r9
// 0x1406bc0a4  4d0f42cf                       cmovbq	%r15, %r9
// Frames: F138 -> F140 -> F136 -> F137 -> F132 -> F211
// 0x1406bc0a8  4c2b4138                       subq	0x38(%rcx), %r8
// 0x1406bc0ac  4d0f42c7                       cmovbq	%r15, %r8
// Frames: F12 -> F13 -> F145 -> F136 -> F137 -> F132 -> F211
// 0x1406bc0b0  4c8b5130                       movq	0x30(%rcx), %r10
// Frames: F141 -> F142 -> F143 -> F30 -> F145 -> F136 -> F137 -> F132 -> F211
// 0x1406bc0b4  4d29c2                         subq	%r8, %r10
// 0x1406bc0b7  4d0f42d7                       cmovbq	%r15, %r10
// Frames: F212 -> F213 -> F214 -> F215 -> F211
// 0x1406bc0bb  410f96c0                       setbe	%r8b
// Frames: F12 -> F13 -> F144 -> F136 -> F137 -> F132 -> F211
// 0x1406bc0bf  4c8b5910                       movq	0x10(%rcx), %r11
// Frames: F141 -> F142 -> F143 -> F30 -> F144 -> F136 -> F137 -> F132 -> F211
// 0x1406bc0c3  4d29cb                         subq	%r9, %r11
// 0x1406bc0c6  4d0f42df                       cmovbq	%r15, %r11
// Frames: F146 -> F136 -> F137 -> F132 -> F211
// 0x1406bc0ca  488b4940                       movq	0x40(%rcx), %rcx
// Frames: F212 -> F216 -> F214 -> F215 -> F211
// 0x1406bc0ce  410f96c1                       setbe	%r9b
// Frames: F217 -> F136 -> F137 -> F132 -> F211
// 0x1406bc0d2  4d01d3                         addq	%r10, %r11
// Frames: F146 -> F136 -> F137 -> F132 -> F211
// 0x1406bc0d5  4989ca                         movq	%rcx, %r10
// 0x1406bc0d8  4d29da                         subq	%r11, %r10
// 0x1406bc0db  4c8910                         movq	%r10, (%rax)
// Frames: F218 -> F214 -> F215 -> F211
// 0x1406bc0de  4584c1                         testb	%r8b, %r9b
// 0x1406bc0e1  0f842d040000                   je	0x1406bc514 <.text+0x6bb514>
// Frames: F131 -> F132 -> F219
// 0x1406bc0e7  488b02                         movq	(%rdx), %rax
// Frames: F134 -> F132 -> F219
// 0x1406bc0ea  488b5208                       movq	0x8(%rdx), %rdx
// Frames: F135 -> F136 -> F137 -> F132 -> F219
// 0x1406bc0ee  4c8b08                         movq	(%rax), %r9
// Frames: F138 -> F139 -> F136 -> F137 -> F132 -> F219
// 0x1406bc0f1  4d89ca                         movq	%r9, %r10
// 0x1406bc0f4  4c2b5218                       subq	0x18(%rdx), %r10
// 0x1406bc0f8  4d0f42d7                       cmovbq	%r15, %r10
// Frames: F138 -> F140 -> F136 -> F137 -> F132 -> F219
// 0x1406bc0fc  4c2b4a38                       subq	0x38(%rdx), %r9
// 0x1406bc100  4d0f42cf                       cmovbq	%r15, %r9
// Frames: F141 -> F142 -> F143 -> F30 -> F144 -> F136 -> F137 -> F132 -> F219
// 0x1406bc104  4c89d1                         movq	%r10, %rcx
// 0x1406bc107  48c1e104                       shlq	$0x4, %rcx
// 0x1406bc10b  48034a08                       addq	0x8(%rdx), %rcx
// Frames: F12 -> F13 -> F144 -> F136 -> F137 -> F132 -> F219
// 0x1406bc10f  4c8b4210                       movq	0x10(%rdx), %r8
// Frames: F141 -> F142 -> F143 -> F30 -> F144 -> F136 -> F137 -> F132 -> F219
// 0x1406bc113  4d29d0                         subq	%r10, %r8
// 0x1406bc116  4d0f42c7                       cmovbq	%r15, %r8
// 0x1406bc11a  be08000000                     movl	$0x8, %esi
// 0x1406bc11f  480f42ce                       cmovbq	%rsi, %rcx
// Frames: F141 -> F142 -> F143 -> F30 -> F145 -> F136 -> F137 -> F132 -> F219
// 0x1406bc123  4d89ca                         movq	%r9, %r10
// 0x1406bc126  49c1e204                       shlq	$0x4, %r10
// 0x1406bc12a  4c035228                       addq	0x28(%rdx), %r10
// Frames: F12 -> F13 -> F145 -> F136 -> F137 -> F132 -> F219
// 0x1406bc12e  4c8b5a30                       movq	0x30(%rdx), %r11
// Frames: F141 -> F142 -> F143 -> F30 -> F145 -> F136 -> F137 -> F132 -> F219
// 0x1406bc132  4d29cb                         subq	%r9, %r11
// 0x1406bc135  41b908000000                   movl	$0x8, %r9d
// 0x1406bc13b  4c894d48                       movq	%r9, 0x48(%rbp)
// 0x1406bc13f  4c0f42d6                       cmovbq	%rsi, %r10
// 0x1406bc143  4d0f42df                       cmovbq	%r15, %r11
// Frames: F217 -> F136 -> F137 -> F132 -> F219
// 0x1406bc147  4f8d0c03                       leaq	(%r11,%r8), %r9
// Frames: F146 -> F136 -> F137 -> F132 -> F219
// 0x1406bc14b  488b5240                       movq	0x40(%rdx), %rdx
// 0x1406bc14f  4c29ca                         subq	%r9, %rdx
// 0x1406bc152  488910                         movq	%rdx, (%rax)
// Frames: F17 -> F18 -> F147 -> F148 -> F136 -> F137 -> F132 -> F219
// 0x1406bc155  49c1e004                       shlq	$0x4, %r8
// 0x1406bc159  4901c8                         addq	%rcx, %r8
// 0x1406bc15c  49c1e304                       shlq	$0x4, %r11
// 0x1406bc160  4d01d3                         addq	%r10, %r11
// Frames: F220 -> F221 -> F222 -> F219
// 0x1406bc163  488945e0                       movq	%rax, -0x20(%rbp)
// 0x1406bc167  48894de8                       movq	%rcx, -0x18(%rbp)
// 0x1406bc16b  4c8945f0                       movq	%r8, -0x10(%rbp)
// 0x1406bc16f  4c8955f8                       movq	%r10, -0x8(%rbp)
// 0x1406bc173  4c895d00                       movq	%r11, (%rbp)
// 0x1406bc177  4c894d08                       movq	%r9, 0x8(%rbp)
// 0x1406bc17b  488d4da0                       leaq	-0x60(%rbp), %rcx
// 0x1406bc17f  488d55e0                       leaq	-0x20(%rbp), %rdx
// Frames: F221 -> F222 -> F219
// 0x1406bc183  e8a8299000                     callq	0x140fbeb30 <.text+0xfbdb30>
// Frames: F2 -> F3 -> F4 -> F223 -> F224 -> F225
// 0x1406bc188  488b0f                         movq	(%rdi), %rcx
// 0x1406bc18b  4c8b6f08                       movq	0x8(%rdi), %r13
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F15 -> F226 -> F224 -> F225
// 0x1406bc18f  4c8bb1f8000000                 movq	0xf8(%rcx), %r14
// Frames: F12 -> F13 -> F14 -> F15 -> F226 -> F224 -> F225
// 0x1406bc196  488b8100010000                 movq	0x100(%rcx), %rax
// Frames: F17 -> F18 -> F19 -> F14 -> F15 -> F226 -> F224 -> F225
// 0x1406bc19d  498d3486                       leaq	(%r14,%rax,4), %rsi
// 0x1406bc1a1  48894d50                       movq	%rcx, 0x50(%rbp)
// Frames: F20 -> F15 -> F226 -> F224 -> F225
// 0x1406bc1a5  0fb68108010000                 movzbl	0x108(%rcx), %eax
// 0x1406bc1ac  88455c                         movb	%al, 0x5c(%rbp)
// 0x1406bc1af  8b431c                         movl	0x1c(%rbx), %eax
// 0x1406bc1b2  894540                         movl	%eax, 0x40(%rbp)
// 0x1406bc1b5  488b03                         movq	(%rbx), %rax
// 0x1406bc1b8  488945b8                       movq	%rax, -0x48(%rbp)
// 0x1406bc1bc  488b4310                       movq	0x10(%rbx), %rax
// 0x1406bc1c0  48894530                       movq	%rax, 0x30(%rbp)
// 0x1406bc1c4  488b4da8                       movq	-0x58(%rbp), %rcx
// 0x1406bc1c8  4c8b55b0                       movq	-0x50(%rbp), %r10
// 0x1406bc1cc  458b5c241c                     movl	0x1c(%r12), %r11d
// 0x1406bc1d1  498b0424                       movq	(%r12), %rax
// 0x1406bc1d5  498b5c2410                     movq	0x10(%r12), %rbx
// Frames: F225
// 0x1406bc1da  49c1e203                       shlq	$0x3, %r10
// 0x1406bc1de  48894538                       movq	%rax, 0x38(%rbp)
// 0x1406bc1e2  4883c020                       addq	$0x20, %rax
// 0x1406bc1e6  488945d8                       movq	%rax, -0x28(%rbp)
// 0x1406bc1ea  48c7456800000000               movq	$0x0, 0x68(%rbp)
// 0x1406bc1f2  31c0                           xorl	%eax, %eax
// 0x1406bc1f4  c7456000000000                 movl	$0x0, 0x60(%rbp)
// Frames: F21 -> F22 -> F225
// 0x1406bc1fb  807d5c00                       cmpb	$0x0, 0x5c(%rbp)
// 0x1406bc1ff  0f850a010000                   jne	0x1406bc30f <.text+0x6bb30f>
// Frames: F56 -> F22 -> F225
// 0x1406bc205  44397d60                       cmpl	%r15d, 0x60(%rbp)
// 0x1406bc209  0f85bb000000                   jne	0x1406bc2ca <.text+0x6bb2ca>
// 0x1406bc20f  90                             nop
// Frames: F24 -> F25 -> F57 -> F22 -> F225
// 0x1406bc210  4939f6                         cmpq	%rsi, %r14
// Frames: F25 -> F57 -> F22 -> F225
// 0x1406bc213  0f8431040000                   je	0x1406bc64a <.text+0x6bb64a>
// Frames: F57 -> F22 -> F225
// 0x1406bc219  418b06                         movl	(%r14), %eax
// Frames: F27 -> F28 -> F57 -> F22 -> F225
// 0x1406bc21c  4983c604                       addq	$0x4, %r14
// Frames: F29 -> F30 -> F58 -> F59 -> F22 -> F225
// 0x1406bc220  4d8b8d00010000                 movq	0x100(%r13), %r9
// 0x1406bc227  488d0480                       leaq	(%rax,%rax,4), %rax
// 0x1406bc22b  48c1e005                       shlq	$0x5, %rax
// Frames: F33 -> F34 -> F60 -> F61 -> F22 -> F225
// 0x1406bc22f  4d8b7c0158                     movq	0x58(%r9,%rax), %r15
// Frames: F61 -> F22 -> F225
// 0x1406bc234  4d85ff                         testq	%r15, %r15
// 0x1406bc237  74d7                           je	0x1406bc210 <.text+0x6bb210>
// Frames: F62 -> F63 -> F22 -> F225
// 0x1406bc239  4901c1                         addq	%rax, %r9
// 0x1406bc23c  418b8194000000                 movl	0x94(%r9), %eax
// Frames: F29 -> F30 -> F64 -> F63 -> F22 -> F225
// 0x1406bc243  498b95a0010000                 movq	0x1a0(%r13), %rdx
// 0x1406bc24a  4c8d04c0                       leaq	(%rax,%rax,8), %r8
// 0x1406bc24e  488b4550                       movq	0x50(%rbp), %rax
// Frames: F174 -> F66 -> F22 -> F225
// 0x1406bc252  488b8010010000                 movq	0x110(%rax), %rax
// Frames: F29 -> F30 -> F43 -> F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F225
// 0x1406bc259  4a3b44c240                     cmpq	0x40(%rdx,%r8,8), %rax
// 0x1406bc25e  732b                           jae	0x1406bc28b <.text+0x6bb28b>
// Frames: F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F225
// 0x1406bc260  4a8d14c2                       leaq	(%rdx,%r8,8), %rdx
// 0x1406bc264  4c8b4238                       movq	0x38(%rdx), %r8
// Frames: F44 -> F45 -> F46 -> F43 -> F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F225
// 0x1406bc268  498b04c0                       movq	(%r8,%rax,8), %rax
// 0x1406bc26c  4885c0                         testq	%rax, %rax
// 0x1406bc26f  741a                           je	0x1406bc28b <.text+0x6bb28b>
// Frames: F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F225
// 0x1406bc271  488b5218                       movq	0x18(%rdx), %rdx
// Frames: F47 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F225
// 0x1406bc275  48f7d0                         notq	%rax
// Frames: F50 -> F51 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F225
// 0x1406bc278  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1406bc27c  48c1e004                       shlq	$0x4, %rax
// Frames: F52 -> F53 -> F54 -> F49 -> F55 -> F42 -> F67 -> F174 -> F66 -> F22 -> F225
// 0x1406bc280  488b440210                     movq	0x10(%rdx,%rax), %rax
// 0x1406bc285  48894568                       movq	%rax, 0x68(%rbp)
// 0x1406bc289  eb08                           jmp	0x1406bc293 <.text+0x6bb293>
// 0x1406bc28b  48c7456800000000               movq	$0x0, 0x68(%rbp)
// Frames: F106 -> F116 -> F174 -> F66 -> F22 -> F225
// 0x1406bc293  488b4550                       movq	0x50(%rbp), %rax
// 0x1406bc297  488b9018010000                 movq	0x118(%rax), %rdx
// 0x1406bc29e  c7456000000000                 movl	$0x0, 0x60(%rbp)
// 0x1406bc2a5  b800000000                     movl	$0x0, %eax
// Frames: F29 -> F30 -> F108 -> F109 -> F117 -> F118 -> F112 -> F116 -> F174 -> F66 -> F22 -> F225
// 0x1406bc2aa  493b9188000000                 cmpq	0x88(%r9), %rdx
// 0x1406bc2b1  730f                           jae	0x1406bc2c2 <.text+0x6bb2c2>
// Frames: F109 -> F117 -> F118 -> F112 -> F116 -> F174 -> F66 -> F22 -> F225
// 0x1406bc2b3  498b8180000000                 movq	0x80(%r9), %rax
// Frames: F87 -> F45 -> F113 -> F108 -> F109 -> F117 -> F118 -> F112 -> F116 -> F174 -> F66 -> F22 -> F225
// 0x1406bc2ba  48833cd000                     cmpq	$0x0, (%rax,%rdx,8)
// 0x1406bc2bf  0f95c0                         setne	%al
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F124 -> F125 -> F22 -> F225
// 0x1406bc2c2  498b5150                       movq	0x50(%r9), %rdx
// 0x1406bc2c6  48895548                       movq	%rdx, 0x48(%rbp)
// Frames: F85 -> F123 -> F175 -> F83 -> F22 -> F225
// 0x1406bc2ca  a801                           testb	$0x1, %al
// 0x1406bc2cc  b001                           movb	$0x1, %al
// 0x1406bc2ce  7532                           jne	0x1406bc302 <.text+0x6bb302>
// Frames: F175 -> F83 -> F22 -> F225
// 0x1406bc2d0  8b4560                         movl	0x60(%rbp), %eax
// 0x1406bc2d3  48c1e004                       shlq	$0x4, %rax
// 0x1406bc2d7  488b5548                       movq	0x48(%rbp), %rdx
// 0x1406bc2db  8b440208                       movl	0x8(%rdx,%rax), %eax
// 0x1406bc2df  f7d0                           notl	%eax
// 0x1406bc2e1  e9e7000000                     jmp	0x1406bc3cd <.text+0x6bb3cd>
// 0x1406bc2e6  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// 0x1406bc2f0  31c0                           xorl	%eax, %eax
// 0x1406bc2f2  488b4d20                       movq	0x20(%rbp), %rcx
// 0x1406bc2f6  4c8b55c8                       movq	-0x38(%rbp), %r10
// 0x1406bc2fa  448b5d2c                       movl	0x2c(%rbp), %r11d
// 0x1406bc2fe  488b5dd0                       movq	-0x30(%rbp), %rbx
// Frames: F21 -> F22 -> F225
// 0x1406bc302  ff4560                         incl	0x60(%rbp)
// 0x1406bc305  807d5c00                       cmpb	$0x0, 0x5c(%rbp)
// 0x1406bc309  0f84f6feffff                   je	0x1406bc205 <.text+0x6bb205>
// Frames: F23 -> F22 -> F225
// 0x1406bc30f  44397d60                       cmpl	%r15d, 0x60(%rbp)
// 0x1406bc313  0f85a7000000                   jne	0x1406bc3c0 <.text+0x6bb3c0>
// 0x1406bc319  0f1f8000000000                 nopl	(%rax)
// Frames: F24 -> F25 -> F26 -> F22 -> F225
// 0x1406bc320  4939f6                         cmpq	%rsi, %r14
// Frames: F25 -> F26 -> F22 -> F225
// 0x1406bc323  0f8421030000                   je	0x1406bc64a <.text+0x6bb64a>
// Frames: F26 -> F22 -> F225
// 0x1406bc329  418b16                         movl	(%r14), %edx
// Frames: F27 -> F28 -> F26 -> F22 -> F225
// 0x1406bc32c  4983c604                       addq	$0x4, %r14
// Frames: F29 -> F30 -> F31 -> F32 -> F22 -> F225
// 0x1406bc330  498b85a0010000                 movq	0x1a0(%r13), %rax
// 0x1406bc337  488d14d2                       leaq	(%rdx,%rdx,8), %rdx
// Frames: F33 -> F34 -> F35 -> F36 -> F22 -> F225
// 0x1406bc33b  4c8b7cd010                     movq	0x10(%rax,%rdx,8), %r15
// Frames: F36 -> F22 -> F225
// 0x1406bc340  4d85ff                         testq	%r15, %r15
// 0x1406bc343  74db                           je	0x1406bc320 <.text+0x6bb320>
// Frames: F173 -> F38 -> F22 -> F225
// 0x1406bc345  4c8d0cd0                       leaq	(%rax,%rdx,8), %r9
// 0x1406bc349  488b4550                       movq	0x50(%rbp), %rax
// 0x1406bc34d  488b8010010000                 movq	0x110(%rax), %rax
// Frames: F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F225
// 0x1406bc354  498b5140                       movq	0x40(%r9), %rdx
// Frames: F29 -> F30 -> F43 -> F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F225
// 0x1406bc358  4839d0                         cmpq	%rdx, %rax
// 0x1406bc35b  7327                           jae	0x1406bc384 <.text+0x6bb384>
// Frames: F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F225
// 0x1406bc35d  4d8b4138                       movq	0x38(%r9), %r8
// Frames: F44 -> F45 -> F46 -> F43 -> F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F225
// 0x1406bc361  498b04c0                       movq	(%r8,%rax,8), %rax
// 0x1406bc365  4885c0                         testq	%rax, %rax
// 0x1406bc368  741a                           je	0x1406bc384 <.text+0x6bb384>
// Frames: F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F225
// 0x1406bc36a  4d8b4118                       movq	0x18(%r9), %r8
// Frames: F47 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F225
// 0x1406bc36e  48f7d0                         notq	%rax
// Frames: F50 -> F51 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F225
// 0x1406bc371  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1406bc375  48c1e004                       shlq	$0x4, %rax
// Frames: F52 -> F53 -> F54 -> F49 -> F55 -> F42 -> F173 -> F38 -> F22 -> F225
// 0x1406bc379  498b440010                     movq	0x10(%r8,%rax), %rax
// 0x1406bc37e  48894568                       movq	%rax, 0x68(%rbp)
// 0x1406bc382  eb08                           jmp	0x1406bc38c <.text+0x6bb38c>
// 0x1406bc384  48c7456800000000               movq	$0x0, 0x68(%rbp)
// Frames: F106 -> F107 -> F173 -> F38 -> F22 -> F225
// 0x1406bc38c  488b4550                       movq	0x50(%rbp), %rax
// 0x1406bc390  4c8b8018010000                 movq	0x118(%rax), %r8
// 0x1406bc397  c7456000000000                 movl	$0x0, 0x60(%rbp)
// 0x1406bc39e  b800000000                     movl	$0x0, %eax
// Frames: F29 -> F30 -> F108 -> F109 -> F110 -> F111 -> F112 -> F107 -> F173 -> F38 -> F22 -> F225
// 0x1406bc3a3  4939d0                         cmpq	%rdx, %r8
// 0x1406bc3a6  7318                           jae	0x1406bc3c0 <.text+0x6bb3c0>
// Frames: F109 -> F110 -> F111 -> F112 -> F107 -> F173 -> F38 -> F22 -> F225
// 0x1406bc3a8  498b4138                       movq	0x38(%r9), %rax
// Frames: F87 -> F45 -> F113 -> F108 -> F109 -> F110 -> F111 -> F112 -> F107 -> F173 -> F38 -> F22 -> F225
// 0x1406bc3ac  4a833cc000                     cmpq	$0x0, (%rax,%r8,8)
// 0x1406bc3b1  0f95c0                         setne	%al
// 0x1406bc3b4  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// Frames: F85 -> F123 -> F175 -> F69 -> F22 -> F225
// 0x1406bc3c0  a801                           testb	$0x1, %al
// 0x1406bc3c2  b001                           movb	$0x1, %al
// 0x1406bc3c4  0f8538ffffff                   jne	0x1406bc302 <.text+0x6bb302>
// 0x1406bc3ca  8b4560                         movl	0x60(%rbp), %eax
// Frames: F175 -> F84 -> F22 -> F225
// 0x1406bc3cd  4c6be058                       imulq	$0x58, %rax, %r12
// 0x1406bc3d1  4c036568                       addq	0x68(%rbp), %r12
// 0x1406bc3d5  498b3c24                       movq	(%r12), %rdi
// 0x1406bc3d9  31c0                           xorl	%eax, %eax
// 0x1406bc3db  0f1f440000                     nopl	(%rax,%rax)
// Frames: F24 -> F25 -> F177 -> F178 -> F179 -> F227
// 0x1406bc3e0  4939c2                         cmpq	%rax, %r10
// Frames: F25 -> F177 -> F178 -> F179 -> F227
// 0x1406bc3e3  0f8417010000                   je	0x1406bc500 <.text+0x6bb500>
// Frames: F180 -> F181 -> F182 -> F178 -> F179 -> F227
// 0x1406bc3e9  48393c01                       cmpq	%rdi, (%rcx,%rax)
// Frames: F182 -> F178 -> F179 -> F227
// 0x1406bc3ed  488d4008                       leaq	0x8(%rax), %rax
// 0x1406bc3f1  75ed                           jne	0x1406bc3e0 <.text+0x6bb3e0>
// Frames: F89 -> F90 -> F169 -> F228
// 0x1406bc3f3  4c8955c8                       movq	%r10, -0x38(%rbp)
// 0x1406bc3f7  48894d20                       movq	%rcx, 0x20(%rbp)
// 0x1406bc3fb  44895d2c                       movl	%r11d, 0x2c(%rbp)
// 0x1406bc3ff  48895dd0                       movq	%rbx, -0x30(%rbp)
// 0x1406bc403  44891b                         movl	%r11d, (%rbx)
// 0x1406bc406  488b4538                       movq	0x38(%rbp), %rax
// Frames: F196 -> F150 -> F167 -> F229 -> F169 -> F228
// 0x1406bc40a  488b5830                       movq	0x30(%rax), %rbx
// Frames: F197 -> F229 -> F169 -> F228
// 0x1406bc40e  488b4840                       movq	0x40(%rax), %rcx
// 0x1406bc412  48894dc0                       movq	%rcx, -0x40(%rbp)
// Frames: F163 -> F150 -> F167 -> F229 -> F169 -> F228
// 0x1406bc416  483b5820                       cmpq	0x20(%rax), %rbx
// 0x1406bc41a  750a                           jne	0x1406bc426 <.text+0x6bb426>
// Frames: F149 -> F150 -> F167 -> F229 -> F169 -> F228
// 0x1406bc41c  488b4dd8                       movq	-0x28(%rbp), %rcx
// 0x1406bc420  e8bb306b02                     callq	0x142d6f4e0 <.text+0x2d6e4e0>
// 0x1406bc425  90                             nop
// Frames: F8 -> F9 -> F10 -> F152 -> F153 -> F150 -> F167 -> F229 -> F169 -> F228
// 0x1406bc426  488b5538                       movq	0x38(%rbp), %rdx
// 0x1406bc42a  488b4228                       movq	0x28(%rdx), %rax
// Frames: F171 -> F153 -> F150 -> F167 -> F229 -> F169 -> F228
// 0x1406bc42e  488d0c5b                       leaq	(%rbx,%rbx,2), %rcx
// 0x1406bc432  4c8b45c0                       movq	-0x40(%rbp), %r8
// Frames: F154 -> F155 -> F150 -> F167 -> F229 -> F169 -> F228
// 0x1406bc436  4c8904c8                       movq	%r8, (%rax,%rcx,8)
// 0x1406bc43a  48897cc808                     movq	%rdi, 0x8(%rax,%rcx,8)
// 0x1406bc43f  c644c81000                     movb	$0x0, 0x10(%rax,%rcx,8)
// Frames: F156 -> F150 -> F167 -> F229 -> F169 -> F228
// 0x1406bc444  48ffc3                         incq	%rbx
// 0x1406bc447  48895a30                       movq	%rbx, 0x30(%rdx)
// Frames: F172 -> F229 -> F169 -> F228
// 0x1406bc44b  48ff4240                       incq	0x40(%rdx)
// 0x1406bc44f  8b4540                         movl	0x40(%rbp), %eax
// 0x1406bc452  488b4d30                       movq	0x30(%rbp), %rcx
// Frames: F89 -> F90 -> F230
// 0x1406bc456  8901                           movl	%eax, (%rcx)
// Frames: F231
// 0x1406bc458  410f10442424                   movups	0x24(%r12), %xmm0
// 0x1406bc45e  410f104c2430                   movups	0x30(%r12), %xmm1
// 0x1406bc464  0f114dec                       movups	%xmm1, -0x14(%rbp)
// 0x1406bc468  0f2945e0                       movaps	%xmm0, -0x20(%rbp)
// Frames: F232
// 0x1406bc46c  418b5c2450                     movl	0x50(%r12), %ebx
// Frames: F233
// 0x1406bc471  498b442440                     movq	0x40(%r12), %rax
// 0x1406bc476  48894510                       movq	%rax, 0x10(%rbp)
// 0x1406bc47a  418b442448                     movl	0x48(%r12), %eax
// 0x1406bc47f  894518                         movl	%eax, 0x18(%rbp)
// Frames: F234 -> F235 -> F236 -> F237 -> F238 -> F239 -> F240 -> F230
// 0x1406bc482  e8896e94ff                     callq	0x140003310 <.text+0x2310>
// Frames: F241 -> F235 -> F236 -> F237 -> F238 -> F239 -> F240 -> F230
// 0x1406bc487  b950000000                     movl	$0x50, %ecx
// 0x1406bc48c  ba08000000                     movl	$0x8, %edx
// 0x1406bc491  e84a94feff                     callq	0x1406a58e0 <.text+0x6a48e0>
// Frames: F238 -> F239 -> F240 -> F230
// 0x1406bc496  4885c0                         testq	%rax, %rax
// 0x1406bc499  0f8405030000                   je	0x1406bc7a4 <.text+0x6bb7a4>
// Frames: F242 -> F240 -> F230
// 0x1406bc49f  48c70001000000                 movq	$0x1, (%rax)
// 0x1406bc4a6  48c7400801000000               movq	$0x1, 0x8(%rax)
// 0x1406bc4ae  c6401001                       movb	$0x1, 0x10(%rax)
// 0x1406bc4b2  895814                         movl	%ebx, 0x14(%rax)
// 0x1406bc4b5  488b4d10                       movq	0x10(%rbp), %rcx
// 0x1406bc4b9  48894818                       movq	%rcx, 0x18(%rax)
// 0x1406bc4bd  8b4d18                         movl	0x18(%rbp), %ecx
// 0x1406bc4c0  894820                         movl	%ecx, 0x20(%rax)
// 0x1406bc4c3  0f2845e0                       movaps	-0x20(%rbp), %xmm0
// 0x1406bc4c7  0f114024                       movups	%xmm0, 0x24(%rax)
// 0x1406bc4cb  0f1045ec                       movups	-0x14(%rbp), %xmm0
// 0x1406bc4cf  0f114030                       movups	%xmm0, 0x30(%rax)
// 0x1406bc4d3  48897840                       movq	%rdi, 0x40(%rax)
// Frames: F230
// 0x1406bc4d7  488945e8                       movq	%rax, -0x18(%rbp)
// 0x1406bc4db  48c745e008000000               movq	$0x8, -0x20(%rbp)
// 0x1406bc4e3  488b4db8                       movq	-0x48(%rbp), %rcx
// 0x1406bc4e7  488d55e0                       leaq	-0x20(%rbp), %rdx
// 0x1406bc4eb  4c8d05e6588c02                 leaq	0x28c58e6(%rip), %r8    # 0x142f81dd8
// 0x1406bc4f2  e829f14800                     callq	0x140b4b620 <.text+0xb4a620>
// 0x1406bc4f7  90                             nop
// 0x1406bc4f8  e9f3fdffff                     jmp	0x1406bc2f0 <.text+0x6bb2f0>
// 0x1406bc4fd  0f1f00                         nopl	(%rax)
// Frames: F21 -> F22 -> F225
// 0x1406bc500  31c0                           xorl	%eax, %eax
// 0x1406bc502  ff4560                         incl	0x60(%rbp)
// 0x1406bc505  807d5c00                       cmpb	$0x0, 0x5c(%rbp)
// 0x1406bc509  0f84f6fcffff                   je	0x1406bc205 <.text+0x6bb205>
// 0x1406bc50f  e9fbfdffff                     jmp	0x1406bc30f <.text+0x6bb30f>
// Frames: F243 -> F215 -> F211
// 0x1406bc514  488908                         movq	%rcx, (%rax)
// Frames: F2 -> F3 -> F4 -> F223 -> F224 -> F244
// 0x1406bc517  488b0f                         movq	(%rdi), %rcx
// 0x1406bc51a  488b7f08                       movq	0x8(%rdi), %rdi
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F15 -> F226 -> F224 -> F244
// 0x1406bc51e  4c8bb1f8000000                 movq	0xf8(%rcx), %r14
// Frames: F12 -> F13 -> F14 -> F15 -> F226 -> F224 -> F244
// 0x1406bc525  488b8100010000                 movq	0x100(%rcx), %rax
// Frames: F17 -> F18 -> F19 -> F14 -> F15 -> F226 -> F224 -> F244
// 0x1406bc52c  4d8d3c86                       leaq	(%r14,%rax,4), %r15
// 0x1406bc530  458b4c241c                     movl	0x1c(%r12), %r9d
// 0x1406bc535  4d8b1424                       movq	(%r12), %r10
// 0x1406bc539  4d8b5c2410                     movq	0x10(%r12), %r11
// 0x1406bc53e  498d4220                       leaq	0x20(%r10), %rax
// 0x1406bc542  48894538                       movq	%rax, 0x38(%rbp)
// 0x1406bc546  48894d48                       movq	%rcx, 0x48(%rbp)
// 0x1406bc54a  80b90801000000                 cmpb	$0x0, 0x108(%rcx)
// 0x1406bc551  44894d5c                       movl	%r9d, 0x5c(%rbp)
// 0x1406bc555  4c895568                       movq	%r10, 0x68(%rbp)
// 0x1406bc559  4c895d50                       movq	%r11, 0x50(%rbp)
// 0x1406bc55d  0f8414010000                   je	0x1406bc677 <.text+0x6bb677>
// 0x1406bc563  31db                           xorl	%ebx, %ebx
// 0x1406bc565  31d2                           xorl	%edx, %edx
// 0x1406bc567  4531e4                         xorl	%r12d, %r12d
// 0x1406bc56a  eb27                           jmp	0x1406bc593 <.text+0x6bb593>
// 0x1406bc56c  0f1f4000                       nopl	(%rax)
// Frames: F8 -> F9 -> F10 -> F152 -> F153 -> F150 -> F167 -> F168 -> F169 -> F245
// 0x1406bc570  ffc3                           incl	%ebx
// 0x1406bc572  498b4228                       movq	0x28(%r10), %rax
// Frames: F171 -> F153 -> F150 -> F167 -> F168 -> F169 -> F245
// 0x1406bc576  488d0c76                       leaq	(%rsi,%rsi,2), %rcx
// Frames: F154 -> F155 -> F150 -> F167 -> F168 -> F169 -> F245
// 0x1406bc57a  4c892cc8                       movq	%r13, (%rax,%rcx,8)
// 0x1406bc57e  4c8944c808                     movq	%r8, 0x8(%rax,%rcx,8)
// 0x1406bc583  c644c81000                     movb	$0x0, 0x10(%rax,%rcx,8)
// Frames: F156 -> F150 -> F167 -> F168 -> F169 -> F245
// 0x1406bc588  48ffc6                         incq	%rsi
// 0x1406bc58b  49897230                       movq	%rsi, 0x30(%r10)
// Frames: F172 -> F168 -> F169 -> F245
// 0x1406bc58f  49ff4240                       incq	0x40(%r10)
// Frames: F23 -> F22 -> F244
// 0x1406bc593  4439e3                         cmpl	%r12d, %ebx
// 0x1406bc596  7569                           jne	0x1406bc601 <.text+0x6bb601>
// 0x1406bc598  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F24 -> F25 -> F26 -> F22 -> F244
// 0x1406bc5a0  4d39fe                         cmpq	%r15, %r14
// Frames: F25 -> F26 -> F22 -> F244
// 0x1406bc5a3  0f84ba000000                   je	0x1406bc663 <.text+0x6bb663>
// Frames: F26 -> F22 -> F244
// 0x1406bc5a9  418b0e                         movl	(%r14), %ecx
// Frames: F27 -> F28 -> F26 -> F22 -> F244
// 0x1406bc5ac  4983c604                       addq	$0x4, %r14
// Frames: F29 -> F30 -> F31 -> F32 -> F22 -> F244
// 0x1406bc5b0  488b87a0010000                 movq	0x1a0(%rdi), %rax
// 0x1406bc5b7  488d0cc9                       leaq	(%rcx,%rcx,8), %rcx
// Frames: F33 -> F34 -> F35 -> F36 -> F22 -> F244
// 0x1406bc5bb  4c8b64c810                     movq	0x10(%rax,%rcx,8), %r12
// Frames: F36 -> F22 -> F244
// 0x1406bc5c0  4d85e4                         testq	%r12, %r12
// 0x1406bc5c3  74db                           je	0x1406bc5a0 <.text+0x6bb5a0>
// Frames: F173 -> F38 -> F22 -> F244
// 0x1406bc5c5  488d04c8                       leaq	(%rax,%rcx,8), %rax
// 0x1406bc5c9  488b4d48                       movq	0x48(%rbp), %rcx
// 0x1406bc5cd  488b8910010000                 movq	0x110(%rcx), %rcx
// Frames: F29 -> F30 -> F43 -> F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F244
// 0x1406bc5d4  483b4840                       cmpq	0x40(%rax), %rcx
// 0x1406bc5d8  7323                           jae	0x1406bc5fd <.text+0x6bb5fd>
// Frames: F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F244
// 0x1406bc5da  488b5038                       movq	0x38(%rax), %rdx
// Frames: F44 -> F45 -> F46 -> F43 -> F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F244
// 0x1406bc5de  488b0cca                       movq	(%rdx,%rcx,8), %rcx
// 0x1406bc5e2  4885c9                         testq	%rcx, %rcx
// 0x1406bc5e5  7416                           je	0x1406bc5fd <.text+0x6bb5fd>
// Frames: F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F244
// 0x1406bc5e7  488b4018                       movq	0x18(%rax), %rax
// Frames: F47 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F244
// 0x1406bc5eb  48f7d1                         notq	%rcx
// Frames: F50 -> F51 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F173 -> F38 -> F22 -> F244
// 0x1406bc5ee  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1406bc5f2  48c1e104                       shlq	$0x4, %rcx
// Frames: F52 -> F53 -> F54 -> F49 -> F55 -> F42 -> F173 -> F38 -> F22 -> F244
// 0x1406bc5f6  488b540810                     movq	0x10(%rax,%rcx), %rdx
// 0x1406bc5fb  eb02                           jmp	0x1406bc5ff <.text+0x6bb5ff>
// 0x1406bc5fd  31d2                           xorl	%edx, %edx
// 0x1406bc5ff  31db                           xorl	%ebx, %ebx
// Frames: F68 -> F22 -> F244
// 0x1406bc601  89d8                           movl	%ebx, %eax
// Frames: F17 -> F70 -> F71 -> F72 -> F73 -> F175 -> F69 -> F22 -> F244
// 0x1406bc603  486bc058                       imulq	$0x58, %rax, %rax
// Frames: F246
// 0x1406bc607  4c8b0402                       movq	(%rdx,%rax), %r8
// Frames: F89 -> F195 -> F169 -> F245
// 0x1406bc60b  45890b                         movl	%r9d, (%r11)
// Frames: F196 -> F150 -> F167 -> F168 -> F169 -> F245
// 0x1406bc60e  498b7230                       movq	0x30(%r10), %rsi
// Frames: F197 -> F168 -> F169 -> F245
// 0x1406bc612  4d8b6a40                       movq	0x40(%r10), %r13
// Frames: F163 -> F150 -> F167 -> F168 -> F169 -> F245
// 0x1406bc616  493b7220                       cmpq	0x20(%r10), %rsi
// 0x1406bc61a  0f8550ffffff                   jne	0x1406bc570 <.text+0x6bb570>
// Frames: F149 -> F150 -> F167 -> F168 -> F169 -> F245
// 0x1406bc620  488b4d38                       movq	0x38(%rbp), %rcx
// 0x1406bc624  48895560                       movq	%rdx, 0x60(%rbp)
// 0x1406bc628  4c894540                       movq	%r8, 0x40(%rbp)
// 0x1406bc62c  e8af2e6b02                     callq	0x142d6f4e0 <.text+0x2d6e4e0>
// 0x1406bc631  4c8b4540                       movq	0x40(%rbp), %r8
// 0x1406bc635  488b5560                       movq	0x60(%rbp), %rdx
// 0x1406bc639  4c8b5d50                       movq	0x50(%rbp), %r11
// 0x1406bc63d  4c8b5568                       movq	0x68(%rbp), %r10
// 0x1406bc641  448b4d5c                       movl	0x5c(%rbp), %r9d
// 0x1406bc645  e926ffffff                     jmp	0x1406bc570 <.text+0x6bb570>
// Frames: F199 -> F200 -> F101 -> F101 -> F247
// 0x1406bc64a  488b55a0                       movq	-0x60(%rbp), %rdx
// Frames: F202 -> F199 -> F200 -> F101 -> F101 -> F247
// 0x1406bc64e  4885d2                         testq	%rdx, %rdx
// 0x1406bc651  7410                           je	0x1406bc663 <.text+0x6bb663>
// Frames: F203 -> F204 -> F199 -> F200 -> F101 -> F101 -> F247
// 0x1406bc653  48c1e203                       shlq	$0x3, %rdx
// Frames: F205 -> F206 -> F207 -> F208 -> F209 -> F200 -> F101 -> F101 -> F247
// 0x1406bc657  41b808000000                   movl	$0x8, %r8d
// 0x1406bc65d  e88e92feff                     callq	0x1406a58f0 <.text+0x6a48f0>
// 0x1406bc662  90                             nop
// Frames: F248
// 0x1406bc663  4881c4f8000000                 addq	$0xf8, %rsp
// 0x1406bc66a  5b                             popq	%rbx
// 0x1406bc66b  5f                             popq	%rdi
// 0x1406bc66c  5e                             popq	%rsi
// 0x1406bc66d  415c                           popq	%r12
// 0x1406bc66f  415d                           popq	%r13
// 0x1406bc671  415e                           popq	%r14
// 0x1406bc673  415f                           popq	%r15
// 0x1406bc675  5d                             popq	%rbp
// 0x1406bc676  c3                             retq
// 0x1406bc677  41b808000000                   movl	$0x8, %r8d
// 0x1406bc67d  31f6                           xorl	%esi, %esi
// 0x1406bc67f  31d2                           xorl	%edx, %edx
// 0x1406bc681  4531e4                         xorl	%r12d, %r12d
// 0x1406bc684  eb31                           jmp	0x1406bc6b7 <.text+0x6bb6b7>
// 0x1406bc686  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// Frames: F8 -> F9 -> F10 -> F152 -> F153 -> F150 -> F167 -> F168 -> F169 -> F245
// 0x1406bc690  ffc6                           incl	%esi
// 0x1406bc692  498b4228                       movq	0x28(%r10), %rax
// Frames: F171 -> F153 -> F150 -> F167 -> F168 -> F169 -> F245
// 0x1406bc696  488d0c5b                       leaq	(%rbx,%rbx,2), %rcx
// Frames: F154 -> F155 -> F150 -> F167 -> F168 -> F169 -> F245
// 0x1406bc69a  4c892cc8                       movq	%r13, (%rax,%rcx,8)
// 0x1406bc69e  4c8b6d60                       movq	0x60(%rbp), %r13
// 0x1406bc6a2  4c896cc808                     movq	%r13, 0x8(%rax,%rcx,8)
// 0x1406bc6a7  c644c81000                     movb	$0x0, 0x10(%rax,%rcx,8)
// Frames: F156 -> F150 -> F167 -> F168 -> F169 -> F245
// 0x1406bc6ac  48ffc3                         incq	%rbx
// 0x1406bc6af  49895a30                       movq	%rbx, 0x30(%r10)
// Frames: F172 -> F168 -> F169 -> F245
// 0x1406bc6b3  49ff4240                       incq	0x40(%r10)
// 0x1406bc6b7  4439e6                         cmpl	%r12d, %esi
// 0x1406bc6ba  0f858c000000                   jne	0x1406bc74c <.text+0x6bb74c>
// Frames: F24 -> F25 -> F57 -> F22 -> F244
// 0x1406bc6c0  4d39fe                         cmpq	%r15, %r14
// Frames: F25 -> F57 -> F22 -> F244
// 0x1406bc6c3  749e                           je	0x1406bc663 <.text+0x6bb663>
// Frames: F57 -> F22 -> F244
// 0x1406bc6c5  418b0e                         movl	(%r14), %ecx
// Frames: F27 -> F28 -> F57 -> F22 -> F244
// 0x1406bc6c8  4983c604                       addq	$0x4, %r14
// Frames: F29 -> F30 -> F58 -> F59 -> F22 -> F244
// 0x1406bc6cc  488b8700010000                 movq	0x100(%rdi), %rax
// 0x1406bc6d3  488d0c89                       leaq	(%rcx,%rcx,4), %rcx
// 0x1406bc6d7  48c1e105                       shlq	$0x5, %rcx
// Frames: F33 -> F34 -> F60 -> F61 -> F22 -> F244
// 0x1406bc6db  4c8b640858                     movq	0x58(%rax,%rcx), %r12
// Frames: F61 -> F22 -> F244
// 0x1406bc6e0  4d85e4                         testq	%r12, %r12
// 0x1406bc6e3  74db                           je	0x1406bc6c0 <.text+0x6bb6c0>
// Frames: F62 -> F63 -> F22 -> F244
// 0x1406bc6e5  4801c8                         addq	%rcx, %rax
// 0x1406bc6e8  8b8894000000                   movl	0x94(%rax), %ecx
// Frames: F29 -> F30 -> F64 -> F63 -> F22 -> F244
// 0x1406bc6ee  488b97a0010000                 movq	0x1a0(%rdi), %rdx
// 0x1406bc6f5  4c8d04c9                       leaq	(%rcx,%rcx,8), %r8
// 0x1406bc6f9  488b4d48                       movq	0x48(%rbp), %rcx
// Frames: F174 -> F66 -> F22 -> F244
// 0x1406bc6fd  488b8910010000                 movq	0x110(%rcx), %rcx
// Frames: F29 -> F30 -> F43 -> F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F244
// 0x1406bc704  4a3b4cc240                     cmpq	0x40(%rdx,%r8,8), %rcx
// 0x1406bc709  7327                           jae	0x1406bc732 <.text+0x6bb732>
// Frames: F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F244
// 0x1406bc70b  4a8d14c2                       leaq	(%rdx,%r8,8), %rdx
// 0x1406bc70f  4c8b4238                       movq	0x38(%rdx), %r8
// Frames: F44 -> F45 -> F46 -> F43 -> F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F244
// 0x1406bc713  498b0cc8                       movq	(%r8,%rcx,8), %rcx
// 0x1406bc717  4885c9                         testq	%rcx, %rcx
// 0x1406bc71a  7416                           je	0x1406bc732 <.text+0x6bb732>
// Frames: F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F244
// 0x1406bc71c  488b5218                       movq	0x18(%rdx), %rdx
// Frames: F47 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F244
// 0x1406bc720  48f7d1                         notq	%rcx
// Frames: F50 -> F51 -> F48 -> F49 -> F39 -> F40 -> F41 -> F42 -> F67 -> F174 -> F66 -> F22 -> F244
// 0x1406bc723  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1406bc727  48c1e104                       shlq	$0x4, %rcx
// Frames: F52 -> F53 -> F54 -> F49 -> F55 -> F42 -> F67 -> F174 -> F66 -> F22 -> F244
// 0x1406bc72b  488b540a10                     movq	0x10(%rdx,%rcx), %rdx
// 0x1406bc730  eb02                           jmp	0x1406bc734 <.text+0x6bb734>
// 0x1406bc732  31d2                           xorl	%edx, %edx
// Frames: F106 -> F116 -> F174 -> F66 -> F22 -> F244
// 0x1406bc734  488b4d48                       movq	0x48(%rbp), %rcx
// 0x1406bc738  488b8918010000                 movq	0x118(%rcx), %rcx
// Frames: F29 -> F30 -> F108 -> F109 -> F117 -> F118 -> F112 -> F116 -> F174 -> F66 -> F22 -> F244
// 0x1406bc73f  483b8888000000                 cmpq	0x88(%rax), %rcx
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F124 -> F125 -> F22 -> F244
// 0x1406bc746  4c8b4050                       movq	0x50(%rax), %r8
// 0x1406bc74a  31f6                           xorl	%esi, %esi
// Frames: F75 -> F22 -> F244
// 0x1406bc74c  89f0                           movl	%esi, %eax
// Frames: F50 -> F76 -> F75 -> F22 -> F244
// 0x1406bc74e  48c1e004                       shlq	$0x4, %rax
// Frames: F79 -> F80 -> F22 -> F244
// 0x1406bc752  418b440008                     movl	0x8(%r8,%rax), %eax
// Frames: F81 -> F82 -> F71 -> F72 -> F73 -> F175 -> F83 -> F22 -> F244
// 0x1406bc757  f7d0                           notl	%eax
// Frames: F17 -> F126 -> F71 -> F72 -> F73 -> F175 -> F83 -> F22 -> F244
// 0x1406bc759  486bc058                       imulq	$0x58, %rax, %rax
// Frames: F246
// 0x1406bc75d  488b0402                       movq	(%rdx,%rax), %rax
// 0x1406bc761  48894560                       movq	%rax, 0x60(%rbp)
// Frames: F89 -> F195 -> F169 -> F245
// 0x1406bc765  45890b                         movl	%r9d, (%r11)
// Frames: F196 -> F150 -> F167 -> F168 -> F169 -> F245
// 0x1406bc768  498b5a30                       movq	0x30(%r10), %rbx
// Frames: F197 -> F168 -> F169 -> F245
// 0x1406bc76c  4d8b6a40                       movq	0x40(%r10), %r13
// Frames: F163 -> F150 -> F167 -> F168 -> F169 -> F245
// 0x1406bc770  493b5a20                       cmpq	0x20(%r10), %rbx
// 0x1406bc774  0f8516ffffff                   jne	0x1406bc690 <.text+0x6bb690>
// Frames: F149 -> F150 -> F167 -> F168 -> F169 -> F245
// 0x1406bc77a  488b4d38                       movq	0x38(%rbp), %rcx
// 0x1406bc77e  48895540                       movq	%rdx, 0x40(%rbp)
// 0x1406bc782  4c894530                       movq	%r8, 0x30(%rbp)
// 0x1406bc786  e8552d6b02                     callq	0x142d6f4e0 <.text+0x2d6e4e0>
// 0x1406bc78b  4c8b4530                       movq	0x30(%rbp), %r8
// 0x1406bc78f  488b5540                       movq	0x40(%rbp), %rdx
// 0x1406bc793  4c8b5d50                       movq	0x50(%rbp), %r11
// 0x1406bc797  4c8b5568                       movq	0x68(%rbp), %r10
// 0x1406bc79b  448b4d5c                       movl	0x5c(%rbp), %r9d
// 0x1406bc79f  e9ecfeffff                     jmp	0x1406bc690 <.text+0x6bb690>
// Frames: F249 -> F239 -> F240 -> F230
// 0x1406bc7a4  b908000000                     movl	$0x8, %ecx
// 0x1406bc7a9  ba50000000                     movl	$0x50, %edx
// 0x1406bc7ae  e82e4b6f02                     callq	0x142db12e1 <.text+0x2db02e1>
// 0x1406bc7b3  90                             nop
// 0x1406bc7b4  0f0b                           ud2
// 0x1406bc7b6  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// 0x1406bc7c0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406bc7c5  55                             pushq	%rbp
// 0x1406bc7c6  4157                           pushq	%r15
// 0x1406bc7c8  4156                           pushq	%r14
// 0x1406bc7ca  4155                           pushq	%r13
// 0x1406bc7cc  4154                           pushq	%r12
// 0x1406bc7ce  56                             pushq	%rsi
// 0x1406bc7cf  57                             pushq	%rdi
// 0x1406bc7d0  53                             pushq	%rbx
// 0x1406bc7d1  4883ec28                       subq	$0x28, %rsp
// 0x1406bc7d5  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F199 -> F200 -> F101 -> F101 -> F247
// 0x1406bc7dc  488b55a0                       movq	-0x60(%rbp), %rdx
// Frames: F202 -> F199 -> F200 -> F101 -> F101 -> F247
// 0x1406bc7e0  4885d2                         testq	%rdx, %rdx
// 0x1406bc7e3  7414                           je	0x1406bc7f9 <.text+0x6bb7f9>
// Frames: F203 -> F204 -> F199 -> F200 -> F101 -> F101 -> F247
// 0x1406bc7e5  48c1e203                       shlq	$0x3, %rdx
// Frames: F205 -> F206 -> F207 -> F208 -> F209 -> F200 -> F101 -> F101 -> F247
// 0x1406bc7e9  41b808000000                   movl	$0x8, %r8d
// 0x1406bc7ef  488b4d20                       movq	0x20(%rbp), %rcx
// 0x1406bc7f3  e8f890feff                     callq	0x1406a58f0 <.text+0x6a48f0>
// 0x1406bc7f8  90                             nop
// Frames: F210
// 0x1406bc7f9  4883c428                       addq	$0x28, %rsp
// 0x1406bc7fd  5b                             popq	%rbx
// 0x1406bc7fe  5f                             popq	%rdi
// 0x1406bc7ff  5e                             popq	%rsi
// 0x1406bc800  415c                           popq	%r12
// 0x1406bc802  415d                           popq	%r13
// 0x1406bc804  415e                           popq	%r14
// 0x1406bc806  415f                           popq	%r15
// 0x1406bc808  5d                             popq	%rbp
// 0x1406bc809  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/delete_roof.rs:76:0 system_roof::delete_roof::delete_roofs::ha1cb929bea95d95e
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:528:0 bevy_ecs::system::query::Query::new
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:643:0 bevy_ecs::system::query::Query::copy_unsafe
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:625:0 bevy_ecs::system::query::Query::reborrow_unsafe
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:607:0 bevy_ecs::system::query::Query::reborrow
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:705:0 bevy_ecs::system::query::Query::iter_mut
// F7: crates/systems/roof/src/delete_roof.rs:86:0 system_roof::delete_roof::delete_roofs::ha1cb929bea95d95e
// F8: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:612:0 alloc::raw_vec::RawVecInner::non_null
// F9: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:607:0 alloc::raw_vec::RawVecInner::ptr
// F10: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:295:0 alloc::raw_vec::RawVec::ptr
// F11: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1916:0 alloc::vec::Vec::as_ptr
// F12: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1815:0 alloc::vec::Vec::as_slice
// F13: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3729:0 alloc::vec::impl$9::deref
// F14: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2421:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F15: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:53:0 bevy_ecs::query::iter::QueryIter::new
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:2510:0 bevy_ecs::system::query::Query::reborrow
// F17: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 core::ptr::mut_ptr::impl$0::add
// F18: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F19: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1041:0 alloc::vec::impl$9::deref
// F20: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2422:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F21: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2517:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F22: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:915:0 bevy_ecs::query::iter::impl$1::next
// F23: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2520:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F24: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:1720:0 core::ptr::non_null::impl$16::eq
// F25: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:180:0 core::slice::iter::impl$171::next
// F26: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2521:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F27: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:659:0 core::ptr::non_null::NonNull::add
// F28: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:185:0 core::slice::iter::impl$171::next
// F29: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:219:0 core::slice::index::impl$2::get
// F30: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 core::slice::impl$0::get
// F31: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:753:0 bevy_ecs::storage::table::Tables::get
// F32: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2522:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F33: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2997:0 alloc::vec::Vec::len
// F34: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3022:0 alloc::vec::Vec::is_empty
// F35: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:647:0 bevy_ecs::storage::table::Table::is_empty
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2523:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F37: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs:199:0 bevy_ecs::query::world_query::impl$4::set_table
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2529:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F39: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:406:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F40: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:490:0 bevy_ecs::storage::table::Table::get_column
// F41: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:381:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F42: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1552:0 bevy_ecs::query::fetch::impl$36::set_table
// F43: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F44: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:745:0 core::option::Option::as_ref
// F45: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F46: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1546:0 core::slice::impl$0::get
// F47: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F48: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:408:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F49: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:0 core::option::Option::map
// F50: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:253:0 core::slice::index::impl$2::get_unchecked
// F51: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 nonmax::NonMaxUsize::get
// F52: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F53: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs:298:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:382:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F55: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:382:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F56: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2561:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F57: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2562:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F58: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:865:0 bevy_ecs::archetype::Archetypes::get
// F59: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2563:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F60: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:656:0 bevy_ecs::archetype::Archetype::is_empty
// F61: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2564:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:479:0 bevy_ecs::archetype::Archetype::table_id
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2567:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F64: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:753:0 bevy_ecs::archetype::Archetype::table_id
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$4::set_archetype
// F66: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2571:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1539:0 bevy_ecs::query::fetch::impl$36::set_archetype
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2540:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2554:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs:1079:0 bevy_ptr::ThinSlicePtr::get
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1609:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2966:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1604:0 bevy_ecs::query::fetch::impl$37::fetch
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2529:0 bevy_ecs::query::fetch::impl$95::fetch
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2593:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F76: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 core::slice::impl$0::get_unchecked
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:339:0 bevy_ecs::archetype::ArchetypeEntity::id
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2614:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:347:0 bevy_ecs::archetype::ArchetypeEntity::table_row
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2615:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxU32::get
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:116:0 bevy_ecs::storage::table::TableRow::index
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2611:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:0:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F85: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/bool.rs:66:0 core::bool::impl$0::then
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2286:0 bevy_ecs::query::fetch::impl$37::fetch
// F87: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:636:0 core::option::Option::is_some
// F88: crates/systems/roof/src/delete_roof.rs:90:0 system_roof::delete_roof::delete_roofs::ha1cb929bea95d95e
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F90: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F91: crates/systems/roof/src/delete_roof.rs:91:0 system_roof::delete_roof::delete_roofs::ha1cb929bea95d95e
// F92: crates/country-core/src/resources/rt/mod.rs:200:0 country_core::resources::rt::RtWorld::remove
// F93: /rust/deps/hashbrown-0.16.1/src/map.rs:258:0 hashbrown::map::make_hash
// F94: /rust/deps/hashbrown-0.16.1/src/map.rs:2048:0 hashbrown::map::HashMap::remove_entry
// F95: /rust/deps/hashbrown-0.16.1/src/map.rs:2010:0 hashbrown::map::HashMap::remove
// F96: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:1355:0 std::collections::hash::map::HashMap::remove
// F97: /rust/deps/hashbrown-0.16.1/src/map.rs:2049:0 hashbrown::map::HashMap::remove_entry
// F98: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3942:0 core::sync::atomic::atomic_sub
// F99: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3167:0 core::sync::atomic::AtomicUsize::fetch_sub
// F100: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2811:0 alloc::sync::impl$42::drop
// F101: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:805:0 core::ptr::drop_in_place
// F102: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2854:0 alloc::sync::impl$42::drop
// F103: crates/country-core/src/resources/rt/mod.rs:201:0 country_core::resources::rt::RtWorld::remove
// F104: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:443:0 bevy_ecs::system::commands::Commands::entity
// F105: crates/systems/roof/src/delete_roof.rs:92:0 system_roof::delete_roof::delete_roofs::ha1cb929bea95d95e
// F106: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1578:0 bevy_ecs::query::fetch::impl$36::matches_component_set
// F107: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2224:0 bevy_ecs::query::fetch::impl$55::set_table
// F108: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:49:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:399:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
// F110: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:511:0 bevy_ecs::storage::table::Table::has_column
// F111: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2224:0 bevy_ecs::query::fetch::impl$55::set_table::closure$0
// F112: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1581:0 bevy_ecs::query::fetch::impl$36::matches_component_set
// F113: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:664:0 core::slice::impl$0::get
// F114: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2225:0 bevy_ecs::query::fetch::impl$55::set_table
// F115: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2228:0 bevy_ecs::query::fetch::impl$55::set_table
// F116: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2209:0 bevy_ecs::query::fetch::impl$55::set_archetype
// F117: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:662:0 bevy_ecs::archetype::Archetype::contains
// F118: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2209:0 bevy_ecs::query::fetch::impl$55::set_archetype::closure$0
// F119: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2210:0 bevy_ecs::query::fetch::impl$55::set_archetype
// F120: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2213:0 bevy_ecs::query::fetch::impl$55::set_archetype
// F121: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:209:0 bevy_ecs::storage::table::Table::entities
// F122: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2532:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F123: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2286:0 bevy_ecs::query::fetch::impl$56::fetch
// F124: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:485:0 bevy_ecs::archetype::Archetype::entities
// F125: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs:2584:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F126: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs:1079:0 bevy_ecs::storage::table::TableRow::index
// F127: crates/systems/roof/src/delete_roof.rs:95:0 system_roof::delete_roof::delete_roofs::ha1cb929bea95d95e
// F128: crates/systems/roof/src/delete_roof.rs:54:0 system_roof::delete_roof::mark_roof_for_deletion::h96ba0cea1dda9db9
// F129: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:464:0 alloc::vec::Vec::new
// F130: crates/systems/roof/src/delete_roof.rs:60:0 system_roof::delete_roof::mark_roof_for_deletion::h96ba0cea1dda9db9
// F131: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:1065:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F132: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs:28:0 bevy_ecs::message::message_reader::MessageReader::read
// F133: crates/systems/roof/src/delete_roof.rs:61:0 system_roof::delete_roof::mark_roof_for_deletion::h96ba0cea1dda9db9
// F134: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:377:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F135: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:55:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F136: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs:93:0 bevy_ecs::message::message_cursor::MessageCursor::read_with_id
// F137: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs:80:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F138: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:2345:0 core::num::impl$11::saturating_sub
// F139: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:57:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F140: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:60:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F141: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:923:0 core::num::impl$11::checked_sub
// F142: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:368:0 core::slice::index::impl$4::get
// F143: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:546:0 core::slice::index::impl$7::get
// F144: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:61:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F145: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:62:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F146: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:67:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F147: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1041:0 core::slice::impl$0::iter
// F148: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:69:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F149: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1029:0 alloc::vec::Vec::push_mut
// F150: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:992:0 alloc::vec::Vec::push
// F151: crates/systems/roof/src/delete_roof.rs:62:0 system_roof::delete_roof::mark_roof_for_deletion::h96ba0cea1dda9db9
// F152: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2000:0 alloc::vec::Vec::as_mut_ptr
// F153: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1032:0 alloc::vec::Vec::push_mut
// F154: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1917:0 core::ptr::write
// F155: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1033:0 alloc::vec::Vec::push_mut
// F156: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1034:0 alloc::vec::Vec::push_mut
// F157: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:767:0 core::option::Option::as_mut
// F158: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:332:0 core::iter::adapters::chain::and_then_or_clear
// F159: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:0 core::iter::adapters::chain::impl$1::next
// F160: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:89:0 bevy_ecs::message::iterators::impl$3::next
// F161: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:15:0 bevy_ecs::message::iterators::impl$0::next
// F162: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:95:0 bevy_ecs::message::iterators::impl$3::next
// F163: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1028:0 alloc::vec::Vec::push_mut
// F164: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:0 core::iter::adapters::chain::impl$1::next::closure$0
// F165: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1656:0 core::option::Option::or_else
// F166: crates/systems/roof/src/delete_roof.rs:65:0 system_roof::delete_roof::mark_roof_for_deletion::h96ba0cea1dda9db9
// F167: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:143:0 bevy_ecs::message::messages::Messages::write_with_caller
// F168: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:126:0 bevy_ecs::message::messages::Messages::write
// F169: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs:70:0 bevy_ecs::message::message_writer::MessageWriter::write
// F170: crates/systems/roof/src/delete_roof.rs:68:0 system_roof::delete_roof::mark_roof_for_deletion::h96ba0cea1dda9db9
// F171: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 alloc::vec::Vec::as_mut_ptr
// F172: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:144:0 bevy_ecs::message::messages::Messages::write_with_caller
// F173: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs:199:0 bevy_ecs::query::world_query::impl$2::set_table
// F174: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$2::set_archetype
// F175: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2529:0 bevy_ecs::query::fetch::impl$89::fetch
// F176: crates/systems/roof/src/delete_roof.rs:66:0 system_roof::delete_roof::mark_roof_for_deletion::h96ba0cea1dda9db9
// F177: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:331:0 core::slice::iter::impl$171::any
// F178: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/cmp.rs:376:0 core::slice::cmp::impl$17::slice_contains
// F179: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:2593:0 core::slice::impl$0::contains
// F180: crates/country-core/src/resources/walls/mod.rs:84:0 country_core::resources::walls::impl$10::eq
// F181: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/cmp.rs:376:0 core::slice::cmp::impl$17::slice_contains::closure$0
// F182: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:332:0 core::slice::iter::impl$171::any
// F183: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:617:0 alloc::raw_vec::RawVecInner::capacity
// F184: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:765:0 alloc::raw_vec::RawVecInner::needs_to_grow
// F185: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:671:0 alloc::raw_vec::RawVecInner::reserve
// F186: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:340:0 alloc::raw_vec::RawVec::reserve
// F187: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1446:0 alloc::vec::Vec::reserve
// F188: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:0:0 bevy_ecs::system::commands::Commands::queue_internal
// F189: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:589:0 bevy_ecs::system::commands::Commands::queue
// F190: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:1893:0 bevy_ecs::system::commands::EntityCommands::queue
// F191: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:1391:0 bevy_ecs::system::commands::EntityCommands::insert
// F192: crates/systems/roof/src/delete_roof.rs:67:0 system_roof::delete_roof::mark_roof_for_deletion::h96ba0cea1dda9db9
// F193: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:2494:0 alloc::raw_vec::RawVecInner::capacity
// F194: crates/systems/roof/src/delete_roof.rs:70:0 system_roof::delete_roof::mark_roof_for_deletion::h96ba0cea1dda9db9
// F195: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::message::messages::Messages::write
// F196: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1025:0 alloc::vec::Vec::push_mut
// F197: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:131:0 bevy_ecs::message::messages::Messages::write_with_caller
// F198: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:673:0 alloc::raw_vec::RawVecInner::reserve
// F199: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:872:0 alloc::raw_vec::RawVecInner::deallocate
// F200: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:423:0 alloc::raw_vec::impl$3::drop
// F201: crates/systems/roof/src/delete_roof.rs:74:0 system_roof::delete_roof::mark_roof_for_deletion::h96ba0cea1dda9db9
// F202: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:632:0 alloc::raw_vec::RawVecInner::current_memory
// F203: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:1232:0 core::num::impl$11::unchecked_mul
// F204: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:640:0 alloc::raw_vec::RawVecInner::current_memory
// F205: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:115:0 alloc::alloc::dealloc
// F206: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:209:0 alloc::alloc::Global::deallocate_impl_runtime
// F207: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:324:0 alloc::alloc::Global::deallocate_impl
// F208: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:442:0 alloc::alloc::impl$1::deallocate
// F209: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:874:0 alloc::raw_vec::RawVecInner::deallocate
// F210: crates/systems/roof/src/delete_roof.rs:14:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F211: crates/systems/roof/src/delete_roof.rs:22:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F212: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1545:0 core::option::Option::and_then
// F213: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:177:0 core::iter::adapters::chain::impl$1::last
// F214: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:119:0 bevy_ecs::message::iterators::impl$3::last
// F215: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:30:0 bevy_ecs::message::iterators::impl$0::last
// F216: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:176:0 core::iter::adapters::chain::impl$1::last
// F217: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:64:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F218: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:2766:0 core::option::impl$47::branch
// F219: crates/systems/roof/src/delete_roof.rs:34:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F220: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:323:0 core::iter::traits::collect::impl$0::into_iter
// F221: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3865:0 alloc::vec::impl$16::from_iter
// F222: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2064:0 core::iter::traits::iterator::Iterator::collect
// F223: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:548:0 bevy_ecs::system::query::Query::as_readonly
// F224: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:674:0 bevy_ecs::system::query::Query::iter
// F225: crates/systems/roof/src/delete_roof.rs:36:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F226: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:2510:0 bevy_ecs::system::query::Query::as_readonly
// F227: crates/systems/roof/src/delete_roof.rs:37:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F228: crates/systems/roof/src/delete_roof.rs:38:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F229: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:126:0 bevy_ecs::change_detection::impl$38::deref_mut
// F230: crates/systems/roof/src/delete_roof.rs:43:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F231: crates/systems/roof/src/delete_roof.rs:45:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F232: crates/systems/roof/src/delete_roof.rs:46:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F233: crates/systems/roof/src/delete_roof.rs:47:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F234: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:93:0 alloc::alloc::alloc
// F235: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:190:0 alloc::alloc::Global::alloc_impl_runtime
// F236: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:312:0 alloc::alloc::Global::alloc_impl
// F237: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:429:0 alloc::alloc::impl$1::allocate
// F238: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:248:0 alloc::boxed::box_new_uninit
// F239: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:286:0 alloc::boxed::impl$0::new
// F240: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:422:0 alloc::sync::Arc::new
// F241: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:95:0 alloc::alloc::alloc
// F242: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:289:0 alloc::boxed::impl$0::new
// F243: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:120:0 bevy_ecs::message::iterators::impl$3::last
// F244: crates/systems/roof/src/delete_roof.rs:27:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F245: crates/systems/roof/src/delete_roof.rs:28:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F246: crates/systems/roof/src/delete_roof.rs:30:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F247: crates/systems/roof/src/delete_roof.rs:51:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F248: crates/systems/roof/src/delete_roof.rs:52:0 system_roof::delete_roof::process_invalid_roof_notifications::ha359c5e68c3e24ca
// F249: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:250:0 alloc::boxed::box_new_uninit
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/delete_roof.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 8caf105b539fc41307fb1bd116487486d542a3e257197100fe7c6eaab65ca7ea
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: delete_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 268
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::delete_roof::process_invalid_roof_notifications
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":268,"record_offset":13128,"rva":7061600,"va":5375770720}
// 编译实例 RVAs: 0x6bc060
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-93110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375770720,"raw_hex":"e84a94feff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375771793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-93554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375770720,"raw_hex":"e88e92feff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375772253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7049591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375770720,"raw_hex":"e8896e94ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375771778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbeb30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9447848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375770720,"raw_hex":"e8a8299000","size":5,"soft_fail":false,"target_va":5385218864,"va":5375771011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40579259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375770720,"raw_hex":"e8bb306b02","size":5,"soft_fail":false,"target_va":5416350944,"va":5375771680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40848174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375770720,"raw_hex":"e82e4b6f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5375772590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4780329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375770720,"raw_hex":"e829f14800","size":5,"soft_fail":false,"target_va":5380552224,"va":5375771890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":268,"record_offset":18752,"parent_record_offset":13128,"ancestor_record_offset":13128,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":268,"record_offset":18696,"parent_record_offset":13128,"ancestor_record_offset":13128,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":268,"record_offset":13872,"parent_record_offset":13128,"ancestor_record_offset":13128,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":268,"record_offset":13308,"parent_record_offset":13128,"ancestor_record_offset":13128,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":268,"record_offset":18340,"parent_record_offset":13128,"ancestor_record_offset":13128,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":268,"record_offset":15304,"parent_record_offset":13128,"ancestor_record_offset":13128,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":14868,"parent_record_offset":13128,"ancestor_record_offset":13128,"inlinee":10312,"source_line":673,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":268,"record_offset":14784,"parent_record_offset":13128,"ancestor_record_offset":13128,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":268,"record_offset":22492,"parent_record_offset":13128,"ancestor_record_offset":13128,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":268,"record_offset":18140,"parent_record_offset":13128,"ancestor_record_offset":13128,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":268,"record_offset":18872,"parent_record_offset":18848,"ancestor_record_offset":13128,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":268,"record_offset":18896,"parent_record_offset":18872,"ancestor_record_offset":13128,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":268,"record_offset":22676,"parent_record_offset":22652,"ancestor_record_offset":13128,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":268,"record_offset":22700,"parent_record_offset":22676,"ancestor_record_offset":13128,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":268,"record_offset":18920,"parent_record_offset":18896,"ancestor_record_offset":13128,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":268,"record_offset":22724,"parent_record_offset":22700,"ancestor_record_offset":13128,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":268,"record_offset":18848,"parent_record_offset":18816,"ancestor_record_offset":13128,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":268,"record_offset":22652,"parent_record_offset":22564,"ancestor_record_offset":13128,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":268,"record_offset":18816,"parent_record_offset":18780,"ancestor_record_offset":13128,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":268,"record_offset":18780,"parent_record_offset":18752,"ancestor_record_offset":13128,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":15136,"parent_record_offset":15112,"ancestor_record_offset":13128,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":22592,"parent_record_offset":22564,"ancestor_record_offset":13128,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":22564,"parent_record_offset":22540,"ancestor_record_offset":13128,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":15184,"parent_record_offset":15160,"ancestor_record_offset":13128,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":15160,"parent_record_offset":15136,"ancestor_record_offset":13128,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":22540,"parent_record_offset":22516,"ancestor_record_offset":13128,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":18528,"parent_record_offset":18488,"ancestor_record_offset":13128,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":15112,"parent_record_offset":15088,"ancestor_record_offset":13128,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":13544,"parent_record_offset":13520,"ancestor_record_offset":13128,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":15636,"parent_record_offset":15612,"ancestor_record_offset":13128,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":15660,"parent_record_offset":15636,"ancestor_record_offset":13128,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":18460,"parent_record_offset":18424,"ancestor_record_offset":13128,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":18488,"parent_record_offset":18460,"ancestor_record_offset":13128,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":14808,"parent_record_offset":14784,"ancestor_record_offset":13128,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":13520,"parent_record_offset":13420,"ancestor_record_offset":13128,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":16420,"parent_record_offset":16396,"ancestor_record_offset":13128,"inlinee":7082,"source_line":661,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":16648,"parent_record_offset":15344,"ancestor_record_offset":13128,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":15612,"parent_record_offset":15344,"ancestor_record_offset":13128,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":15696,"parent_record_offset":15344,"ancestor_record_offset":13128,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":22252,"parent_record_offset":19824,"ancestor_record_offset":13128,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":15528,"parent_record_offset":15344,"ancestor_record_offset":13128,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":268,"record_offset":18396,"parent_record_offset":18364,"ancestor_record_offset":13128,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":268,"record_offset":18364,"parent_record_offset":18340,"ancestor_record_offset":13128,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":268,"record_offset":13420,"parent_record_offset":13388,"ancestor_record_offset":13128,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":268,"record_offset":13908,"parent_record_offset":13872,"ancestor_record_offset":13128,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":268,"record_offset":13388,"parent_record_offset":13340,"ancestor_record_offset":13128,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":268,"record_offset":19436,"parent_record_offset":19408,"ancestor_record_offset":13128,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":268,"record_offset":18424,"parent_record_offset":18364,"ancestor_record_offset":13128,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":20740,"parent_record_offset":20716,"ancestor_record_offset":13128,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::matches_component_set
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":16368,"parent_record_offset":16344,"ancestor_record_offset":13128,"inlinee":7078,"source_line":1577,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":15812,"parent_record_offset":15788,"ancestor_record_offset":13128,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":15836,"parent_record_offset":15812,"ancestor_record_offset":13128,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":20716,"parent_record_offset":20692,"ancestor_record_offset":13128,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":20764,"parent_record_offset":20740,"ancestor_record_offset":13128,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":16344,"parent_record_offset":15788,"ancestor_record_offset":13128,"inlinee":18914,"source_line":2203,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":16396,"parent_record_offset":16368,"ancestor_record_offset":13128,"inlinee":18916,"source_line":2209,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":17716,"parent_record_offset":17188,"ancestor_record_offset":13128,"inlinee":18910,"source_line":2219,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":17768,"parent_record_offset":17740,"ancestor_record_offset":13128,"inlinee":18912,"source_line":2224,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$56::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":16868,"parent_record_offset":16844,"ancestor_record_offset":13128,"inlinee":18918,"source_line":2277,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":16844,"parent_record_offset":15344,"ancestor_record_offset":13128,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":268,"record_offset":15004,"parent_record_offset":14892,"ancestor_record_offset":13128,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":268,"record_offset":15028,"parent_record_offset":15004,"ancestor_record_offset":13128,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":268,"record_offset":15344,"parent_record_offset":15304,"ancestor_record_offset":13128,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":268,"record_offset":15788,"parent_record_offset":15344,"ancestor_record_offset":13128,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":268,"record_offset":17188,"parent_record_offset":15344,"ancestor_record_offset":13128,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":268,"record_offset":16292,"parent_record_offset":16268,"ancestor_record_offset":13128,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":268,"record_offset":16468,"parent_record_offset":16444,"ancestor_record_offset":13128,"inlinee":7086,"source_line":47,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":268,"record_offset":15936,"parent_record_offset":15912,"ancestor_record_offset":13128,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":268,"record_offset":16444,"parent_record_offset":16420,"ancestor_record_offset":13128,"inlinee":7084,"source_line":398,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":268,"record_offset":15912,"parent_record_offset":15888,"ancestor_record_offset":13128,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":268,"record_offset":16116,"parent_record_offset":16092,"ancestor_record_offset":13128,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":15888,"parent_record_offset":15860,"ancestor_record_offset":13128,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":15860,"parent_record_offset":15836,"ancestor_record_offset":13128,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":17792,"parent_record_offset":17768,"ancestor_record_offset":13128,"inlinee":18913,"source_line":510,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":17104,"parent_record_offset":15344,"ancestor_record_offset":13128,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":22376,"parent_record_offset":22352,"ancestor_record_offset":13128,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":17020,"parent_record_offset":15344,"ancestor_record_offset":13128,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":268,"record_offset":16268,"parent_record_offset":16244,"ancestor_record_offset":13128,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":16244,"parent_record_offset":16220,"ancestor_record_offset":13128,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":14892,"parent_record_offset":14868,"ancestor_record_offset":13128,"inlinee":11849,"source_line":545,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":14944,"parent_record_offset":14920,"ancestor_record_offset":13128,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":14968,"parent_record_offset":14944,"ancestor_record_offset":13128,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":14920,"parent_record_offset":14892,"ancestor_record_offset":13128,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":268,"record_offset":13340,"parent_record_offset":13308,"ancestor_record_offset":13128,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":268,"record_offset":20788,"parent_record_offset":20764,"ancestor_record_offset":13128,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":268,"record_offset":16892,"parent_record_offset":16868,"ancestor_record_offset":13128,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":268,"record_offset":13948,"parent_record_offset":13908,"ancestor_record_offset":13128,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":268,"record_offset":14832,"parent_record_offset":14808,"ancestor_record_offset":13128,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":268,"record_offset":13648,"parent_record_offset":13624,"ancestor_record_offset":13128,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":268,"record_offset":13464,"parent_record_offset":13420,"ancestor_record_offset":13128,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":268,"record_offset":22620,"parent_record_offset":22592,"ancestor_record_offset":13128,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":268,"record_offset":16028,"parent_record_offset":15964,"ancestor_record_offset":13128,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":268,"record_offset":13976,"parent_record_offset":13948,"ancestor_record_offset":13128,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":268,"record_offset":16052,"parent_record_offset":16028,"ancestor_record_offset":13128,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":268,"record_offset":16584,"parent_record_offset":16560,"ancestor_record_offset":13128,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":268,"record_offset":16092,"parent_record_offset":15912,"ancestor_record_offset":13128,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":268,"record_offset":14036,"parent_record_offset":13908,"ancestor_record_offset":13128,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":268,"record_offset":22516,"parent_record_offset":22492,"ancestor_record_offset":13128,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":268,"record_offset":14676,"parent_record_offset":14652,"ancestor_record_offset":13128,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":268,"record_offset":15496,"parent_record_offset":15436,"ancestor_record_offset":13128,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":268,"record_offset":15468,"parent_record_offset":15436,"ancestor_record_offset":13128,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":268,"record_offset":18648,"parent_record_offset":18488,"ancestor_record_offset":13128,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":268,"record_offset":18164,"parent_record_offset":18140,"ancestor_record_offset":13128,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":268,"record_offset":18272,"parent_record_offset":18188,"ancestor_record_offset":13128,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":268,"record_offset":13576,"parent_record_offset":13420,"ancestor_record_offset":13128,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":268,"record_offset":22196,"parent_record_offset":19824,"ancestor_record_offset":13128,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":268,"record_offset":14628,"parent_record_offset":14160,"ancestor_record_offset":13128,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":268,"record_offset":15576,"parent_record_offset":15552,"ancestor_record_offset":13128,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":268,"record_offset":16172,"parent_record_offset":16140,"ancestor_record_offset":13128,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":268,"record_offset":13624,"parent_record_offset":13600,"ancestor_record_offset":13128,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":268,"record_offset":13600,"parent_record_offset":13576,"ancestor_record_offset":13128,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":268,"record_offset":14652,"parent_record_offset":14628,"ancestor_record_offset":13128,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":268,"record_offset":18188,"parent_record_offset":18164,"ancestor_record_offset":13128,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":268,"record_offset":15436,"parent_record_offset":15344,"ancestor_record_offset":13128,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":268,"record_offset":18296,"parent_record_offset":18272,"ancestor_record_offset":13128,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":268,"record_offset":22408,"parent_record_offset":22376,"ancestor_record_offset":13128,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":268,"record_offset":16140,"parent_record_offset":16116,"ancestor_record_offset":13128,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::delete_roof::mark_roof_for_deletion
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":268,"record_offset":7168,"rva":7060368,"va":5375769488}
// 编译实例 RVAs: 0x6bbb90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-91904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769488,"raw_hex":"e80099feff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375770603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40579961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769488,"raw_hex":"e879336b02","size":5,"soft_fail":false,"target_va":5416350496,"va":5375770530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40581015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769488,"raw_hex":"e897376b02","size":5,"soft_fail":false,"target_va":5416350720,"va":5375769700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40582466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769488,"raw_hex":"e8423d6b02","size":5,"soft_fail":false,"target_va":5416353040,"va":5375770569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5427043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769488,"raw_hex":"e863cf5200","size":5,"soft_fail":false,"target_va":5381197520,"va":5375770472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":7336,"parent_record_offset":7168,"ancestor_record_offset":7168,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":8072,"parent_record_offset":7168,"ancestor_record_offset":7168,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":268,"record_offset":8292,"parent_record_offset":7168,"ancestor_record_offset":7168,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":268,"record_offset":7364,"parent_record_offset":7168,"ancestor_record_offset":7168,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":268,"record_offset":8968,"parent_record_offset":7168,"ancestor_record_offset":7168,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":268,"record_offset":9356,"parent_record_offset":7168,"ancestor_record_offset":7168,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":268,"record_offset":12248,"parent_record_offset":7168,"ancestor_record_offset":7168,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":8596,"parent_record_offset":7168,"ancestor_record_offset":7168,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":268,"record_offset":12540,"parent_record_offset":7168,"ancestor_record_offset":7168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":268,"record_offset":12048,"parent_record_offset":7168,"ancestor_record_offset":7168,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":268,"record_offset":12728,"parent_record_offset":12704,"ancestor_record_offset":7168,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":268,"record_offset":12752,"parent_record_offset":12728,"ancestor_record_offset":7168,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":268,"record_offset":12776,"parent_record_offset":12752,"ancestor_record_offset":7168,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":268,"record_offset":12704,"parent_record_offset":12612,"ancestor_record_offset":7168,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":7168,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":12388,"parent_record_offset":12360,"ancestor_record_offset":7168,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":12472,"parent_record_offset":12448,"ancestor_record_offset":7168,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":12640,"parent_record_offset":12612,"ancestor_record_offset":7168,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":12612,"parent_record_offset":12588,"ancestor_record_offset":7168,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":12448,"parent_record_offset":12416,"ancestor_record_offset":7168,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":8216,"parent_record_offset":8192,"ancestor_record_offset":7168,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":8192,"parent_record_offset":8168,"ancestor_record_offset":7168,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":12416,"parent_record_offset":12388,"ancestor_record_offset":7168,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":12588,"parent_record_offset":12564,"ancestor_record_offset":7168,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":8144,"parent_record_offset":8104,"ancestor_record_offset":7168,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":8832,"parent_record_offset":8808,"ancestor_record_offset":7168,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":7700,"parent_record_offset":7676,"ancestor_record_offset":7168,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":9668,"parent_record_offset":9644,"ancestor_record_offset":7168,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":9692,"parent_record_offset":9668,"ancestor_record_offset":7168,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":9084,"parent_record_offset":9040,"ancestor_record_offset":7168,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":8104,"parent_record_offset":8072,"ancestor_record_offset":7168,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":12360,"parent_record_offset":12332,"ancestor_record_offset":7168,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":7676,"parent_record_offset":7452,"ancestor_record_offset":7168,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":11424,"parent_record_offset":9380,"ancestor_record_offset":7168,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":10436,"parent_record_offset":9380,"ancestor_record_offset":7168,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":10520,"parent_record_offset":9380,"ancestor_record_offset":7168,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":11676,"parent_record_offset":9380,"ancestor_record_offset":7168,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":10352,"parent_record_offset":9380,"ancestor_record_offset":7168,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":268,"record_offset":9320,"parent_record_offset":8996,"ancestor_record_offset":7168,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":268,"record_offset":7452,"parent_record_offset":7428,"ancestor_record_offset":7168,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":268,"record_offset":8320,"parent_record_offset":8292,"ancestor_record_offset":7168,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":268,"record_offset":7428,"parent_record_offset":7388,"ancestor_record_offset":7168,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":268,"record_offset":8996,"parent_record_offset":8968,"ancestor_record_offset":7168,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":268,"record_offset":9040,"parent_record_offset":8996,"ancestor_record_offset":7168,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":11752,"parent_record_offset":11728,"ancestor_record_offset":7168,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":10636,"parent_record_offset":10612,"ancestor_record_offset":7168,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":9752,"parent_record_offset":9728,"ancestor_record_offset":7168,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":11728,"parent_record_offset":11704,"ancestor_record_offset":7168,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":11776,"parent_record_offset":11752,"ancestor_record_offset":7168,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":11704,"parent_record_offset":9380,"ancestor_record_offset":7168,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":268,"record_offset":8736,"parent_record_offset":8620,"ancestor_record_offset":7168,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":268,"record_offset":8760,"parent_record_offset":8736,"ancestor_record_offset":7168,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":268,"record_offset":9380,"parent_record_offset":9356,"ancestor_record_offset":7168,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":268,"record_offset":10612,"parent_record_offset":9380,"ancestor_record_offset":7168,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":268,"record_offset":9728,"parent_record_offset":9380,"ancestor_record_offset":7168,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":268,"record_offset":10208,"parent_record_offset":10184,"ancestor_record_offset":7168,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":268,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":7168,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":268,"record_offset":9828,"parent_record_offset":9804,"ancestor_record_offset":7168,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":268,"record_offset":10032,"parent_record_offset":10008,"ancestor_record_offset":7168,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":11172,"parent_record_offset":9380,"ancestor_record_offset":7168,"inlinee":10328,"source_line":208,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":9804,"parent_record_offset":9776,"ancestor_record_offset":7168,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":9776,"parent_record_offset":9752,"ancestor_record_offset":7168,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":9644,"parent_record_offset":9380,"ancestor_record_offset":7168,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":11800,"parent_record_offset":11776,"ancestor_record_offset":7168,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":9560,"parent_record_offset":9380,"ancestor_record_offset":7168,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":268,"record_offset":10184,"parent_record_offset":10160,"ancestor_record_offset":7168,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":10160,"parent_record_offset":10136,"ancestor_record_offset":7168,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":268,"record_offset":12304,"parent_record_offset":12276,"ancestor_record_offset":7168,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":268,"record_offset":12332,"parent_record_offset":12304,"ancestor_record_offset":7168,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":268,"record_offset":12276,"parent_record_offset":12248,"ancestor_record_offset":7168,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":8676,"parent_record_offset":8652,"ancestor_record_offset":7168,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":7168,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":8620,"parent_record_offset":8596,"ancestor_record_offset":7168,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":8652,"parent_record_offset":8620,"ancestor_record_offset":7168,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":268,"record_offset":7388,"parent_record_offset":7364,"ancestor_record_offset":7168,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":268,"record_offset":11968,"parent_record_offset":11944,"ancestor_record_offset":7168,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":268,"record_offset":8388,"parent_record_offset":8360,"ancestor_record_offset":7168,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":268,"record_offset":8360,"parent_record_offset":8320,"ancestor_record_offset":7168,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":268,"record_offset":8468,"parent_record_offset":8444,"ancestor_record_offset":7168,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":268,"record_offset":7632,"parent_record_offset":7604,"ancestor_record_offset":7168,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":268,"record_offset":7492,"parent_record_offset":7452,"ancestor_record_offset":7168,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":268,"record_offset":12672,"parent_record_offset":12640,"ancestor_record_offset":7168,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":268,"record_offset":9944,"parent_record_offset":9880,"ancestor_record_offset":7168,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":268,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":7168,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":268,"record_offset":9968,"parent_record_offset":9944,"ancestor_record_offset":7168,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":268,"record_offset":10008,"parent_record_offset":9828,"ancestor_record_offset":7168,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":268,"record_offset":8444,"parent_record_offset":8360,"ancestor_record_offset":7168,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":268,"record_offset":12564,"parent_record_offset":12540,"ancestor_record_offset":7168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":268,"record_offset":7964,"parent_record_offset":7940,"ancestor_record_offset":7168,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":268,"record_offset":8544,"parent_record_offset":8520,"ancestor_record_offset":7168,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":268,"record_offset":9500,"parent_record_offset":9468,"ancestor_record_offset":7168,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":268,"record_offset":8256,"parent_record_offset":8104,"ancestor_record_offset":7168,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":268,"record_offset":12072,"parent_record_offset":12048,"ancestor_record_offset":7168,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":268,"record_offset":12180,"parent_record_offset":12096,"ancestor_record_offset":7168,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":268,"record_offset":7548,"parent_record_offset":7452,"ancestor_record_offset":7168,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":268,"record_offset":11368,"parent_record_offset":9380,"ancestor_record_offset":7168,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":268,"record_offset":7916,"parent_record_offset":7452,"ancestor_record_offset":7168,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":268,"record_offset":9608,"parent_record_offset":9584,"ancestor_record_offset":7168,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":268,"record_offset":10088,"parent_record_offset":10056,"ancestor_record_offset":7168,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":268,"record_offset":7604,"parent_record_offset":7576,"ancestor_record_offset":7168,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":268,"record_offset":7576,"parent_record_offset":7548,"ancestor_record_offset":7168,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":268,"record_offset":7940,"parent_record_offset":7916,"ancestor_record_offset":7168,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":268,"record_offset":12096,"parent_record_offset":12072,"ancestor_record_offset":7168,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":268,"record_offset":8520,"parent_record_offset":8468,"ancestor_record_offset":7168,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":268,"record_offset":12204,"parent_record_offset":12180,"ancestor_record_offset":7168,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":268,"record_offset":11824,"parent_record_offset":11800,"ancestor_record_offset":7168,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":268,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":7168,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::delete_roof::delete_roofs
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":268,"record_offset":268,"rva":7059024,"va":5375768144}
// 编译实例 RVAs: 0x6bb650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c921f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6121644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375768144,"raw_hex":"e8ac685d00","size":5,"soft_fail":false,"target_va":5381890544,"va":5375768895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca4490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6196114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375768144,"raw_hex":"e8928b5e00","size":5,"soft_fail":false,"target_va":5381964944,"va":5375768825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbb960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9437306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375768144,"raw_hex":"e87a009000","size":5,"soft_fail":false,"target_va":5385206112,"va":5375768801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8650002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375768144,"raw_hex":"e812fd8300","size":5,"soft_fail":false,"target_va":5384418976,"va":5375768969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":268,"record_offset":6724,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":268,"record_offset":848,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":268,"record_offset":7136,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":10329,"source_line":442,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":412,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":268,"record_offset":6696,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::rt::RtWorld::remove
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/mod.rs
//     evidence={"module":268,"record_offset":6780,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":18909,"source_line":199,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":268,"record_offset":704,"parent_record_offset":680,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":268,"record_offset":6996,"parent_record_offset":6972,"ancestor_record_offset":268,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":656,"parent_record_offset":632,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":632,"parent_record_offset":600,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":268,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":268,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":268,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":3980,"parent_record_offset":3956,"ancestor_record_offset":268,"inlinee":7082,"source_line":661,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":6492,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":3152,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":3236,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":5628,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":10336,"source_line":338,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":5660,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":268,"record_offset":3068,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":268,"record_offset":6748,"parent_record_offset":6724,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":5396,"parent_record_offset":5372,"ancestor_record_offset":268,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":1844,"parent_record_offset":1812,"ancestor_record_offset":268,"inlinee":7078,"source_line":1577,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":3364,"parent_record_offset":3328,"ancestor_record_offset":268,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":1308,"parent_record_offset":1272,"ancestor_record_offset":268,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":5372,"parent_record_offset":5344,"ancestor_record_offset":268,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":5420,"parent_record_offset":5396,"ancestor_record_offset":268,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":3896,"parent_record_offset":3328,"ancestor_record_offset":268,"inlinee":18914,"source_line":2203,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":3956,"parent_record_offset":3928,"ancestor_record_offset":268,"inlinee":18916,"source_line":2209,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":1812,"parent_record_offset":1272,"ancestor_record_offset":268,"inlinee":18910,"source_line":2219,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":1872,"parent_record_offset":1844,"ancestor_record_offset":268,"inlinee":18912,"source_line":2224,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$56::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":5512,"parent_record_offset":5344,"ancestor_record_offset":268,"inlinee":18918,"source_line":2277,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$95::fetch
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":268,"record_offset":5344,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":18908,"source_line":2520,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":268,"record_offset":548,"parent_record_offset":436,"ancestor_record_offset":268,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":268,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":268,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":268,"record_offset":880,"parent_record_offset":848,"ancestor_record_offset":268,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":268,"record_offset":3328,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":18906,"source_line":182,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":268,"record_offset":1272,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":18905,"source_line":195,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":268,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":268,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":268,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":268,"inlinee":7086,"source_line":47,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":268,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":268,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":268,"record_offset":1920,"parent_record_offset":1896,"ancestor_record_offset":268,"inlinee":7084,"source_line":398,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":268,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":268,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":268,"record_offset":1588,"parent_record_offset":1564,"ancestor_record_offset":268,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":6296,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":10328,"source_line":208,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":1360,"parent_record_offset":1332,"ancestor_record_offset":268,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":268,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":268,"inlinee":18913,"source_line":510,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":1188,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":5804,"parent_record_offset":5776,"ancestor_record_offset":268,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":1104,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":268,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":268,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":268,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":268,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":488,"parent_record_offset":464,"ancestor_record_offset":268,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":268,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":436,"parent_record_offset":412,"ancestor_record_offset":268,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":268,"record_offset":464,"parent_record_offset":436,"ancestor_record_offset":268,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":268,"record_offset":5444,"parent_record_offset":5420,"ancestor_record_offset":268,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":268,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":268,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":268,"record_offset":1500,"parent_record_offset":1436,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":268,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":268,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":268,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":268,"record_offset":1564,"parent_record_offset":1384,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":268,"record_offset":6924,"parent_record_offset":6780,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":268,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":268,"record_offset":1072,"parent_record_offset":1012,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":268,"record_offset":1044,"parent_record_offset":1012,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":268,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":268,"record_offset":5572,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":268,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":268,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":268,"record_offset":1644,"parent_record_offset":1612,"ancestor_record_offset":268,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":268,"record_offset":772,"parent_record_offset":600,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":268,"record_offset":1012,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":268,"record_offset":7032,"parent_record_offset":6996,"ancestor_record_offset":268,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":268,"record_offset":7056,"parent_record_offset":7032,"ancestor_record_offset":268,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":268,"record_offset":7088,"parent_record_offset":6996,"ancestor_record_offset":268,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":268,"record_offset":6832,"parent_record_offset":6808,"ancestor_record_offset":268,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":268,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":268,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":268,"record_offset":6884,"parent_record_offset":6856,"ancestor_record_offset":268,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":268,"record_offset":5840,"parent_record_offset":5804,"ancestor_record_offset":268,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":268,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":268,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00268.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":268,"record_offset":6808,"parent_record_offset":6780,"ancestor_record_offset":268,"inlinee":7238,"source_line":1350,"archive":"_pdb/C/modules/00268.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>

// ------------------------------------------------------------------
// 函数: system_roof::delete_roof::delete_roofs
// 信息 ID: 239b8f552ce49a2d1dc0b323
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a036a1adbc56c746e85782c5: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>)> @ 0xfd4110 -> 0x6bb650 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>)> > @ 0x1f2c561 -> 0x6bb650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::delete_roof::mark_roof_for_deletion
// 信息 ID: 42a98aa1b4a09ae221647163
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2def2f6ddc709910cdfe3344: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> @ 0xfd4379 -> 0x6bbb90 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)> > @ 0x1f2945d -> 0x6bbb90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::delete_roof::process_invalid_roof_notifications
// 信息 ID: 94e32d70de8c1724c3f8b7d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c84d717d4ec3c7d6f672c3d8: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>)> @ 0xee831 -> 0x6bc060 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>)> > @ 0x29fdf5 -> 0x6bc060 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
