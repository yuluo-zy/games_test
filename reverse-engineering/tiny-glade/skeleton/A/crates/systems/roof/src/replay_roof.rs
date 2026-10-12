// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: replay_roof.rs.assembly.json.gz

// FUNCTION system_roof::replay_roof::replay_roof
// Candidate interfaces: b91a709c3973e2ca043534a6
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 1943:248 -> rva_12e2240_len_4cd
//
// FUNCTION system_roof::replay_roof::impl$0::process
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 3202:248 -> rva_198df90_len_3f7
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_12e2240_len_4cd: RVA 0x12e2240, 1229 PDB bytes; 321 instructions
// Evidence: _assembly/roof-v1/A/pack-001/rva_12e2240_len_4cd.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 1943:248: system_roof::replay_roof::replay_roof
// Frames: F1
// 0x1412e2240  55                             pushq	%rbp
// 0x1412e2241  4157                           pushq	%r15
// 0x1412e2243  4156                           pushq	%r14
// 0x1412e2245  4155                           pushq	%r13
// 0x1412e2247  4154                           pushq	%r12
// 0x1412e2249  56                             pushq	%rsi
// 0x1412e224a  57                             pushq	%rdi
// 0x1412e224b  53                             pushq	%rbx
// 0x1412e224c  4881ecb8000000                 subq	$0xb8, %rsp
// 0x1412e2253  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x1412e225b  0f297520                       movaps	%xmm6, 0x20(%rbp)
// 0x1412e225f  48c74518feffffff               movq	$-0x2, 0x18(%rbp)
// 0x1412e2267  4c894d10                       movq	%r9, 0x10(%rbp)
// 0x1412e226b  4c89c3                         movq	%r8, %rbx
// 0x1412e226e  4989d5                         movq	%rdx, %r13
// 0x1412e2271  4889ce                         movq	%rcx, %rsi
// Frames: F2
// 0x1412e2274  4c8b39                         movq	(%rcx), %r15
// 0x1412e2277  4c89f9                         movq	%r15, %rcx
// 0x1412e227a  e811d93bff                     callq	0x14069fb90 <.text+0x69eb90>
// Frames: F3
// 0x1412e227f  f3410f107720                   movss	0x20(%r15), %xmm6
// Frames: F4 -> F5 -> F6
// 0x1412e2285  8b461c                         movl	0x1c(%rsi), %eax
// 0x1412e2288  488b4e10                       movq	0x10(%rsi), %rcx
// 0x1412e228c  8901                           movl	%eax, (%rcx)
// Frames: F6
// 0x1412e228e  4c89f9                         movq	%r15, %rcx
// 0x1412e2291  e8ca51beff                     callq	0x140ec7460 <.text+0xec6460>
// 0x1412e2296  4885c0                         testq	%rax, %rax
// 0x1412e2299  0f847b030000                   je	0x1412e261a <.text+0x12e161a>
// Frames: F7 -> F8
// 0x1412e229f  4889c6                         movq	%rax, %rsi
// 0x1412e22a2  48c745f800000000               movq	$0x0, -0x8(%rbp)
// 0x1412e22aa  48c7450008000000               movq	$0x8, (%rbp)
// 0x1412e22b2  48c7450800000000               movq	$0x0, 0x8(%rbp)
// Frames: F9
// 0x1412e22ba  4c8b7008                       movq	0x8(%rax), %r14
// 0x1412e22be  488b4010                       movq	0x10(%rax), %rax
// Frames: F10 -> F11 -> F12 -> F9
// 0x1412e22c2  488d0480                       leaq	(%rax,%rax,4), %rax
// 0x1412e22c6  498d04c6                       leaq	(%r14,%rax,8), %rax
// 0x1412e22ca  488945b8                       movq	%rax, -0x48(%rbp)
// 0x1412e22ce  4d8b6d00                       movq	(%r13), %r13
// 0x1412e22d2  31ff                           xorl	%edi, %edi
// 0x1412e22d4  49bf0500000000000080           movabsq	$-0x7ffffffffffffffb, %r15 # imm = 0x8000000000000005
// 0x1412e22de  eb0c                           jmp	0x1412e22ec <.text+0x12e12ec>
// Frames: F13 -> F14 -> F15 -> F16
// 0x1412e22e0  488b4500                       movq	(%rbp), %rax
// Frames: F17 -> F18 -> F16
// 0x1412e22e4  4c8924d0                       movq	%r12, (%rax,%rdx,8)
// Frames: F19 -> F16
// 0x1412e22e8  48ff4508                       incq	0x8(%rbp)
// Frames: F20 -> F21 -> F22 -> F9
// 0x1412e22ec  4c3b75b8                       cmpq	-0x48(%rbp), %r14
// Frames: F23 -> F22 -> F9
// 0x1412e22f0  746b                           je	0x1412e235d <.text+0x12e135d>
// Frames: F24 -> F25 -> F26 -> F22 -> F9
// 0x1412e22f2  4989fc                         movq	%rdi, %r12
// 0x1412e22f5  4c89f0                         movq	%r14, %rax
// 0x1412e22f8  4983c628                       addq	$0x28, %r14
// Frames: F27 -> F9
// 0x1412e22fc  48ffc7                         incq	%rdi
// Frames: F28
// 0x1412e22ff  4c3938                         cmpq	%r15, (%rax)
// 0x1412e2302  75e8                           jne	0x1412e22ec <.text+0x12e12ec>
// Frames: F29
// 0x1412e2304  488945c0                       movq	%rax, -0x40(%rbp)
// 0x1412e2308  488d4808                       leaq	0x8(%rax), %rcx
// 0x1412e230c  f3410f104d28                   movss	0x28(%r13), %xmm1
// Frames: F30
// 0x1412e2312  f30f59ce                       mulss	%xmm6, %xmm1
// Frames: F29
// 0x1412e2316  488b85a8000000                 movq	0xa8(%rbp), %rax
// 0x1412e231d  4889442428                     movq	%rax, 0x28(%rsp)
// 0x1412e2322  488b85a0000000                 movq	0xa0(%rbp), %rax
// 0x1412e2329  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1412e232e  4989d8                         movq	%rbx, %r8
// 0x1412e2331  4c8b4d10                       movq	0x10(%rbp), %r9
// 0x1412e2335  e856bc6a00                     callq	0x14198df90 <.text+0x198cf90>
// Frames: F31
// 0x1412e233a  488b45c0                       movq	-0x40(%rbp), %rax
// 0x1412e233e  80781d00                       cmpb	$0x0, 0x1d(%rax)
// 0x1412e2342  74a8                           je	0x1412e22ec <.text+0x12e12ec>
// Frames: F32 -> F16
// 0x1412e2344  488b5508                       movq	0x8(%rbp), %rdx
// 0x1412e2348  483b55f8                       cmpq	-0x8(%rbp), %rdx
// 0x1412e234c  7592                           jne	0x1412e22e0 <.text+0x12e12e0>
// Frames: F33 -> F16
// 0x1412e234e  488d4df8                       leaq	-0x8(%rbp), %rcx
// 0x1412e2352  e80979f9fe                     callq	0x140279c60 <.text+0x278c60>
// Frames: F15 -> F16
// 0x1412e2357  488b5508                       movq	0x8(%rbp), %rdx
// 0x1412e235b  eb83                           jmp	0x1412e22e0 <.text+0x12e12e0>
// Frames: F34
// 0x1412e235d  4c8b5500                       movq	(%rbp), %r10
// 0x1412e2361  488b4508                       movq	0x8(%rbp), %rax
// Frames: F35 -> F36 -> F34
// 0x1412e2365  4883f802                       cmpq	$0x2, %rax
// 0x1412e2369  4c8975c0                       movq	%r14, -0x40(%rbp)
// 0x1412e236d  731a                           jae	0x1412e2389 <.text+0x12e1389>
// Frames: F37
// 0x1412e236f  4c895510                       movq	%r10, 0x10(%rbp)
// 0x1412e2373  488b4df8                       movq	-0x8(%rbp), %rcx
// 0x1412e2377  48894dc8                       movq	%rcx, -0x38(%rbp)
// Frames: F20 -> F38 -> F37
// 0x1412e237b  4885c0                         testq	%rax, %rax
// Frames: F38 -> F37
// 0x1412e237e  0f85d3000000                   jne	0x1412e2457 <.text+0x12e1457>
// 0x1412e2384  e975020000                     jmp	0x1412e25fe <.text+0x12e15fe>
// Frames: F35 -> F36 -> F34
// 0x1412e2389  4889c1                         movq	%rax, %rcx
// 0x1412e238c  48d1e9                         shrq	%rcx
// 0x1412e238f  4883f808                       cmpq	$0x8, %rax
// 0x1412e2393  7304                           jae	0x1412e2399 <.text+0x12e1399>
// 0x1412e2395  31d2                           xorl	%edx, %edx
// 0x1412e2397  eb6d                           jmp	0x1412e2406 <.text+0x12e1406>
// 0x1412e2399  4889ca                         movq	%rcx, %rdx
// 0x1412e239c  4883e2fc                       andq	$-0x4, %rdx
// 0x1412e23a0  4d8d04c2                       leaq	(%r10,%rax,8), %r8
// 0x1412e23a4  4983c0f0                       addq	$-0x10, %r8
// 0x1412e23a8  4531c9                         xorl	%r9d, %r9d
// 0x1412e23ab  0f1f440000                     nopl	(%rax,%rax)
// Frames: F39 -> F40 -> F41 -> F42 -> F36 -> F34
// 0x1412e23b0  f3430f6f04ca                   movdqu	(%r10,%r9,8), %xmm0
// 0x1412e23b6  f3430f6f4cca10                 movdqu	0x10(%r10,%r9,8), %xmm1
// Frames: F39 -> F43 -> F41 -> F42 -> F36 -> F34
// 0x1412e23bd  f3410f6f50f0                   movdqu	-0x10(%r8), %xmm2
// 0x1412e23c3  f3410f6f18                     movdqu	(%r8), %xmm3
// 0x1412e23c8  660f70db4e                     pshufd	$0x4e, %xmm3, %xmm3     # xmm3 = xmm3[2,3,0,1]
// 0x1412e23cd  660f70d24e                     pshufd	$0x4e, %xmm2, %xmm2     # xmm2 = xmm2[2,3,0,1]
// Frames: F17 -> F44 -> F41 -> F42 -> F36 -> F34
// 0x1412e23d2  f3430f7f1cca                   movdqu	%xmm3, (%r10,%r9,8)
// 0x1412e23d8  f3430f7f54ca10                 movdqu	%xmm2, 0x10(%r10,%r9,8)
// Frames: F17 -> F45 -> F41 -> F42 -> F36 -> F34
// 0x1412e23df  660f70c04e                     pshufd	$0x4e, %xmm0, %xmm0     # xmm0 = xmm0[2,3,0,1]
// 0x1412e23e4  f3410f7f00                     movdqu	%xmm0, (%r8)
// 0x1412e23e9  660f70c14e                     pshufd	$0x4e, %xmm1, %xmm0     # xmm0 = xmm1[2,3,0,1]
// 0x1412e23ee  f3410f7f40f0                   movdqu	%xmm0, -0x10(%r8)
// Frames: F46 -> F36 -> F34
// 0x1412e23f4  4983c104                       addq	$0x4, %r9
// 0x1412e23f8  4983c0e0                       addq	$-0x20, %r8
// 0x1412e23fc  4c39ca                         cmpq	%r9, %rdx
// 0x1412e23ff  75af                           jne	0x1412e23b0 <.text+0x12e13b0>
// Frames: F35 -> F36 -> F34
// 0x1412e2401  4839d1                         cmpq	%rdx, %rcx
// 0x1412e2404  7434                           je	0x1412e243a <.text+0x12e143a>
// 0x1412e2406  48f7d9                         negq	%rcx
// 0x1412e2409  4d8d04d2                       leaq	(%r10,%rdx,8), %r8
// 0x1412e240d  48f7da                         negq	%rdx
// 0x1412e2410  498d04c2                       leaq	(%r10,%rax,8), %rax
// 0x1412e2414  4883c0f8                       addq	$-0x8, %rax
// 0x1412e2418  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F39 -> F40 -> F41 -> F42 -> F36 -> F34
// 0x1412e2420  4d8b08                         movq	(%r8), %r9
// Frames: F39 -> F43 -> F41 -> F42 -> F36 -> F34
// 0x1412e2423  4c8b14d0                       movq	(%rax,%rdx,8), %r10
// Frames: F17 -> F44 -> F41 -> F42 -> F36 -> F34
// 0x1412e2427  4d8910                         movq	%r10, (%r8)
// Frames: F17 -> F45 -> F41 -> F42 -> F36 -> F34
// 0x1412e242a  4c890cd0                       movq	%r9, (%rax,%rdx,8)
// Frames: F35 -> F36 -> F34
// 0x1412e242e  48ffca                         decq	%rdx
// 0x1412e2431  4983c008                       addq	$0x8, %r8
// 0x1412e2435  4839d1                         cmpq	%rdx, %rcx
// 0x1412e2438  75e6                           jne	0x1412e2420 <.text+0x12e1420>
// Frames: F37
// 0x1412e243a  488b4500                       movq	(%rbp), %rax
// 0x1412e243e  48894510                       movq	%rax, 0x10(%rbp)
// 0x1412e2442  488b4508                       movq	0x8(%rbp), %rax
// 0x1412e2446  488b4df8                       movq	-0x8(%rbp), %rcx
// 0x1412e244a  48894dc8                       movq	%rcx, -0x38(%rbp)
// Frames: F20 -> F38 -> F37
// 0x1412e244e  4885c0                         testq	%rax, %rax
// Frames: F38 -> F37
// 0x1412e2451  0f84a7010000                   je	0x1412e25fe <.text+0x12e15fe>
// 0x1412e2457  488b5d10                       movq	0x10(%rbp), %rbx
// 0x1412e245b  4c8d24c3                       leaq	(%rbx,%rax,8), %r12
// 0x1412e245f  49bd0000000000000080           movabsq	$-0x8000000000000000, %r13 # imm = 0x8000000000000000
// 0x1412e2469  41be04000000                   movl	$0x4, %r14d
// 0x1412e246f  488d3d6a7f2d01                 leaq	0x12d7f6a(%rip), %rdi   # 0x1425ba3e0
// 0x1412e2476  eb25                           jmp	0x1412e249d <.text+0x12e149d>
// Frames: F47 -> F48
// 0x1412e2478  488d4dd0                       leaq	-0x30(%rbp), %rcx
// 0x1412e247c  e8efd05eff                     callq	0x1408cf570 <.text+0x8ce570>
// 0x1412e2481  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F20 -> F38 -> F37
// 0x1412e2490  4883c308                       addq	$0x8, %rbx
// 0x1412e2494  4c39e3                         cmpq	%r12, %rbx
// Frames: F38 -> F37
// 0x1412e2497  0f8461010000                   je	0x1412e25fe <.text+0x12e15fe>
// Frames: F39 -> F49 -> F50 -> F37
// 0x1412e249d  488b0b                         movq	(%rbx), %rcx
// Frames: F51 -> F52 -> F48
// 0x1412e24a0  4c8b7e10                       movq	0x10(%rsi), %r15
// Frames: F53 -> F48
// 0x1412e24a4  4939cf                         cmpq	%rcx, %r15
// 0x1412e24a7  0f8685010000                   jbe	0x1412e2632 <.text+0x12e1632>
// Frames: F13 -> F14 -> F54 -> F48
// 0x1412e24ad  488b5608                       movq	0x8(%rsi), %rdx
// Frames: F55 -> F54 -> F48
// 0x1412e24b1  4c8d0489                       leaq	(%rcx,%rcx,4), %r8
// 0x1412e24b5  4a8d04c2                       leaq	(%rdx,%r8,8), %rax
// Frames: F39 -> F56 -> F48
// 0x1412e24b9  4e8b4cc220                     movq	0x20(%rdx,%r8,8), %r9
// 0x1412e24be  4c894df0                       movq	%r9, -0x10(%rbp)
// 0x1412e24c2  f3420f6f04c2                   movdqu	(%rdx,%r8,8), %xmm0
// 0x1412e24c8  420f104cc210                   movups	0x10(%rdx,%r8,8), %xmm1
// 0x1412e24ce  0f294de0                       movaps	%xmm1, -0x20(%rbp)
// 0x1412e24d2  660f7f45d0                     movdqa	%xmm0, -0x30(%rbp)
// Frames: F10 -> F57 -> F48
// 0x1412e24d7  4a8d14c2                       leaq	(%rdx,%r8,8), %rdx
// 0x1412e24db  4883c228                       addq	$0x28, %rdx
// Frames: F57 -> F48
// 0x1412e24df  48f7d1                         notq	%rcx
// 0x1412e24e2  4c01f9                         addq	%r15, %rcx
// Frames: F58 -> F57 -> F48
// 0x1412e24e5  48c1e103                       shlq	$0x3, %rcx
// 0x1412e24e9  4c8d0489                       leaq	(%rcx,%rcx,4), %r8
// 0x1412e24ed  4889c1                         movq	%rax, %rcx
// 0x1412e24f0  e8ab25e600                     callq	0x142144aa0 <.text+0x2143aa0>
// Frames: F59 -> F48
// 0x1412e24f5  49ffcf                         decq	%r15
// Frames: F60 -> F59 -> F48
// 0x1412e24f8  4c897e10                       movq	%r15, 0x10(%rsi)
// 0x1412e24fc  488b45d0                       movq	-0x30(%rbp), %rax
// Frames: F47 -> F48
// 0x1412e2500  4c31e8                         xorq	%r13, %rax
// 0x1412e2503  4883f809                       cmpq	$0x9, %rax
// 0x1412e2507  490f43c6                       cmovaeq	%r14, %rax
// 0x1412e250b  4883f807                       cmpq	$0x7, %rax
// 0x1412e250f  0f87cd000000                   ja	0x1412e25e2 <.text+0x12e15e2>
// 0x1412e2515  48630487                       movslq	(%rdi,%rax,4), %rax
// 0x1412e2519  4801f8                         addq	%rdi, %rax
// 0x1412e251c  ffe0                           jmpq	*%rax
// Frames: F61 -> F62 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e251e  488b45d8                       movq	-0x28(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e2522  f0                             lock
// 0x1412e2523  48ff08                         decq	(%rax)
// Frames: F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e2526  0f8564ffffff                   jne	0x1412e2490 <.text+0x12e1490>
// Frames: F66 -> F47 -> F47 -> F47 -> F48
// 0x1412e252c  488d4dd8                       leaq	-0x28(%rbp), %rcx
// 0x1412e2530  e89bdffefe                     callq	0x1402d04d0 <.text+0x2cf4d0>
// 0x1412e2535  e956ffffff                     jmp	0x1412e2490 <.text+0x12e1490>
// Frames: F61 -> F62 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e253a  488b45d8                       movq	-0x28(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e253e  f0                             lock
// 0x1412e253f  48ff08                         decq	(%rax)
// Frames: F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e2542  0f8548ffffff                   jne	0x1412e2490 <.text+0x12e1490>
// Frames: F66 -> F47 -> F47 -> F47 -> F48
// 0x1412e2548  488d4dd8                       leaq	-0x28(%rbp), %rcx
// 0x1412e254c  e87fdffefe                     callq	0x1402d04d0 <.text+0x2cf4d0>
// 0x1412e2551  e93affffff                     jmp	0x1412e2490 <.text+0x12e1490>
// Frames: F61 -> F62 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e2556  488b45d8                       movq	-0x28(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e255a  f0                             lock
// 0x1412e255b  48ff08                         decq	(%rax)
// Frames: F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e255e  0f852cffffff                   jne	0x1412e2490 <.text+0x12e1490>
// Frames: F66 -> F47 -> F47 -> F47 -> F48
// 0x1412e2564  488d4dd8                       leaq	-0x28(%rbp), %rcx
// 0x1412e2568  e863d0fefe                     callq	0x1402cf5d0 <.text+0x2ce5d0>
// 0x1412e256d  e91effffff                     jmp	0x1412e2490 <.text+0x12e1490>
// Frames: F61 -> F62 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e2572  488b45e0                       movq	-0x20(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e2576  f0                             lock
// 0x1412e2577  48ff08                         decq	(%rax)
// Frames: F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e257a  0f8510ffffff                   jne	0x1412e2490 <.text+0x12e1490>
// Frames: F66 -> F47 -> F47 -> F47 -> F48
// 0x1412e2580  488d4de0                       leaq	-0x20(%rbp), %rcx
// 0x1412e2584  e8f7c8fefe                     callq	0x1402cee80 <.text+0x2cde80>
// 0x1412e2589  e902ffffff                     jmp	0x1412e2490 <.text+0x12e1490>
// Frames: F61 -> F62 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e258e  488b45d8                       movq	-0x28(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e2592  f0                             lock
// 0x1412e2593  48ff08                         decq	(%rax)
// Frames: F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e2596  0f85f4feffff                   jne	0x1412e2490 <.text+0x12e1490>
// Frames: F66 -> F47 -> F47 -> F47 -> F48
// 0x1412e259c  488d4dd8                       leaq	-0x28(%rbp), %rcx
// 0x1412e25a0  e82bdffefe                     callq	0x1402d04d0 <.text+0x2cf4d0>
// 0x1412e25a5  e9e6feffff                     jmp	0x1412e2490 <.text+0x12e1490>
// Frames: F61 -> F62 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e25aa  488b45e0                       movq	-0x20(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e25ae  f0                             lock
// 0x1412e25af  48ff08                         decq	(%rax)
// Frames: F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e25b2  0f85d8feffff                   jne	0x1412e2490 <.text+0x12e1490>
// Frames: F66 -> F47 -> F47 -> F47 -> F48
// 0x1412e25b8  488d4de0                       leaq	-0x20(%rbp), %rcx
// 0x1412e25bc  e8cfe2fefe                     callq	0x1402d0890 <.text+0x2cf890>
// 0x1412e25c1  e9cafeffff                     jmp	0x1412e2490 <.text+0x12e1490>
// Frames: F61 -> F62 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e25c6  488b45e0                       movq	-0x20(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e25ca  f0                             lock
// 0x1412e25cb  48ff08                         decq	(%rax)
// Frames: F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e25ce  0f85bcfeffff                   jne	0x1412e2490 <.text+0x12e1490>
// Frames: F66 -> F47 -> F47 -> F47 -> F48
// 0x1412e25d4  488d4de0                       leaq	-0x20(%rbp), %rcx
// 0x1412e25d8  e8a3c8fefe                     callq	0x1402cee80 <.text+0x2cde80>
// 0x1412e25dd  e9aefeffff                     jmp	0x1412e2490 <.text+0x12e1490>
// Frames: F61 -> F62 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e25e2  488b45e0                       movq	-0x20(%rbp), %rax
// Frames: F64 -> F65 -> F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e25e6  f0                             lock
// 0x1412e25e7  48ff08                         decq	(%rax)
// Frames: F63 -> F47 -> F47 -> F47 -> F48
// 0x1412e25ea  0f85a0feffff                   jne	0x1412e2490 <.text+0x12e1490>
// Frames: F66 -> F47 -> F47 -> F47 -> F48
// 0x1412e25f0  488d4de0                       leaq	-0x20(%rbp), %rcx
// 0x1412e25f4  e887d2fefe                     callq	0x1402cf880 <.text+0x2ce880>
// 0x1412e25f9  e992feffff                     jmp	0x1412e2490 <.text+0x12e1490>
// Frames: F67 -> F68 -> F47 -> F69 -> F47 -> F70 -> F47 -> F37
// 0x1412e25fe  488b55c8                       movq	-0x38(%rbp), %rdx
// 0x1412e2602  4885d2                         testq	%rdx, %rdx
// 0x1412e2605  7413                           je	0x1412e261a <.text+0x12e161a>
// Frames: F71 -> F72 -> F68 -> F47 -> F69 -> F47 -> F70 -> F47 -> F37
// 0x1412e2607  48c1e203                       shlq	$0x3, %rdx
// Frames: F73 -> F74 -> F75 -> F47 -> F69 -> F47 -> F70 -> F47 -> F37
// 0x1412e260b  41b808000000                   movl	$0x8, %r8d
// 0x1412e2611  488b4d10                       movq	0x10(%rbp), %rcx
// 0x1412e2615  e876810fff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F76
// 0x1412e261a  0f287520                       movaps	0x20(%rbp), %xmm6
// 0x1412e261e  4881c4b8000000                 addq	$0xb8, %rsp
// 0x1412e2625  5b                             popq	%rbx
// 0x1412e2626  5f                             popq	%rdi
// 0x1412e2627  5e                             popq	%rsi
// 0x1412e2628  415c                           popq	%r12
// 0x1412e262a  415d                           popq	%r13
// 0x1412e262c  415e                           popq	%r14
// 0x1412e262e  415f                           popq	%r15
// 0x1412e2630  5d                             popq	%rbp
// 0x1412e2631  c3                             retq
// Frames: F77 -> F48
// 0x1412e2632  4c8d058f7d2d01                 leaq	0x12d7d8f(%rip), %r8    # 0x1425ba3c8
// 0x1412e2639  4c89fa                         movq	%r15, %rdx
// 0x1412e263c  e83fc2dc00                     callq	0x1420ae880 <.text+0x20ad880>
// 0x1412e2641  0f0b                           ud2
// 0x1412e2643  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x1412e2650  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1412e2655  55                             pushq	%rbp
// 0x1412e2656  4157                           pushq	%r15
// 0x1412e2658  4156                           pushq	%r14
// 0x1412e265a  4155                           pushq	%r13
// 0x1412e265c  4154                           pushq	%r12
// 0x1412e265e  56                             pushq	%rsi
// 0x1412e265f  57                             pushq	%rdi
// 0x1412e2660  53                             pushq	%rbx
// 0x1412e2661  4883ec48                       subq	$0x48, %rsp
// 0x1412e2665  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1412e266c  0f29742430                     movaps	%xmm6, 0x30(%rsp)
// Frames: F67 -> F68 -> F47 -> F69 -> F47 -> F70 -> F47 -> F37
// 0x1412e2671  48837dc800                     cmpq	$0x0, -0x38(%rbp)
// 0x1412e2676  7417                           je	0x1412e268f <.text+0x12e168f>
// Frames: F71 -> F72 -> F68 -> F47 -> F69 -> F47 -> F70 -> F47 -> F37
// 0x1412e2678  488b55c8                       movq	-0x38(%rbp), %rdx
// 0x1412e267c  48c1e203                       shlq	$0x3, %rdx
// Frames: F73 -> F74 -> F75 -> F47 -> F69 -> F47 -> F70 -> F47 -> F37
// 0x1412e2680  41b808000000                   movl	$0x8, %r8d
// 0x1412e2686  488b4d10                       movq	0x10(%rbp), %rcx
// 0x1412e268a  e801810fff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F37
// 0x1412e268f  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x1412e2694  4883c448                       addq	$0x48, %rsp
// 0x1412e2698  5b                             popq	%rbx
// 0x1412e2699  5f                             popq	%rdi
// 0x1412e269a  5e                             popq	%rsi
// 0x1412e269b  415c                           popq	%r12
// 0x1412e269d  415d                           popq	%r13
// 0x1412e269f  415e                           popq	%r14
// 0x1412e26a1  415f                           popq	%r15
// 0x1412e26a3  5d                             popq	%rbp
// 0x1412e26a4  c3                             retq
// 0x1412e26a5  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// 0x1412e26b0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1412e26b5  55                             pushq	%rbp
// 0x1412e26b6  4157                           pushq	%r15
// 0x1412e26b8  4156                           pushq	%r14
// 0x1412e26ba  4155                           pushq	%r13
// 0x1412e26bc  4154                           pushq	%r12
// 0x1412e26be  56                             pushq	%rsi
// 0x1412e26bf  57                             pushq	%rdi
// 0x1412e26c0  53                             pushq	%rbx
// 0x1412e26c1  4883ec48                       subq	$0x48, %rsp
// 0x1412e26c5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1412e26cc  0f29742430                     movaps	%xmm6, 0x30(%rsp)
// 0x1412e26d1  488b45c0                       movq	-0x40(%rbp), %rax
// Frames: F20 -> F21 -> F22 -> F9
// 0x1412e26d5  483b45b8                       cmpq	-0x48(%rbp), %rax
// Frames: F78
// 0x1412e26d9  741c                           je	0x1412e26f7 <.text+0x12e16f7>
// Frames: F47 -> F78
// 0x1412e26db  488b55f8                       movq	-0x8(%rbp), %rdx
// Frames: F67 -> F68 -> F47 -> F47 -> F78
// 0x1412e26df  4885d2                         testq	%rdx, %rdx
// 0x1412e26e2  7413                           je	0x1412e26f7 <.text+0x12e16f7>
// Frames: F47 -> F78
// 0x1412e26e4  488b4d00                       movq	(%rbp), %rcx
// Frames: F71 -> F72 -> F68 -> F47 -> F47 -> F78
// 0x1412e26e8  48c1e203                       shlq	$0x3, %rdx
// Frames: F73 -> F74 -> F75 -> F47 -> F47 -> F78
// 0x1412e26ec  41b808000000                   movl	$0x8, %r8d
// 0x1412e26f2  e899800fff                     callq	0x1403da790 <.text+0x3d9790>
// Frames: F1
// 0x1412e26f7  0f28742430                     movaps	0x30(%rsp), %xmm6
// 0x1412e26fc  4883c448                       addq	$0x48, %rsp
// 0x1412e2700  5b                             popq	%rbx
// 0x1412e2701  5f                             popq	%rdi
// 0x1412e2702  5e                             popq	%rsi
// 0x1412e2703  415c                           popq	%r12
// 0x1412e2705  415d                           popq	%r13
// 0x1412e2707  415e                           popq	%r14
// 0x1412e2709  415f                           popq	%r15
// 0x1412e270b  5d                             popq	%rbp
// 0x1412e270c  c3                             retq
//
// RANGE rva_198df90_len_3f7: RVA 0x198df90, 1015 PDB bytes; 243 instructions
// Evidence: _assembly/roof-v1/A/pack-003/rva_198df90_len_3f7.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3202:248: system_roof::replay_roof::impl$0::process
// Frames: F79
// 0x14198df90  4157                           pushq	%r15
// 0x14198df92  4156                           pushq	%r14
// 0x14198df94  56                             pushq	%rsi
// 0x14198df95  57                             pushq	%rdi
// 0x14198df96  53                             pushq	%rbx
// 0x14198df97  4881ec90000000                 subq	$0x90, %rsp
// 0x14198df9e  440f29942480000000             movaps	%xmm10, 0x80(%rsp)
// 0x14198dfa7  440f294c2470                   movaps	%xmm9, 0x70(%rsp)
// 0x14198dfad  440f29442460                   movaps	%xmm8, 0x60(%rsp)
// 0x14198dfb3  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x14198dfb8  0f29742440                     movaps	%xmm6, 0x40(%rsp)
// Frames: F80
// 0x14198dfbd  488b4108                       movq	0x8(%rcx), %rax
// 0x14198dfc1  0fb65010                       movzbl	0x10(%rax), %edx
// 0x14198dfc5  4c8d15945bd600                 leaq	0xd65b94(%rip), %r10    # 0x1426f3b60
// 0x14198dfcc  49631492                       movslq	(%r10,%rdx,4), %rdx
// 0x14198dfd0  4c01d2                         addq	%r10, %rdx
// 0x14198dfd3  ffe2                           jmpq	*%rdx
// Frames: F81
// 0x14198dfd5  80791400                       cmpb	$0x0, 0x14(%rcx)
// 0x14198dfd9  0f8523030000                   jne	0x14198e302 <.text+0x198d302>
// Frames: F82
// 0x14198dfdf  488b7038                       movq	0x38(%rax), %rsi
// Frames: F83
// 0x14198dfe3  0f104018                       movups	0x18(%rax), %xmm0
// 0x14198dfe7  0f104824                       movups	0x24(%rax), %xmm1
// 0x14198dfeb  0f114c242c                     movups	%xmm1, 0x2c(%rsp)
// 0x14198dff0  0f29442420                     movaps	%xmm0, 0x20(%rsp)
// Frames: F84
// 0x14198dff5  8b5814                         movl	0x14(%rax), %ebx
// Frames: F85 -> F5 -> F86 -> F87
// 0x14198dff8  418b401c                       movl	0x1c(%r8), %eax
// 0x14198dffc  498b5010                       movq	0x10(%r8), %rdx
// 0x14198e000  8902                           movl	%eax, (%rdx)
// Frames: F88 -> F86 -> F87
// 0x14198e002  498b38                         movq	(%r8), %rdi
// Frames: F32 -> F89 -> F86 -> F87
// 0x14198e005  488b5730                       movq	0x30(%rdi), %rdx
// Frames: F90 -> F86 -> F87
// 0x14198e009  4c8b7740                       movq	0x40(%rdi), %r14
// Frames: F32 -> F89 -> F86 -> F87
// 0x14198e00d  483b5720                       cmpq	0x20(%rdi), %rdx
// 0x14198e011  7516                           jne	0x14198e029 <.text+0x198d029>
// Frames: F89 -> F86 -> F87
// 0x14198e013  488d4720                       leaq	0x20(%rdi), %rax
// 0x14198e017  4989cf                         movq	%rcx, %r15
// Frames: F33 -> F89 -> F86 -> F87
// 0x14198e01a  4889c1                         movq	%rax, %rcx
// 0x14198e01d  e8deca8efe                     callq	0x14027ab00 <.text+0x279b00>
// 0x14198e022  4c89f9                         movq	%r15, %rcx
// Frames: F15 -> F89 -> F86 -> F87
// 0x14198e025  488b5730                       movq	0x30(%rdi), %rdx
// Frames: F13 -> F14 -> F15 -> F89 -> F86 -> F87
// 0x14198e029  488b4728                       movq	0x28(%rdi), %rax
// Frames: F55 -> F15 -> F89 -> F86 -> F87
// 0x14198e02d  486bd238                       imulq	$0x38, %rdx, %rdx
// Frames: F17 -> F18 -> F89 -> F86 -> F87
// 0x14198e031  4c893410                       movq	%r14, (%rax,%rdx)
// 0x14198e035  4889741008                     movq	%rsi, 0x8(%rax,%rdx)
// 0x14198e03a  895c1010                       movl	%ebx, 0x10(%rax,%rdx)
// 0x14198e03e  0f28442420                     movaps	0x20(%rsp), %xmm0
// 0x14198e043  0f11441014                     movups	%xmm0, 0x14(%rax,%rdx)
// 0x14198e048  0f1044242c                     movups	0x2c(%rsp), %xmm0
// 0x14198e04d  0f11441020                     movups	%xmm0, 0x20(%rax,%rdx)
// 0x14198e052  c644103000                     movb	$0x0, 0x30(%rax,%rdx)
// 0x14198e057  e9f1020000                     jmp	0x14198e34d <.text+0x198d34d>
// Frames: F91
// 0x14198e05c  833900                         cmpl	$0x0, (%rcx)
// 0x14198e05f  742c                           je	0x14198e08d <.text+0x198d08d>
// Frames: F92
// 0x14198e061  f30f584910                     addss	0x10(%rcx), %xmm1
// 0x14198e066  f30f114910                     movss	%xmm1, 0x10(%rcx)
// Frames: F93
// 0x14198e06b  f30f5e4904                     divss	0x4(%rcx), %xmm1
// 0x14198e070  0f57c0                         xorps	%xmm0, %xmm0
// Frames: F94 -> F93
// 0x14198e073  f30f5fc1                       maxss	%xmm1, %xmm0
// 0x14198e077  f30f100d41a77b00               movss	0x7ba741(%rip), %xmm1   # 0x1421487c0
// Frames: F95 -> F93
// 0x14198e07f  0f28f1                         movaps	%xmm1, %xmm6
// 0x14198e082  f30f5df0                       minss	%xmm0, %xmm6
// Frames: F96
// 0x14198e086  0f2ef1                         ucomiss	%xmm1, %xmm6
// 0x14198e089  750e                           jne	0x14198e099 <.text+0x198d099>
// 0x14198e08b  7a0c                           jp	0x14198e099 <.text+0x198d099>
// Frames: F97
// 0x14198e08d  c6411501                       movb	$0x1, 0x15(%rcx)
// 0x14198e091  f30f103527a77b00               movss	0x7ba727(%rip), %xmm6   # 0x1421487c0
// Frames: F98
// 0x14198e099  488b9424e0000000               movq	0xe0(%rsp), %rdx
// 0x14198e0a1  80791400                       cmpb	$0x0, 0x14(%rcx)
// 0x14198e0a5  f30f100513a77b00               movss	0x7ba713(%rip), %xmm0   # 0x1421487c0
// 0x14198e0ad  740a                           je	0x14198e0b9 <.text+0x198d0b9>
// 0x14198e0af  0f28c8                         movaps	%xmm0, %xmm1
// 0x14198e0b2  f30f5cce                       subss	%xmm6, %xmm1
// 0x14198e0b6  0f28f1                         movaps	%xmm1, %xmm6
// Frames: F99
// 0x14198e0b9  488b7038                       movq	0x38(%rax), %rsi
// Frames: F100 -> F101 -> F102
// 0x14198e0bd  f30f5cc6                       subss	%xmm6, %xmm0
// 0x14198e0c1  f30f104818                     movss	0x18(%rax), %xmm1
// Frames: F103 -> F101 -> F102
// 0x14198e0c6  f30f59c8                       mulss	%xmm0, %xmm1
// 0x14198e0ca  f30f107828                     movss	0x28(%rax), %xmm7
// 0x14198e0cf  f30f59fe                       mulss	%xmm6, %xmm7
// Frames: F104 -> F101 -> F102
// 0x14198e0d3  f30f58f9                       addss	%xmm1, %xmm7
// 0x14198e0d7  f30f104814                     movss	0x14(%rax), %xmm1
// Frames: F103 -> F101 -> F105
// 0x14198e0dc  f30f59c8                       mulss	%xmm0, %xmm1
// 0x14198e0e0  f3440f104024                   movss	0x24(%rax), %xmm8
// 0x14198e0e6  f3440f59c6                     mulss	%xmm6, %xmm8
// Frames: F104 -> F101 -> F105
// 0x14198e0eb  f3440f58c1                     addss	%xmm1, %xmm8
// 0x14198e0f0  f20f104840                     movsd	0x40(%rax), %xmm1
// 0x14198e0f5  f20f105048                     movsd	0x48(%rax), %xmm2
// Frames: F106 -> F107 -> F108 -> F109
// 0x14198e0fa  0f5cd1                         subps	%xmm1, %xmm2
// Frames: F110 -> F111 -> F108 -> F109
// 0x14198e0fd  f3440f12ce                     movsldup	%xmm6, %xmm9            # xmm9 = xmm6[0,0,2,2]
// 0x14198e102  440f59ca                       mulps	%xmm2, %xmm9
// Frames: F112 -> F113 -> F108 -> F109
// 0x14198e106  440f58c9                       addps	%xmm1, %xmm9
// 0x14198e10a  f30f10481c                     movss	0x1c(%rax), %xmm1
// Frames: F103 -> F101 -> F114
// 0x14198e10f  f30f59c8                       mulss	%xmm0, %xmm1
// 0x14198e113  f3440f10502c                   movss	0x2c(%rax), %xmm10
// 0x14198e119  f3440f59d6                     mulss	%xmm6, %xmm10
// Frames: F103 -> F101 -> F115
// 0x14198e11e  f30f594020                     mulss	0x20(%rax), %xmm0
// Frames: F104 -> F101 -> F114
// 0x14198e123  f3440f58d1                     addss	%xmm1, %xmm10
// Frames: F103 -> F101 -> F115
// 0x14198e128  f30f597030                     mulss	0x30(%rax), %xmm6
// Frames: F104 -> F101 -> F115
// 0x14198e12d  f30f58f0                       addss	%xmm0, %xmm6
// Frames: F85 -> F5 -> F86 -> F116
// 0x14198e131  8b421c                         movl	0x1c(%rdx), %eax
// 0x14198e134  488b4a10                       movq	0x10(%rdx), %rcx
// 0x14198e138  8901                           movl	%eax, (%rcx)
// Frames: F88 -> F86 -> F116
// 0x14198e13a  488b3a                         movq	(%rdx), %rdi
// Frames: F32 -> F89 -> F86 -> F116
// 0x14198e13d  488b5730                       movq	0x30(%rdi), %rdx
// Frames: F90 -> F86 -> F116
// 0x14198e141  488b5f40                       movq	0x40(%rdi), %rbx
// Frames: F32 -> F89 -> F86 -> F116
// 0x14198e145  483b5720                       cmpq	0x20(%rdi), %rdx
// 0x14198e149  750d                           jne	0x14198e158 <.text+0x198d158>
// Frames: F89 -> F86 -> F116
// 0x14198e14b  488d4f20                       leaq	0x20(%rdi), %rcx
// Frames: F33 -> F89 -> F86 -> F116
// 0x14198e14f  e82cbd8efe                     callq	0x140279e80 <.text+0x278e80>
// Frames: F15 -> F89 -> F86 -> F116
// 0x14198e154  488b5730                       movq	0x30(%rdi), %rdx
// Frames: F13 -> F14 -> F15 -> F89 -> F86 -> F116
// 0x14198e158  488b4728                       movq	0x28(%rdi), %rax
// Frames: F55 -> F15 -> F89 -> F86 -> F116
// 0x14198e15c  48c1e206                       shlq	$0x6, %rdx
// Frames: F17 -> F18 -> F89 -> F86 -> F116
// 0x14198e160  c7041001000000                 movl	$0x1, (%rax,%rdx)
// 0x14198e167  f30f117c1004                   movss	%xmm7, 0x4(%rax,%rdx)
// 0x14198e16d  c744100801000000               movl	$0x1, 0x8(%rax,%rdx)
// 0x14198e175  f3440f1144100c                 movss	%xmm8, 0xc(%rax,%rdx)
// 0x14198e17c  c744101001000000               movl	$0x1, 0x10(%rax,%rdx)
// 0x14198e184  f3440f11541014                 movss	%xmm10, 0x14(%rax,%rdx)
// 0x14198e18b  c744101801000000               movl	$0x1, 0x18(%rax,%rdx)
// 0x14198e193  f30f1174101c                   movss	%xmm6, 0x1c(%rax,%rdx)
// 0x14198e199  4889741020                     movq	%rsi, 0x20(%rax,%rdx)
// 0x14198e19e  c744102801000000               movl	$0x1, 0x28(%rax,%rdx)
// 0x14198e1a6  440f134c102c                   movlps	%xmm9, 0x2c(%rax,%rdx)
// 0x14198e1ac  c644103401                     movb	$0x1, 0x34(%rax,%rdx)
// 0x14198e1b1  48895c1038                     movq	%rbx, 0x38(%rax,%rdx)
// Frames: F19 -> F89 -> F86 -> F116
// 0x14198e1b6  48ff4730                       incq	0x30(%rdi)
// Frames: F117 -> F86 -> F116
// 0x14198e1ba  48ff4740                       incq	0x40(%rdi)
// 0x14198e1be  e996010000                     jmp	0x14198e359 <.text+0x198d359>
// Frames: F118
// 0x14198e1c3  f30f107110                     movss	0x10(%rcx), %xmm6
// 0x14198e1c8  0f57c0                         xorps	%xmm0, %xmm0
// 0x14198e1cb  0f2ef0                         ucomiss	%xmm0, %xmm6
// 0x14198e1ce  7571                           jne	0x14198e241 <.text+0x198d241>
// 0x14198e1d0  7a6f                           jp	0x14198e241 <.text+0x198d241>
// Frames: F119
// 0x14198e1d2  488b9424e8000000               movq	0xe8(%rsp), %rdx
// 0x14198e1da  80791401                       cmpb	$0x1, 0x14(%rcx)
// 0x14198e1de  4989c0                         movq	%rax, %r8
// 0x14198e1e1  4983d011                       adcq	$0x11, %r8
// 0x14198e1e5  410fb618                       movzbl	(%r8), %ebx
// Frames: F120
// 0x14198e1e9  488b7018                       movq	0x18(%rax), %rsi
// Frames: F85 -> F5 -> F86 -> F121
// 0x14198e1ed  8b421c                         movl	0x1c(%rdx), %eax
// 0x14198e1f0  4c8b4210                       movq	0x10(%rdx), %r8
// 0x14198e1f4  418900                         movl	%eax, (%r8)
// Frames: F88 -> F86 -> F121
// 0x14198e1f7  488b3a                         movq	(%rdx), %rdi
// Frames: F32 -> F89 -> F86 -> F121
// 0x14198e1fa  488b5730                       movq	0x30(%rdi), %rdx
// Frames: F90 -> F86 -> F121
// 0x14198e1fe  4c8b7740                       movq	0x40(%rdi), %r14
// Frames: F32 -> F89 -> F86 -> F121
// 0x14198e202  483b5720                       cmpq	0x20(%rdi), %rdx
// 0x14198e206  751c                           jne	0x14198e224 <.text+0x198d224>
// Frames: F89 -> F86 -> F121
// 0x14198e208  488d4720                       leaq	0x20(%rdi), %rax
// 0x14198e20c  4989cf                         movq	%rcx, %r15
// Frames: F33 -> F89 -> F86 -> F121
// 0x14198e20f  4889c1                         movq	%rax, %rcx
// 0x14198e212  0f28f9                         movaps	%xmm1, %xmm7
// 0x14198e215  e886c18efe                     callq	0x14027a3a0 <.text+0x2793a0>
// 0x14198e21a  0f28cf                         movaps	%xmm7, %xmm1
// 0x14198e21d  4c89f9                         movq	%r15, %rcx
// Frames: F15 -> F89 -> F86 -> F121
// 0x14198e220  488b5730                       movq	0x30(%rdi), %rdx
// Frames: F13 -> F14 -> F15 -> F89 -> F86 -> F121
// 0x14198e224  488b4728                       movq	0x28(%rdi), %rax
// Frames: F55 -> F15 -> F89 -> F86 -> F121
// 0x14198e228  488d1452                       leaq	(%rdx,%rdx,2), %rdx
// Frames: F17 -> F18 -> F89 -> F86 -> F121
// 0x14198e22c  4c8934d0                       movq	%r14, (%rax,%rdx,8)
// 0x14198e230  488974d008                     movq	%rsi, 0x8(%rax,%rdx,8)
// 0x14198e235  885cd010                       movb	%bl, 0x10(%rax,%rdx,8)
// Frames: F19 -> F89 -> F86 -> F121
// 0x14198e239  48ff4730                       incq	0x30(%rdi)
// Frames: F117 -> F86 -> F121
// 0x14198e23d  48ff4740                       incq	0x40(%rdi)
// Frames: F122
// 0x14198e241  833900                         cmpl	$0x0, (%rcx)
// 0x14198e244  0f840b010000                   je	0x14198e355 <.text+0x198d355>
// Frames: F123
// 0x14198e24a  f30f58f1                       addss	%xmm1, %xmm6
// 0x14198e24e  f30f117110                     movss	%xmm6, 0x10(%rcx)
// Frames: F124
// 0x14198e253  f30f5e7104                     divss	0x4(%rcx), %xmm6
// 0x14198e258  0f57c0                         xorps	%xmm0, %xmm0
// Frames: F94 -> F124
// 0x14198e25b  f30f5fc6                       maxss	%xmm6, %xmm0
// 0x14198e25f  f30f100d59a57b00               movss	0x7ba559(%rip), %xmm1   # 0x1421487c0
// Frames: F95 -> F124
// 0x14198e267  0f28d1                         movaps	%xmm1, %xmm2
// 0x14198e26a  f30f5dd0                       minss	%xmm0, %xmm2
// Frames: F125
// 0x14198e26e  0f2ed1                         ucomiss	%xmm1, %xmm2
// 0x14198e271  0f85e2000000                   jne	0x14198e359 <.text+0x198d359>
// 0x14198e277  0f8bd8000000                   jnp	0x14198e355 <.text+0x198d355>
// 0x14198e27d  e9d7000000                     jmp	0x14198e359 <.text+0x198d359>
// Frames: F126
// 0x14198e282  80791400                       cmpb	$0x0, 0x14(%rcx)
// 0x14198e286  747a                           je	0x14198e302 <.text+0x198d302>
// Frames: F127
// 0x14198e288  488b7038                       movq	0x38(%rax), %rsi
// Frames: F128
// 0x14198e28c  0f104018                       movups	0x18(%rax), %xmm0
// 0x14198e290  0f104824                       movups	0x24(%rax), %xmm1
// 0x14198e294  0f114c242c                     movups	%xmm1, 0x2c(%rsp)
// 0x14198e299  0f29442420                     movaps	%xmm0, 0x20(%rsp)
// Frames: F129
// 0x14198e29e  8b5814                         movl	0x14(%rax), %ebx
// Frames: F85 -> F5 -> F86 -> F130
// 0x14198e2a1  418b401c                       movl	0x1c(%r8), %eax
// 0x14198e2a5  498b5010                       movq	0x10(%r8), %rdx
// 0x14198e2a9  8902                           movl	%eax, (%rdx)
// Frames: F88 -> F86 -> F130
// 0x14198e2ab  498b38                         movq	(%r8), %rdi
// Frames: F32 -> F89 -> F86 -> F130
// 0x14198e2ae  488b5730                       movq	0x30(%rdi), %rdx
// Frames: F90 -> F86 -> F130
// 0x14198e2b2  4c8b7740                       movq	0x40(%rdi), %r14
// Frames: F32 -> F89 -> F86 -> F130
// 0x14198e2b6  483b5720                       cmpq	0x20(%rdi), %rdx
// 0x14198e2ba  7516                           jne	0x14198e2d2 <.text+0x198d2d2>
// Frames: F89 -> F86 -> F130
// 0x14198e2bc  488d4720                       leaq	0x20(%rdi), %rax
// 0x14198e2c0  4989cf                         movq	%rcx, %r15
// Frames: F33 -> F89 -> F86 -> F130
// 0x14198e2c3  4889c1                         movq	%rax, %rcx
// 0x14198e2c6  e835c88efe                     callq	0x14027ab00 <.text+0x279b00>
// 0x14198e2cb  4c89f9                         movq	%r15, %rcx
// Frames: F15 -> F89 -> F86 -> F130
// 0x14198e2ce  488b5730                       movq	0x30(%rdi), %rdx
// Frames: F13 -> F14 -> F15 -> F89 -> F86 -> F130
// 0x14198e2d2  488b4728                       movq	0x28(%rdi), %rax
// Frames: F55 -> F15 -> F89 -> F86 -> F130
// 0x14198e2d6  486bd238                       imulq	$0x38, %rdx, %rdx
// Frames: F17 -> F18 -> F89 -> F86 -> F130
// 0x14198e2da  4c893410                       movq	%r14, (%rax,%rdx)
// 0x14198e2de  4889741008                     movq	%rsi, 0x8(%rax,%rdx)
// 0x14198e2e3  895c1010                       movl	%ebx, 0x10(%rax,%rdx)
// 0x14198e2e7  0f28442420                     movaps	0x20(%rsp), %xmm0
// 0x14198e2ec  0f11441014                     movups	%xmm0, 0x14(%rax,%rdx)
// 0x14198e2f1  0f1044242c                     movups	0x2c(%rsp), %xmm0
// 0x14198e2f6  0f11441020                     movups	%xmm0, 0x20(%rax,%rdx)
// 0x14198e2fb  c644103001                     movb	$0x1, 0x30(%rax,%rdx)
// 0x14198e300  eb4b                           jmp	0x14198e34d <.text+0x198d34d>
// Frames: F85 -> F5 -> F86 -> F131
// 0x14198e302  488b7038                       movq	0x38(%rax), %rsi
// 0x14198e306  418b411c                       movl	0x1c(%r9), %eax
// 0x14198e30a  498b5110                       movq	0x10(%r9), %rdx
// 0x14198e30e  8902                           movl	%eax, (%rdx)
// Frames: F88 -> F86 -> F131
// 0x14198e310  498b39                         movq	(%r9), %rdi
// Frames: F32 -> F132 -> F86 -> F131
// 0x14198e313  488b5730                       movq	0x30(%rdi), %rdx
// Frames: F133 -> F86 -> F131
// 0x14198e317  488b5f40                       movq	0x40(%rdi), %rbx
// Frames: F32 -> F132 -> F86 -> F131
// 0x14198e31b  483b5720                       cmpq	0x20(%rdi), %rdx
// 0x14198e31f  7516                           jne	0x14198e337 <.text+0x198d337>
// Frames: F132 -> F86 -> F131
// 0x14198e321  488d4720                       leaq	0x20(%rdi), %rax
// 0x14198e325  4989ce                         movq	%rcx, %r14
// Frames: F33 -> F132 -> F86 -> F131
// 0x14198e328  4889c1                         movq	%rax, %rcx
// 0x14198e32b  e870c08efe                     callq	0x14027a3a0 <.text+0x2793a0>
// 0x14198e330  4c89f1                         movq	%r14, %rcx
// Frames: F15 -> F132 -> F86 -> F131
// 0x14198e333  488b5730                       movq	0x30(%rdi), %rdx
// Frames: F13 -> F14 -> F15 -> F132 -> F86 -> F131
// 0x14198e337  488b4728                       movq	0x28(%rdi), %rax
// Frames: F55 -> F15 -> F132 -> F86 -> F131
// 0x14198e33b  488d1452                       leaq	(%rdx,%rdx,2), %rdx
// Frames: F17 -> F18 -> F132 -> F86 -> F131
// 0x14198e33f  48891cd0                       movq	%rbx, (%rax,%rdx,8)
// 0x14198e343  488974d008                     movq	%rsi, 0x8(%rax,%rdx,8)
// 0x14198e348  c644d01001                     movb	$0x1, 0x10(%rax,%rdx,8)
// 0x14198e34d  48ff4730                       incq	0x30(%rdi)
// 0x14198e351  48ff4740                       incq	0x40(%rdi)
// 0x14198e355  c6411501                       movb	$0x1, 0x15(%rcx)
// Frames: F134
// 0x14198e359  0f28742440                     movaps	0x40(%rsp), %xmm6
// 0x14198e35e  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x14198e363  440f28442460                   movaps	0x60(%rsp), %xmm8
// 0x14198e369  440f284c2470                   movaps	0x70(%rsp), %xmm9
// 0x14198e36f  440f28942480000000             movaps	0x80(%rsp), %xmm10
// 0x14198e378  4881c490000000                 addq	$0x90, %rsp
// 0x14198e37f  5b                             popq	%rbx
// 0x14198e380  5f                             popq	%rdi
// 0x14198e381  5e                             popq	%rsi
// 0x14198e382  415e                           popq	%r14
// 0x14198e384  415f                           popq	%r15
// 0x14198e386  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/replay_roof.rs:23:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F2: crates/systems/roof/src/replay_roof.rs:31:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F3: crates/systems/roof/src/replay_roof.rs:32:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F4: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:310:0 bevy_ecs::change_detection::impl$29::set_changed
// F5: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$30::deref_mut
// F6: crates/systems/roof/src/replay_roof.rs:34:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F7: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:423:0 alloc::vec::Vec::new
// F8: crates/systems/roof/src/replay_roof.rs:35:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F9: crates/systems/roof/src/replay_roof.rs:37:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F10: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 core::ptr::mut_ptr::impl$0::add
// F11: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter.rs:233:0 core::slice::iter::IterMut::new
// F12: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:1054:0 core::slice::impl$0::iter_mut
// F13: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:258:0 alloc::raw_vec::RawVec::ptr
// F14: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1365:0 alloc::vec::Vec::as_mut_ptr
// F15: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1974:0 alloc::vec::Vec::push
// F16: crates/systems/roof/src/replay_roof.rs:62:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F17: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1493:0 core::ptr::write
// F18: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1975:0 alloc::vec::Vec::push
// F19: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1976:0 alloc::vec::Vec::push
// F20: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:1798:0 core::ptr::non_null::impl$12::eq
// F21: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$189::next
// F22: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/enumerate.rs:47:0 core::iter::adapters::enumerate::impl$1::next
// F23: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:162:0 core::slice::iter::impl$189::next
// F24: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:626:0 core::ptr::non_null::NonNull::add
// F25: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:109:0 core::slice::iter::IterMut::post_inc_start
// F26: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:77:0 core::slice::iter::impl$189::next
// F27: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/iter/adapters/enumerate.rs:49:0 core::iter::adapters::enumerate::impl$1::next
// F28: crates/systems/roof/src/replay_roof.rs:38:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F29: crates/systems/roof/src/replay_roof.rs:53:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F30: crates/systems/roof/src/replay_roof.rs:54:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F31: crates/systems/roof/src/replay_roof.rs:61:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F32: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1970:0 alloc::vec::Vec::push
// F33: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1971:0 alloc::vec::Vec::push
// F34: crates/systems/roof/src/replay_roof.rs:68:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F35: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:1010:0 core::slice::impl$0::reverse::revswap
// F36: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:996:0 core::slice::impl$0::reverse
// F37: crates/systems/roof/src/replay_roof.rs:70:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F38: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/into_iter.rs:212:0 alloc::vec::into_iter::impl$5::next
// F39: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1284:0 core::ptr::read
// F40: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/mem/mod.rs:781:0 core::mem::swap_simple
// F41: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/mem/mod.rs:753:0 core::mem::swap
// F42: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:1011:0 core::slice::impl$0::reverse::revswap
// F43: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/mem/mod.rs:782:0 core::mem::swap_simple
// F44: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/mem/mod.rs:783:0 core::mem::swap_simple
// F45: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/mem/mod.rs:784:0 core::mem::swap_simple
// F46: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:1012:0 core::slice::impl$0::reverse::revswap
// F47: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:515:0 core::ptr::drop_in_place
// F48: crates/systems/roof/src/replay_roof.rs:71:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F49: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:976:0 core::ptr::non_null::NonNull::read
// F50: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/into_iter.rs:219:0 alloc::vec::into_iter::impl$5::next
// F51: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2203:0 alloc::vec::Vec::len
// F52: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1606:0 alloc::vec::Vec::remove
// F53: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1607:0 alloc::vec::Vec::remove
// F54: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1615:0 alloc::vec::Vec::remove
// F55: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 alloc::vec::Vec::as_mut_ptr
// F56: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1618:0 alloc::vec::Vec::remove
// F57: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1621:0 alloc::vec::Vec::remove
// F58: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/intrinsics.rs:3085:0 core::ptr::mut_ptr::impl$0::add
// F59: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1623:0 alloc::vec::Vec::remove
// F60: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1458:0 alloc::vec::Vec::set_len
// F61: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:402:0 core::ptr::non_null::NonNull::as_ref
// F62: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/sync.rs:1796:0 alloc::sync::Arc::inner
// F63: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/sync.rs:2424:0 alloc::sync::impl$33::drop
// F64: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3359:0 core::sync::atomic::atomic_sub
// F65: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:2697:0 alloc::sync::Arc::inner
// F66: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/sync.rs:2459:0 alloc::sync::impl$33::drop
// F67: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:280:0 alloc::raw_vec::RawVec::current_memory
// F68: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:554:0 alloc::raw_vec::impl$4::drop
// F69: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/into_iter.rs:436:0 alloc::vec::into_iter::impl$15::drop::impl$0::drop
// F70: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/into_iter.rs:447:0 alloc::vec::into_iter::impl$15::drop
// F71: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/mod.rs:1281:0 core::num::impl$11::unchecked_mul
// F72: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:290:0 alloc::raw_vec::RawVec::current_memory
// F73: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:119:0 alloc::alloc::dealloc
// F74: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/alloc.rs:256:0 alloc::alloc::impl$1::deallocate
// F75: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:555:0 alloc::raw_vec::impl$4::drop
// F76: crates/systems/roof/src/replay_roof.rs:74:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F77: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1608:0 alloc::vec::Vec::remove
// F78: crates/systems/roof/src/replay_roof.rs:73:0 system_roof::replay_roof::replay_roof::hed983eff9c24a8e4
// F79: crates/systems/roof/src/replay_roof.rs:78:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F80: crates/systems/roof/src/replay_roof.rs:88:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F81: crates/systems/roof/src/replay_roof.rs:191:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F82: crates/systems/roof/src/replay_roof.rs:195:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F83: crates/systems/roof/src/replay_roof.rs:196:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F84: crates/systems/roof/src/replay_roof.rs:197:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F85: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:592:0 bevy_ecs::change_detection::impl$29::set_changed
// F86: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:625:0 bevy_ecs::event::EventWriter::send
// F87: crates/systems/roof/src/replay_roof.rs:193:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F88: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:592:0 bevy_ecs::change_detection::impl$30::deref_mut
// F89: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:240:0 bevy_ecs::event::Events::send
// F90: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:233:0 bevy_ecs::event::Events::send
// F91: crates/systems/roof/src/replay_roof.rs:154:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F92: crates/systems/roof/src/replay_roof.rs:157:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F93: crates/systems/roof/src/replay_roof.rs:158:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F94: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1521:0 core::f32::impl$0::clamp
// F95: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1524:0 core::f32::impl$0::clamp
// F96: crates/systems/roof/src/replay_roof.rs:162:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F97: crates/systems/roof/src/replay_roof.rs:163:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F98: crates/systems/roof/src/replay_roof.rs:167:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F99: crates/systems/roof/src/replay_roof.rs:174:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F100: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:214:0 core::ops::arith::impl$68::sub
// F101: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs:172:0 lerp::impl$0::lerp
// F102: crates/systems/roof/src/replay_roof.rs:176:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F103: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:344:0 core::ops::arith::impl$124::mul
// F104: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:105:0 core::ops::arith::impl$12::add
// F105: crates/systems/roof/src/replay_roof.rs:177:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F106: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:221:0 core::ops::arith::impl$68::sub
// F107: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:1053:0 glam::f32::vec2::impl$17::sub
// F108: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:676:0 glam::f32::vec2::Vec2::lerp
// F109: crates/systems/roof/src/replay_roof.rs:179:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F110: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:351:0 core::ops::arith::impl$124::mul
// F111: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:974:0 glam::f32::vec2::impl$17::sub
// F112: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:112:0 core::ops::arith::impl$12::add
// F113: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs:1004:0 glam::f32::vec2::impl$17::sub
// F114: crates/systems/roof/src/replay_roof.rs:182:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F115: crates/systems/roof/src/replay_roof.rs:183:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F116: crates/systems/roof/src/replay_roof.rs:173:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F117: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:241:0 bevy_ecs::event::Events::send
// F118: crates/systems/roof/src/replay_roof.rs:118:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F119: crates/systems/roof/src/replay_roof.rs:119:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F120: crates/systems/roof/src/replay_roof.rs:124:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F121: crates/systems/roof/src/replay_roof.rs:123:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F122: crates/systems/roof/src/replay_roof.rs:129:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F123: crates/systems/roof/src/replay_roof.rs:132:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F124: crates/systems/roof/src/replay_roof.rs:133:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F125: crates/systems/roof/src/replay_roof.rs:137:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F126: crates/systems/roof/src/replay_roof.rs:94:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F127: crates/systems/roof/src/replay_roof.rs:104:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F128: crates/systems/roof/src/replay_roof.rs:105:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F129: crates/systems/roof/src/replay_roof.rs:106:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F130: crates/systems/roof/src/replay_roof.rs:102:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F131: crates/systems/roof/src/replay_roof.rs:0:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// F132: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:240:0 bevy_ecs::change_detection::impl$30::deref_mut
// F133: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:233:0 bevy_ecs::change_detection::impl$30::deref_mut
// F134: crates/systems/roof/src/replay_roof.rs:210:0 _$LT$country_core..resources..history..roof_edit..RoofReplayCmd$u20$as$u20$system_roof..replay_roof..ProcessRoofReplayCmd$GT$::process::ha65e706462a8607d
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/replay_roof.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5ea67842dd3f04efd824771d3864463e743218f27961ec0f02cb01208e3ca3f6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1943, 3202
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::replay_roof::impl$0::process
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3202,"record_offset":248,"rva":26795920,"va":5395505040}
// 编译实例 RVAs: 0x198df90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-24196386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395505040,"raw_hex":"e8deca8efe","size":5,"soft_fail":false,"target_va":5371308800,"va":5395505181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3202,"record_offset":388,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3202,"record_offset":644,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3202,"record_offset":928,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3202,"record_offset":676,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3202,"record_offset":572,"parent_record_offset":540,"ancestor_record_offset":248,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3202,"record_offset":540,"parent_record_offset":504,"ancestor_record_offset":248,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3202,"record_offset":504,"parent_record_offset":472,"ancestor_record_offset":248,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3202,"record_offset":440,"parent_record_offset":412,"ancestor_record_offset":248,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3202,"record_offset":412,"parent_record_offset":388,"ancestor_record_offset":248,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3202,"record_offset":472,"parent_record_offset":388,"ancestor_record_offset":248,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3202,"record_offset":728,"parent_record_offset":676,"ancestor_record_offset":248,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3202,"record_offset":784,"parent_record_offset":676,"ancestor_record_offset":248,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3202,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":248,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3202,"record_offset":604,"parent_record_offset":504,"ancestor_record_offset":248,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3202,"record_offset":952,"parent_record_offset":928,"ancestor_record_offset":248,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::replay_roof::replay_roof
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1943,"record_offset":248,"rva":19800640,"va":5388509760}
// 编译实例 RVAs: 0x12e2240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cee80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16856841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388509760,"raw_hex":"e8f7c8fefe","size":5,"soft_fail":false,"target_va":5371653760,"va":5388510596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16854941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388509760,"raw_hex":"e863d0fefe","size":5,"soft_fail":false,"target_va":5371655632,"va":5388510568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16854393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388509760,"raw_hex":"e887d2fefe","size":5,"soft_fail":false,"target_va":5371656320,"va":5388510708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d04d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16851045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388509760,"raw_hex":"e89bdffefe","size":5,"soft_fail":false,"target_va":5371659472,"va":5388510512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d0890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16850225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388509760,"raw_hex":"e8cfe2fefe","size":5,"soft_fail":false,"target_va":5371660432,"va":5388510652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15761034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388509760,"raw_hex":"e876810fff","size":5,"soft_fail":false,"target_va":5372749712,"va":5388510741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069fb90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12855023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388509760,"raw_hex":"e811d93bff","size":5,"soft_fail":false,"target_va":5375654800,"va":5388509818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408cf570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10563345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388509760,"raw_hex":"e8efd05eff","size":5,"soft_fail":false,"target_va":5377946992,"va":5388510332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14198df90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6995030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388509760,"raw_hex":"e856bc6a00","size":5,"soft_fail":false,"target_va":5395505040,"va":5388510005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15082923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388509760,"raw_hex":"e8ab25e600","size":5,"soft_fail":false,"target_va":5403593376,"va":5388510448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17204983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388509760,"raw_hex":"e80979f9fe","size":5,"soft_fail":false,"target_va":5371305056,"va":5388510034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14467647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388509760,"raw_hex":"e83fc2dc00","size":5,"soft_fail":false,"target_va":5402978432,"va":5388510780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4304438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388509760,"raw_hex":"e8ca51beff","size":5,"soft_fail":false,"target_va":5384205408,"va":5388509841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1943,"record_offset":460,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1943,"record_offset":572,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1943,"record_offset":3056,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8653,"source_line":1598,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1943,"record_offset":1164,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1943,"record_offset":404,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1943,"record_offset":700,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1943,"record_offset":1296,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1943,"record_offset":488,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1943,"record_offset":876,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9336,"source_line":976,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1943,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":248,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1943,"record_offset":3508,"parent_record_offset":3416,"ancestor_record_offset":248,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1943,"record_offset":3444,"parent_record_offset":3416,"ancestor_record_offset":248,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1943,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":248,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1943,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":248,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1943,"record_offset":1408,"parent_record_offset":1372,"ancestor_record_offset":248,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1943,"record_offset":1372,"parent_record_offset":1348,"ancestor_record_offset":248,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1943,"record_offset":612,"parent_record_offset":572,"ancestor_record_offset":248,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1943,"record_offset":3104,"parent_record_offset":3056,"ancestor_record_offset":248,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1943,"record_offset":3264,"parent_record_offset":3056,"ancestor_record_offset":248,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1943,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":248,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1943,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":248,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1943,"record_offset":428,"parent_record_offset":404,"ancestor_record_offset":248,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1943,"record_offset":948,"parent_record_offset":904,"ancestor_record_offset":248,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1943,"record_offset":976,"parent_record_offset":948,"ancestor_record_offset":248,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1943,"record_offset":3472,"parent_record_offset":3444,"ancestor_record_offset":248,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1943,"record_offset":1324,"parent_record_offset":1296,"ancestor_record_offset":248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1943,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1943,"record_offset":836,"parent_record_offset":812,"ancestor_record_offset":248,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1943,"record_offset":1440,"parent_record_offset":1408,"ancestor_record_offset":248,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1943,"record_offset":1236,"parent_record_offset":1164,"ancestor_record_offset":248,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1943,"record_offset":780,"parent_record_offset":736,"ancestor_record_offset":248,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1943,"record_offset":1020,"parent_record_offset":976,"ancestor_record_offset":248,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1943,"record_offset":668,"parent_record_offset":572,"ancestor_record_offset":248,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1943,"record_offset":904,"parent_record_offset":876,"ancestor_record_offset":248,"inlinee":9338,"source_line":999,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1943,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":248,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1943,"record_offset":812,"parent_record_offset":736,"ancestor_record_offset":248,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1943,"record_offset":736,"parent_record_offset":700,"ancestor_record_offset":248,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1943,"record_offset":1468,"parent_record_offset":1408,"ancestor_record_offset":248,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01943.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1943,"record_offset":1500,"parent_record_offset":1372,"ancestor_record_offset":248,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01943.jsonl.gz"}
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
// 完整结构及原始类型记录: replay_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-history-owners/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/roof/src/replay_roof.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/roof/src/replay_roof.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/roof/src/replay_roof.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>

// ------------------------------------------------------------------
// 函数: system_roof::replay_roof::impl$0::process
// 信息 ID: e99043cc422f6c8fc23badba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::replay_roof::replay_roof @ 0x12e2335 -> 0x198df90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::replay_roof::replay_roof
// 信息 ID: fb9215deeb42a2c6a7f2c7e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b91a709c3973e2ca043534a6: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>)> @ 0x8e7b97 -> 0x12e2240 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>)> > @ 0x12d199b -> 0x12e2240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
