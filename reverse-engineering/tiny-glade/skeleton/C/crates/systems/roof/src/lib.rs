// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: lib.rs.assembly.json.gz

// FUNCTION system_roof::impl$0::add_roof_systems
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 270:1520 -> rva_6bc810_len_d6e
//
// FUNCTION system_roof::impl$0::add_roof_events_and_resources
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 270:7616 -> rva_6bd580_len_a6c
//
// FUNCTION system_roof::gpu_flush_buffers
// Candidate interfaces: 5d58aab8b83fbd13d0042c71
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 965:4312 -> host 965:1852 / rva_fd7330_len_22d; exact_symbolizer_function_frame
//   Function-frame RVA span [0xfd73f1, 0xfd7445), 21 instructions
//   Function-frame RVA span [0xfd74aa, 0xfd74d8), 8 instructions
//   Function-frame RVA span [0xfd7550, 0xfd755d), 3 instructions
// Inline site 2426:329348 -> host 2426:326748 / rva_1f29080_len_20f; exact_symbolizer_function_frame
//   Function-frame RVA span [0x1f29141, 0x1f29195), 21 instructions
//   Function-frame RVA span [0x1f29254, 0x1f2928f), 11 instructions
//
// FUNCTION core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 7; complete records in enriched/assembly sidecars.
// Compiled procedure 2425:27416 -> rva_1f20f20_len_230
// Compiled procedure 2435:25644 -> rva_1f36190_len_230
// Compiled procedure 2437:13356 -> rva_1f37fc0_len_230
// Compiled procedure 2438:25540 -> rva_1f3aba0_len_230
// Compiled procedure 2444:10320 -> rva_1f3e5b0_len_230
// Compiled procedure 2449:37572 -> rva_1f42050_len_230
// Compiled procedure 3750:7760 -> rva_281b220_len_230
//
// FUNCTION arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>,enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > > >
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 3756:952 -> rva_281bb00_len_1dc
//
// FUNCTION rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 3788:640 -> rva_2831600_len_581
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_6bc810_len_d6e: RVA 0x6bc810, 3438 PDB bytes; 742 instructions
// Evidence: _assembly/roof-v1/C/pack-000/rva_6bc810_len_d6e.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 270:1520: system_roof::impl$0::add_roof_systems
// Frames: F1
// 0x1406bc810  55                             pushq	%rbp
// 0x1406bc811  4156                           pushq	%r14
// 0x1406bc813  56                             pushq	%rsi
// 0x1406bc814  57                             pushq	%rdi
// 0x1406bc815  53                             pushq	%rbx
// 0x1406bc816  4881ec10080000                 subq	$0x810, %rsp            # imm = 0x810
// 0x1406bc81d  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x1406bc825  48c78588070000feffffff         movq	$-0x2, 0x788(%rbp)
// 0x1406bc830  4889ce                         movq	%rcx, %rsi
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc833  4883b98800000000               cmpq	$0x0, 0x88(%rcx)
// 0x1406bc83b  0f84d1000000                   je	0x1406bc912 <.text+0x6bb912>
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc841  488b4670                       movq	0x70(%rsi), %rax
// 0x1406bc845  488b4e78                       movq	0x78(%rsi), %rcx
// 0x1406bc849  48badc3871eb8b13c34b           movabsq	$0x4bc3138beb7138dc, %rdx # imm = 0x4BC3138BEB7138DC
// 0x1406bc853  4531c0                         xorl	%r8d, %r8d
// 0x1406bc856  660f6f0512037602               movdqa	0x2760312(%rip), %xmm0  # 0x142e1cb70
// 0x1406bc85e  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1406bc862  660f6f1586558c02               movdqa	0x28c5586(%rip), %xmm2  # 0x142f81df0
// Frames: F14 -> F15 -> F16 -> F17 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc86a  4821ca                         andq	%rcx, %rdx
// 0x1406bc86d  f30f6f1c10                     movdqu	(%rax,%rdx), %xmm3
// Frames: F18 -> F19 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc872  660f6fe3                       movdqa	%xmm3, %xmm4
// 0x1406bc876  660f74e0                       pcmpeqb	%xmm0, %xmm4
// 0x1406bc87a  66440fd7cc                     pmovmskb	%xmm4, %r9d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc87f  4585c9                         testl	%r9d, %r9d
// 0x1406bc882  743c                           je	0x1406bc8c0 <.text+0x6bb8c0>
// Frames: F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc884  f3450fbcd1                     tzcntl	%r9d, %r10d
// Frames: F26 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc889  4901d2                         addq	%rdx, %r10
// 0x1406bc88c  4921ca                         andq	%rcx, %r10
// Frames: F27 -> F28 -> F29 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc88f  49f7da                         negq	%r10
// 0x1406bc892  4f8d1452                       leaq	(%r10,%r10,2), %r10
// Frames: F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc896  f3420f6f64d0e8                 movdqu	-0x18(%rax,%r10,8), %xmm4
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc89d  660f74e2                       pcmpeqb	%xmm2, %xmm4
// 0x1406bc8a1  66440fd7dc                     pmovmskb	%xmm4, %r11d
// 0x1406bc8a6  4181fbffff0000                 cmpl	$0xffff, %r11d          # imm = 0xFFFF
// Frames: F36 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc8ad  742c                           je	0x1406bc8db <.text+0x6bb8db>
// Frames: F37 -> F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc8af  458d51ff                       leal	-0x1(%r9), %r10d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc8b3  664521ca                       andw	%r9w, %r10w
// 0x1406bc8b7  4589d1                         movl	%r10d, %r9d
// 0x1406bc8ba  75c8                           jne	0x1406bc884 <.text+0x6bb884>
// 0x1406bc8bc  0f1f4000                       nopl	(%rax)
// Frames: F18 -> F19 -> F38 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc8c0  660f74d9                       pcmpeqb	%xmm1, %xmm3
// 0x1406bc8c4  66440fd7cb                     pmovmskb	%xmm3, %r9d
// Frames: F40 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc8c9  4585c9                         testl	%r9d, %r9d
// 0x1406bc8cc  7544                           jne	0x1406bc912 <.text+0x6bb912>
// Frames: F41 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc8ce  4c01c2                         addq	%r8, %rdx
// 0x1406bc8d1  4883c210                       addq	$0x10, %rdx
// Frames: F43 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc8d5  4983c010                       addq	$0x10, %r8
// Frames: F44 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc8d9  eb8f                           jmp	0x1406bc86a <.text+0x6bb86a>
// Frames: F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc8db  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F45 -> F4 -> F5 -> F6 -> F7 -> F8
// 0x1406bc8df  488b40f8                       movq	-0x8(%rax), %rax
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F8
// 0x1406bc8e3  483b8610020000                 cmpq	0x210(%rsi), %rax
// 0x1406bc8ea  7326                           jae	0x1406bc912 <.text+0x6bb912>
// Frames: F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F8
// 0x1406bc8ec  488b8e08020000                 movq	0x208(%rsi), %rcx
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F8
// 0x1406bc8f3  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bc8f7  4885c0                         testq	%rax, %rax
// 0x1406bc8fa  7416                           je	0x1406bc912 <.text+0x6bb912>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F8
// 0x1406bc8fc  488b8ed8010000                 movq	0x1d8(%rsi), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F8
// 0x1406bc903  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F8
// 0x1406bc906  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F71 -> F6 -> F7 -> F8
// 0x1406bc90a  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F6 -> F7 -> F8
// 0x1406bc910  7516                           jne	0x1406bc928 <.text+0x6bb928>
// Frames: F72 -> F7 -> F8
// 0x1406bc912  4889f1                         movq	%rsi, %rcx
// 0x1406bc915  e8a69f9000                     callq	0x140fc68c0 <.text+0xfc58c0>
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc91a  4883be8800000000               cmpq	$0x0, 0x88(%rsi)
// 0x1406bc922  0f84da000000                   je	0x1406bca02 <.text+0x6bba02>
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc928  488b4670                       movq	0x70(%rsi), %rax
// 0x1406bc92c  488b4e78                       movq	0x78(%rsi), %rcx
// 0x1406bc930  48bac501ebf30fd352f8           movabsq	$-0x7ad2cf00c14fe3b, %rdx # imm = 0xF852D30FF3EB01C5
// 0x1406bc93a  4531c0                         xorl	%r8d, %r8d
// 0x1406bc93d  660f6f058b007602               movdqa	0x276008b(%rip), %xmm0  # 0x142e1c9d0
// 0x1406bc945  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1406bc949  660f6f15af548c02               movdqa	0x28c54af(%rip), %xmm2  # 0x142f81e00
// Frames: F14 -> F15 -> F16 -> F17 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc951  4821ca                         andq	%rcx, %rdx
// 0x1406bc954  f30f6f1c10                     movdqu	(%rax,%rdx), %xmm3
// Frames: F18 -> F19 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc959  660f6fe3                       movdqa	%xmm3, %xmm4
// 0x1406bc95d  660f74e0                       pcmpeqb	%xmm0, %xmm4
// 0x1406bc961  66440fd7cc                     pmovmskb	%xmm4, %r9d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc966  4585c9                         testl	%r9d, %r9d
// 0x1406bc969  7445                           je	0x1406bc9b0 <.text+0x6bb9b0>
// Frames: F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc96b  f3450fbcd1                     tzcntl	%r9d, %r10d
// Frames: F26 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc970  4901d2                         addq	%rdx, %r10
// 0x1406bc973  4921ca                         andq	%rcx, %r10
// Frames: F27 -> F28 -> F29 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc976  49f7da                         negq	%r10
// 0x1406bc979  4f8d1452                       leaq	(%r10,%r10,2), %r10
// Frames: F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc97d  f3420f6f64d0e8                 movdqu	-0x18(%rax,%r10,8), %xmm4
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc984  660f74e2                       pcmpeqb	%xmm2, %xmm4
// 0x1406bc988  66440fd7dc                     pmovmskb	%xmm4, %r11d
// 0x1406bc98d  4181fbffff0000                 cmpl	$0xffff, %r11d          # imm = 0xFFFF
// Frames: F36 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc994  7435                           je	0x1406bc9cb <.text+0x6bb9cb>
// Frames: F37 -> F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc996  458d51ff                       leal	-0x1(%r9), %r10d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc99a  664521ca                       andw	%r9w, %r10w
// 0x1406bc99e  4589d1                         movl	%r10d, %r9d
// 0x1406bc9a1  75c8                           jne	0x1406bc96b <.text+0x6bb96b>
// 0x1406bc9a3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F18 -> F19 -> F38 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc9b0  660f74d9                       pcmpeqb	%xmm1, %xmm3
// 0x1406bc9b4  66440fd7cb                     pmovmskb	%xmm3, %r9d
// Frames: F40 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc9b9  4585c9                         testl	%r9d, %r9d
// 0x1406bc9bc  7544                           jne	0x1406bca02 <.text+0x6bba02>
// Frames: F41 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc9be  4c01c2                         addq	%r8, %rdx
// 0x1406bc9c1  4883c210                       addq	$0x10, %rdx
// Frames: F43 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc9c5  4983c010                       addq	$0x10, %r8
// Frames: F44 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc9c9  eb86                           jmp	0x1406bc951 <.text+0x6bb951>
// Frames: F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc9cb  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F45 -> F4 -> F5 -> F6 -> F7 -> F73
// 0x1406bc9cf  488b40f8                       movq	-0x8(%rax), %rax
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F73
// 0x1406bc9d3  483b8610020000                 cmpq	0x210(%rsi), %rax
// 0x1406bc9da  7326                           jae	0x1406bca02 <.text+0x6bba02>
// Frames: F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F73
// 0x1406bc9dc  488b8e08020000                 movq	0x208(%rsi), %rcx
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F73
// 0x1406bc9e3  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bc9e7  4885c0                         testq	%rax, %rax
// 0x1406bc9ea  7416                           je	0x1406bca02 <.text+0x6bba02>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F73
// 0x1406bc9ec  488b8ed8010000                 movq	0x1d8(%rsi), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F73
// 0x1406bc9f3  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F73
// 0x1406bc9f6  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F71 -> F6 -> F7 -> F73
// 0x1406bc9fa  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F6 -> F7 -> F73
// 0x1406bca00  7516                           jne	0x1406bca18 <.text+0x6bba18>
// Frames: F72 -> F7 -> F73
// 0x1406bca02  4889f1                         movq	%rsi, %rcx
// 0x1406bca05  e8b6749000                     callq	0x140fc3ec0 <.text+0xfc2ec0>
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bca0a  4883be8800000000               cmpq	$0x0, 0x88(%rsi)
// 0x1406bca12  0f84da000000                   je	0x1406bcaf2 <.text+0x6bbaf2>
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bca18  488b4670                       movq	0x70(%rsi), %rax
// 0x1406bca1c  488b4e78                       movq	0x78(%rsi), %rcx
// 0x1406bca20  48ba0e508a24b0f569c6           movabsq	$-0x39960a4fdb75aff2, %rdx # imm = 0xC669F5B0248A500E
// 0x1406bca2a  4531c0                         xorl	%r8d, %r8d
// 0x1406bca2d  660f6f05bba78902               movdqa	0x289a7bb(%rip), %xmm0  # 0x142f571f0
// 0x1406bca35  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1406bca39  660f6f15cf538c02               movdqa	0x28c53cf(%rip), %xmm2  # 0x142f81e10
// Frames: F14 -> F15 -> F16 -> F17 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bca41  4821ca                         andq	%rcx, %rdx
// 0x1406bca44  f30f6f1c10                     movdqu	(%rax,%rdx), %xmm3
// Frames: F18 -> F19 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bca49  660f6fe3                       movdqa	%xmm3, %xmm4
// 0x1406bca4d  660f74e0                       pcmpeqb	%xmm0, %xmm4
// 0x1406bca51  66440fd7cc                     pmovmskb	%xmm4, %r9d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bca56  4585c9                         testl	%r9d, %r9d
// 0x1406bca59  7445                           je	0x1406bcaa0 <.text+0x6bbaa0>
// Frames: F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bca5b  f3450fbcd1                     tzcntl	%r9d, %r10d
// Frames: F26 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bca60  4901d2                         addq	%rdx, %r10
// 0x1406bca63  4921ca                         andq	%rcx, %r10
// Frames: F27 -> F28 -> F29 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bca66  49f7da                         negq	%r10
// 0x1406bca69  4f8d1452                       leaq	(%r10,%r10,2), %r10
// Frames: F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bca6d  f3420f6f64d0e8                 movdqu	-0x18(%rax,%r10,8), %xmm4
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bca74  660f74e2                       pcmpeqb	%xmm2, %xmm4
// 0x1406bca78  66440fd7dc                     pmovmskb	%xmm4, %r11d
// 0x1406bca7d  4181fbffff0000                 cmpl	$0xffff, %r11d          # imm = 0xFFFF
// Frames: F36 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bca84  7435                           je	0x1406bcabb <.text+0x6bbabb>
// Frames: F37 -> F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bca86  458d51ff                       leal	-0x1(%r9), %r10d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bca8a  664521ca                       andw	%r9w, %r10w
// 0x1406bca8e  4589d1                         movl	%r10d, %r9d
// 0x1406bca91  75c8                           jne	0x1406bca5b <.text+0x6bba5b>
// 0x1406bca93  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F18 -> F19 -> F38 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bcaa0  660f74d9                       pcmpeqb	%xmm1, %xmm3
// 0x1406bcaa4  66440fd7cb                     pmovmskb	%xmm3, %r9d
// Frames: F40 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bcaa9  4585c9                         testl	%r9d, %r9d
// 0x1406bcaac  7544                           jne	0x1406bcaf2 <.text+0x6bbaf2>
// Frames: F41 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bcaae  4c01c2                         addq	%r8, %rdx
// 0x1406bcab1  4883c210                       addq	$0x10, %rdx
// Frames: F43 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bcab5  4983c010                       addq	$0x10, %r8
// Frames: F44 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bcab9  eb86                           jmp	0x1406bca41 <.text+0x6bba41>
// Frames: F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bcabb  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F45 -> F4 -> F5 -> F6 -> F7 -> F74
// 0x1406bcabf  488b40f8                       movq	-0x8(%rax), %rax
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F74
// 0x1406bcac3  483b8610020000                 cmpq	0x210(%rsi), %rax
// 0x1406bcaca  7326                           jae	0x1406bcaf2 <.text+0x6bbaf2>
// Frames: F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F74
// 0x1406bcacc  488b8e08020000                 movq	0x208(%rsi), %rcx
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F74
// 0x1406bcad3  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bcad7  4885c0                         testq	%rax, %rax
// 0x1406bcada  7416                           je	0x1406bcaf2 <.text+0x6bbaf2>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F74
// 0x1406bcadc  488b8ed8010000                 movq	0x1d8(%rsi), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F74
// 0x1406bcae3  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F74
// 0x1406bcae6  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F71 -> F6 -> F7 -> F74
// 0x1406bcaea  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F6 -> F7 -> F74
// 0x1406bcaf0  7508                           jne	0x1406bcafa <.text+0x6bbafa>
// Frames: F72 -> F7 -> F74
// 0x1406bcaf2  4889f1                         movq	%rsi, %rcx
// 0x1406bcaf5  e8c67f9000                     callq	0x140fc4ac0 <.text+0xfc3ac0>
// Frames: F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F82 -> F83
// 0x1406bcafa  e8116894ff                     callq	0x140003310 <.text+0x2310>
// Frames: F84 -> F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F82 -> F83
// 0x1406bcaff  b948010000                     movl	$0x148, %ecx            # imm = 0x148
// 0x1406bcb04  ba08000000                     movl	$0x8, %edx
// 0x1406bcb09  e8d28dfeff                     callq	0x1406a58e0 <.text+0x6a48e0>
// Frames: F79 -> F80 -> F81 -> F82 -> F83
// 0x1406bcb0e  4885c0                         testq	%rax, %rax
// 0x1406bcb11  0f841e080000                   je	0x1406bd335 <.text+0x6bc335>
// Frames: F85 -> F81 -> F82 -> F83
// 0x1406bcb17  c70000000000                   movl	$0x0, (%rax)
// 0x1406bcb1d  c6400400                       movb	$0x0, 0x4(%rax)
// 0x1406bcb21  49be0000000000000080           movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
// 0x1406bcb2b  4c89b000010000                 movq	%r14, 0x100(%rax)
// Frames: F86 -> F82 -> F83
// 0x1406bcb32  4c8d05df41b902                 leaq	0x2b941df(%rip), %r8    # 0x143250d18
// 0x1406bcb39  488dbdd0010000                 leaq	0x1d0(%rbp), %rdi
// 0x1406bcb40  4889f9                         movq	%rdi, %rcx
// 0x1406bcb43  4889c2                         movq	%rax, %rdx
// 0x1406bcb46  e8a5dd8000                     callq	0x140eca8f0 <.text+0xec98f0>
// 0x1406bcb4b  488d9d10070000                 leaq	0x710(%rbp), %rbx
// Frames: F82 -> F83
// 0x1406bcb52  4889d9                         movq	%rbx, %rcx
// 0x1406bcb55  4889fa                         movq	%rdi, %rdx
// 0x1406bcb58  e8d3e69000                     callq	0x140fcb230 <.text+0xfca230>
// Frames: F87
// 0x1406bcb5d  4c8d0d4c538c02                 leaq	0x28c534c(%rip), %r9    # 0x142f81eb0
// 0x1406bcb64  488d8d60010000                 leaq	0x160(%rbp), %rcx
// 0x1406bcb6b  4889da                         movq	%rbx, %rdx
// 0x1406bcb6e  41b011                         movb	$0x11, %r8b
// 0x1406bcb71  e8ea209100                     callq	0x140fcec60 <.text+0xfcdc60>
// Frames: F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F82 -> F88
// 0x1406bcb76  e8956794ff                     callq	0x140003310 <.text+0x2310>
// Frames: F84 -> F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F82 -> F88
// 0x1406bcb7b  b9b0010000                     movl	$0x1b0, %ecx            # imm = 0x1B0
// 0x1406bcb80  ba08000000                     movl	$0x8, %edx
// 0x1406bcb85  e8568dfeff                     callq	0x1406a58e0 <.text+0x6a48e0>
// Frames: F79 -> F80 -> F81 -> F82 -> F88
// 0x1406bcb8a  4885c0                         testq	%rax, %rax
// 0x1406bcb8d  0f84b2070000                   je	0x1406bd345 <.text+0x6bc345>
// Frames: F85 -> F81 -> F82 -> F88
// 0x1406bcb93  c70000000000                   movl	$0x0, (%rax)
// 0x1406bcb99  c6400400                       movb	$0x0, 0x4(%rax)
// 0x1406bcb9d  4c89b000010000                 movq	%r14, 0x100(%rax)
// Frames: F86 -> F82 -> F88
// 0x1406bcba4  4c8d05e54db902                 leaq	0x2b94de5(%rip), %r8    # 0x143251990
// 0x1406bcbab  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// 0x1406bcbb2  4889c2                         movq	%rax, %rdx
// 0x1406bcbb5  e836dd8000                     callq	0x140eca8f0 <.text+0xec98f0>
// 0x1406bcbba  90                             nop
// Frames: F82 -> F88
// 0x1406bcbbb  488d8da0060000                 leaq	0x6a0(%rbp), %rcx
// 0x1406bcbc2  488d95d0010000                 leaq	0x1d0(%rbp), %rdx
// 0x1406bcbc9  e862e69000                     callq	0x140fcb230 <.text+0xfca230>
// 0x1406bcbce  90                             nop
// Frames: F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F82 -> F89
// 0x1406bcbcf  e83c6794ff                     callq	0x140003310 <.text+0x2310>
// Frames: F84 -> F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F82 -> F89
// 0x1406bcbd4  b980010000                     movl	$0x180, %ecx            # imm = 0x180
// 0x1406bcbd9  ba08000000                     movl	$0x8, %edx
// 0x1406bcbde  e8fd8cfeff                     callq	0x1406a58e0 <.text+0x6a48e0>
// Frames: F79 -> F80 -> F81 -> F82 -> F89
// 0x1406bcbe3  4885c0                         testq	%rax, %rax
// 0x1406bcbe6  0f846b070000                   je	0x1406bd357 <.text+0x6bc357>
// Frames: F85 -> F81 -> F82 -> F89
// 0x1406bcbec  c70000000000                   movl	$0x0, (%rax)
// 0x1406bcbf2  c6400400                       movb	$0x0, 0x4(%rax)
// 0x1406bcbf6  4c89b000010000                 movq	%r14, 0x100(%rax)
// Frames: F86 -> F82 -> F89
// 0x1406bcbfd  4c8d051452b902                 leaq	0x2b95214(%rip), %r8    # 0x143251e18
// 0x1406bcc04  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// 0x1406bcc0b  4889c2                         movq	%rax, %rdx
// 0x1406bcc0e  e8dddc8000                     callq	0x140eca8f0 <.text+0xec98f0>
// 0x1406bcc13  90                             nop
// Frames: F82 -> F89
// 0x1406bcc14  488d8d10070000                 leaq	0x710(%rbp), %rcx
// 0x1406bcc1b  488d95d0010000                 leaq	0x1d0(%rbp), %rdx
// 0x1406bcc22  e809e69000                     callq	0x140fcb230 <.text+0xfca230>
// 0x1406bcc27  90                             nop
// Frames: F90
// 0x1406bcc28  0f1085a0060000                 movups	0x6a0(%rbp), %xmm0
// 0x1406bcc2f  0f108db0060000                 movups	0x6b0(%rbp), %xmm1
// 0x1406bcc36  0f1095c0060000                 movups	0x6c0(%rbp), %xmm2
// 0x1406bcc3d  0f109dd0060000                 movups	0x6d0(%rbp), %xmm3
// 0x1406bcc44  0f2985d0010000                 movaps	%xmm0, 0x1d0(%rbp)
// 0x1406bcc4b  0f298de0010000                 movaps	%xmm1, 0x1e0(%rbp)
// 0x1406bcc52  0f2995f0010000                 movaps	%xmm2, 0x1f0(%rbp)
// 0x1406bcc59  0f299d00020000                 movaps	%xmm3, 0x200(%rbp)
// 0x1406bcc60  0f1085e0060000                 movups	0x6e0(%rbp), %xmm0
// 0x1406bcc67  0f298510020000                 movaps	%xmm0, 0x210(%rbp)
// 0x1406bcc6e  0f1085f0060000                 movups	0x6f0(%rbp), %xmm0
// 0x1406bcc75  0f298520020000                 movaps	%xmm0, 0x220(%rbp)
// 0x1406bcc7c  0f108500070000                 movups	0x700(%rbp), %xmm0
// 0x1406bcc83  0f298530020000                 movaps	%xmm0, 0x230(%rbp)
// 0x1406bcc8a  0f108510070000                 movups	0x710(%rbp), %xmm0
// 0x1406bcc91  f30f6f8d20070000               movdqu	0x720(%rbp), %xmm1
// 0x1406bcc99  f30f6f9530070000               movdqu	0x730(%rbp), %xmm2
// 0x1406bcca1  f30f6f9d40070000               movdqu	0x740(%rbp), %xmm3
// 0x1406bcca9  0f298540020000                 movaps	%xmm0, 0x240(%rbp)
// 0x1406bccb0  660f7f8d50020000               movdqa	%xmm1, 0x250(%rbp)
// 0x1406bccb8  660f7f9560020000               movdqa	%xmm2, 0x260(%rbp)
// 0x1406bccc0  660f7f9d70020000               movdqa	%xmm3, 0x270(%rbp)
// 0x1406bccc8  0f108550070000                 movups	0x750(%rbp), %xmm0
// 0x1406bcccf  0f298580020000                 movaps	%xmm0, 0x280(%rbp)
// 0x1406bccd6  0f108560070000                 movups	0x760(%rbp), %xmm0
// 0x1406bccdd  0f298590020000                 movaps	%xmm0, 0x290(%rbp)
// 0x1406bcce4  f30f6f8570070000               movdqu	0x770(%rbp), %xmm0
// 0x1406bccec  660f7f85a0020000               movdqa	%xmm0, 0x2a0(%rbp)
// Frames: F91
// 0x1406bccf4  488d8d10070000                 leaq	0x710(%rbp), %rcx
// 0x1406bccfb  488d95d0010000                 leaq	0x1d0(%rbp), %rdx
// 0x1406bcd02  e8f9189100                     callq	0x140fce600 <.text+0xfcd600>
// 0x1406bcd07  90                             nop
// Frames: F92 -> F93 -> F94
// 0x1406bcd08  c685d001000012                 movb	$0x12, 0x1d0(%rbp)
// 0x1406bcd0f  488b0dba205d03                 movq	0x35d20ba(%rip), %rcx   # 0x143c8edd0
// 0x1406bcd16  4c8d054b56b902                 leaq	0x2b9564b(%rip), %r8    # 0x143252368
// 0x1406bcd1d  488d95d0010000                 leaq	0x1d0(%rbp), %rdx
// 0x1406bcd24  e8e7619100                     callq	0x140fd2f10 <.text+0xfd1f10>
// 0x1406bcd29  90                             nop
// Frames: F93 -> F94
// 0x1406bcd2a  4989d0                         movq	%rdx, %r8
// 0x1406bcd2d  488d8d10070000                 leaq	0x710(%rbp), %rcx
// 0x1406bcd34  4889c2                         movq	%rax, %rdx
// 0x1406bcd37  e8d404b1ff                     callq	0x1401cd210 <.text+0x1cc210>
// 0x1406bcd3c  90                             nop
// Frames: F95 -> F94
// 0x1406bcd3d  0f108570070000                 movups	0x770(%rbp), %xmm0
// 0x1406bcd44  0f298590060000                 movaps	%xmm0, 0x690(%rbp)
// 0x1406bcd4b  0f108560070000                 movups	0x760(%rbp), %xmm0
// 0x1406bcd52  0f298580060000                 movaps	%xmm0, 0x680(%rbp)
// 0x1406bcd59  0f108550070000                 movups	0x750(%rbp), %xmm0
// 0x1406bcd60  0f298570060000                 movaps	%xmm0, 0x670(%rbp)
// 0x1406bcd67  f30f6f8510070000               movdqu	0x710(%rbp), %xmm0
// 0x1406bcd6f  f30f6f8d20070000               movdqu	0x720(%rbp), %xmm1
// 0x1406bcd77  f30f6f9530070000               movdqu	0x730(%rbp), %xmm2
// 0x1406bcd7f  f30f6f9d40070000               movdqu	0x740(%rbp), %xmm3
// 0x1406bcd87  660f7f9d60060000               movdqa	%xmm3, 0x660(%rbp)
// 0x1406bcd8f  660f7f9550060000               movdqa	%xmm2, 0x650(%rbp)
// 0x1406bcd97  660f7f8d40060000               movdqa	%xmm1, 0x640(%rbp)
// 0x1406bcd9f  660f7f8530060000               movdqa	%xmm0, 0x630(%rbp)
// Frames: F96
// 0x1406bcda7  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// 0x1406bcdae  e81d7a9100                     callq	0x140fd47d0 <.text+0xfd37d0>
// 0x1406bcdb3  90                             nop
// Frames: F97
// 0x1406bcdb4  0f108530020000                 movups	0x230(%rbp), %xmm0
// 0x1406bcdbb  0f298570070000                 movaps	%xmm0, 0x770(%rbp)
// 0x1406bcdc2  0f108520020000                 movups	0x220(%rbp), %xmm0
// 0x1406bcdc9  0f298560070000                 movaps	%xmm0, 0x760(%rbp)
// 0x1406bcdd0  0f108510020000                 movups	0x210(%rbp), %xmm0
// 0x1406bcdd7  0f298550070000                 movaps	%xmm0, 0x750(%rbp)
// 0x1406bcdde  f30f6f85d0010000               movdqu	0x1d0(%rbp), %xmm0
// 0x1406bcde6  f30f6f8de0010000               movdqu	0x1e0(%rbp), %xmm1
// 0x1406bcdee  f30f6f95f0010000               movdqu	0x1f0(%rbp), %xmm2
// 0x1406bcdf6  f30f6f9d00020000               movdqu	0x200(%rbp), %xmm3
// 0x1406bcdfe  660f7f9d40070000               movdqa	%xmm3, 0x740(%rbp)
// 0x1406bce06  660f7f9530070000               movdqa	%xmm2, 0x730(%rbp)
// 0x1406bce0e  660f7f8d20070000               movdqa	%xmm1, 0x720(%rbp)
// 0x1406bce16  660f7f8510070000               movdqa	%xmm0, 0x710(%rbp)
// Frames: F98
// 0x1406bce1e  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// 0x1406bce25  488d9510070000                 leaq	0x710(%rbp), %rdx
// 0x1406bce2c  e83fed9000                     callq	0x140fcbb70 <.text+0xfcab70>
// 0x1406bce31  90                             nop
// Frames: F99
// 0x1406bce32  0f1085d0010000                 movups	0x1d0(%rbp), %xmm0
// 0x1406bce39  f30f6f8de0010000               movdqu	0x1e0(%rbp), %xmm1
// 0x1406bce41  f30f6f95f0010000               movdqu	0x1f0(%rbp), %xmm2
// 0x1406bce49  f30f6f9d00020000               movdqu	0x200(%rbp), %xmm3
// 0x1406bce51  0f298510070000                 movaps	%xmm0, 0x710(%rbp)
// 0x1406bce58  660f7f8d20070000               movdqa	%xmm1, 0x720(%rbp)
// 0x1406bce60  660f7f9530070000               movdqa	%xmm2, 0x730(%rbp)
// 0x1406bce68  660f7f9d40070000               movdqa	%xmm3, 0x740(%rbp)
// 0x1406bce70  0f108510020000                 movups	0x210(%rbp), %xmm0
// 0x1406bce77  0f298550070000                 movaps	%xmm0, 0x750(%rbp)
// 0x1406bce7e  0f108520020000                 movups	0x220(%rbp), %xmm0
// 0x1406bce85  0f298560070000                 movaps	%xmm0, 0x760(%rbp)
// 0x1406bce8c  f30f6f8530020000               movdqu	0x230(%rbp), %xmm0
// 0x1406bce94  660f7f8570070000               movdqa	%xmm0, 0x770(%rbp)
// Frames: F100
// 0x1406bce9c  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// 0x1406bcea3  488d9510070000                 leaq	0x710(%rbp), %rdx
// 0x1406bceaa  e861f19000                     callq	0x140fcc010 <.text+0xfcb010>
// 0x1406bceaf  90                             nop
// Frames: F92 -> F93 -> F101
// 0x1406bceb0  c6851007000014                 movb	$0x14, 0x710(%rbp)
// 0x1406bceb7  488b0d121f5d03                 movq	0x35d1f12(%rip), %rcx   # 0x143c8edd0
// 0x1406bcebe  4c8d05a354b902                 leaq	0x2b954a3(%rip), %r8    # 0x143252368
// 0x1406bcec5  488d9510070000                 leaq	0x710(%rbp), %rdx
// 0x1406bcecc  e83f609100                     callq	0x140fd2f10 <.text+0xfd1f10>
// 0x1406bced1  90                             nop
// Frames: F93 -> F101
// 0x1406bced2  4989d0                         movq	%rdx, %r8
// 0x1406bced5  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// 0x1406bcedc  4889c2                         movq	%rax, %rdx
// 0x1406bcedf  e82c03b1ff                     callq	0x1401cd210 <.text+0x1cc210>
// 0x1406bcee4  90                             nop
// Frames: F95 -> F101
// 0x1406bcee5  0f108530020000                 movups	0x230(%rbp), %xmm0
// 0x1406bceec  0f298520060000                 movaps	%xmm0, 0x620(%rbp)
// 0x1406bcef3  0f108520020000                 movups	0x220(%rbp), %xmm0
// 0x1406bcefa  0f298510060000                 movaps	%xmm0, 0x610(%rbp)
// 0x1406bcf01  0f108510020000                 movups	0x210(%rbp), %xmm0
// 0x1406bcf08  0f298500060000                 movaps	%xmm0, 0x600(%rbp)
// 0x1406bcf0f  f30f6f85d0010000               movdqu	0x1d0(%rbp), %xmm0
// 0x1406bcf17  f30f6f8de0010000               movdqu	0x1e0(%rbp), %xmm1
// 0x1406bcf1f  f30f6f95f0010000               movdqu	0x1f0(%rbp), %xmm2
// 0x1406bcf27  f30f6f9d00020000               movdqu	0x200(%rbp), %xmm3
// 0x1406bcf2f  660f7f9df0050000               movdqa	%xmm3, 0x5f0(%rbp)
// 0x1406bcf37  660f7f95e0050000               movdqa	%xmm2, 0x5e0(%rbp)
// 0x1406bcf3f  660f7f8dd0050000               movdqa	%xmm1, 0x5d0(%rbp)
// 0x1406bcf47  660f7f85c0050000               movdqa	%xmm0, 0x5c0(%rbp)
// Frames: F102
// 0x1406bcf4f  4c8d05724f8c02                 leaq	0x28c4f72(%rip), %r8    # 0x142f81ec8
// 0x1406bcf56  488d8df0000000                 leaq	0xf0(%rbp), %rcx
// 0x1406bcf5d  b21d                           movb	$0x1d, %dl
// 0x1406bcf5f  e85c889100                     callq	0x140fd57c0 <.text+0xfd47c0>
// 0x1406bcf64  90                             nop
// Frames: F103
// 0x1406bcf65  4c8d05744f8c02                 leaq	0x28c4f74(%rip), %r8    # 0x142f81ee0
// 0x1406bcf6c  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x1406bcf73  b215                           movb	$0x15, %dl
// 0x1406bcf75  e8067d9100                     callq	0x140fd4c80 <.text+0xfd3c80>
// 0x1406bcf7a  90                             nop
// Frames: F104
// 0x1406bcf7b  4c8d05764f8c02                 leaq	0x28c4f76(%rip), %r8    # 0x142f81ef8
// 0x1406bcf82  488d4d10                       leaq	0x10(%rbp), %rcx
// 0x1406bcf86  b228                           movb	$0x28, %dl
// 0x1406bcf88  e8a3989100                     callq	0x140fd6830 <.text+0xfd5830>
// 0x1406bcf8d  90                             nop
// Frames: F105
// 0x1406bcf8e  4c8d057b4f8c02                 leaq	0x28c4f7b(%rip), %r8    # 0x142f81f10
// 0x1406bcf95  488d4da0                       leaq	-0x60(%rbp), %rcx
// 0x1406bcf99  b222                           movb	$0x22, %dl
// 0x1406bcf9b  e840a09100                     callq	0x140fd6fe0 <.text+0xfd5fe0>
// 0x1406bcfa0  90                             nop
// Frames: F106
// 0x1406bcfa1  4c8d05804f8c02                 leaq	0x28c4f80(%rip), %r8    # 0x142f81f28
// 0x1406bcfa8  488d8da0060000                 leaq	0x6a0(%rbp), %rcx
// 0x1406bcfaf  b222                           movb	$0x22, %dl
// 0x1406bcfb1  e8fa5d9100                     callq	0x140fd2db0 <.text+0xfd1db0>
// 0x1406bcfb6  90                             nop
// Frames: F107
// 0x1406bcfb7  4c8d05824f8c02                 leaq	0x28c4f82(%rip), %r8    # 0x142f81f40
// 0x1406bcfbe  488d8d10070000                 leaq	0x710(%rbp), %rcx
// 0x1406bcfc5  b22c                           movb	$0x2c, %dl
// 0x1406bcfc7  e824a79100                     callq	0x140fd76f0 <.text+0xfd66f0>
// 0x1406bcfcc  90                             nop
// Frames: F108
// 0x1406bcfcd  0f1085c0010000                 movups	0x1c0(%rbp), %xmm0
// 0x1406bcfd4  0f298530020000                 movaps	%xmm0, 0x230(%rbp)
// 0x1406bcfdb  0f1085b0010000                 movups	0x1b0(%rbp), %xmm0
// 0x1406bcfe2  0f298520020000                 movaps	%xmm0, 0x220(%rbp)
// 0x1406bcfe9  0f1085a0010000                 movups	0x1a0(%rbp), %xmm0
// 0x1406bcff0  0f298510020000                 movaps	%xmm0, 0x210(%rbp)
// 0x1406bcff7  0f108560010000                 movups	0x160(%rbp), %xmm0
// 0x1406bcffe  0f108d70010000                 movups	0x170(%rbp), %xmm1
// 0x1406bd005  0f109580010000                 movups	0x180(%rbp), %xmm2
// 0x1406bd00c  0f109d90010000                 movups	0x190(%rbp), %xmm3
// 0x1406bd013  0f299d00020000                 movaps	%xmm3, 0x200(%rbp)
// 0x1406bd01a  0f2995f0010000                 movaps	%xmm2, 0x1f0(%rbp)
// 0x1406bd021  0f298de0010000                 movaps	%xmm1, 0x1e0(%rbp)
// 0x1406bd028  0f2985d0010000                 movaps	%xmm0, 0x1d0(%rbp)
// 0x1406bd02f  0f288530060000                 movaps	0x630(%rbp), %xmm0
// 0x1406bd036  0f288d40060000                 movaps	0x640(%rbp), %xmm1
// 0x1406bd03d  0f289550060000                 movaps	0x650(%rbp), %xmm2
// 0x1406bd044  0f289d60060000                 movaps	0x660(%rbp), %xmm3
// 0x1406bd04b  0f298540020000                 movaps	%xmm0, 0x240(%rbp)
// 0x1406bd052  0f298d50020000                 movaps	%xmm1, 0x250(%rbp)
// 0x1406bd059  0f299560020000                 movaps	%xmm2, 0x260(%rbp)
// 0x1406bd060  0f299d70020000                 movaps	%xmm3, 0x270(%rbp)
// 0x1406bd067  0f288570060000                 movaps	0x670(%rbp), %xmm0
// 0x1406bd06e  0f298580020000                 movaps	%xmm0, 0x280(%rbp)
// 0x1406bd075  0f288580060000                 movaps	0x680(%rbp), %xmm0
// 0x1406bd07c  0f298590020000                 movaps	%xmm0, 0x290(%rbp)
// 0x1406bd083  0f288590060000                 movaps	0x690(%rbp), %xmm0
// 0x1406bd08a  0f2985a0020000                 movaps	%xmm0, 0x2a0(%rbp)
// 0x1406bd091  0f288520060000                 movaps	0x620(%rbp), %xmm0
// 0x1406bd098  0f298510030000                 movaps	%xmm0, 0x310(%rbp)
// 0x1406bd09f  0f288510060000                 movaps	0x610(%rbp), %xmm0
// 0x1406bd0a6  0f298500030000                 movaps	%xmm0, 0x300(%rbp)
// 0x1406bd0ad  0f288500060000                 movaps	0x600(%rbp), %xmm0
// 0x1406bd0b4  0f2985f0020000                 movaps	%xmm0, 0x2f0(%rbp)
// 0x1406bd0bb  0f2885c0050000                 movaps	0x5c0(%rbp), %xmm0
// 0x1406bd0c2  0f288dd0050000                 movaps	0x5d0(%rbp), %xmm1
// 0x1406bd0c9  0f2895e0050000                 movaps	0x5e0(%rbp), %xmm2
// 0x1406bd0d0  0f289df0050000                 movaps	0x5f0(%rbp), %xmm3
// 0x1406bd0d7  0f299de0020000                 movaps	%xmm3, 0x2e0(%rbp)
// 0x1406bd0de  0f2995d0020000                 movaps	%xmm2, 0x2d0(%rbp)
// 0x1406bd0e5  0f298dc0020000                 movaps	%xmm1, 0x2c0(%rbp)
// 0x1406bd0ec  0f2985b0020000                 movaps	%xmm0, 0x2b0(%rbp)
// 0x1406bd0f3  0f108550010000                 movups	0x150(%rbp), %xmm0
// 0x1406bd0fa  0f298580030000                 movaps	%xmm0, 0x380(%rbp)
// 0x1406bd101  0f108540010000                 movups	0x140(%rbp), %xmm0
// 0x1406bd108  0f298570030000                 movaps	%xmm0, 0x370(%rbp)
// 0x1406bd10f  0f108530010000                 movups	0x130(%rbp), %xmm0
// 0x1406bd116  0f298560030000                 movaps	%xmm0, 0x360(%rbp)
// 0x1406bd11d  0f1085f0000000                 movups	0xf0(%rbp), %xmm0
// 0x1406bd124  0f108d00010000                 movups	0x100(%rbp), %xmm1
// 0x1406bd12b  0f109510010000                 movups	0x110(%rbp), %xmm2
// 0x1406bd132  0f109d20010000                 movups	0x120(%rbp), %xmm3
// 0x1406bd139  0f299d50030000                 movaps	%xmm3, 0x350(%rbp)
// 0x1406bd140  0f299540030000                 movaps	%xmm2, 0x340(%rbp)
// 0x1406bd147  0f298d30030000                 movaps	%xmm1, 0x330(%rbp)
// 0x1406bd14e  0f298520030000                 movaps	%xmm0, 0x320(%rbp)
// 0x1406bd155  0f1085e0000000                 movups	0xe0(%rbp), %xmm0
// 0x1406bd15c  0f2985f0030000                 movaps	%xmm0, 0x3f0(%rbp)
// 0x1406bd163  0f1085d0000000                 movups	0xd0(%rbp), %xmm0
// 0x1406bd16a  0f2985e0030000                 movaps	%xmm0, 0x3e0(%rbp)
// 0x1406bd171  0f1085c0000000                 movups	0xc0(%rbp), %xmm0
// 0x1406bd178  0f2985d0030000                 movaps	%xmm0, 0x3d0(%rbp)
// 0x1406bd17f  0f108580000000                 movups	0x80(%rbp), %xmm0
// 0x1406bd186  0f108d90000000                 movups	0x90(%rbp), %xmm1
// 0x1406bd18d  0f1095a0000000                 movups	0xa0(%rbp), %xmm2
// 0x1406bd194  0f109db0000000                 movups	0xb0(%rbp), %xmm3
// 0x1406bd19b  0f299dc0030000                 movaps	%xmm3, 0x3c0(%rbp)
// 0x1406bd1a2  0f2995b0030000                 movaps	%xmm2, 0x3b0(%rbp)
// 0x1406bd1a9  0f298da0030000                 movaps	%xmm1, 0x3a0(%rbp)
// 0x1406bd1b0  0f298590030000                 movaps	%xmm0, 0x390(%rbp)
// 0x1406bd1b7  0f104570                       movups	0x70(%rbp), %xmm0
// 0x1406bd1bb  0f298560040000                 movaps	%xmm0, 0x460(%rbp)
// 0x1406bd1c2  0f104560                       movups	0x60(%rbp), %xmm0
// 0x1406bd1c6  0f298550040000                 movaps	%xmm0, 0x450(%rbp)
// 0x1406bd1cd  0f104550                       movups	0x50(%rbp), %xmm0
// 0x1406bd1d1  0f298540040000                 movaps	%xmm0, 0x440(%rbp)
// 0x1406bd1d8  0f104510                       movups	0x10(%rbp), %xmm0
// 0x1406bd1dc  0f104d20                       movups	0x20(%rbp), %xmm1
// 0x1406bd1e0  0f105530                       movups	0x30(%rbp), %xmm2
// 0x1406bd1e4  0f105d40                       movups	0x40(%rbp), %xmm3
// 0x1406bd1e8  0f299d30040000                 movaps	%xmm3, 0x430(%rbp)
// 0x1406bd1ef  0f299520040000                 movaps	%xmm2, 0x420(%rbp)
// 0x1406bd1f6  0f298d10040000                 movaps	%xmm1, 0x410(%rbp)
// 0x1406bd1fd  0f298500040000                 movaps	%xmm0, 0x400(%rbp)
// 0x1406bd204  0f104500                       movups	(%rbp), %xmm0
// 0x1406bd208  0f2985d0040000                 movaps	%xmm0, 0x4d0(%rbp)
// 0x1406bd20f  0f1045f0                       movups	-0x10(%rbp), %xmm0
// 0x1406bd213  0f2985c0040000                 movaps	%xmm0, 0x4c0(%rbp)
// 0x1406bd21a  0f1045e0                       movups	-0x20(%rbp), %xmm0
// 0x1406bd21e  0f2985b0040000                 movaps	%xmm0, 0x4b0(%rbp)
// 0x1406bd225  0f1045a0                       movups	-0x60(%rbp), %xmm0
// 0x1406bd229  0f104db0                       movups	-0x50(%rbp), %xmm1
// 0x1406bd22d  0f1055c0                       movups	-0x40(%rbp), %xmm2
// 0x1406bd231  0f105dd0                       movups	-0x30(%rbp), %xmm3
// 0x1406bd235  0f299da0040000                 movaps	%xmm3, 0x4a0(%rbp)
// 0x1406bd23c  0f299590040000                 movaps	%xmm2, 0x490(%rbp)
// 0x1406bd243  0f298d80040000                 movaps	%xmm1, 0x480(%rbp)
// 0x1406bd24a  0f298570040000                 movaps	%xmm0, 0x470(%rbp)
// 0x1406bd251  0f108500070000                 movups	0x700(%rbp), %xmm0
// 0x1406bd258  0f298540050000                 movaps	%xmm0, 0x540(%rbp)
// 0x1406bd25f  0f1085f0060000                 movups	0x6f0(%rbp), %xmm0
// 0x1406bd266  0f298530050000                 movaps	%xmm0, 0x530(%rbp)
// 0x1406bd26d  0f1085e0060000                 movups	0x6e0(%rbp), %xmm0
// 0x1406bd274  0f298520050000                 movaps	%xmm0, 0x520(%rbp)
// 0x1406bd27b  0f1085a0060000                 movups	0x6a0(%rbp), %xmm0
// 0x1406bd282  0f108db0060000                 movups	0x6b0(%rbp), %xmm1
// 0x1406bd289  0f1095c0060000                 movups	0x6c0(%rbp), %xmm2
// 0x1406bd290  0f109dd0060000                 movups	0x6d0(%rbp), %xmm3
// 0x1406bd297  0f299d10050000                 movaps	%xmm3, 0x510(%rbp)
// 0x1406bd29e  0f299500050000                 movaps	%xmm2, 0x500(%rbp)
// 0x1406bd2a5  0f298df0040000                 movaps	%xmm1, 0x4f0(%rbp)
// 0x1406bd2ac  0f2985e0040000                 movaps	%xmm0, 0x4e0(%rbp)
// 0x1406bd2b3  0f108570070000                 movups	0x770(%rbp), %xmm0
// 0x1406bd2ba  0f2985b0050000                 movaps	%xmm0, 0x5b0(%rbp)
// 0x1406bd2c1  0f108560070000                 movups	0x760(%rbp), %xmm0
// 0x1406bd2c8  0f2985a0050000                 movaps	%xmm0, 0x5a0(%rbp)
// 0x1406bd2cf  0f108550070000                 movups	0x750(%rbp), %xmm0
// 0x1406bd2d6  0f298590050000                 movaps	%xmm0, 0x590(%rbp)
// 0x1406bd2dd  0f108510070000                 movups	0x710(%rbp), %xmm0
// 0x1406bd2e4  0f108d20070000                 movups	0x720(%rbp), %xmm1
// 0x1406bd2eb  0f109530070000                 movups	0x730(%rbp), %xmm2
// 0x1406bd2f2  0f109d40070000                 movups	0x740(%rbp), %xmm3
// 0x1406bd2f9  0f299d80050000                 movaps	%xmm3, 0x580(%rbp)
// 0x1406bd300  0f299570050000                 movaps	%xmm2, 0x570(%rbp)
// 0x1406bd307  0f298d60050000                 movaps	%xmm1, 0x560(%rbp)
// 0x1406bd30e  0f298550050000                 movaps	%xmm0, 0x550(%rbp)
// 0x1406bd315  488d95d0010000                 leaq	0x1d0(%rbp), %rdx
// Frames: F109 -> F110
// 0x1406bd31c  4889f1                         movq	%rsi, %rcx
// 0x1406bd31f  e86cb19100                     callq	0x140fd8490 <.text+0xfd7490>
// Frames: F111
// 0x1406bd324  4889f0                         movq	%rsi, %rax
// 0x1406bd327  4881c410080000                 addq	$0x810, %rsp            # imm = 0x810
// 0x1406bd32e  5b                             popq	%rbx
// 0x1406bd32f  5f                             popq	%rdi
// 0x1406bd330  5e                             popq	%rsi
// 0x1406bd331  415e                           popq	%r14
// 0x1406bd333  5d                             popq	%rbp
// 0x1406bd334  c3                             retq
// Frames: F112 -> F80 -> F81 -> F82 -> F83
// 0x1406bd335  b908000000                     movl	$0x8, %ecx
// 0x1406bd33a  ba48010000                     movl	$0x148, %edx            # imm = 0x148
// 0x1406bd33f  e89d3f6f02                     callq	0x142db12e1 <.text+0x2db02e1>
// 0x1406bd344  90                             nop
// Frames: F112 -> F80 -> F81 -> F82 -> F88
// 0x1406bd345  b908000000                     movl	$0x8, %ecx
// 0x1406bd34a  bab0010000                     movl	$0x1b0, %edx            # imm = 0x1B0
// 0x1406bd34f  e88d3f6f02                     callq	0x142db12e1 <.text+0x2db02e1>
// 0x1406bd354  90                             nop
// 0x1406bd355  eb10                           jmp	0x1406bd367 <.text+0x6bc367>
// Frames: F112 -> F80 -> F81 -> F82 -> F89
// 0x1406bd357  b908000000                     movl	$0x8, %ecx
// 0x1406bd35c  ba80010000                     movl	$0x180, %edx            # imm = 0x180
// 0x1406bd361  e87b3f6f02                     callq	0x142db12e1 <.text+0x2db02e1>
// 0x1406bd366  90                             nop
// 0x1406bd367  0f0b                           ud2
// 0x1406bd369  0f1f8000000000                 nopl	(%rax)
// 0x1406bd370  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406bd375  55                             pushq	%rbp
// 0x1406bd376  4156                           pushq	%r14
// 0x1406bd378  56                             pushq	%rsi
// 0x1406bd379  57                             pushq	%rdi
// 0x1406bd37a  53                             pushq	%rbx
// 0x1406bd37b  4883ec20                       subq	$0x20, %rsp
// 0x1406bd37f  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406bd386  488d8d60010000                 leaq	0x160(%rbp), %rcx
// Frames: F113
// 0x1406bd38d  e80ec09bff                     callq	0x1400793a0 <.text+0x783a0>
// 0x1406bd392  90                             nop
// Frames: F1
// 0x1406bd393  4883c420                       addq	$0x20, %rsp
// 0x1406bd397  5b                             popq	%rbx
// 0x1406bd398  5f                             popq	%rdi
// 0x1406bd399  5e                             popq	%rsi
// 0x1406bd39a  415e                           popq	%r14
// 0x1406bd39c  5d                             popq	%rbp
// 0x1406bd39d  c3                             retq
// 0x1406bd39e  6690                           nop
// 0x1406bd3a0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406bd3a5  55                             pushq	%rbp
// 0x1406bd3a6  4156                           pushq	%r14
// 0x1406bd3a8  56                             pushq	%rsi
// 0x1406bd3a9  57                             pushq	%rdi
// 0x1406bd3aa  53                             pushq	%rbx
// 0x1406bd3ab  4883ec20                       subq	$0x20, %rsp
// 0x1406bd3af  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406bd3b6  488d8da0060000                 leaq	0x6a0(%rbp), %rcx
// Frames: F114
// 0x1406bd3bd  e8debf9bff                     callq	0x1400793a0 <.text+0x783a0>
// 0x1406bd3c2  90                             nop
// 0x1406bd3c3  4883c420                       addq	$0x20, %rsp
// 0x1406bd3c7  5b                             popq	%rbx
// 0x1406bd3c8  5f                             popq	%rdi
// 0x1406bd3c9  5e                             popq	%rsi
// 0x1406bd3ca  415e                           popq	%r14
// 0x1406bd3cc  5d                             popq	%rbp
// 0x1406bd3cd  c3                             retq
// 0x1406bd3ce  6690                           nop
// 0x1406bd3d0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406bd3d5  55                             pushq	%rbp
// 0x1406bd3d6  4156                           pushq	%r14
// 0x1406bd3d8  56                             pushq	%rsi
// 0x1406bd3d9  57                             pushq	%rdi
// 0x1406bd3da  53                             pushq	%rbx
// 0x1406bd3db  4883ec20                       subq	$0x20, %rsp
// 0x1406bd3df  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406bd3e6  488d8d10070000                 leaq	0x710(%rbp), %rcx
// Frames: F115 -> F94
// 0x1406bd3ed  e8aebf9bff                     callq	0x1400793a0 <.text+0x783a0>
// 0x1406bd3f2  90                             nop
// 0x1406bd3f3  4883c420                       addq	$0x20, %rsp
// 0x1406bd3f7  5b                             popq	%rbx
// 0x1406bd3f8  5f                             popq	%rdi
// 0x1406bd3f9  5e                             popq	%rsi
// 0x1406bd3fa  415e                           popq	%r14
// 0x1406bd3fc  5d                             popq	%rbp
// 0x1406bd3fd  c3                             retq
// 0x1406bd3fe  6690                           nop
// 0x1406bd400  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406bd405  55                             pushq	%rbp
// 0x1406bd406  4156                           pushq	%r14
// 0x1406bd408  56                             pushq	%rsi
// 0x1406bd409  57                             pushq	%rdi
// 0x1406bd40a  53                             pushq	%rbx
// 0x1406bd40b  4883ec20                       subq	$0x20, %rsp
// 0x1406bd40f  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406bd416  488d8d30060000                 leaq	0x630(%rbp), %rcx
// Frames: F113
// 0x1406bd41d  e87ebf9bff                     callq	0x1400793a0 <.text+0x783a0>
// 0x1406bd422  90                             nop
// 0x1406bd423  4883c420                       addq	$0x20, %rsp
// 0x1406bd427  5b                             popq	%rbx
// 0x1406bd428  5f                             popq	%rdi
// 0x1406bd429  5e                             popq	%rsi
// 0x1406bd42a  415e                           popq	%r14
// 0x1406bd42c  5d                             popq	%rbp
// 0x1406bd42d  c3                             retq
// 0x1406bd42e  6690                           nop
// 0x1406bd430  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406bd435  55                             pushq	%rbp
// 0x1406bd436  4156                           pushq	%r14
// 0x1406bd438  56                             pushq	%rsi
// 0x1406bd439  57                             pushq	%rdi
// 0x1406bd43a  53                             pushq	%rbx
// 0x1406bd43b  4883ec20                       subq	$0x20, %rsp
// 0x1406bd43f  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406bd446  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// Frames: F115 -> F101
// 0x1406bd44d  e84ebf9bff                     callq	0x1400793a0 <.text+0x783a0>
// 0x1406bd452  90                             nop
// 0x1406bd453  4883c420                       addq	$0x20, %rsp
// 0x1406bd457  5b                             popq	%rbx
// 0x1406bd458  5f                             popq	%rdi
// 0x1406bd459  5e                             popq	%rsi
// 0x1406bd45a  415e                           popq	%r14
// 0x1406bd45c  5d                             popq	%rbp
// 0x1406bd45d  c3                             retq
// 0x1406bd45e  6690                           nop
// 0x1406bd460  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406bd465  55                             pushq	%rbp
// 0x1406bd466  4156                           pushq	%r14
// 0x1406bd468  56                             pushq	%rsi
// 0x1406bd469  57                             pushq	%rdi
// 0x1406bd46a  53                             pushq	%rbx
// 0x1406bd46b  4883ec20                       subq	$0x20, %rsp
// 0x1406bd46f  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406bd476  488d8dc0050000                 leaq	0x5c0(%rbp), %rcx
// Frames: F113
// 0x1406bd47d  e81ebf9bff                     callq	0x1400793a0 <.text+0x783a0>
// 0x1406bd482  90                             nop
// 0x1406bd483  4883c420                       addq	$0x20, %rsp
// 0x1406bd487  5b                             popq	%rbx
// 0x1406bd488  5f                             popq	%rdi
// 0x1406bd489  5e                             popq	%rsi
// 0x1406bd48a  415e                           popq	%r14
// 0x1406bd48c  5d                             popq	%rbp
// 0x1406bd48d  c3                             retq
// 0x1406bd48e  6690                           nop
// 0x1406bd490  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406bd495  55                             pushq	%rbp
// 0x1406bd496  4156                           pushq	%r14
// 0x1406bd498  56                             pushq	%rsi
// 0x1406bd499  57                             pushq	%rdi
// 0x1406bd49a  53                             pushq	%rbx
// 0x1406bd49b  4883ec20                       subq	$0x20, %rsp
// 0x1406bd49f  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406bd4a6  488d8df0000000                 leaq	0xf0(%rbp), %rcx
// 0x1406bd4ad  e8eebe9bff                     callq	0x1400793a0 <.text+0x783a0>
// 0x1406bd4b2  90                             nop
// 0x1406bd4b3  4883c420                       addq	$0x20, %rsp
// 0x1406bd4b7  5b                             popq	%rbx
// 0x1406bd4b8  5f                             popq	%rdi
// 0x1406bd4b9  5e                             popq	%rsi
// 0x1406bd4ba  415e                           popq	%r14
// 0x1406bd4bc  5d                             popq	%rbp
// 0x1406bd4bd  c3                             retq
// 0x1406bd4be  6690                           nop
// 0x1406bd4c0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406bd4c5  55                             pushq	%rbp
// 0x1406bd4c6  4156                           pushq	%r14
// 0x1406bd4c8  56                             pushq	%rsi
// 0x1406bd4c9  57                             pushq	%rdi
// 0x1406bd4ca  53                             pushq	%rbx
// 0x1406bd4cb  4883ec20                       subq	$0x20, %rsp
// 0x1406bd4cf  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406bd4d6  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x1406bd4dd  e8bebe9bff                     callq	0x1400793a0 <.text+0x783a0>
// 0x1406bd4e2  90                             nop
// 0x1406bd4e3  4883c420                       addq	$0x20, %rsp
// 0x1406bd4e7  5b                             popq	%rbx
// 0x1406bd4e8  5f                             popq	%rdi
// 0x1406bd4e9  5e                             popq	%rsi
// 0x1406bd4ea  415e                           popq	%r14
// 0x1406bd4ec  5d                             popq	%rbp
// 0x1406bd4ed  c3                             retq
// 0x1406bd4ee  6690                           nop
// 0x1406bd4f0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406bd4f5  55                             pushq	%rbp
// 0x1406bd4f6  4156                           pushq	%r14
// 0x1406bd4f8  56                             pushq	%rsi
// 0x1406bd4f9  57                             pushq	%rdi
// 0x1406bd4fa  53                             pushq	%rbx
// 0x1406bd4fb  4883ec20                       subq	$0x20, %rsp
// 0x1406bd4ff  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406bd506  488d4d10                       leaq	0x10(%rbp), %rcx
// 0x1406bd50a  e891be9bff                     callq	0x1400793a0 <.text+0x783a0>
// 0x1406bd50f  90                             nop
// 0x1406bd510  4883c420                       addq	$0x20, %rsp
// 0x1406bd514  5b                             popq	%rbx
// 0x1406bd515  5f                             popq	%rdi
// 0x1406bd516  5e                             popq	%rsi
// 0x1406bd517  415e                           popq	%r14
// 0x1406bd519  5d                             popq	%rbp
// 0x1406bd51a  c3                             retq
// 0x1406bd51b  0f1f440000                     nopl	(%rax,%rax)
// 0x1406bd520  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406bd525  55                             pushq	%rbp
// 0x1406bd526  4156                           pushq	%r14
// 0x1406bd528  56                             pushq	%rsi
// 0x1406bd529  57                             pushq	%rdi
// 0x1406bd52a  53                             pushq	%rbx
// 0x1406bd52b  4883ec20                       subq	$0x20, %rsp
// 0x1406bd52f  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406bd536  488d4da0                       leaq	-0x60(%rbp), %rcx
// 0x1406bd53a  e861be9bff                     callq	0x1400793a0 <.text+0x783a0>
// 0x1406bd53f  90                             nop
// 0x1406bd540  4883c420                       addq	$0x20, %rsp
// 0x1406bd544  5b                             popq	%rbx
// 0x1406bd545  5f                             popq	%rdi
// 0x1406bd546  5e                             popq	%rsi
// 0x1406bd547  415e                           popq	%r14
// 0x1406bd549  5d                             popq	%rbp
// 0x1406bd54a  c3                             retq
// 0x1406bd54b  0f1f440000                     nopl	(%rax,%rax)
// 0x1406bd550  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406bd555  55                             pushq	%rbp
// 0x1406bd556  4156                           pushq	%r14
// 0x1406bd558  56                             pushq	%rsi
// 0x1406bd559  57                             pushq	%rdi
// 0x1406bd55a  53                             pushq	%rbx
// 0x1406bd55b  4883ec20                       subq	$0x20, %rsp
// 0x1406bd55f  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406bd566  488d8da0060000                 leaq	0x6a0(%rbp), %rcx
// 0x1406bd56d  e82ebe9bff                     callq	0x1400793a0 <.text+0x783a0>
// 0x1406bd572  90                             nop
// 0x1406bd573  4883c420                       addq	$0x20, %rsp
// 0x1406bd577  5b                             popq	%rbx
// 0x1406bd578  5f                             popq	%rdi
// 0x1406bd579  5e                             popq	%rsi
// 0x1406bd57a  415e                           popq	%r14
// 0x1406bd57c  5d                             popq	%rbp
// 0x1406bd57d  c3                             retq
//
// RANGE rva_6bd580_len_a6c: RVA 0x6bd580, 2668 PDB bytes; 611 instructions
// Evidence: _assembly/roof-v1/C/pack-000/rva_6bd580_len_a6c.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 270:7616: system_roof::impl$0::add_roof_events_and_resources
// Frames: F116
// 0x1406bd580  56                             pushq	%rsi
// 0x1406bd581  4883ec60                       subq	$0x60, %rsp
// Frames: F117
// 0x1406bd585  c744242802000000               movl	$0x2, 0x28(%rsp)
// 0x1406bd58d  488d542428                     leaq	0x28(%rsp), %rdx
// 0x1406bd592  e8d9b09100                     callq	0x140fd8670 <.text+0xfd7670>
// Frames: F118
// 0x1406bd597  4889c1                         movq	%rax, %rcx
// 0x1406bd59a  4531c0                         xorl	%r8d, %r8d
// 0x1406bd59d  e81eb69100                     callq	0x140fd8bc0 <.text+0xfd7bc0>
// 0x1406bd5a2  4889c6                         movq	%rax, %rsi
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd5a5  4883b88800000000               cmpq	$0x0, 0x88(%rax)
// 0x1406bd5ad  0f84cf000000                   je	0x1406bd682 <.text+0x6bc682>
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd5b3  488b4670                       movq	0x70(%rsi), %rax
// 0x1406bd5b7  488b4e78                       movq	0x78(%rsi), %rcx
// 0x1406bd5bb  48ba56d18679ab957344           movabsq	$0x447395ab7986d156, %rdx # imm = 0x447395AB7986D156
// 0x1406bd5c5  4531c0                         xorl	%r8d, %r8d
// 0x1406bd5c8  660f6f05b0f07502               movdqa	0x275f0b0(%rip), %xmm0  # 0x142e1c680
// 0x1406bd5d0  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1406bd5d4  660f6f15b4f07502               movdqa	0x275f0b4(%rip), %xmm2  # 0x142e1c690
// Frames: F14 -> F15 -> F16 -> F17 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd5dc  4821ca                         andq	%rcx, %rdx
// 0x1406bd5df  f30f6f1c10                     movdqu	(%rax,%rdx), %xmm3
// Frames: F18 -> F19 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd5e4  660f6fe3                       movdqa	%xmm3, %xmm4
// 0x1406bd5e8  660f74e0                       pcmpeqb	%xmm0, %xmm4
// 0x1406bd5ec  66440fd7cc                     pmovmskb	%xmm4, %r9d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd5f1  4585c9                         testl	%r9d, %r9d
// 0x1406bd5f4  743a                           je	0x1406bd630 <.text+0x6bc630>
// Frames: F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd5f6  f3450fbcd1                     tzcntl	%r9d, %r10d
// Frames: F26 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd5fb  4901d2                         addq	%rdx, %r10
// 0x1406bd5fe  4921ca                         andq	%rcx, %r10
// Frames: F27 -> F28 -> F29 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd601  49f7da                         negq	%r10
// 0x1406bd604  4f8d1452                       leaq	(%r10,%r10,2), %r10
// Frames: F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd608  f3420f6f64d0e8                 movdqu	-0x18(%rax,%r10,8), %xmm4
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd60f  660f74e2                       pcmpeqb	%xmm2, %xmm4
// 0x1406bd613  66440fd7dc                     pmovmskb	%xmm4, %r11d
// 0x1406bd618  4181fbffff0000                 cmpl	$0xffff, %r11d          # imm = 0xFFFF
// Frames: F36 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd61f  742a                           je	0x1406bd64b <.text+0x6bc64b>
// Frames: F37 -> F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd621  458d51ff                       leal	-0x1(%r9), %r10d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd625  664521ca                       andw	%r9w, %r10w
// 0x1406bd629  4589d1                         movl	%r10d, %r9d
// 0x1406bd62c  75c8                           jne	0x1406bd5f6 <.text+0x6bc5f6>
// 0x1406bd62e  6690                           nop
// Frames: F18 -> F19 -> F38 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd630  660f74d9                       pcmpeqb	%xmm1, %xmm3
// 0x1406bd634  66440fd7cb                     pmovmskb	%xmm3, %r9d
// Frames: F40 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd639  4585c9                         testl	%r9d, %r9d
// 0x1406bd63c  7544                           jne	0x1406bd682 <.text+0x6bc682>
// Frames: F41 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd63e  4c01c2                         addq	%r8, %rdx
// 0x1406bd641  4883c210                       addq	$0x10, %rdx
// Frames: F43 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd645  4983c010                       addq	$0x10, %r8
// Frames: F44 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd649  eb91                           jmp	0x1406bd5dc <.text+0x6bc5dc>
// Frames: F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd64b  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F45 -> F4 -> F5 -> F6 -> F7 -> F119
// 0x1406bd64f  488b40f8                       movq	-0x8(%rax), %rax
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F119
// 0x1406bd653  483b8610020000                 cmpq	0x210(%rsi), %rax
// 0x1406bd65a  7326                           jae	0x1406bd682 <.text+0x6bc682>
// Frames: F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F119
// 0x1406bd65c  488b8e08020000                 movq	0x208(%rsi), %rcx
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F119
// 0x1406bd663  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bd667  4885c0                         testq	%rax, %rax
// 0x1406bd66a  7416                           je	0x1406bd682 <.text+0x6bc682>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F119
// 0x1406bd66c  488b8ed8010000                 movq	0x1d8(%rsi), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F119
// 0x1406bd673  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F119
// 0x1406bd676  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F71 -> F6 -> F7 -> F119
// 0x1406bd67a  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F6 -> F7 -> F119
// 0x1406bd680  7516                           jne	0x1406bd698 <.text+0x6bc698>
// Frames: F72 -> F7 -> F119
// 0x1406bd682  4889f1                         movq	%rsi, %rcx
// 0x1406bd685  e8367a9000                     callq	0x140fc50c0 <.text+0xfc40c0>
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd68a  4883be8800000000               cmpq	$0x0, 0x88(%rsi)
// 0x1406bd692  0f84ca000000                   je	0x1406bd762 <.text+0x6bc762>
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd698  488b4670                       movq	0x70(%rsi), %rax
// 0x1406bd69c  488b4e78                       movq	0x78(%rsi), %rcx
// 0x1406bd6a0  48ba59e1d494e96f3a01           movabsq	$0x13a6fe994d4e159, %rdx # imm = 0x13A6FE994D4E159
// 0x1406bd6aa  4531c0                         xorl	%r8d, %r8d
// 0x1406bd6ad  660fefc0                       pxor	%xmm0, %xmm0
// 0x1406bd6b1  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1406bd6b5  660f6f1563478c02               movdqa	0x28c4763(%rip), %xmm2  # 0x142f81e20
// Frames: F14 -> F15 -> F16 -> F17 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd6bd  4821ca                         andq	%rcx, %rdx
// 0x1406bd6c0  f30f6f1c10                     movdqu	(%rax,%rdx), %xmm3
// Frames: F18 -> F19 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd6c5  660f6fe3                       movdqa	%xmm3, %xmm4
// 0x1406bd6c9  660f74e0                       pcmpeqb	%xmm0, %xmm4
// 0x1406bd6cd  66440fd7cc                     pmovmskb	%xmm4, %r9d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd6d2  4585c9                         testl	%r9d, %r9d
// 0x1406bd6d5  7439                           je	0x1406bd710 <.text+0x6bc710>
// Frames: F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd6d7  f3450fbcd1                     tzcntl	%r9d, %r10d
// Frames: F26 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd6dc  4901d2                         addq	%rdx, %r10
// 0x1406bd6df  4921ca                         andq	%rcx, %r10
// Frames: F27 -> F28 -> F29 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd6e2  49f7da                         negq	%r10
// 0x1406bd6e5  4f8d1452                       leaq	(%r10,%r10,2), %r10
// Frames: F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd6e9  f3420f6f64d0e8                 movdqu	-0x18(%rax,%r10,8), %xmm4
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd6f0  660f74e2                       pcmpeqb	%xmm2, %xmm4
// 0x1406bd6f4  66440fd7dc                     pmovmskb	%xmm4, %r11d
// 0x1406bd6f9  4181fbffff0000                 cmpl	$0xffff, %r11d          # imm = 0xFFFF
// Frames: F36 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd700  7429                           je	0x1406bd72b <.text+0x6bc72b>
// Frames: F37 -> F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd702  458d51ff                       leal	-0x1(%r9), %r10d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd706  664521ca                       andw	%r9w, %r10w
// 0x1406bd70a  4589d1                         movl	%r10d, %r9d
// 0x1406bd70d  75c8                           jne	0x1406bd6d7 <.text+0x6bc6d7>
// 0x1406bd70f  90                             nop
// Frames: F18 -> F19 -> F38 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd710  660f74d9                       pcmpeqb	%xmm1, %xmm3
// 0x1406bd714  66440fd7cb                     pmovmskb	%xmm3, %r9d
// Frames: F40 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd719  4585c9                         testl	%r9d, %r9d
// 0x1406bd71c  7544                           jne	0x1406bd762 <.text+0x6bc762>
// Frames: F41 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd71e  4c01c2                         addq	%r8, %rdx
// 0x1406bd721  4883c210                       addq	$0x10, %rdx
// Frames: F43 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd725  4983c010                       addq	$0x10, %r8
// Frames: F44 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd729  eb92                           jmp	0x1406bd6bd <.text+0x6bc6bd>
// Frames: F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd72b  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F45 -> F4 -> F5 -> F6 -> F7 -> F120
// 0x1406bd72f  488b40f8                       movq	-0x8(%rax), %rax
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F120
// 0x1406bd733  483b8610020000                 cmpq	0x210(%rsi), %rax
// 0x1406bd73a  7326                           jae	0x1406bd762 <.text+0x6bc762>
// Frames: F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F120
// 0x1406bd73c  488b8e08020000                 movq	0x208(%rsi), %rcx
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F120
// 0x1406bd743  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bd747  4885c0                         testq	%rax, %rax
// 0x1406bd74a  7416                           je	0x1406bd762 <.text+0x6bc762>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F120
// 0x1406bd74c  488b8ed8010000                 movq	0x1d8(%rsi), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F120
// 0x1406bd753  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F120
// 0x1406bd756  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F71 -> F6 -> F7 -> F120
// 0x1406bd75a  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F6 -> F7 -> F120
// 0x1406bd760  7516                           jne	0x1406bd778 <.text+0x6bc778>
// Frames: F72 -> F7 -> F120
// 0x1406bd762  4889f1                         movq	%rsi, %rcx
// 0x1406bd765  e8569d9000                     callq	0x140fc74c0 <.text+0xfc64c0>
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd76a  4883be8800000000               cmpq	$0x0, 0x88(%rsi)
// 0x1406bd772  0f84da000000                   je	0x1406bd852 <.text+0x6bc852>
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd778  488b4670                       movq	0x70(%rsi), %rax
// 0x1406bd77c  488b4e78                       movq	0x78(%rsi), %rcx
// 0x1406bd780  48ba7812f14073dabd08           movabsq	$0x8bdda7340f11278, %rdx # imm = 0x8BDDA7340F11278
// 0x1406bd78a  4531c0                         xorl	%r8d, %r8d
// 0x1406bd78d  660f6f053bf07502               movdqa	0x275f03b(%rip), %xmm0  # 0x142e1c7d0
// 0x1406bd795  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1406bd799  660f6f153ff07502               movdqa	0x275f03f(%rip), %xmm2  # 0x142e1c7e0
// Frames: F14 -> F15 -> F16 -> F17 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd7a1  4821ca                         andq	%rcx, %rdx
// 0x1406bd7a4  f30f6f1c10                     movdqu	(%rax,%rdx), %xmm3
// Frames: F18 -> F19 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd7a9  660f6fe3                       movdqa	%xmm3, %xmm4
// 0x1406bd7ad  660f74e0                       pcmpeqb	%xmm0, %xmm4
// 0x1406bd7b1  66440fd7cc                     pmovmskb	%xmm4, %r9d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd7b6  4585c9                         testl	%r9d, %r9d
// 0x1406bd7b9  7445                           je	0x1406bd800 <.text+0x6bc800>
// Frames: F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd7bb  f3450fbcd1                     tzcntl	%r9d, %r10d
// Frames: F26 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd7c0  4901d2                         addq	%rdx, %r10
// 0x1406bd7c3  4921ca                         andq	%rcx, %r10
// Frames: F27 -> F28 -> F29 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd7c6  49f7da                         negq	%r10
// 0x1406bd7c9  4f8d1452                       leaq	(%r10,%r10,2), %r10
// Frames: F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd7cd  f3420f6f64d0e8                 movdqu	-0x18(%rax,%r10,8), %xmm4
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd7d4  660f74e2                       pcmpeqb	%xmm2, %xmm4
// 0x1406bd7d8  66440fd7dc                     pmovmskb	%xmm4, %r11d
// 0x1406bd7dd  4181fbffff0000                 cmpl	$0xffff, %r11d          # imm = 0xFFFF
// Frames: F36 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd7e4  7435                           je	0x1406bd81b <.text+0x6bc81b>
// Frames: F37 -> F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd7e6  458d51ff                       leal	-0x1(%r9), %r10d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd7ea  664521ca                       andw	%r9w, %r10w
// 0x1406bd7ee  4589d1                         movl	%r10d, %r9d
// 0x1406bd7f1  75c8                           jne	0x1406bd7bb <.text+0x6bc7bb>
// 0x1406bd7f3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F18 -> F19 -> F38 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd800  660f74d9                       pcmpeqb	%xmm1, %xmm3
// 0x1406bd804  66440fd7cb                     pmovmskb	%xmm3, %r9d
// Frames: F40 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd809  4585c9                         testl	%r9d, %r9d
// 0x1406bd80c  7544                           jne	0x1406bd852 <.text+0x6bc852>
// Frames: F41 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd80e  4c01c2                         addq	%r8, %rdx
// 0x1406bd811  4883c210                       addq	$0x10, %rdx
// Frames: F43 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd815  4983c010                       addq	$0x10, %r8
// Frames: F44 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd819  eb86                           jmp	0x1406bd7a1 <.text+0x6bc7a1>
// Frames: F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd81b  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F45 -> F4 -> F5 -> F6 -> F7 -> F121
// 0x1406bd81f  488b40f8                       movq	-0x8(%rax), %rax
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F121
// 0x1406bd823  483b8610020000                 cmpq	0x210(%rsi), %rax
// 0x1406bd82a  7326                           jae	0x1406bd852 <.text+0x6bc852>
// Frames: F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F121
// 0x1406bd82c  488b8e08020000                 movq	0x208(%rsi), %rcx
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F121
// 0x1406bd833  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bd837  4885c0                         testq	%rax, %rax
// 0x1406bd83a  7416                           je	0x1406bd852 <.text+0x6bc852>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F121
// 0x1406bd83c  488b8ed8010000                 movq	0x1d8(%rsi), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F121
// 0x1406bd843  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F121
// 0x1406bd846  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F71 -> F6 -> F7 -> F121
// 0x1406bd84a  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F6 -> F7 -> F121
// 0x1406bd850  7516                           jne	0x1406bd868 <.text+0x6bc868>
// Frames: F72 -> F7 -> F121
// 0x1406bd852  4889f1                         movq	%rsi, %rcx
// 0x1406bd855  e866849000                     callq	0x140fc5cc0 <.text+0xfc4cc0>
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd85a  4883be8800000000               cmpq	$0x0, 0x88(%rsi)
// 0x1406bd862  0f84da000000                   je	0x1406bd942 <.text+0x6bc942>
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd868  488b4670                       movq	0x70(%rsi), %rax
// 0x1406bd86c  488b4e78                       movq	0x78(%rsi), %rcx
// 0x1406bd870  48ba9d4710ed991b4ab3           movabsq	$-0x4cb5e46612efb863, %rdx # imm = 0xB34A1B99ED10479D
// 0x1406bd87a  4531c0                         xorl	%r8d, %r8d
// 0x1406bd87d  660f6f05abfa7502               movdqa	0x275faab(%rip), %xmm0  # 0x142e1d330
// 0x1406bd885  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1406bd889  660f6f159f458c02               movdqa	0x28c459f(%rip), %xmm2  # 0x142f81e30
// Frames: F14 -> F15 -> F16 -> F17 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd891  4821ca                         andq	%rcx, %rdx
// 0x1406bd894  f30f6f1c10                     movdqu	(%rax,%rdx), %xmm3
// Frames: F18 -> F19 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd899  660f6fe3                       movdqa	%xmm3, %xmm4
// 0x1406bd89d  660f74e0                       pcmpeqb	%xmm0, %xmm4
// 0x1406bd8a1  66440fd7cc                     pmovmskb	%xmm4, %r9d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd8a6  4585c9                         testl	%r9d, %r9d
// 0x1406bd8a9  7445                           je	0x1406bd8f0 <.text+0x6bc8f0>
// Frames: F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd8ab  f3450fbcd1                     tzcntl	%r9d, %r10d
// Frames: F26 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd8b0  4901d2                         addq	%rdx, %r10
// 0x1406bd8b3  4921ca                         andq	%rcx, %r10
// Frames: F27 -> F28 -> F29 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd8b6  49f7da                         negq	%r10
// 0x1406bd8b9  4f8d1452                       leaq	(%r10,%r10,2), %r10
// Frames: F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd8bd  f3420f6f64d0e8                 movdqu	-0x18(%rax,%r10,8), %xmm4
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd8c4  660f74e2                       pcmpeqb	%xmm2, %xmm4
// 0x1406bd8c8  66440fd7dc                     pmovmskb	%xmm4, %r11d
// 0x1406bd8cd  4181fbffff0000                 cmpl	$0xffff, %r11d          # imm = 0xFFFF
// Frames: F36 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd8d4  7435                           je	0x1406bd90b <.text+0x6bc90b>
// Frames: F37 -> F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd8d6  458d51ff                       leal	-0x1(%r9), %r10d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd8da  664521ca                       andw	%r9w, %r10w
// 0x1406bd8de  4589d1                         movl	%r10d, %r9d
// 0x1406bd8e1  75c8                           jne	0x1406bd8ab <.text+0x6bc8ab>
// 0x1406bd8e3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F18 -> F19 -> F38 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd8f0  660f74d9                       pcmpeqb	%xmm1, %xmm3
// 0x1406bd8f4  66440fd7cb                     pmovmskb	%xmm3, %r9d
// Frames: F40 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd8f9  4585c9                         testl	%r9d, %r9d
// 0x1406bd8fc  7544                           jne	0x1406bd942 <.text+0x6bc942>
// Frames: F41 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd8fe  4c01c2                         addq	%r8, %rdx
// 0x1406bd901  4883c210                       addq	$0x10, %rdx
// Frames: F43 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd905  4983c010                       addq	$0x10, %r8
// Frames: F44 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd909  eb86                           jmp	0x1406bd891 <.text+0x6bc891>
// Frames: F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd90b  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F45 -> F4 -> F5 -> F6 -> F7 -> F122
// 0x1406bd90f  488b40f8                       movq	-0x8(%rax), %rax
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F122
// 0x1406bd913  483b8610020000                 cmpq	0x210(%rsi), %rax
// 0x1406bd91a  7326                           jae	0x1406bd942 <.text+0x6bc942>
// Frames: F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F122
// 0x1406bd91c  488b8e08020000                 movq	0x208(%rsi), %rcx
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F122
// 0x1406bd923  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bd927  4885c0                         testq	%rax, %rax
// 0x1406bd92a  7416                           je	0x1406bd942 <.text+0x6bc942>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F122
// 0x1406bd92c  488b8ed8010000                 movq	0x1d8(%rsi), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F122
// 0x1406bd933  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F122
// 0x1406bd936  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F71 -> F6 -> F7 -> F122
// 0x1406bd93a  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F6 -> F7 -> F122
// 0x1406bd940  7516                           jne	0x1406bd958 <.text+0x6bc958>
// Frames: F72 -> F7 -> F122
// 0x1406bd942  4889f1                         movq	%rsi, %rcx
// 0x1406bd945  e866a79000                     callq	0x140fc80b0 <.text+0xfc70b0>
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd94a  4883be8800000000               cmpq	$0x0, 0x88(%rsi)
// 0x1406bd952  0f84da000000                   je	0x1406bda32 <.text+0x6bca32>
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd958  488b4670                       movq	0x70(%rsi), %rax
// 0x1406bd95c  488b4e78                       movq	0x78(%rsi), %rcx
// 0x1406bd960  48ba6b8b8b26ddbc9b67           movabsq	$0x679bbcdd268b8b6b, %rdx # imm = 0x679BBCDD268B8B6B
// 0x1406bd96a  4531c0                         xorl	%r8d, %r8d
// 0x1406bd96d  660f6f05db9f8402               movdqa	0x2849fdb(%rip), %xmm0  # 0x142f07950
// 0x1406bd975  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1406bd979  660f6f15bf448c02               movdqa	0x28c44bf(%rip), %xmm2  # 0x142f81e40
// Frames: F14 -> F15 -> F16 -> F17 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd981  4821ca                         andq	%rcx, %rdx
// 0x1406bd984  f30f6f1c10                     movdqu	(%rax,%rdx), %xmm3
// Frames: F18 -> F19 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd989  660f6fe3                       movdqa	%xmm3, %xmm4
// 0x1406bd98d  660f74e0                       pcmpeqb	%xmm0, %xmm4
// 0x1406bd991  66440fd7cc                     pmovmskb	%xmm4, %r9d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd996  4585c9                         testl	%r9d, %r9d
// 0x1406bd999  7445                           je	0x1406bd9e0 <.text+0x6bc9e0>
// Frames: F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd99b  f3450fbcd1                     tzcntl	%r9d, %r10d
// Frames: F26 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd9a0  4901d2                         addq	%rdx, %r10
// 0x1406bd9a3  4921ca                         andq	%rcx, %r10
// Frames: F27 -> F28 -> F29 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd9a6  49f7da                         negq	%r10
// 0x1406bd9a9  4f8d1452                       leaq	(%r10,%r10,2), %r10
// Frames: F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd9ad  f3420f6f64d0e8                 movdqu	-0x18(%rax,%r10,8), %xmm4
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd9b4  660f74e2                       pcmpeqb	%xmm2, %xmm4
// 0x1406bd9b8  66440fd7dc                     pmovmskb	%xmm4, %r11d
// 0x1406bd9bd  4181fbffff0000                 cmpl	$0xffff, %r11d          # imm = 0xFFFF
// Frames: F36 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd9c4  7435                           je	0x1406bd9fb <.text+0x6bc9fb>
// Frames: F37 -> F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd9c6  458d51ff                       leal	-0x1(%r9), %r10d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd9ca  664521ca                       andw	%r9w, %r10w
// 0x1406bd9ce  4589d1                         movl	%r10d, %r9d
// 0x1406bd9d1  75c8                           jne	0x1406bd99b <.text+0x6bc99b>
// 0x1406bd9d3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F18 -> F19 -> F38 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd9e0  660f74d9                       pcmpeqb	%xmm1, %xmm3
// 0x1406bd9e4  66440fd7cb                     pmovmskb	%xmm3, %r9d
// Frames: F40 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd9e9  4585c9                         testl	%r9d, %r9d
// 0x1406bd9ec  7544                           jne	0x1406bda32 <.text+0x6bca32>
// Frames: F41 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd9ee  4c01c2                         addq	%r8, %rdx
// 0x1406bd9f1  4883c210                       addq	$0x10, %rdx
// Frames: F43 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd9f5  4983c010                       addq	$0x10, %r8
// Frames: F44 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd9f9  eb86                           jmp	0x1406bd981 <.text+0x6bc981>
// Frames: F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd9fb  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F45 -> F4 -> F5 -> F6 -> F7 -> F123
// 0x1406bd9ff  488b40f8                       movq	-0x8(%rax), %rax
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F123
// 0x1406bda03  483b8610020000                 cmpq	0x210(%rsi), %rax
// 0x1406bda0a  7326                           jae	0x1406bda32 <.text+0x6bca32>
// Frames: F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F123
// 0x1406bda0c  488b8e08020000                 movq	0x208(%rsi), %rcx
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F123
// 0x1406bda13  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bda17  4885c0                         testq	%rax, %rax
// 0x1406bda1a  7416                           je	0x1406bda32 <.text+0x6bca32>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F123
// 0x1406bda1c  488b8ed8010000                 movq	0x1d8(%rsi), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F123
// 0x1406bda23  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F123
// 0x1406bda26  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F71 -> F6 -> F7 -> F123
// 0x1406bda2a  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F6 -> F7 -> F123
// 0x1406bda30  7516                           jne	0x1406bda48 <.text+0x6bca48>
// Frames: F72 -> F7 -> F123
// 0x1406bda32  4889f1                         movq	%rsi, %rcx
// 0x1406bda35  e876ac9000                     callq	0x140fc86b0 <.text+0xfc76b0>
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bda3a  4883be8800000000               cmpq	$0x0, 0x88(%rsi)
// 0x1406bda42  0f84da000000                   je	0x1406bdb22 <.text+0x6bcb22>
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bda48  488b4670                       movq	0x70(%rsi), %rax
// 0x1406bda4c  488b4e78                       movq	0x78(%rsi), %rcx
// 0x1406bda50  48ba38bb626b2db4abe6           movabsq	$-0x19544bd2949d44c8, %rdx # imm = 0xE6ABB42D6B62BB38
// 0x1406bda5a  4531c0                         xorl	%r8d, %r8d
// 0x1406bda5d  660f6f057bee7502               movdqa	0x275ee7b(%rip), %xmm0  # 0x142e1c8e0
// 0x1406bda65  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1406bda69  660f6f154ffd7502               movdqa	0x275fd4f(%rip), %xmm2  # 0x142e1d7c0
// Frames: F14 -> F15 -> F16 -> F17 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bda71  4821ca                         andq	%rcx, %rdx
// 0x1406bda74  f30f6f1c10                     movdqu	(%rax,%rdx), %xmm3
// Frames: F18 -> F19 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bda79  660f6fe3                       movdqa	%xmm3, %xmm4
// 0x1406bda7d  660f74e0                       pcmpeqb	%xmm0, %xmm4
// 0x1406bda81  66440fd7cc                     pmovmskb	%xmm4, %r9d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bda86  4585c9                         testl	%r9d, %r9d
// 0x1406bda89  7445                           je	0x1406bdad0 <.text+0x6bcad0>
// Frames: F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bda8b  f3450fbcd1                     tzcntl	%r9d, %r10d
// Frames: F26 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bda90  4901d2                         addq	%rdx, %r10
// 0x1406bda93  4921ca                         andq	%rcx, %r10
// Frames: F27 -> F28 -> F29 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bda96  49f7da                         negq	%r10
// 0x1406bda99  4f8d1452                       leaq	(%r10,%r10,2), %r10
// Frames: F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bda9d  f3420f6f64d0e8                 movdqu	-0x18(%rax,%r10,8), %xmm4
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bdaa4  660f74e2                       pcmpeqb	%xmm2, %xmm4
// 0x1406bdaa8  66440fd7dc                     pmovmskb	%xmm4, %r11d
// 0x1406bdaad  4181fbffff0000                 cmpl	$0xffff, %r11d          # imm = 0xFFFF
// Frames: F36 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bdab4  7435                           je	0x1406bdaeb <.text+0x6bcaeb>
// Frames: F37 -> F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bdab6  458d51ff                       leal	-0x1(%r9), %r10d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bdaba  664521ca                       andw	%r9w, %r10w
// 0x1406bdabe  4589d1                         movl	%r10d, %r9d
// 0x1406bdac1  75c8                           jne	0x1406bda8b <.text+0x6bca8b>
// 0x1406bdac3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F18 -> F19 -> F38 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bdad0  660f74d9                       pcmpeqb	%xmm1, %xmm3
// 0x1406bdad4  66440fd7cb                     pmovmskb	%xmm3, %r9d
// Frames: F40 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bdad9  4585c9                         testl	%r9d, %r9d
// 0x1406bdadc  7544                           jne	0x1406bdb22 <.text+0x6bcb22>
// Frames: F41 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bdade  4c01c2                         addq	%r8, %rdx
// 0x1406bdae1  4883c210                       addq	$0x10, %rdx
// Frames: F43 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bdae5  4983c010                       addq	$0x10, %r8
// Frames: F44 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bdae9  eb86                           jmp	0x1406bda71 <.text+0x6bca71>
// Frames: F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bdaeb  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F45 -> F4 -> F5 -> F6 -> F7 -> F124
// 0x1406bdaef  488b40f8                       movq	-0x8(%rax), %rax
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F124
// 0x1406bdaf3  483b8610020000                 cmpq	0x210(%rsi), %rax
// 0x1406bdafa  7326                           jae	0x1406bdb22 <.text+0x6bcb22>
// Frames: F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F124
// 0x1406bdafc  488b8e08020000                 movq	0x208(%rsi), %rcx
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F124
// 0x1406bdb03  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bdb07  4885c0                         testq	%rax, %rax
// 0x1406bdb0a  7416                           je	0x1406bdb22 <.text+0x6bcb22>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F124
// 0x1406bdb0c  488b8ed8010000                 movq	0x1d8(%rsi), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F124
// 0x1406bdb13  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F124
// 0x1406bdb16  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F71 -> F6 -> F7 -> F124
// 0x1406bdb1a  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F6 -> F7 -> F124
// 0x1406bdb20  7516                           jne	0x1406bdb38 <.text+0x6bcb38>
// Frames: F72 -> F7 -> F124
// 0x1406bdb22  4889f1                         movq	%rsi, %rcx
// 0x1406bdb25  e896879000                     callq	0x140fc62c0 <.text+0xfc52c0>
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdb2a  4883be8800000000               cmpq	$0x0, 0x88(%rsi)
// 0x1406bdb32  0f84da000000                   je	0x1406bdc12 <.text+0x6bcc12>
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdb38  488b4670                       movq	0x70(%rsi), %rax
// 0x1406bdb3c  488b4e78                       movq	0x78(%rsi), %rcx
// 0x1406bdb40  48ba8e37345fcced90d4           movabsq	$-0x2b6f1233a0cbc872, %rdx # imm = 0xD490EDCC5F34378E
// 0x1406bdb4a  4531c0                         xorl	%r8d, %r8d
// 0x1406bdb4d  660f6f052bec7502               movdqa	0x275ec2b(%rip), %xmm0  # 0x142e1c780
// 0x1406bdb55  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1406bdb59  660f6f152fec7502               movdqa	0x275ec2f(%rip), %xmm2  # 0x142e1c790
// Frames: F14 -> F15 -> F16 -> F17 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdb61  4821ca                         andq	%rcx, %rdx
// 0x1406bdb64  f30f6f1c10                     movdqu	(%rax,%rdx), %xmm3
// Frames: F18 -> F19 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdb69  660f6fe3                       movdqa	%xmm3, %xmm4
// 0x1406bdb6d  660f74e0                       pcmpeqb	%xmm0, %xmm4
// 0x1406bdb71  66440fd7cc                     pmovmskb	%xmm4, %r9d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdb76  4585c9                         testl	%r9d, %r9d
// 0x1406bdb79  7445                           je	0x1406bdbc0 <.text+0x6bcbc0>
// Frames: F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdb7b  f3450fbcd1                     tzcntl	%r9d, %r10d
// Frames: F26 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdb80  4901d2                         addq	%rdx, %r10
// 0x1406bdb83  4921ca                         andq	%rcx, %r10
// Frames: F27 -> F28 -> F29 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdb86  49f7da                         negq	%r10
// 0x1406bdb89  4f8d1452                       leaq	(%r10,%r10,2), %r10
// Frames: F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdb8d  f3420f6f64d0e8                 movdqu	-0x18(%rax,%r10,8), %xmm4
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdb94  660f74e2                       pcmpeqb	%xmm2, %xmm4
// 0x1406bdb98  66440fd7dc                     pmovmskb	%xmm4, %r11d
// 0x1406bdb9d  4181fbffff0000                 cmpl	$0xffff, %r11d          # imm = 0xFFFF
// Frames: F36 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdba4  7435                           je	0x1406bdbdb <.text+0x6bcbdb>
// Frames: F37 -> F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdba6  458d51ff                       leal	-0x1(%r9), %r10d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdbaa  664521ca                       andw	%r9w, %r10w
// 0x1406bdbae  4589d1                         movl	%r10d, %r9d
// 0x1406bdbb1  75c8                           jne	0x1406bdb7b <.text+0x6bcb7b>
// 0x1406bdbb3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F18 -> F19 -> F38 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdbc0  660f74d9                       pcmpeqb	%xmm1, %xmm3
// 0x1406bdbc4  66440fd7cb                     pmovmskb	%xmm3, %r9d
// Frames: F40 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdbc9  4585c9                         testl	%r9d, %r9d
// 0x1406bdbcc  7544                           jne	0x1406bdc12 <.text+0x6bcc12>
// Frames: F41 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdbce  4c01c2                         addq	%r8, %rdx
// 0x1406bdbd1  4883c210                       addq	$0x10, %rdx
// Frames: F43 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdbd5  4983c010                       addq	$0x10, %r8
// Frames: F44 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdbd9  eb86                           jmp	0x1406bdb61 <.text+0x6bcb61>
// Frames: F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdbdb  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F45 -> F4 -> F5 -> F6 -> F7 -> F125
// 0x1406bdbdf  488b40f8                       movq	-0x8(%rax), %rax
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F125
// 0x1406bdbe3  483b8610020000                 cmpq	0x210(%rsi), %rax
// 0x1406bdbea  7326                           jae	0x1406bdc12 <.text+0x6bcc12>
// Frames: F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F125
// 0x1406bdbec  488b8e08020000                 movq	0x208(%rsi), %rcx
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F125
// 0x1406bdbf3  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bdbf7  4885c0                         testq	%rax, %rax
// 0x1406bdbfa  7416                           je	0x1406bdc12 <.text+0x6bcc12>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F125
// 0x1406bdbfc  488b8ed8010000                 movq	0x1d8(%rsi), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F125
// 0x1406bdc03  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F125
// 0x1406bdc06  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F71 -> F6 -> F7 -> F125
// 0x1406bdc0a  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F6 -> F7 -> F125
// 0x1406bdc10  7516                           jne	0x1406bdc28 <.text+0x6bcc28>
// Frames: F72 -> F7 -> F125
// 0x1406bdc12  4889f1                         movq	%rsi, %rcx
// 0x1406bdc15  e8a6929000                     callq	0x140fc6ec0 <.text+0xfc5ec0>
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdc1a  4883be8800000000               cmpq	$0x0, 0x88(%rsi)
// 0x1406bdc22  0f84da000000                   je	0x1406bdd02 <.text+0x6bcd02>
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdc28  488b4670                       movq	0x70(%rsi), %rax
// 0x1406bdc2c  488b4e78                       movq	0x78(%rsi), %rcx
// 0x1406bdc30  48ba0f623078fb32e8e3           movabsq	$-0x1c17cd0487cf9df1, %rdx # imm = 0xE3E832FB7830620F
// 0x1406bdc3a  4531c0                         xorl	%r8d, %r8d
// 0x1406bdc3d  660f6f053bec7502               movdqa	0x275ec3b(%rip), %xmm0  # 0x142e1c880
// 0x1406bdc45  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1406bdc49  660f6f15ff418c02               movdqa	0x28c41ff(%rip), %xmm2  # 0x142f81e50
// Frames: F14 -> F15 -> F16 -> F17 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdc51  4821ca                         andq	%rcx, %rdx
// 0x1406bdc54  f30f6f1c10                     movdqu	(%rax,%rdx), %xmm3
// Frames: F18 -> F19 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdc59  660f6fe3                       movdqa	%xmm3, %xmm4
// 0x1406bdc5d  660f74e0                       pcmpeqb	%xmm0, %xmm4
// 0x1406bdc61  66440fd7cc                     pmovmskb	%xmm4, %r9d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdc66  4585c9                         testl	%r9d, %r9d
// 0x1406bdc69  7445                           je	0x1406bdcb0 <.text+0x6bccb0>
// Frames: F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdc6b  f3450fbcd1                     tzcntl	%r9d, %r10d
// Frames: F26 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdc70  4901d2                         addq	%rdx, %r10
// 0x1406bdc73  4921ca                         andq	%rcx, %r10
// Frames: F27 -> F28 -> F29 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdc76  49f7da                         negq	%r10
// 0x1406bdc79  4f8d1452                       leaq	(%r10,%r10,2), %r10
// Frames: F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdc7d  f3420f6f64d0e8                 movdqu	-0x18(%rax,%r10,8), %xmm4
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdc84  660f74e2                       pcmpeqb	%xmm2, %xmm4
// 0x1406bdc88  66440fd7dc                     pmovmskb	%xmm4, %r11d
// 0x1406bdc8d  4181fbffff0000                 cmpl	$0xffff, %r11d          # imm = 0xFFFF
// Frames: F36 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdc94  7435                           je	0x1406bdccb <.text+0x6bcccb>
// Frames: F37 -> F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdc96  458d51ff                       leal	-0x1(%r9), %r10d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdc9a  664521ca                       andw	%r9w, %r10w
// 0x1406bdc9e  4589d1                         movl	%r10d, %r9d
// 0x1406bdca1  75c8                           jne	0x1406bdc6b <.text+0x6bcc6b>
// 0x1406bdca3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F18 -> F19 -> F38 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdcb0  660f74d9                       pcmpeqb	%xmm1, %xmm3
// 0x1406bdcb4  66440fd7cb                     pmovmskb	%xmm3, %r9d
// Frames: F40 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdcb9  4585c9                         testl	%r9d, %r9d
// 0x1406bdcbc  7544                           jne	0x1406bdd02 <.text+0x6bcd02>
// Frames: F41 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdcbe  4c01c2                         addq	%r8, %rdx
// 0x1406bdcc1  4883c210                       addq	$0x10, %rdx
// Frames: F43 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdcc5  4983c010                       addq	$0x10, %r8
// Frames: F44 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdcc9  eb86                           jmp	0x1406bdc51 <.text+0x6bcc51>
// Frames: F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdccb  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F45 -> F4 -> F5 -> F6 -> F7 -> F126
// 0x1406bdccf  488b40f8                       movq	-0x8(%rax), %rax
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F126
// 0x1406bdcd3  483b8610020000                 cmpq	0x210(%rsi), %rax
// 0x1406bdcda  7326                           jae	0x1406bdd02 <.text+0x6bcd02>
// Frames: F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F126
// 0x1406bdcdc  488b8e08020000                 movq	0x208(%rsi), %rcx
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F126
// 0x1406bdce3  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bdce7  4885c0                         testq	%rax, %rax
// 0x1406bdcea  7416                           je	0x1406bdd02 <.text+0x6bcd02>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F126
// 0x1406bdcec  488b8ed8010000                 movq	0x1d8(%rsi), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F126
// 0x1406bdcf3  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F126
// 0x1406bdcf6  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F71 -> F6 -> F7 -> F126
// 0x1406bdcfa  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F6 -> F7 -> F126
// 0x1406bdd00  7516                           jne	0x1406bdd18 <.text+0x6bcd18>
// Frames: F72 -> F7 -> F126
// 0x1406bdd02  4889f1                         movq	%rsi, %rcx
// 0x1406bdd05  e8a69d9000                     callq	0x140fc7ab0 <.text+0xfc6ab0>
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdd0a  4883be8800000000               cmpq	$0x0, 0x88(%rsi)
// 0x1406bdd12  0f84da000000                   je	0x1406bddf2 <.text+0x6bcdf2>
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdd18  488b4670                       movq	0x70(%rsi), %rax
// 0x1406bdd1c  488b4e78                       movq	0x78(%rsi), %rcx
// 0x1406bdd20  48ba349b20668007def8           movabsq	$-0x721f87f99df64cc, %rdx # imm = 0xF8DE078066209B34
// 0x1406bdd2a  4531c0                         xorl	%r8d, %r8d
// 0x1406bdd2d  660f6f059bec7502               movdqa	0x275ec9b(%rip), %xmm0  # 0x142e1c9d0
// 0x1406bdd35  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1406bdd39  660f6f151f418c02               movdqa	0x28c411f(%rip), %xmm2  # 0x142f81e60
// Frames: F14 -> F15 -> F16 -> F17 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdd41  4821ca                         andq	%rcx, %rdx
// 0x1406bdd44  f30f6f1c10                     movdqu	(%rax,%rdx), %xmm3
// Frames: F18 -> F19 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdd49  660f6fe3                       movdqa	%xmm3, %xmm4
// 0x1406bdd4d  660f74e0                       pcmpeqb	%xmm0, %xmm4
// 0x1406bdd51  66440fd7cc                     pmovmskb	%xmm4, %r9d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdd56  4585c9                         testl	%r9d, %r9d
// 0x1406bdd59  7445                           je	0x1406bdda0 <.text+0x6bcda0>
// Frames: F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdd5b  f3450fbcd1                     tzcntl	%r9d, %r10d
// Frames: F26 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdd60  4901d2                         addq	%rdx, %r10
// 0x1406bdd63  4921ca                         andq	%rcx, %r10
// Frames: F27 -> F28 -> F29 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdd66  49f7da                         negq	%r10
// 0x1406bdd69  4f8d1452                       leaq	(%r10,%r10,2), %r10
// Frames: F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdd6d  f3420f6f64d0e8                 movdqu	-0x18(%rax,%r10,8), %xmm4
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdd74  660f74e2                       pcmpeqb	%xmm2, %xmm4
// 0x1406bdd78  66440fd7dc                     pmovmskb	%xmm4, %r11d
// 0x1406bdd7d  4181fbffff0000                 cmpl	$0xffff, %r11d          # imm = 0xFFFF
// Frames: F36 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdd84  7435                           je	0x1406bddbb <.text+0x6bcdbb>
// Frames: F37 -> F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdd86  458d51ff                       leal	-0x1(%r9), %r10d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdd8a  664521ca                       andw	%r9w, %r10w
// 0x1406bdd8e  4589d1                         movl	%r10d, %r9d
// 0x1406bdd91  75c8                           jne	0x1406bdd5b <.text+0x6bcd5b>
// 0x1406bdd93  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F18 -> F19 -> F38 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdda0  660f74d9                       pcmpeqb	%xmm1, %xmm3
// 0x1406bdda4  66440fd7cb                     pmovmskb	%xmm3, %r9d
// Frames: F40 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bdda9  4585c9                         testl	%r9d, %r9d
// 0x1406bddac  7544                           jne	0x1406bddf2 <.text+0x6bcdf2>
// Frames: F41 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bddae  4c01c2                         addq	%r8, %rdx
// 0x1406bddb1  4883c210                       addq	$0x10, %rdx
// Frames: F43 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bddb5  4983c010                       addq	$0x10, %r8
// Frames: F44 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bddb9  eb86                           jmp	0x1406bdd41 <.text+0x6bcd41>
// Frames: F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bddbb  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F45 -> F4 -> F5 -> F6 -> F7 -> F127
// 0x1406bddbf  488b40f8                       movq	-0x8(%rax), %rax
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F127
// 0x1406bddc3  483b8610020000                 cmpq	0x210(%rsi), %rax
// 0x1406bddca  7326                           jae	0x1406bddf2 <.text+0x6bcdf2>
// Frames: F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F127
// 0x1406bddcc  488b8e08020000                 movq	0x208(%rsi), %rcx
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F127
// 0x1406bddd3  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bddd7  4885c0                         testq	%rax, %rax
// 0x1406bddda  7416                           je	0x1406bddf2 <.text+0x6bcdf2>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F127
// 0x1406bdddc  488b8ed8010000                 movq	0x1d8(%rsi), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F127
// 0x1406bdde3  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F127
// 0x1406bdde6  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F71 -> F6 -> F7 -> F127
// 0x1406bddea  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F6 -> F7 -> F127
// 0x1406bddf0  7516                           jne	0x1406bde08 <.text+0x6bce08>
// Frames: F72 -> F7 -> F127
// 0x1406bddf2  4889f1                         movq	%rsi, %rcx
// 0x1406bddf5  e8b6ae9000                     callq	0x140fc8cb0 <.text+0xfc7cb0>
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bddfa  4883be8800000000               cmpq	$0x0, 0x88(%rsi)
// 0x1406bde02  0f84da000000                   je	0x1406bdee2 <.text+0x6bcee2>
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde08  488b4670                       movq	0x70(%rsi), %rax
// 0x1406bde0c  488b4e78                       movq	0x78(%rsi), %rcx
// 0x1406bde10  48badca451786f03a212           movabsq	$0x12a2036f7851a4dc, %rdx # imm = 0x12A2036F7851A4DC
// 0x1406bde1a  4531c0                         xorl	%r8d, %r8d
// 0x1406bde1d  660f6f051be67502               movdqa	0x275e61b(%rip), %xmm0  # 0x142e1c440
// 0x1406bde25  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1406bde29  660f6f153f408c02               movdqa	0x28c403f(%rip), %xmm2  # 0x142f81e70
// Frames: F14 -> F15 -> F16 -> F17 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde31  4821ca                         andq	%rcx, %rdx
// 0x1406bde34  f30f6f1c10                     movdqu	(%rax,%rdx), %xmm3
// Frames: F18 -> F19 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde39  660f6fe3                       movdqa	%xmm3, %xmm4
// 0x1406bde3d  660f74e0                       pcmpeqb	%xmm0, %xmm4
// 0x1406bde41  66440fd7cc                     pmovmskb	%xmm4, %r9d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde46  4585c9                         testl	%r9d, %r9d
// 0x1406bde49  7445                           je	0x1406bde90 <.text+0x6bce90>
// Frames: F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde4b  f3450fbcd1                     tzcntl	%r9d, %r10d
// Frames: F26 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde50  4901d2                         addq	%rdx, %r10
// 0x1406bde53  4921ca                         andq	%rcx, %r10
// Frames: F27 -> F28 -> F29 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde56  49f7da                         negq	%r10
// 0x1406bde59  4f8d1452                       leaq	(%r10,%r10,2), %r10
// Frames: F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde5d  f3420f6f64d0e8                 movdqu	-0x18(%rax,%r10,8), %xmm4
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde64  660f74e2                       pcmpeqb	%xmm2, %xmm4
// 0x1406bde68  66440fd7dc                     pmovmskb	%xmm4, %r11d
// 0x1406bde6d  4181fbffff0000                 cmpl	$0xffff, %r11d          # imm = 0xFFFF
// Frames: F36 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde74  7435                           je	0x1406bdeab <.text+0x6bceab>
// Frames: F37 -> F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde76  458d51ff                       leal	-0x1(%r9), %r10d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde7a  664521ca                       andw	%r9w, %r10w
// 0x1406bde7e  4589d1                         movl	%r10d, %r9d
// 0x1406bde81  75c8                           jne	0x1406bde4b <.text+0x6bce4b>
// 0x1406bde83  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F18 -> F19 -> F38 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde90  660f74d9                       pcmpeqb	%xmm1, %xmm3
// 0x1406bde94  66440fd7cb                     pmovmskb	%xmm3, %r9d
// Frames: F40 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde99  4585c9                         testl	%r9d, %r9d
// 0x1406bde9c  7544                           jne	0x1406bdee2 <.text+0x6bcee2>
// Frames: F41 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bde9e  4c01c2                         addq	%r8, %rdx
// 0x1406bdea1  4883c210                       addq	$0x10, %rdx
// Frames: F43 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bdea5  4983c010                       addq	$0x10, %r8
// Frames: F44 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bdea9  eb86                           jmp	0x1406bde31 <.text+0x6bce31>
// Frames: F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bdeab  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F45 -> F4 -> F5 -> F6 -> F7 -> F128
// 0x1406bdeaf  488b40f8                       movq	-0x8(%rax), %rax
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F128
// 0x1406bdeb3  483b8610020000                 cmpq	0x210(%rsi), %rax
// 0x1406bdeba  7326                           jae	0x1406bdee2 <.text+0x6bcee2>
// Frames: F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F128
// 0x1406bdebc  488b8e08020000                 movq	0x208(%rsi), %rcx
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F128
// 0x1406bdec3  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bdec7  4885c0                         testq	%rax, %rax
// 0x1406bdeca  7416                           je	0x1406bdee2 <.text+0x6bcee2>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F128
// 0x1406bdecc  488b8ed8010000                 movq	0x1d8(%rsi), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F128
// 0x1406bded3  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F128
// 0x1406bded6  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F71 -> F6 -> F7 -> F128
// 0x1406bdeda  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F6 -> F7 -> F128
// 0x1406bdee0  7516                           jne	0x1406bdef8 <.text+0x6bcef8>
// Frames: F72 -> F7 -> F128
// 0x1406bdee2  4889f1                         movq	%rsi, %rcx
// 0x1406bdee5  e8d6659000                     callq	0x140fc44c0 <.text+0xfc34c0>
// Frames: F2 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdeea  4883be8800000000               cmpq	$0x0, 0x88(%rsi)
// 0x1406bdef2  0f84e3000000                   je	0x1406bdfdb <.text+0x6bcfdb>
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdef8  488b4670                       movq	0x70(%rsi), %rax
// 0x1406bdefc  488b4e78                       movq	0x78(%rsi), %rcx
// 0x1406bdf00  48ba6c1d751de461b4c9           movabsq	$-0x364b9e1be28ae294, %rdx # imm = 0xC9B461E41D751D6C
// 0x1406bdf0a  4531c0                         xorl	%r8d, %r8d
// 0x1406bdf0d  660f6f05ebef7502               movdqa	0x275efeb(%rip), %xmm0  # 0x142e1cf00
// 0x1406bdf15  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1406bdf19  660f6f155f3f8c02               movdqa	0x28c3f5f(%rip), %xmm2  # 0x142f81e80
// Frames: F14 -> F15 -> F16 -> F17 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf21  4821ca                         andq	%rcx, %rdx
// 0x1406bdf24  f30f6f1c10                     movdqu	(%rax,%rdx), %xmm3
// Frames: F18 -> F19 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf29  660f6fe3                       movdqa	%xmm3, %xmm4
// 0x1406bdf2d  660f74e0                       pcmpeqb	%xmm0, %xmm4
// 0x1406bdf31  66440fd7cc                     pmovmskb	%xmm4, %r9d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf36  4585c9                         testl	%r9d, %r9d
// 0x1406bdf39  7445                           je	0x1406bdf80 <.text+0x6bcf80>
// Frames: F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf3b  f3450fbcd1                     tzcntl	%r9d, %r10d
// Frames: F26 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf40  4901d2                         addq	%rdx, %r10
// 0x1406bdf43  4921ca                         andq	%rcx, %r10
// Frames: F27 -> F28 -> F29 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf46  49f7da                         negq	%r10
// 0x1406bdf49  4f8d1452                       leaq	(%r10,%r10,2), %r10
// Frames: F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf4d  f3420f6f64d0e8                 movdqu	-0x18(%rax,%r10,8), %xmm4
// Frames: F32 -> F33 -> F34 -> F35 -> F30 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf54  660f74e2                       pcmpeqb	%xmm2, %xmm4
// 0x1406bdf58  66440fd7dc                     pmovmskb	%xmm4, %r11d
// 0x1406bdf5d  4181fbffff0000                 cmpl	$0xffff, %r11d          # imm = 0xFFFF
// Frames: F36 -> F31 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf64  7435                           je	0x1406bdf9b <.text+0x6bcf9b>
// Frames: F37 -> F23 -> F24 -> F25 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf66  458d51ff                       leal	-0x1(%r9), %r10d
// Frames: F21 -> F22 -> F20 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf6a  664521ca                       andw	%r9w, %r10w
// 0x1406bdf6e  4589d1                         movl	%r10d, %r9d
// 0x1406bdf71  75c8                           jne	0x1406bdf3b <.text+0x6bcf3b>
// 0x1406bdf73  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F18 -> F19 -> F38 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf80  660f74d9                       pcmpeqb	%xmm1, %xmm3
// 0x1406bdf84  66440fd7cb                     pmovmskb	%xmm3, %r9d
// Frames: F40 -> F39 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf89  4585c9                         testl	%r9d, %r9d
// 0x1406bdf8c  754d                           jne	0x1406bdfdb <.text+0x6bcfdb>
// Frames: F41 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf8e  4c01c2                         addq	%r8, %rdx
// 0x1406bdf91  4883c210                       addq	$0x10, %rdx
// Frames: F43 -> F42 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf95  4983c010                       addq	$0x10, %r8
// Frames: F44 -> F11 -> F12 -> F13 -> F3 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf99  eb86                           jmp	0x1406bdf21 <.text+0x6bcf21>
// Frames: F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf9b  4a8d04d0                       leaq	(%rax,%r10,8), %rax
// Frames: F45 -> F4 -> F5 -> F6 -> F7 -> F129
// 0x1406bdf9f  488b40f8                       movq	-0x8(%rax), %rax
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F129
// 0x1406bdfa3  483b8610020000                 cmpq	0x210(%rsi), %rax
// 0x1406bdfaa  732f                           jae	0x1406bdfdb <.text+0x6bcfdb>
// Frames: F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F129
// 0x1406bdfac  488b8e08020000                 movq	0x208(%rsi), %rcx
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F129
// 0x1406bdfb3  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1406bdfb7  4885c0                         testq	%rax, %rax
// 0x1406bdfba  741f                           je	0x1406bdfdb <.text+0x6bcfdb>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F129
// 0x1406bdfbc  488b8ed8010000                 movq	0x1d8(%rsi), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F129
// 0x1406bdfc3  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F51 -> F52 -> F53 -> F6 -> F7 -> F129
// 0x1406bdfc6  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F71 -> F6 -> F7 -> F129
// 0x1406bdfca  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F6 -> F7 -> F129
// 0x1406bdfd0  7409                           je	0x1406bdfdb <.text+0x6bcfdb>
// Frames: F130
// 0x1406bdfd2  4889f0                         movq	%rsi, %rax
// 0x1406bdfd5  4883c460                       addq	$0x60, %rsp
// 0x1406bdfd9  5e                             popq	%rsi
// 0x1406bdfda  c3                             retq
// Frames: F72 -> F7 -> F129
// 0x1406bdfdb  4889f1                         movq	%rsi, %rcx
// 0x1406bdfde  e8dd769000                     callq	0x140fc56c0 <.text+0xfc46c0>
// Frames: F130
// 0x1406bdfe3  4889f0                         movq	%rsi, %rax
// 0x1406bdfe6  4883c460                       addq	$0x60, %rsp
// 0x1406bdfea  5e                             popq	%rsi
// 0x1406bdfeb  c3                             retq
//
// RANGE rva_fd7330_len_22d: RVA 0xfd7330, 557 PDB bytes; 123 instructions
// Evidence: _assembly/roof-v1/C/pack-000/rva_fd7330_len_22d.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 965:1852: bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>
// Frames: F131
// 0x140fd7330  4156                           pushq	%r14
// 0x140fd7332  56                             pushq	%rsi
// 0x140fd7333  57                             pushq	%rdi
// 0x140fd7334  55                             pushq	%rbp
// 0x140fd7335  53                             pushq	%rbx
// 0x140fd7336  4883ec60                       subq	$0x60, %rsp
// 0x140fd733a  bd01000000                     movl	$0x1, %ebp
// Frames: F132 -> F133 -> F134 -> F135
// 0x140fd733f  f0                             lock
// 0x140fd7340  410fc1a8a8050000               xaddl	%ebp, 0x5a8(%r8)
// Frames: F136 -> F137
// 0x140fd7348  833a01                         cmpl	$0x1, (%rdx)
// 0x140fd734b  0f8512010000                   jne	0x140fd7463 <.text+0xfd6463>
// Frames: F138 -> F139 -> F140
// 0x140fd7351  4889d6                         movq	%rdx, %rsi
// 0x140fd7354  498b8098050000                 movq	0x598(%r8), %rax
// 0x140fd735b  4889442458                     movq	%rax, 0x58(%rsp)
// Frames: F141 -> F140
// 0x140fd7360  48394218                       cmpq	%rax, 0x18(%rdx)
// Frames: F140
// 0x140fd7364  0f8511010000                   jne	0x140fd747b <.text+0xfd647b>
// Frames: F142
// 0x140fd736a  488b4608                       movq	0x8(%rsi), %rax
// Frames: F49 -> F50 -> F143 -> F144 -> F145 -> F142
// 0x140fd736e  498b9010020000                 movq	0x210(%r8), %rdx
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F143 -> F144 -> F145 -> F142
// 0x140fd7375  4839d0                         cmpq	%rdx, %rax
// 0x140fd7378  0f835a010000                   jae	0x140fd74d8 <.text+0xfd64d8>
// Frames: F49 -> F50 -> F143 -> F144 -> F145 -> F142
// 0x140fd737e  4d8b8808020000                 movq	0x208(%r8), %r9
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F143 -> F144 -> F145 -> F142
// 0x140fd7385  498b04c1                       movq	(%r9,%rax,8), %rax
// 0x140fd7389  4885c0                         testq	%rax, %rax
// 0x140fd738c  0f8446010000                   je	0x140fd74d8 <.text+0xfd64d8>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F143 -> F144 -> F145 -> F142
// 0x140fd7392  4889cf                         movq	%rcx, %rdi
// 0x140fd7395  498b88d8010000                 movq	0x1d8(%r8), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F143 -> F144 -> F145 -> F142
// 0x140fd739c  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F143 -> F144 -> F145 -> F142
// 0x140fd739f  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F146 -> F147 -> F144 -> F145 -> F142
// 0x140fd73a3  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F148 -> F146 -> F147 -> F144 -> F145 -> F142
// 0x140fd73a9  0f8429010000                   je	0x140fd74d8 <.text+0xfd64d8>
// Frames: F142
// 0x140fd73af  4c8b5610                       movq	0x10(%rsi), %r10
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F149 -> F150 -> F145 -> F142
// 0x140fd73b3  4939d2                         cmpq	%rdx, %r10
// 0x140fd73b6  0f8358010000                   jae	0x140fd7514 <.text+0xfd6514>
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F149 -> F150 -> F145 -> F142
// 0x140fd73bc  4b8b14d1                       movq	(%r9,%r10,8), %rdx
// 0x140fd73c0  4885d2                         testq	%rdx, %rdx
// 0x140fd73c3  0f844b010000                   je	0x140fd7514 <.text+0xfd6514>
// Frames: F151 -> F63 -> F64 -> F49 -> F50 -> F149 -> F150 -> F145 -> F142
// 0x140fd73c9  48f7d2                         notq	%rdx
// Frames: F66 -> F152 -> F63 -> F64 -> F49 -> F50 -> F149 -> F150 -> F145 -> F142
// 0x140fd73cc  48c1e206                       shlq	$0x6, %rdx
// Frames: F68 -> F69 -> F146 -> F153 -> F150 -> F145 -> F142
// 0x140fd73d0  48837c112000                   cmpq	$0x0, 0x20(%rcx,%rdx)
// Frames: F148 -> F146 -> F153 -> F150 -> F145 -> F142
// 0x140fd73d6  0f8438010000                   je	0x140fd7514 <.text+0xfd6514>
// Frames: F145 -> F142
// 0x140fd73dc  4801c8                         addq	%rcx, %rax
// 0x140fd73df  488b5810                       movq	0x10(%rax), %rbx
// 0x140fd73e3  4801ca                         addq	%rcx, %rdx
// Frames: F154 -> F155 -> F156 -> F148 -> F146 -> F153 -> F150 -> F145 -> F142
// 0x140fd73e6  488b4a10                       movq	0x10(%rdx), %rcx
// Frames: F157 -> F158 -> F159 -> F160 -> F161 -> F150 -> F145 -> F142
// 0x140fd73ea  418b80a8050000                 movl	0x5a8(%r8), %eax
// Frames: F162 -> F163 -> F164 -> F165 -> F166 -> F167 -> F168 -> F169
// 0x140fd73f1  896a3c                         movl	%ebp, 0x3c(%rdx)
// Frames: F170 -> F171 -> F172 -> F173 -> F174 -> F175 -> F176 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F169
// 0x140fd73f4  0fb64108                       movzbl	0x8(%rcx), %eax
// Frames: F178 -> F179 -> F175 -> F176 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F169
// 0x140fd73f8  4883c110                       addq	$0x10, %rcx
// Frames: F173 -> F174 -> F175 -> F176 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F169
// 0x140fd73fc  84c0                           testb	%al, %al
// Frames: F180 -> F175 -> F176 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F169
// 0x140fd73fe  0f85a6000000                   jne	0x140fd74aa <.text+0xfd64aa>
// Frames: F181 -> F176 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F169
// 0x140fd7404  488d1595244402                 leaq	0x2442495(%rip), %rdx   # 0x1434198a0
// 0x140fd740b  e8702ff300                     callq	0x141f0a380 <.text+0x1f09380>
// Frames: F176 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F169
// 0x140fd7410  4885c0                         testq	%rax, %rax
// 0x140fd7413  0f8437010000                   je	0x140fd7550 <.text+0xfd6550>
// Frames: F182 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F169
// 0x140fd7419  488b00                         movq	(%rax), %rax
// 0x140fd741c  488b08                         movq	(%rax), %rcx
// 0x140fd741f  488b4008                       movq	0x8(%rax), %rax
// Frames: F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F169
// 0x140fd7423  ff5020                         callq	*0x20(%rax)
// Frames: F183 -> F164 -> F165 -> F166 -> F167 -> F168 -> F169
// 0x140fd7426  4885c0                         testq	%rax, %rax
// 0x140fd7429  0f8421010000                   je	0x140fd7550 <.text+0xfd6550>
// Frames: F184 -> F165 -> F166 -> F167 -> F168 -> F169
// 0x140fd742f  4989c6                         movq	%rax, %r14
// 0x140fd7432  4889d9                         movq	%rbx, %rcx
// 0x140fd7435  e85680aaff                     callq	0x140a7f490 <.text+0xa7e490>
// 0x140fd743a  4c89f1                         movq	%r14, %rcx
// 0x140fd743d  4889c2                         movq	%rax, %rdx
// 0x140fd7440  e83b82f600                     callq	0x141f3f680 <.text+0x1f3e680>
// Frames: F185
// 0x140fd7445  896e20                         movl	%ebp, 0x20(%rsi)
// 0x140fd7448  48b80200000000000080           movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
// Frames: F186 -> F187
// 0x140fd7452  488907                         movq	%rax, (%rdi)
// Frames: F188
// 0x140fd7455  4889f8                         movq	%rdi, %rax
// 0x140fd7458  4883c460                       addq	$0x60, %rsp
// 0x140fd745c  5b                             popq	%rbx
// 0x140fd745d  5d                             popq	%rbp
// 0x140fd745e  5f                             popq	%rdi
// 0x140fd745f  5e                             popq	%rsi
// 0x140fd7460  415e                           popq	%r14
// 0x140fd7462  c3                             retq
// Frames: F189 -> F137
// 0x140fd7463  488d0d46c32702                 leaq	0x227c346(%rip), %rcx   # 0x1432537b0
// 0x140fd746a  4c8d059fc32702                 leaq	0x227c39f(%rip), %r8    # 0x143253810
// 0x140fd7471  ba59000000                     movl	$0x59, %edx
// 0x140fd7476  e865a1dd01                     callq	0x142db15e0 <.text+0x2db05e0>
// Frames: F140
// 0x140fd747b  4883c618                       addq	$0x18, %rsi
// 0x140fd747f  488d0512c42702                 leaq	0x227c412(%rip), %rax   # 0x143253898
// 0x140fd7486  4889442428                     movq	%rax, 0x28(%rsp)
// 0x140fd748b  48c7442420df000000             movq	$0xdf, 0x20(%rsp)
// 0x140fd7494  4c8d0d8dc32702                 leaq	0x227c38d(%rip), %r9    # 0x143253828
// 0x140fd749b  4c8d442458                     leaq	0x58(%rsp), %r8
// 0x140fd74a0  31c9                           xorl	%ecx, %ecx
// 0x140fd74a2  4889f2                         movq	%rsi, %rdx
// 0x140fd74a5  e8e3bbdc01                     callq	0x142da308d <.text+0x2da208d>
// Frames: F190 -> F175 -> F176 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F169
// 0x140fd74aa  48894c2438                     movq	%rcx, 0x38(%rsp)
// 0x140fd74af  488d05922a4402                 leaq	0x2442a92(%rip), %rax   # 0x143419f48
// 0x140fd74b6  4889442420                     movq	%rax, 0x20(%rsp)
// 0x140fd74bb  488d0dfe294402                 leaq	0x24429fe(%rip), %rcx   # 0x143419ec0
// 0x140fd74c2  4c8d0d272a4402                 leaq	0x2442a27(%rip), %r9    # 0x143419ef0
// 0x140fd74c9  4c8d442438                     leaq	0x38(%rsp), %r8
// 0x140fd74ce  ba2b000000                     movl	$0x2b, %edx
// 0x140fd74d3  e868a1dd01                     callq	0x142db1640 <.text+0x2db0640>
// Frames: F191 -> F192 -> F193 -> F194 -> F195 -> F145 -> F142
// 0x140fd74d8  4883c624                       addq	$0x24, %rsi
// Frames: F193 -> F194 -> F195 -> F145 -> F142
// 0x140fd74dc  4889742438                     movq	%rsi, 0x38(%rsp)
// 0x140fd74e1  488d056835f5ff                 leaq	-0xaca98(%rip), %rax    # 0x140f2aa50 <.text+0xf29a50>
// 0x140fd74e8  4889442440                     movq	%rax, 0x40(%rsp)
// 0x140fd74ed  488d4c2437                     leaq	0x37(%rsp), %rcx
// 0x140fd74f2  48894c2448                     movq	%rcx, 0x48(%rsp)
// 0x140fd74f7  4889442450                     movq	%rax, 0x50(%rsp)
// 0x140fd74fc  488d0d1dc12702                 leaq	0x227c11d(%rip), %rcx   # 0x143253620
// 0x140fd7503  4c8d05aec12702                 leaq	0x227c1ae(%rip), %r8    # 0x1432536b8
// 0x140fd750a  488d542438                     leaq	0x38(%rsp), %rdx
// 0x140fd750f  e88ca3dd01                     callq	0x142db18a0 <.text+0x2db08a0>
// Frames: F191 -> F192 -> F196 -> F194 -> F197 -> F145 -> F142
// 0x140fd7514  4883c624                       addq	$0x24, %rsi
// Frames: F196 -> F194 -> F197 -> F145 -> F142
// 0x140fd7518  4889742438                     movq	%rsi, 0x38(%rsp)
// 0x140fd751d  488d052c35f5ff                 leaq	-0xacad4(%rip), %rax    # 0x140f2aa50 <.text+0xf29a50>
// 0x140fd7524  4889442440                     movq	%rax, 0x40(%rsp)
// 0x140fd7529  488d4c2437                     leaq	0x37(%rsp), %rcx
// 0x140fd752e  48894c2448                     movq	%rcx, 0x48(%rsp)
// 0x140fd7533  4889442450                     movq	%rax, 0x50(%rsp)
// 0x140fd7538  488d0de1c02702                 leaq	0x227c0e1(%rip), %rcx   # 0x143253620
// 0x140fd753f  4c8d058ac12702                 leaq	0x227c18a(%rip), %r8    # 0x1432536d0
// 0x140fd7546  488d542438                     leaq	0x38(%rsp), %rdx
// 0x140fd754b  e850a3dd01                     callq	0x142db18a0 <.text+0x2db08a0>
// Frames: F198 -> F164 -> F165 -> F166 -> F167 -> F168 -> F169
// 0x140fd7550  488d0d292a4402                 leaq	0x2442a29(%rip), %rcx   # 0x143419f80
// 0x140fd7557  e8c4a0dd01                     callq	0x142db1620 <.text+0x2db0620>
// 0x140fd755c  cc                             int3
//
// RANGE rva_1f20f20_len_230: RVA 0x1f20f20, 560 PDB bytes; 146 instructions
// Evidence: _assembly/roof-v1/C/pack-000/rva_1f20f20_len_230.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2425:27416: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Frames: F199
// 0x141f20f20  4157                           pushq	%r15
// 0x141f20f22  4156                           pushq	%r14
// 0x141f20f24  4155                           pushq	%r13
// 0x141f20f26  4154                           pushq	%r12
// 0x141f20f28  56                             pushq	%rsi
// 0x141f20f29  57                             pushq	%rdi
// 0x141f20f2a  55                             pushq	%rbp
// 0x141f20f2b  53                             pushq	%rbx
// 0x141f20f2c  4883ec38                       subq	$0x38, %rsp
// 0x141f20f30  4889d0                         movq	%rdx, %rax
// 0x141f20f33  48894c2428                     movq	%rcx, 0x28(%rsp)
// 0x141f20f38  488b9424a8000000               movq	0xa8(%rsp), %rdx
// 0x141f20f40  488b6828                       movq	0x28(%rax), %rbp
// Frames: F200 -> F201
// 0x141f20f44  4989ea                         movq	%rbp, %r10
// 0x141f20f47  4929d2                         subq	%rdx, %r10
// Frames: F46 -> F202 -> F201
// 0x141f20f4a  4d39ca                         cmpq	%r9, %r10
// 0x141f20f4d  0f8368010000                   jae	0x141f210bb <.text+0x1f200bb>
// 0x141f20f53  440fb6bc24b0000000             movzbl	0xb0(%rsp), %r15d
// 0x141f20f5c  488bbc24a0000000               movq	0xa0(%rsp), %rdi
// 0x141f20f64  488b5818                       movq	0x18(%rax), %rbx
// 0x141f20f68  488b4808                       movq	0x8(%rax), %rcx
// 0x141f20f6c  4c8b5810                       movq	0x10(%rax), %r11
// 0x141f20f70  4c895c2430                     movq	%r11, 0x30(%rsp)
// 0x141f20f75  4c8b6838                       movq	0x38(%rax), %r13
// 0x141f20f79  eb1b                           jmp	0x141f20f96 <.text+0x1f1ff96>
// 0x141f20f7b  0f1f440000                     nopl	(%rax,%rax)
// 0x141f20f80  48897038                       movq	%rsi, 0x38(%rax)
// 0x141f20f84  4989f5                         movq	%rsi, %r13
// Frames: F200 -> F201
// 0x141f20f87  4c89d5                         movq	%r10, %rbp
// 0x141f20f8a  4929d2                         subq	%rdx, %r10
// Frames: F46 -> F202 -> F201
// 0x141f20f8d  4d39ca                         cmpq	%r9, %r10
// 0x141f20f90  0f8325010000                   jae	0x141f210bb <.text+0x1f200bb>
// Frames: F203
// 0x141f20f96  470fb61c10                     movzbl	(%r8,%r10), %r11d
// 0x141f20f9b  4c0fa3db                       btq	%r11, %rbx
// 0x141f20f9f  0f838b000000                   jae	0x141f21030 <.text+0x1f20030>
// Frames: F204
// 0x141f20fa5  4939cd                         cmpq	%rcx, %r13
// 0x141f20fa8  4989cb                         movq	%rcx, %r11
// 0x141f20fab  4d0f42dd                       cmovbq	%r13, %r11
// 0x141f20faf  4589fe                         movl	%r15d, %r14d
// 0x141f20fb2  41f6c701                       testb	$0x1, %r15b
// 0x141f20fb6  4c0f45d9                       cmovneq	%rcx, %r11
// Frames: F205
// 0x141f20fba  498d73ff                       leaq	-0x1(%r11), %rsi
// 0x141f20fbe  4839d6                         cmpq	%rdx, %rsi
// 0x141f20fc1  0f8389000000                   jae	0x141f21050 <.text+0x1f20050>
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f20fc7  4d89dc                         movq	%r11, %r12
// 0x141f20fca  49f7dc                         negq	%r12
// 0x141f20fcd  488d77ff                       leaq	-0x1(%rdi), %rsi
// 0x141f20fd1  4e8d3c1e                       leaq	(%rsi,%r11), %r15
// 0x141f20fd5  4b8d3413                       leaq	(%r11,%r10), %rsi
// 0x141f20fd9  48ffce                         decq	%rsi
// 0x141f20fdc  0f1f4000                       nopl	(%rax)
// Frames: F209 -> F206 -> F207 -> F208 -> F205
// 0x141f20fe0  4d85e4                         testq	%r12, %r12
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f20fe3  7474                           je	0x141f21059 <.text+0x1f20059>
// Frames: F210
// 0x141f20fe5  4c39ce                         cmpq	%r9, %rsi
// 0x141f20fe8  0f8340010000                   jae	0x141f2112e <.text+0x1f2012e>
// 0x141f20fee  450fb61f                       movzbl	(%r15), %r11d
// 0x141f20ff2  49ffc4                         incq	%r12
// 0x141f20ff5  49ffcf                         decq	%r15
// 0x141f20ff8  453a1c30                       cmpb	(%r8,%rsi), %r11b
// 0x141f20ffc  488d76ff                       leaq	-0x1(%rsi), %rsi
// 0x141f21000  74de                           je	0x141f20fe0 <.text+0x1f1ffe0>
// Frames: F211
// 0x141f21002  4829cd                         subq	%rcx, %rbp
// 0x141f21005  4c29e5                         subq	%r12, %rbp
// 0x141f21008  48896828                       movq	%rbp, 0x28(%rax)
// 0x141f2100c  4889d6                         movq	%rdx, %rsi
// 0x141f2100f  4989ea                         movq	%rbp, %r10
// 0x141f21012  4589f7                         movl	%r14d, %r15d
// 0x141f21015  4584f6                         testb	%r14b, %r14b
// 0x141f21018  0f8462ffffff                   je	0x141f20f80 <.text+0x1f1ff80>
// 0x141f2101e  e964ffffff                     jmp	0x141f20f87 <.text+0x1f1ff87>
// 0x141f21023  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F212
// 0x141f21030  4c895028                       movq	%r10, 0x28(%rax)
// 0x141f21034  4889d6                         movq	%rdx, %rsi
// Frames: F213
// 0x141f21037  4584ff                         testb	%r15b, %r15b
// 0x141f2103a  0f8440ffffff                   je	0x141f20f80 <.text+0x1f1ff80>
// 0x141f21040  e942ffffff                     jmp	0x141f20f87 <.text+0x1f1ff87>
// 0x141f21045  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F209 -> F206 -> F207 -> F208 -> F205
// 0x141f21050  4d85db                         testq	%r11, %r11
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f21053  0f85e7000000                   jne	0x141f21140 <.text+0x1f20140>
// Frames: F214
// 0x141f21059  4584f6                         testb	%r14b, %r14b
// 0x141f2105c  4c89ee                         movq	%r13, %rsi
// 0x141f2105f  480f45f2                       cmovneq	%rdx, %rsi
// Frames: F215
// 0x141f21063  4f8d3c10                       leaq	(%r8,%r10), %r15
// 0x141f21067  4989cc                         movq	%rcx, %r12
// 0x141f2106a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F209 -> F216 -> F217 -> F215
// 0x141f21070  4939f4                         cmpq	%rsi, %r12
// Frames: F216 -> F217 -> F215
// 0x141f21073  7357                           jae	0x141f210cc <.text+0x1f200cc>
// Frames: F218
// 0x141f21075  4939d4                         cmpq	%rdx, %r12
// 0x141f21078  0f8381000000                   jae	0x141f210ff <.text+0x1f200ff>
// 0x141f2107e  4f8d1c22                       leaq	(%r10,%r12), %r11
// 0x141f21082  4d39cb                         cmpq	%r9, %r11
// 0x141f21085  0f8387000000                   jae	0x141f21112 <.text+0x1f20112>
// 0x141f2108b  460fb61c27                     movzbl	(%rdi,%r12), %r11d
// 0x141f21090  473a1c27                       cmpb	(%r15,%r12), %r11b
// Frames: F219 -> F220 -> F221 -> F217 -> F215
// 0x141f21094  4d8d642401                     leaq	0x1(%r12), %r12
// Frames: F218
// 0x141f21099  74d5                           je	0x141f21070 <.text+0x1f20070>
// Frames: F222
// 0x141f2109b  488b742430                     movq	0x30(%rsp), %rsi
// Frames: F223
// 0x141f210a0  4829f5                         subq	%rsi, %rbp
// 0x141f210a3  48896828                       movq	%rbp, 0x28(%rax)
// 0x141f210a7  4989ea                         movq	%rbp, %r10
// 0x141f210aa  4589f7                         movl	%r14d, %r15d
// 0x141f210ad  4584f6                         testb	%r14b, %r14b
// 0x141f210b0  0f84cafeffff                   je	0x141f20f80 <.text+0x1f1ff80>
// 0x141f210b6  e9ccfeffff                     jmp	0x141f20f87 <.text+0x1f1ff87>
// 0x141f210bb  48c7402800000000               movq	$0x0, 0x28(%rax)
// 0x141f210c3  31c0                           xorl	%eax, %eax
// 0x141f210c5  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x141f210ca  eb1f                           jmp	0x141f210eb <.text+0x1f200eb>
// 0x141f210cc  4c895028                       movq	%r10, 0x28(%rax)
// 0x141f210d0  4584f6                         testb	%r14b, %r14b
// 0x141f210d3  7504                           jne	0x141f210d9 <.text+0x1f200d9>
// 0x141f210d5  48895038                       movq	%rdx, 0x38(%rax)
// Frames: F224 -> F225
// 0x141f210d9  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x141f210de  4c895108                       movq	%r10, 0x8(%rcx)
// 0x141f210e2  48896910                       movq	%rbp, 0x10(%rcx)
// 0x141f210e6  b801000000                     movl	$0x1, %eax
// Frames: F226
// 0x141f210eb  488901                         movq	%rax, (%rcx)
// 0x141f210ee  4883c438                       addq	$0x38, %rsp
// 0x141f210f2  5b                             popq	%rbx
// 0x141f210f3  5d                             popq	%rbp
// 0x141f210f4  5f                             popq	%rdi
// 0x141f210f5  5e                             popq	%rsi
// 0x141f210f6  415c                           popq	%r12
// 0x141f210f8  415d                           popq	%r13
// 0x141f210fa  415e                           popq	%r14
// 0x141f210fc  415f                           popq	%r15
// 0x141f210fe  c3                             retq
// Frames: F46 -> F202 -> F201
// 0x141f210ff  4839d1                         cmpq	%rdx, %rcx
// 0x141f21102  480f46ca                       cmovbeq	%rdx, %rcx
// Frames: F218
// 0x141f21106  4c8d05a30e4f01                 leaq	0x14f0ea3(%rip), %r8    # 0x143411fb0
// 0x141f2110d  e8b005e900                     callq	0x142db16c2 <.text+0x2db06c2>
// Frames: F215
// 0x141f21112  4901ca                         addq	%rcx, %r10
// 0x141f21115  4d39d1                         cmpq	%r10, %r9
// 0x141f21118  4d0f47d1                       cmovaq	%r9, %r10
// Frames: F218
// 0x141f2111c  4c8d05a50e4f01                 leaq	0x14f0ea5(%rip), %r8    # 0x143411fc8
// 0x141f21123  4c89d1                         movq	%r10, %rcx
// 0x141f21126  4c89ca                         movq	%r9, %rdx
// 0x141f21129  e89405e900                     callq	0x142db16c2 <.text+0x2db06c2>
// Frames: F210
// 0x141f2112e  4c8d05c30e4f01                 leaq	0x14f0ec3(%rip), %r8    # 0x143411ff8
// 0x141f21135  4889f1                         movq	%rsi, %rcx
// 0x141f21138  4c89ca                         movq	%r9, %rdx
// 0x141f2113b  e88205e900                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f21140  4c8d05990e4f01                 leaq	0x14f0e99(%rip), %r8    # 0x143411fe0
// 0x141f21147  4889f1                         movq	%rsi, %rcx
// 0x141f2114a  e87305e900                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f2114f  cc                             int3
//
// RANGE rva_1f29080_len_20f: RVA 0x1f29080, 527 PDB bytes; 115 instructions
// Evidence: _assembly/roof-v1/C/pack-000/rva_1f29080_len_20f.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2426:326748: bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >
// Frames: F227
// 0x141f29080  4156                           pushq	%r14
// 0x141f29082  56                             pushq	%rsi
// 0x141f29083  57                             pushq	%rdi
// 0x141f29084  55                             pushq	%rbp
// 0x141f29085  53                             pushq	%rbx
// 0x141f29086  4883ec70                       subq	$0x70, %rsp
// Frames: F136 -> F228 -> F229
// 0x141f2908a  833a01                         cmpl	$0x1, (%rdx)
// 0x141f2908d  0f857a010000                   jne	0x141f2920d <.text+0x1f2820d>
// Frames: F138 -> F139 -> F230 -> F229
// 0x141f29093  4889d7                         movq	%rdx, %rdi
// 0x141f29096  498b8098050000                 movq	0x598(%r8), %rax
// 0x141f2909d  4889442438                     movq	%rax, 0x38(%rsp)
// Frames: F141 -> F230 -> F229
// 0x141f290a2  48394218                       cmpq	%rax, 0x18(%rdx)
// Frames: F230 -> F229
// 0x141f290a6  0f8579010000                   jne	0x141f29225 <.text+0x1f28225>
// Frames: F231 -> F229
// 0x141f290ac  4889ce                         movq	%rcx, %rsi
// 0x141f290af  488b4708                       movq	0x8(%rdi), %rax
// Frames: F49 -> F50 -> F232 -> F233 -> F231 -> F229
// 0x141f290b3  498b9010020000                 movq	0x210(%r8), %rdx
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F232 -> F233 -> F231 -> F229
// 0x141f290ba  4839d0                         cmpq	%rdx, %rax
// 0x141f290bd  0f83e1000000                   jae	0x141f291a4 <.text+0x1f281a4>
// Frames: F49 -> F50 -> F232 -> F233 -> F231 -> F229
// 0x141f290c3  4d8b8808020000                 movq	0x208(%r8), %r9
// Frames: F54 -> F55 -> F56 -> F48 -> F49 -> F50 -> F232 -> F233 -> F231 -> F229
// 0x141f290ca  498b04c1                       movq	(%r9,%rax,8), %rax
// 0x141f290ce  4885c0                         testq	%rax, %rax
// 0x141f290d1  0f84cd000000                   je	0x141f291a4 <.text+0x1f281a4>
// Frames: F57 -> F58 -> F59 -> F60 -> F61 -> F62 -> F63 -> F64 -> F49 -> F50 -> F232 -> F233 -> F231 -> F229
// 0x141f290d7  498b88d8010000                 movq	0x1d8(%r8), %rcx
// Frames: F65 -> F63 -> F64 -> F49 -> F50 -> F232 -> F233 -> F231 -> F229
// 0x141f290de  48f7d0                         notq	%rax
// Frames: F66 -> F67 -> F63 -> F64 -> F49 -> F50 -> F232 -> F233 -> F231 -> F229
// 0x141f290e1  48c1e006                       shlq	$0x6, %rax
// Frames: F68 -> F69 -> F55 -> F70 -> F234 -> F233 -> F231 -> F229
// 0x141f290e5  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F235 -> F233 -> F231 -> F229
// 0x141f290eb  0f84b3000000                   je	0x141f291a4 <.text+0x1f281a4>
// Frames: F231 -> F229
// 0x141f290f1  4c8b5710                       movq	0x10(%rdi), %r10
// Frames: F235 -> F233 -> F231 -> F229
// 0x141f290f5  4939d2                         cmpq	%rdx, %r10
// 0x141f290f8  0f83a6000000                   jae	0x141f291a4 <.text+0x1f281a4>
// Frames: F54 -> F55 -> F52 -> F48 -> F49 -> F50 -> F236 -> F233 -> F231 -> F229
// 0x141f290fe  4b8b14d1                       movq	(%r9,%r10,8), %rdx
// 0x141f29102  4885d2                         testq	%rdx, %rdx
// 0x141f29105  0f8499000000                   je	0x141f291a4 <.text+0x1f281a4>
// Frames: F151 -> F63 -> F64 -> F49 -> F50 -> F236 -> F233 -> F231 -> F229
// 0x141f2910b  48f7d2                         notq	%rdx
// Frames: F66 -> F152 -> F63 -> F64 -> F49 -> F50 -> F236 -> F233 -> F231 -> F229
// 0x141f2910e  48c1e206                       shlq	$0x6, %rdx
// Frames: F68 -> F69 -> F55 -> F70 -> F237 -> F233 -> F231 -> F229
// 0x141f29112  48837c112000                   cmpq	$0x0, 0x20(%rcx,%rdx)
// Frames: F238 -> F233 -> F231 -> F229
// 0x141f29118  0f8486000000                   je	0x141f291a4 <.text+0x1f281a4>
// Frames: F132 -> F133 -> F134 -> F239 -> F240
// 0x141f2911e  4801c8                         addq	%rcx, %rax
// 0x141f29121  bd01000000                     movl	$0x1, %ebp
// 0x141f29126  f0                             lock
// 0x141f29127  410fc1a8a8050000               xaddl	%ebp, 0x5a8(%r8)
// 0x141f2912f  4801ca                         addq	%rcx, %rdx
// Frames: F154 -> F155 -> F156 -> F241 -> F146 -> F147 -> F144 -> F145 -> F242 -> F240
// 0x141f29132  488b5810                       movq	0x10(%rax), %rbx
// Frames: F154 -> F155 -> F156 -> F241 -> F146 -> F153 -> F150 -> F145 -> F242 -> F240
// 0x141f29136  488b4a10                       movq	0x10(%rdx), %rcx
// Frames: F157 -> F158 -> F159 -> F160 -> F161 -> F150 -> F145 -> F242 -> F240
// 0x141f2913a  418b80a8050000                 movl	0x5a8(%r8), %eax
// Frames: F162 -> F163 -> F164 -> F165 -> F166 -> F167 -> F168 -> F243 -> F240
// 0x141f29141  896a3c                         movl	%ebp, 0x3c(%rdx)
// Frames: F170 -> F171 -> F172 -> F173 -> F174 -> F175 -> F176 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F243 -> F240
// 0x141f29144  0fb64108                       movzbl	0x8(%rcx), %eax
// Frames: F178 -> F179 -> F175 -> F176 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F243 -> F240
// 0x141f29148  4883c110                       addq	$0x10, %rcx
// Frames: F173 -> F174 -> F175 -> F176 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F243 -> F240
// 0x141f2914c  84c0                           testb	%al, %al
// Frames: F180 -> F175 -> F176 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F243 -> F240
// 0x141f2914e  0f8500010000                   jne	0x141f29254 <.text+0x1f28254>
// Frames: F181 -> F176 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F243 -> F240
// 0x141f29154  488d1545074f01                 leaq	0x14f0745(%rip), %rdx   # 0x1434198a0
// 0x141f2915b  e82012feff                     callq	0x141f0a380 <.text+0x1f09380>
// Frames: F176 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F243 -> F240
// 0x141f29160  4885c0                         testq	%rax, %rax
// 0x141f29163  0f8419010000                   je	0x141f29282 <.text+0x1f28282>
// Frames: F182 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F243 -> F240
// 0x141f29169  488b00                         movq	(%rax), %rax
// 0x141f2916c  488b08                         movq	(%rax), %rcx
// 0x141f2916f  488b4008                       movq	0x8(%rax), %rax
// Frames: F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F243 -> F240
// 0x141f29173  ff5020                         callq	*0x20(%rax)
// Frames: F183 -> F164 -> F165 -> F166 -> F167 -> F168 -> F243 -> F240
// 0x141f29176  4885c0                         testq	%rax, %rax
// 0x141f29179  0f8403010000                   je	0x141f29282 <.text+0x1f28282>
// Frames: F184 -> F165 -> F166 -> F167 -> F168 -> F243 -> F240
// 0x141f2917f  4989c6                         movq	%rax, %r14
// 0x141f29182  4889d9                         movq	%rbx, %rcx
// 0x141f29185  e80663b5fe                     callq	0x140a7f490 <.text+0xa7e490>
// 0x141f2918a  4c89f1                         movq	%r14, %rcx
// 0x141f2918d  4889c2                         movq	%rax, %rdx
// 0x141f29190  e8eb640100                     callq	0x141f3f680 <.text+0x1f3e680>
// Frames: F244 -> F240
// 0x141f29195  896f20                         movl	%ebp, 0x20(%rdi)
// 0x141f29198  48b80200000000000080           movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
// 0x141f291a2  eb58                           jmp	0x141f291fc <.text+0x1f281fc>
// Frames: F245 -> F246 -> F229
// 0x141f291a4  48b80000000000000080           movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
// 0x141f291ae  4889442438                     movq	%rax, 0x38(%rsp)
// 0x141f291b3  488d0d8e924e01                 leaq	0x14e928e(%rip), %rcx   # 0x143412448
// 0x141f291ba  48894c2440                     movq	%rcx, 0x40(%rsp)
// 0x141f291bf  48c744244817000000             movq	$0x17, 0x48(%rsp)
// 0x141f291c8  4889442450                     movq	%rax, 0x50(%rsp)
// 0x141f291cd  48c744245801000000             movq	$0x1, 0x58(%rsp)
// 0x141f291d6  48c744246000000000             movq	$0x0, 0x60(%rsp)
// 0x141f291df  c644246800                     movb	$0x0, 0x68(%rsp)
// 0x141f291e4  488d4c2438                     leaq	0x38(%rsp), %rcx
// 0x141f291e9  e87249ed00                     callq	0x142dfdb60 <.text+0x2dfcb60>
// 0x141f291ee  48894608                       movq	%rax, 0x8(%rsi)
// 0x141f291f2  48b80100000000000080           movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
// Frames: F247
// 0x141f291fc  488906                         movq	%rax, (%rsi)
// 0x141f291ff  4889f0                         movq	%rsi, %rax
// 0x141f29202  4883c470                       addq	$0x70, %rsp
// 0x141f29206  5b                             popq	%rbx
// 0x141f29207  5d                             popq	%rbp
// 0x141f29208  5f                             popq	%rdi
// 0x141f29209  5e                             popq	%rsi
// 0x141f2920a  415e                           popq	%r14
// 0x141f2920c  c3                             retq
// Frames: F189 -> F228 -> F229
// 0x141f2920d  488d0d14934e01                 leaq	0x14e9314(%rip), %rcx   # 0x143412528
// 0x141f29214  4c8d052dac4e01                 leaq	0x14eac2d(%rip), %r8    # 0x143413e48
// 0x141f2921b  ba59000000                     movl	$0x59, %edx
// 0x141f29220  e8bb83e800                     callq	0x142db15e0 <.text+0x2db05e0>
// Frames: F230 -> F229
// 0x141f29225  4883c718                       addq	$0x18, %rdi
// 0x141f29229  488d0530ac4e01                 leaq	0x14eac30(%rip), %rax   # 0x143413e60
// 0x141f29230  4889442428                     movq	%rax, 0x28(%rsp)
// 0x141f29235  48c7442420df000000             movq	$0xdf, 0x20(%rsp)
// 0x141f2923e  4c8d0dc3934e01                 leaq	0x14e93c3(%rip), %r9    # 0x143412608
// 0x141f29245  4c8d442438                     leaq	0x38(%rsp), %r8
// 0x141f2924a  31c9                           xorl	%ecx, %ecx
// 0x141f2924c  4889fa                         movq	%rdi, %rdx
// 0x141f2924f  e8399ee700                     callq	0x142da308d <.text+0x2da208d>
// Frames: F190 -> F175 -> F176 -> F177 -> F164 -> F165 -> F166 -> F167 -> F168 -> F243 -> F240
// 0x141f29254  48894c2438                     movq	%rcx, 0x38(%rsp)
// 0x141f29259  488d05e80c4f01                 leaq	0x14f0ce8(%rip), %rax   # 0x143419f48
// 0x141f29260  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141f29265  488d0d540c4f01                 leaq	0x14f0c54(%rip), %rcx   # 0x143419ec0
// 0x141f2926c  4c8d0d7d0c4f01                 leaq	0x14f0c7d(%rip), %r9    # 0x143419ef0
// 0x141f29273  4c8d442438                     leaq	0x38(%rsp), %r8
// 0x141f29278  ba2b000000                     movl	$0x2b, %edx
// 0x141f2927d  e8be83e800                     callq	0x142db1640 <.text+0x2db0640>
// Frames: F198 -> F164 -> F165 -> F166 -> F167 -> F168 -> F243 -> F240
// 0x141f29282  488d0df70c4f01                 leaq	0x14f0cf7(%rip), %rcx   # 0x143419f80
// 0x141f29289  e89283e800                     callq	0x142db1620 <.text+0x2db0620>
// 0x141f2928e  cc                             int3
//
// RANGE rva_1f36190_len_230: RVA 0x1f36190, 560 PDB bytes; 146 instructions
// Evidence: _assembly/roof-v1/C/pack-001/rva_1f36190_len_230.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2435:25644: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Frames: F199
// 0x141f36190  4157                           pushq	%r15
// 0x141f36192  4156                           pushq	%r14
// 0x141f36194  4155                           pushq	%r13
// 0x141f36196  4154                           pushq	%r12
// 0x141f36198  56                             pushq	%rsi
// 0x141f36199  57                             pushq	%rdi
// 0x141f3619a  55                             pushq	%rbp
// 0x141f3619b  53                             pushq	%rbx
// 0x141f3619c  4883ec38                       subq	$0x38, %rsp
// 0x141f361a0  4889d0                         movq	%rdx, %rax
// 0x141f361a3  48894c2428                     movq	%rcx, 0x28(%rsp)
// 0x141f361a8  488b9424a8000000               movq	0xa8(%rsp), %rdx
// 0x141f361b0  488b6828                       movq	0x28(%rax), %rbp
// Frames: F200 -> F201
// 0x141f361b4  4989ea                         movq	%rbp, %r10
// 0x141f361b7  4929d2                         subq	%rdx, %r10
// Frames: F46 -> F202 -> F201
// 0x141f361ba  4d39ca                         cmpq	%r9, %r10
// 0x141f361bd  0f8368010000                   jae	0x141f3632b <.text+0x1f3532b>
// 0x141f361c3  440fb6bc24b0000000             movzbl	0xb0(%rsp), %r15d
// 0x141f361cc  488bbc24a0000000               movq	0xa0(%rsp), %rdi
// 0x141f361d4  488b5818                       movq	0x18(%rax), %rbx
// 0x141f361d8  488b4808                       movq	0x8(%rax), %rcx
// 0x141f361dc  4c8b5810                       movq	0x10(%rax), %r11
// 0x141f361e0  4c895c2430                     movq	%r11, 0x30(%rsp)
// 0x141f361e5  4c8b6838                       movq	0x38(%rax), %r13
// 0x141f361e9  eb1b                           jmp	0x141f36206 <.text+0x1f35206>
// 0x141f361eb  0f1f440000                     nopl	(%rax,%rax)
// 0x141f361f0  48897038                       movq	%rsi, 0x38(%rax)
// 0x141f361f4  4989f5                         movq	%rsi, %r13
// Frames: F200 -> F201
// 0x141f361f7  4c89d5                         movq	%r10, %rbp
// 0x141f361fa  4929d2                         subq	%rdx, %r10
// Frames: F46 -> F202 -> F201
// 0x141f361fd  4d39ca                         cmpq	%r9, %r10
// 0x141f36200  0f8325010000                   jae	0x141f3632b <.text+0x1f3532b>
// Frames: F203
// 0x141f36206  470fb61c10                     movzbl	(%r8,%r10), %r11d
// 0x141f3620b  4c0fa3db                       btq	%r11, %rbx
// 0x141f3620f  0f838b000000                   jae	0x141f362a0 <.text+0x1f352a0>
// Frames: F204
// 0x141f36215  4939cd                         cmpq	%rcx, %r13
// 0x141f36218  4989cb                         movq	%rcx, %r11
// 0x141f3621b  4d0f42dd                       cmovbq	%r13, %r11
// 0x141f3621f  4589fe                         movl	%r15d, %r14d
// 0x141f36222  41f6c701                       testb	$0x1, %r15b
// 0x141f36226  4c0f45d9                       cmovneq	%rcx, %r11
// Frames: F205
// 0x141f3622a  498d73ff                       leaq	-0x1(%r11), %rsi
// 0x141f3622e  4839d6                         cmpq	%rdx, %rsi
// 0x141f36231  0f8389000000                   jae	0x141f362c0 <.text+0x1f352c0>
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f36237  4d89dc                         movq	%r11, %r12
// 0x141f3623a  49f7dc                         negq	%r12
// 0x141f3623d  488d77ff                       leaq	-0x1(%rdi), %rsi
// 0x141f36241  4e8d3c1e                       leaq	(%rsi,%r11), %r15
// 0x141f36245  4b8d3413                       leaq	(%r11,%r10), %rsi
// 0x141f36249  48ffce                         decq	%rsi
// 0x141f3624c  0f1f4000                       nopl	(%rax)
// Frames: F209 -> F206 -> F207 -> F208 -> F205
// 0x141f36250  4d85e4                         testq	%r12, %r12
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f36253  7474                           je	0x141f362c9 <.text+0x1f352c9>
// Frames: F210
// 0x141f36255  4c39ce                         cmpq	%r9, %rsi
// 0x141f36258  0f8340010000                   jae	0x141f3639e <.text+0x1f3539e>
// 0x141f3625e  450fb61f                       movzbl	(%r15), %r11d
// 0x141f36262  49ffc4                         incq	%r12
// 0x141f36265  49ffcf                         decq	%r15
// 0x141f36268  453a1c30                       cmpb	(%r8,%rsi), %r11b
// 0x141f3626c  488d76ff                       leaq	-0x1(%rsi), %rsi
// 0x141f36270  74de                           je	0x141f36250 <.text+0x1f35250>
// Frames: F211
// 0x141f36272  4829cd                         subq	%rcx, %rbp
// 0x141f36275  4c29e5                         subq	%r12, %rbp
// 0x141f36278  48896828                       movq	%rbp, 0x28(%rax)
// 0x141f3627c  4889d6                         movq	%rdx, %rsi
// 0x141f3627f  4989ea                         movq	%rbp, %r10
// 0x141f36282  4589f7                         movl	%r14d, %r15d
// 0x141f36285  4584f6                         testb	%r14b, %r14b
// 0x141f36288  0f8462ffffff                   je	0x141f361f0 <.text+0x1f351f0>
// 0x141f3628e  e964ffffff                     jmp	0x141f361f7 <.text+0x1f351f7>
// 0x141f36293  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F212
// 0x141f362a0  4c895028                       movq	%r10, 0x28(%rax)
// 0x141f362a4  4889d6                         movq	%rdx, %rsi
// Frames: F213
// 0x141f362a7  4584ff                         testb	%r15b, %r15b
// 0x141f362aa  0f8440ffffff                   je	0x141f361f0 <.text+0x1f351f0>
// 0x141f362b0  e942ffffff                     jmp	0x141f361f7 <.text+0x1f351f7>
// 0x141f362b5  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F209 -> F206 -> F207 -> F208 -> F205
// 0x141f362c0  4d85db                         testq	%r11, %r11
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f362c3  0f85e7000000                   jne	0x141f363b0 <.text+0x1f353b0>
// Frames: F214
// 0x141f362c9  4584f6                         testb	%r14b, %r14b
// 0x141f362cc  4c89ee                         movq	%r13, %rsi
// 0x141f362cf  480f45f2                       cmovneq	%rdx, %rsi
// Frames: F215
// 0x141f362d3  4f8d3c10                       leaq	(%r8,%r10), %r15
// 0x141f362d7  4989cc                         movq	%rcx, %r12
// 0x141f362da  660f1f440000                   nopw	(%rax,%rax)
// Frames: F209 -> F216 -> F217 -> F215
// 0x141f362e0  4939f4                         cmpq	%rsi, %r12
// Frames: F216 -> F217 -> F215
// 0x141f362e3  7357                           jae	0x141f3633c <.text+0x1f3533c>
// Frames: F218
// 0x141f362e5  4939d4                         cmpq	%rdx, %r12
// 0x141f362e8  0f8381000000                   jae	0x141f3636f <.text+0x1f3536f>
// 0x141f362ee  4f8d1c22                       leaq	(%r10,%r12), %r11
// 0x141f362f2  4d39cb                         cmpq	%r9, %r11
// 0x141f362f5  0f8387000000                   jae	0x141f36382 <.text+0x1f35382>
// 0x141f362fb  460fb61c27                     movzbl	(%rdi,%r12), %r11d
// 0x141f36300  473a1c27                       cmpb	(%r15,%r12), %r11b
// Frames: F219 -> F220 -> F221 -> F217 -> F215
// 0x141f36304  4d8d642401                     leaq	0x1(%r12), %r12
// Frames: F218
// 0x141f36309  74d5                           je	0x141f362e0 <.text+0x1f352e0>
// Frames: F222
// 0x141f3630b  488b742430                     movq	0x30(%rsp), %rsi
// Frames: F248
// 0x141f36310  4829f5                         subq	%rsi, %rbp
// 0x141f36313  48896828                       movq	%rbp, 0x28(%rax)
// 0x141f36317  4989ea                         movq	%rbp, %r10
// 0x141f3631a  4589f7                         movl	%r14d, %r15d
// 0x141f3631d  4584f6                         testb	%r14b, %r14b
// 0x141f36320  0f84cafeffff                   je	0x141f361f0 <.text+0x1f351f0>
// 0x141f36326  e9ccfeffff                     jmp	0x141f361f7 <.text+0x1f351f7>
// 0x141f3632b  48c7402800000000               movq	$0x0, 0x28(%rax)
// 0x141f36333  31c0                           xorl	%eax, %eax
// 0x141f36335  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x141f3633a  eb1f                           jmp	0x141f3635b <.text+0x1f3535b>
// 0x141f3633c  4c895028                       movq	%r10, 0x28(%rax)
// Frames: F249
// 0x141f36340  4584f6                         testb	%r14b, %r14b
// 0x141f36343  7504                           jne	0x141f36349 <.text+0x1f35349>
// Frames: F250
// 0x141f36345  48895038                       movq	%rdx, 0x38(%rax)
// Frames: F224 -> F225
// 0x141f36349  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x141f3634e  4c895108                       movq	%r10, 0x8(%rcx)
// 0x141f36352  48896910                       movq	%rbp, 0x10(%rcx)
// 0x141f36356  b801000000                     movl	$0x1, %eax
// Frames: F226
// 0x141f3635b  488901                         movq	%rax, (%rcx)
// 0x141f3635e  4883c438                       addq	$0x38, %rsp
// 0x141f36362  5b                             popq	%rbx
// 0x141f36363  5d                             popq	%rbp
// 0x141f36364  5f                             popq	%rdi
// 0x141f36365  5e                             popq	%rsi
// 0x141f36366  415c                           popq	%r12
// 0x141f36368  415d                           popq	%r13
// 0x141f3636a  415e                           popq	%r14
// 0x141f3636c  415f                           popq	%r15
// 0x141f3636e  c3                             retq
// Frames: F46 -> F202 -> F201
// 0x141f3636f  4839d1                         cmpq	%rdx, %rcx
// 0x141f36372  480f46ca                       cmovbeq	%rdx, %rcx
// Frames: F218
// 0x141f36376  4c8d056bf84d01                 leaq	0x14df86b(%rip), %r8    # 0x143415be8
// 0x141f3637d  e840b3e700                     callq	0x142db16c2 <.text+0x2db06c2>
// Frames: F215
// 0x141f36382  4901ca                         addq	%rcx, %r10
// 0x141f36385  4d39d1                         cmpq	%r10, %r9
// 0x141f36388  4d0f47d1                       cmovaq	%r9, %r10
// Frames: F218
// 0x141f3638c  4c8d056df84d01                 leaq	0x14df86d(%rip), %r8    # 0x143415c00
// 0x141f36393  4c89d1                         movq	%r10, %rcx
// 0x141f36396  4c89ca                         movq	%r9, %rdx
// 0x141f36399  e824b3e700                     callq	0x142db16c2 <.text+0x2db06c2>
// Frames: F210
// 0x141f3639e  4c8d058bf84d01                 leaq	0x14df88b(%rip), %r8    # 0x143415c30
// 0x141f363a5  4889f1                         movq	%rsi, %rcx
// 0x141f363a8  4c89ca                         movq	%r9, %rdx
// 0x141f363ab  e812b3e700                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f363b0  4c8d0561f84d01                 leaq	0x14df861(%rip), %r8    # 0x143415c18
// 0x141f363b7  4889f1                         movq	%rsi, %rcx
// 0x141f363ba  e803b3e700                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f363bf  cc                             int3
//
// RANGE rva_1f37fc0_len_230: RVA 0x1f37fc0, 560 PDB bytes; 146 instructions
// Evidence: _assembly/roof-v1/C/pack-001/rva_1f37fc0_len_230.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2437:13356: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Frames: F199
// 0x141f37fc0  4157                           pushq	%r15
// 0x141f37fc2  4156                           pushq	%r14
// 0x141f37fc4  4155                           pushq	%r13
// 0x141f37fc6  4154                           pushq	%r12
// 0x141f37fc8  56                             pushq	%rsi
// 0x141f37fc9  57                             pushq	%rdi
// 0x141f37fca  55                             pushq	%rbp
// 0x141f37fcb  53                             pushq	%rbx
// 0x141f37fcc  4883ec38                       subq	$0x38, %rsp
// 0x141f37fd0  4889d0                         movq	%rdx, %rax
// 0x141f37fd3  48894c2428                     movq	%rcx, 0x28(%rsp)
// 0x141f37fd8  488b9424a8000000               movq	0xa8(%rsp), %rdx
// 0x141f37fe0  488b6828                       movq	0x28(%rax), %rbp
// Frames: F200 -> F201
// 0x141f37fe4  4989ea                         movq	%rbp, %r10
// 0x141f37fe7  4929d2                         subq	%rdx, %r10
// Frames: F46 -> F202 -> F201
// 0x141f37fea  4d39ca                         cmpq	%r9, %r10
// 0x141f37fed  0f8368010000                   jae	0x141f3815b <.text+0x1f3715b>
// 0x141f37ff3  440fb6bc24b0000000             movzbl	0xb0(%rsp), %r15d
// 0x141f37ffc  488bbc24a0000000               movq	0xa0(%rsp), %rdi
// 0x141f38004  488b5818                       movq	0x18(%rax), %rbx
// 0x141f38008  488b4808                       movq	0x8(%rax), %rcx
// 0x141f3800c  4c8b5810                       movq	0x10(%rax), %r11
// 0x141f38010  4c895c2430                     movq	%r11, 0x30(%rsp)
// 0x141f38015  4c8b6838                       movq	0x38(%rax), %r13
// 0x141f38019  eb1b                           jmp	0x141f38036 <.text+0x1f37036>
// 0x141f3801b  0f1f440000                     nopl	(%rax,%rax)
// 0x141f38020  48897038                       movq	%rsi, 0x38(%rax)
// 0x141f38024  4989f5                         movq	%rsi, %r13
// Frames: F200 -> F201
// 0x141f38027  4c89d5                         movq	%r10, %rbp
// 0x141f3802a  4929d2                         subq	%rdx, %r10
// Frames: F46 -> F202 -> F201
// 0x141f3802d  4d39ca                         cmpq	%r9, %r10
// 0x141f38030  0f8325010000                   jae	0x141f3815b <.text+0x1f3715b>
// Frames: F203
// 0x141f38036  470fb61c10                     movzbl	(%r8,%r10), %r11d
// 0x141f3803b  4c0fa3db                       btq	%r11, %rbx
// 0x141f3803f  0f838b000000                   jae	0x141f380d0 <.text+0x1f370d0>
// Frames: F204
// 0x141f38045  4939cd                         cmpq	%rcx, %r13
// 0x141f38048  4989cb                         movq	%rcx, %r11
// 0x141f3804b  4d0f42dd                       cmovbq	%r13, %r11
// 0x141f3804f  4589fe                         movl	%r15d, %r14d
// 0x141f38052  41f6c701                       testb	$0x1, %r15b
// 0x141f38056  4c0f45d9                       cmovneq	%rcx, %r11
// Frames: F205
// 0x141f3805a  498d73ff                       leaq	-0x1(%r11), %rsi
// 0x141f3805e  4839d6                         cmpq	%rdx, %rsi
// 0x141f38061  0f8389000000                   jae	0x141f380f0 <.text+0x1f370f0>
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f38067  4d89dc                         movq	%r11, %r12
// 0x141f3806a  49f7dc                         negq	%r12
// 0x141f3806d  488d77ff                       leaq	-0x1(%rdi), %rsi
// 0x141f38071  4e8d3c1e                       leaq	(%rsi,%r11), %r15
// 0x141f38075  4b8d3413                       leaq	(%r11,%r10), %rsi
// 0x141f38079  48ffce                         decq	%rsi
// 0x141f3807c  0f1f4000                       nopl	(%rax)
// Frames: F209 -> F206 -> F207 -> F208 -> F205
// 0x141f38080  4d85e4                         testq	%r12, %r12
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f38083  7474                           je	0x141f380f9 <.text+0x1f370f9>
// Frames: F210
// 0x141f38085  4c39ce                         cmpq	%r9, %rsi
// 0x141f38088  0f8340010000                   jae	0x141f381ce <.text+0x1f371ce>
// 0x141f3808e  450fb61f                       movzbl	(%r15), %r11d
// 0x141f38092  49ffc4                         incq	%r12
// 0x141f38095  49ffcf                         decq	%r15
// 0x141f38098  453a1c30                       cmpb	(%r8,%rsi), %r11b
// 0x141f3809c  488d76ff                       leaq	-0x1(%rsi), %rsi
// 0x141f380a0  74de                           je	0x141f38080 <.text+0x1f37080>
// Frames: F211
// 0x141f380a2  4829cd                         subq	%rcx, %rbp
// 0x141f380a5  4c29e5                         subq	%r12, %rbp
// 0x141f380a8  48896828                       movq	%rbp, 0x28(%rax)
// 0x141f380ac  4889d6                         movq	%rdx, %rsi
// 0x141f380af  4989ea                         movq	%rbp, %r10
// 0x141f380b2  4589f7                         movl	%r14d, %r15d
// 0x141f380b5  4584f6                         testb	%r14b, %r14b
// 0x141f380b8  0f8462ffffff                   je	0x141f38020 <.text+0x1f37020>
// 0x141f380be  e964ffffff                     jmp	0x141f38027 <.text+0x1f37027>
// 0x141f380c3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F212
// 0x141f380d0  4c895028                       movq	%r10, 0x28(%rax)
// 0x141f380d4  4889d6                         movq	%rdx, %rsi
// Frames: F213
// 0x141f380d7  4584ff                         testb	%r15b, %r15b
// 0x141f380da  0f8440ffffff                   je	0x141f38020 <.text+0x1f37020>
// 0x141f380e0  e942ffffff                     jmp	0x141f38027 <.text+0x1f37027>
// 0x141f380e5  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F209 -> F206 -> F207 -> F208 -> F205
// 0x141f380f0  4d85db                         testq	%r11, %r11
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f380f3  0f85e7000000                   jne	0x141f381e0 <.text+0x1f371e0>
// Frames: F214
// 0x141f380f9  4584f6                         testb	%r14b, %r14b
// 0x141f380fc  4c89ee                         movq	%r13, %rsi
// 0x141f380ff  480f45f2                       cmovneq	%rdx, %rsi
// Frames: F215
// 0x141f38103  4f8d3c10                       leaq	(%r8,%r10), %r15
// 0x141f38107  4989cc                         movq	%rcx, %r12
// 0x141f3810a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F209 -> F216 -> F217 -> F215
// 0x141f38110  4939f4                         cmpq	%rsi, %r12
// Frames: F216 -> F217 -> F215
// 0x141f38113  7357                           jae	0x141f3816c <.text+0x1f3716c>
// Frames: F218
// 0x141f38115  4939d4                         cmpq	%rdx, %r12
// 0x141f38118  0f8381000000                   jae	0x141f3819f <.text+0x1f3719f>
// 0x141f3811e  4f8d1c22                       leaq	(%r10,%r12), %r11
// 0x141f38122  4d39cb                         cmpq	%r9, %r11
// 0x141f38125  0f8387000000                   jae	0x141f381b2 <.text+0x1f371b2>
// 0x141f3812b  460fb61c27                     movzbl	(%rdi,%r12), %r11d
// 0x141f38130  473a1c27                       cmpb	(%r15,%r12), %r11b
// Frames: F219 -> F220 -> F221 -> F217 -> F215
// 0x141f38134  4d8d642401                     leaq	0x1(%r12), %r12
// Frames: F218
// 0x141f38139  74d5                           je	0x141f38110 <.text+0x1f37110>
// Frames: F222
// 0x141f3813b  488b742430                     movq	0x30(%rsp), %rsi
// 0x141f38140  4829f5                         subq	%rsi, %rbp
// 0x141f38143  48896828                       movq	%rbp, 0x28(%rax)
// 0x141f38147  4989ea                         movq	%rbp, %r10
// 0x141f3814a  4589f7                         movl	%r14d, %r15d
// 0x141f3814d  4584f6                         testb	%r14b, %r14b
// 0x141f38150  0f84cafeffff                   je	0x141f38020 <.text+0x1f37020>
// 0x141f38156  e9ccfeffff                     jmp	0x141f38027 <.text+0x1f37027>
// Frames: F251
// 0x141f3815b  48c7402800000000               movq	$0x0, 0x28(%rax)
// 0x141f38163  31c0                           xorl	%eax, %eax
// 0x141f38165  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x141f3816a  eb1f                           jmp	0x141f3818b <.text+0x1f3718b>
// 0x141f3816c  4c895028                       movq	%r10, 0x28(%rax)
// 0x141f38170  4584f6                         testb	%r14b, %r14b
// 0x141f38173  7504                           jne	0x141f38179 <.text+0x1f37179>
// 0x141f38175  48895038                       movq	%rdx, 0x38(%rax)
// Frames: F224 -> F251
// 0x141f38179  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x141f3817e  4c895108                       movq	%r10, 0x8(%rcx)
// 0x141f38182  48896910                       movq	%rbp, 0x10(%rcx)
// 0x141f38186  b801000000                     movl	$0x1, %eax
// Frames: F251
// 0x141f3818b  488901                         movq	%rax, (%rcx)
// Frames: F226
// 0x141f3818e  4883c438                       addq	$0x38, %rsp
// 0x141f38192  5b                             popq	%rbx
// 0x141f38193  5d                             popq	%rbp
// 0x141f38194  5f                             popq	%rdi
// 0x141f38195  5e                             popq	%rsi
// 0x141f38196  415c                           popq	%r12
// 0x141f38198  415d                           popq	%r13
// 0x141f3819a  415e                           popq	%r14
// 0x141f3819c  415f                           popq	%r15
// 0x141f3819e  c3                             retq
// Frames: F46 -> F202 -> F201
// 0x141f3819f  4839d1                         cmpq	%rdx, %rcx
// 0x141f381a2  480f46ca                       cmovbeq	%rdx, %rcx
// Frames: F218
// 0x141f381a6  4c8d05ebde4d01                 leaq	0x14ddeeb(%rip), %r8    # 0x143416098
// 0x141f381ad  e81095e700                     callq	0x142db16c2 <.text+0x2db06c2>
// Frames: F215
// 0x141f381b2  4901ca                         addq	%rcx, %r10
// 0x141f381b5  4d39d1                         cmpq	%r10, %r9
// 0x141f381b8  4d0f47d1                       cmovaq	%r9, %r10
// Frames: F218
// 0x141f381bc  4c8d05edde4d01                 leaq	0x14ddeed(%rip), %r8    # 0x1434160b0
// 0x141f381c3  4c89d1                         movq	%r10, %rcx
// 0x141f381c6  4c89ca                         movq	%r9, %rdx
// 0x141f381c9  e8f494e700                     callq	0x142db16c2 <.text+0x2db06c2>
// Frames: F210
// 0x141f381ce  4c8d050bdf4d01                 leaq	0x14ddf0b(%rip), %r8    # 0x1434160e0
// 0x141f381d5  4889f1                         movq	%rsi, %rcx
// 0x141f381d8  4c89ca                         movq	%r9, %rdx
// 0x141f381db  e8e294e700                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f381e0  4c8d05e1de4d01                 leaq	0x14ddee1(%rip), %r8    # 0x1434160c8
// 0x141f381e7  4889f1                         movq	%rsi, %rcx
// 0x141f381ea  e8d394e700                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f381ef  cc                             int3
//
// RANGE rva_1f3aba0_len_230: RVA 0x1f3aba0, 560 PDB bytes; 146 instructions
// Evidence: _assembly/roof-v1/C/pack-001/rva_1f3aba0_len_230.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2438:25540: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Frames: F199
// 0x141f3aba0  4157                           pushq	%r15
// 0x141f3aba2  4156                           pushq	%r14
// 0x141f3aba4  4155                           pushq	%r13
// 0x141f3aba6  4154                           pushq	%r12
// 0x141f3aba8  56                             pushq	%rsi
// 0x141f3aba9  57                             pushq	%rdi
// 0x141f3abaa  55                             pushq	%rbp
// 0x141f3abab  53                             pushq	%rbx
// 0x141f3abac  4883ec38                       subq	$0x38, %rsp
// 0x141f3abb0  4889d0                         movq	%rdx, %rax
// 0x141f3abb3  48894c2428                     movq	%rcx, 0x28(%rsp)
// 0x141f3abb8  488b9424a8000000               movq	0xa8(%rsp), %rdx
// 0x141f3abc0  488b6828                       movq	0x28(%rax), %rbp
// Frames: F200 -> F201
// 0x141f3abc4  4989ea                         movq	%rbp, %r10
// 0x141f3abc7  4929d2                         subq	%rdx, %r10
// Frames: F46 -> F202 -> F201
// 0x141f3abca  4d39ca                         cmpq	%r9, %r10
// 0x141f3abcd  0f8368010000                   jae	0x141f3ad3b <.text+0x1f39d3b>
// 0x141f3abd3  440fb6bc24b0000000             movzbl	0xb0(%rsp), %r15d
// 0x141f3abdc  488bbc24a0000000               movq	0xa0(%rsp), %rdi
// 0x141f3abe4  488b5818                       movq	0x18(%rax), %rbx
// 0x141f3abe8  488b4808                       movq	0x8(%rax), %rcx
// 0x141f3abec  4c8b5810                       movq	0x10(%rax), %r11
// 0x141f3abf0  4c895c2430                     movq	%r11, 0x30(%rsp)
// 0x141f3abf5  4c8b6838                       movq	0x38(%rax), %r13
// 0x141f3abf9  eb1b                           jmp	0x141f3ac16 <.text+0x1f39c16>
// 0x141f3abfb  0f1f440000                     nopl	(%rax,%rax)
// 0x141f3ac00  48897038                       movq	%rsi, 0x38(%rax)
// 0x141f3ac04  4989f5                         movq	%rsi, %r13
// Frames: F200 -> F201
// 0x141f3ac07  4c89d5                         movq	%r10, %rbp
// 0x141f3ac0a  4929d2                         subq	%rdx, %r10
// Frames: F46 -> F202 -> F201
// 0x141f3ac0d  4d39ca                         cmpq	%r9, %r10
// 0x141f3ac10  0f8325010000                   jae	0x141f3ad3b <.text+0x1f39d3b>
// Frames: F203
// 0x141f3ac16  470fb61c10                     movzbl	(%r8,%r10), %r11d
// 0x141f3ac1b  4c0fa3db                       btq	%r11, %rbx
// 0x141f3ac1f  0f838b000000                   jae	0x141f3acb0 <.text+0x1f39cb0>
// Frames: F204
// 0x141f3ac25  4939cd                         cmpq	%rcx, %r13
// 0x141f3ac28  4989cb                         movq	%rcx, %r11
// 0x141f3ac2b  4d0f42dd                       cmovbq	%r13, %r11
// 0x141f3ac2f  4589fe                         movl	%r15d, %r14d
// 0x141f3ac32  41f6c701                       testb	$0x1, %r15b
// 0x141f3ac36  4c0f45d9                       cmovneq	%rcx, %r11
// Frames: F205
// 0x141f3ac3a  498d73ff                       leaq	-0x1(%r11), %rsi
// 0x141f3ac3e  4839d6                         cmpq	%rdx, %rsi
// 0x141f3ac41  0f8389000000                   jae	0x141f3acd0 <.text+0x1f39cd0>
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f3ac47  4d89dc                         movq	%r11, %r12
// 0x141f3ac4a  49f7dc                         negq	%r12
// 0x141f3ac4d  488d77ff                       leaq	-0x1(%rdi), %rsi
// 0x141f3ac51  4e8d3c1e                       leaq	(%rsi,%r11), %r15
// 0x141f3ac55  4b8d3413                       leaq	(%r11,%r10), %rsi
// 0x141f3ac59  48ffce                         decq	%rsi
// 0x141f3ac5c  0f1f4000                       nopl	(%rax)
// Frames: F209 -> F206 -> F207 -> F208 -> F205
// 0x141f3ac60  4d85e4                         testq	%r12, %r12
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f3ac63  7474                           je	0x141f3acd9 <.text+0x1f39cd9>
// Frames: F210
// 0x141f3ac65  4c39ce                         cmpq	%r9, %rsi
// 0x141f3ac68  0f8340010000                   jae	0x141f3adae <.text+0x1f39dae>
// 0x141f3ac6e  450fb61f                       movzbl	(%r15), %r11d
// 0x141f3ac72  49ffc4                         incq	%r12
// 0x141f3ac75  49ffcf                         decq	%r15
// 0x141f3ac78  453a1c30                       cmpb	(%r8,%rsi), %r11b
// 0x141f3ac7c  488d76ff                       leaq	-0x1(%rsi), %rsi
// 0x141f3ac80  74de                           je	0x141f3ac60 <.text+0x1f39c60>
// Frames: F211
// 0x141f3ac82  4829cd                         subq	%rcx, %rbp
// 0x141f3ac85  4c29e5                         subq	%r12, %rbp
// 0x141f3ac88  48896828                       movq	%rbp, 0x28(%rax)
// 0x141f3ac8c  4889d6                         movq	%rdx, %rsi
// 0x141f3ac8f  4989ea                         movq	%rbp, %r10
// 0x141f3ac92  4589f7                         movl	%r14d, %r15d
// 0x141f3ac95  4584f6                         testb	%r14b, %r14b
// 0x141f3ac98  0f8462ffffff                   je	0x141f3ac00 <.text+0x1f39c00>
// 0x141f3ac9e  e964ffffff                     jmp	0x141f3ac07 <.text+0x1f39c07>
// 0x141f3aca3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F212
// 0x141f3acb0  4c895028                       movq	%r10, 0x28(%rax)
// 0x141f3acb4  4889d6                         movq	%rdx, %rsi
// Frames: F213
// 0x141f3acb7  4584ff                         testb	%r15b, %r15b
// 0x141f3acba  0f8440ffffff                   je	0x141f3ac00 <.text+0x1f39c00>
// 0x141f3acc0  e942ffffff                     jmp	0x141f3ac07 <.text+0x1f39c07>
// 0x141f3acc5  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F209 -> F206 -> F207 -> F208 -> F205
// 0x141f3acd0  4d85db                         testq	%r11, %r11
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f3acd3  0f85e7000000                   jne	0x141f3adc0 <.text+0x1f39dc0>
// Frames: F214
// 0x141f3acd9  4584f6                         testb	%r14b, %r14b
// 0x141f3acdc  4c89ee                         movq	%r13, %rsi
// 0x141f3acdf  480f45f2                       cmovneq	%rdx, %rsi
// Frames: F215
// 0x141f3ace3  4f8d3c10                       leaq	(%r8,%r10), %r15
// 0x141f3ace7  4989cc                         movq	%rcx, %r12
// 0x141f3acea  660f1f440000                   nopw	(%rax,%rax)
// Frames: F209 -> F216 -> F217 -> F215
// 0x141f3acf0  4939f4                         cmpq	%rsi, %r12
// Frames: F216 -> F217 -> F215
// 0x141f3acf3  7357                           jae	0x141f3ad4c <.text+0x1f39d4c>
// Frames: F218
// 0x141f3acf5  4939d4                         cmpq	%rdx, %r12
// 0x141f3acf8  0f8381000000                   jae	0x141f3ad7f <.text+0x1f39d7f>
// 0x141f3acfe  4f8d1c22                       leaq	(%r10,%r12), %r11
// 0x141f3ad02  4d39cb                         cmpq	%r9, %r11
// 0x141f3ad05  0f8387000000                   jae	0x141f3ad92 <.text+0x1f39d92>
// 0x141f3ad0b  460fb61c27                     movzbl	(%rdi,%r12), %r11d
// 0x141f3ad10  473a1c27                       cmpb	(%r15,%r12), %r11b
// Frames: F219 -> F220 -> F221 -> F217 -> F215
// 0x141f3ad14  4d8d642401                     leaq	0x1(%r12), %r12
// Frames: F218
// 0x141f3ad19  74d5                           je	0x141f3acf0 <.text+0x1f39cf0>
// Frames: F222
// 0x141f3ad1b  488b742430                     movq	0x30(%rsp), %rsi
// 0x141f3ad20  4829f5                         subq	%rsi, %rbp
// 0x141f3ad23  48896828                       movq	%rbp, 0x28(%rax)
// 0x141f3ad27  4989ea                         movq	%rbp, %r10
// 0x141f3ad2a  4589f7                         movl	%r14d, %r15d
// 0x141f3ad2d  4584f6                         testb	%r14b, %r14b
// 0x141f3ad30  0f84cafeffff                   je	0x141f3ac00 <.text+0x1f39c00>
// 0x141f3ad36  e9ccfeffff                     jmp	0x141f3ac07 <.text+0x1f39c07>
// Frames: F251
// 0x141f3ad3b  48c7402800000000               movq	$0x0, 0x28(%rax)
// 0x141f3ad43  31c0                           xorl	%eax, %eax
// 0x141f3ad45  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x141f3ad4a  eb1f                           jmp	0x141f3ad6b <.text+0x1f39d6b>
// 0x141f3ad4c  4c895028                       movq	%r10, 0x28(%rax)
// 0x141f3ad50  4584f6                         testb	%r14b, %r14b
// 0x141f3ad53  7504                           jne	0x141f3ad59 <.text+0x1f39d59>
// 0x141f3ad55  48895038                       movq	%rdx, 0x38(%rax)
// Frames: F224 -> F251
// 0x141f3ad59  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x141f3ad5e  4c895108                       movq	%r10, 0x8(%rcx)
// 0x141f3ad62  48896910                       movq	%rbp, 0x10(%rcx)
// 0x141f3ad66  b801000000                     movl	$0x1, %eax
// Frames: F251
// 0x141f3ad6b  488901                         movq	%rax, (%rcx)
// 0x141f3ad6e  4883c438                       addq	$0x38, %rsp
// 0x141f3ad72  5b                             popq	%rbx
// 0x141f3ad73  5d                             popq	%rbp
// 0x141f3ad74  5f                             popq	%rdi
// 0x141f3ad75  5e                             popq	%rsi
// 0x141f3ad76  415c                           popq	%r12
// 0x141f3ad78  415d                           popq	%r13
// 0x141f3ad7a  415e                           popq	%r14
// 0x141f3ad7c  415f                           popq	%r15
// 0x141f3ad7e  c3                             retq
// Frames: F46 -> F202 -> F251
// 0x141f3ad7f  4839d1                         cmpq	%rdx, %rcx
// 0x141f3ad82  480f46ca                       cmovbeq	%rdx, %rcx
// Frames: F251
// 0x141f3ad86  4c8d0543b84d01                 leaq	0x14db843(%rip), %r8    # 0x1434165d0
// 0x141f3ad8d  e83069e700                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f3ad92  4901ca                         addq	%rcx, %r10
// 0x141f3ad95  4d39d1                         cmpq	%r10, %r9
// 0x141f3ad98  4d0f47d1                       cmovaq	%r9, %r10
// 0x141f3ad9c  4c8d0545b84d01                 leaq	0x14db845(%rip), %r8    # 0x1434165e8
// 0x141f3ada3  4c89d1                         movq	%r10, %rcx
// 0x141f3ada6  4c89ca                         movq	%r9, %rdx
// 0x141f3ada9  e81469e700                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f3adae  4c8d0563b84d01                 leaq	0x14db863(%rip), %r8    # 0x143416618
// 0x141f3adb5  4889f1                         movq	%rsi, %rcx
// 0x141f3adb8  4c89ca                         movq	%r9, %rdx
// 0x141f3adbb  e80269e700                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f3adc0  4c8d0539b84d01                 leaq	0x14db839(%rip), %r8    # 0x143416600
// 0x141f3adc7  4889f1                         movq	%rsi, %rcx
// 0x141f3adca  e8f368e700                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f3adcf  cc                             int3
//
// RANGE rva_1f3e5b0_len_230: RVA 0x1f3e5b0, 560 PDB bytes; 146 instructions
// Evidence: _assembly/roof-v1/C/pack-001/rva_1f3e5b0_len_230.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2444:10320: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Frames: F199
// 0x141f3e5b0  4157                           pushq	%r15
// 0x141f3e5b2  4156                           pushq	%r14
// 0x141f3e5b4  4155                           pushq	%r13
// 0x141f3e5b6  4154                           pushq	%r12
// 0x141f3e5b8  56                             pushq	%rsi
// 0x141f3e5b9  57                             pushq	%rdi
// 0x141f3e5ba  55                             pushq	%rbp
// 0x141f3e5bb  53                             pushq	%rbx
// 0x141f3e5bc  4883ec38                       subq	$0x38, %rsp
// 0x141f3e5c0  4889d0                         movq	%rdx, %rax
// 0x141f3e5c3  48894c2428                     movq	%rcx, 0x28(%rsp)
// 0x141f3e5c8  488b9424a8000000               movq	0xa8(%rsp), %rdx
// 0x141f3e5d0  488b6828                       movq	0x28(%rax), %rbp
// Frames: F200 -> F201
// 0x141f3e5d4  4989ea                         movq	%rbp, %r10
// 0x141f3e5d7  4929d2                         subq	%rdx, %r10
// Frames: F46 -> F202 -> F201
// 0x141f3e5da  4d39ca                         cmpq	%r9, %r10
// 0x141f3e5dd  0f8368010000                   jae	0x141f3e74b <.text+0x1f3d74b>
// 0x141f3e5e3  440fb6bc24b0000000             movzbl	0xb0(%rsp), %r15d
// 0x141f3e5ec  488bbc24a0000000               movq	0xa0(%rsp), %rdi
// 0x141f3e5f4  488b5818                       movq	0x18(%rax), %rbx
// 0x141f3e5f8  488b4808                       movq	0x8(%rax), %rcx
// 0x141f3e5fc  4c8b5810                       movq	0x10(%rax), %r11
// 0x141f3e600  4c895c2430                     movq	%r11, 0x30(%rsp)
// 0x141f3e605  4c8b6838                       movq	0x38(%rax), %r13
// 0x141f3e609  eb1b                           jmp	0x141f3e626 <.text+0x1f3d626>
// 0x141f3e60b  0f1f440000                     nopl	(%rax,%rax)
// 0x141f3e610  48897038                       movq	%rsi, 0x38(%rax)
// 0x141f3e614  4989f5                         movq	%rsi, %r13
// Frames: F200 -> F201
// 0x141f3e617  4c89d5                         movq	%r10, %rbp
// 0x141f3e61a  4929d2                         subq	%rdx, %r10
// Frames: F46 -> F202 -> F201
// 0x141f3e61d  4d39ca                         cmpq	%r9, %r10
// 0x141f3e620  0f8325010000                   jae	0x141f3e74b <.text+0x1f3d74b>
// Frames: F203
// 0x141f3e626  470fb61c10                     movzbl	(%r8,%r10), %r11d
// 0x141f3e62b  4c0fa3db                       btq	%r11, %rbx
// 0x141f3e62f  0f838b000000                   jae	0x141f3e6c0 <.text+0x1f3d6c0>
// Frames: F204
// 0x141f3e635  4939cd                         cmpq	%rcx, %r13
// 0x141f3e638  4989cb                         movq	%rcx, %r11
// 0x141f3e63b  4d0f42dd                       cmovbq	%r13, %r11
// 0x141f3e63f  4589fe                         movl	%r15d, %r14d
// 0x141f3e642  41f6c701                       testb	$0x1, %r15b
// 0x141f3e646  4c0f45d9                       cmovneq	%rcx, %r11
// Frames: F205
// 0x141f3e64a  498d73ff                       leaq	-0x1(%r11), %rsi
// 0x141f3e64e  4839d6                         cmpq	%rdx, %rsi
// 0x141f3e651  0f8389000000                   jae	0x141f3e6e0 <.text+0x1f3d6e0>
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f3e657  4d89dc                         movq	%r11, %r12
// 0x141f3e65a  49f7dc                         negq	%r12
// 0x141f3e65d  488d77ff                       leaq	-0x1(%rdi), %rsi
// 0x141f3e661  4e8d3c1e                       leaq	(%rsi,%r11), %r15
// 0x141f3e665  4b8d3413                       leaq	(%r11,%r10), %rsi
// 0x141f3e669  48ffce                         decq	%rsi
// 0x141f3e66c  0f1f4000                       nopl	(%rax)
// Frames: F209 -> F206 -> F207 -> F208 -> F205
// 0x141f3e670  4d85e4                         testq	%r12, %r12
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f3e673  7474                           je	0x141f3e6e9 <.text+0x1f3d6e9>
// Frames: F210
// 0x141f3e675  4c39ce                         cmpq	%r9, %rsi
// 0x141f3e678  0f8340010000                   jae	0x141f3e7be <.text+0x1f3d7be>
// 0x141f3e67e  450fb61f                       movzbl	(%r15), %r11d
// 0x141f3e682  49ffc4                         incq	%r12
// 0x141f3e685  49ffcf                         decq	%r15
// 0x141f3e688  453a1c30                       cmpb	(%r8,%rsi), %r11b
// 0x141f3e68c  488d76ff                       leaq	-0x1(%rsi), %rsi
// 0x141f3e690  74de                           je	0x141f3e670 <.text+0x1f3d670>
// Frames: F211
// 0x141f3e692  4829cd                         subq	%rcx, %rbp
// 0x141f3e695  4c29e5                         subq	%r12, %rbp
// 0x141f3e698  48896828                       movq	%rbp, 0x28(%rax)
// 0x141f3e69c  4889d6                         movq	%rdx, %rsi
// 0x141f3e69f  4989ea                         movq	%rbp, %r10
// 0x141f3e6a2  4589f7                         movl	%r14d, %r15d
// 0x141f3e6a5  4584f6                         testb	%r14b, %r14b
// 0x141f3e6a8  0f8462ffffff                   je	0x141f3e610 <.text+0x1f3d610>
// 0x141f3e6ae  e964ffffff                     jmp	0x141f3e617 <.text+0x1f3d617>
// 0x141f3e6b3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F212
// 0x141f3e6c0  4c895028                       movq	%r10, 0x28(%rax)
// 0x141f3e6c4  4889d6                         movq	%rdx, %rsi
// Frames: F213
// 0x141f3e6c7  4584ff                         testb	%r15b, %r15b
// 0x141f3e6ca  0f8440ffffff                   je	0x141f3e610 <.text+0x1f3d610>
// 0x141f3e6d0  e942ffffff                     jmp	0x141f3e617 <.text+0x1f3d617>
// 0x141f3e6d5  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F209 -> F206 -> F207 -> F208 -> F205
// 0x141f3e6e0  4d85db                         testq	%r11, %r11
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f3e6e3  0f85e7000000                   jne	0x141f3e7d0 <.text+0x1f3d7d0>
// Frames: F214
// 0x141f3e6e9  4584f6                         testb	%r14b, %r14b
// 0x141f3e6ec  4c89ee                         movq	%r13, %rsi
// 0x141f3e6ef  480f45f2                       cmovneq	%rdx, %rsi
// Frames: F215
// 0x141f3e6f3  4f8d3c10                       leaq	(%r8,%r10), %r15
// 0x141f3e6f7  4989cc                         movq	%rcx, %r12
// 0x141f3e6fa  660f1f440000                   nopw	(%rax,%rax)
// Frames: F209 -> F216 -> F217 -> F215
// 0x141f3e700  4939f4                         cmpq	%rsi, %r12
// Frames: F216 -> F217 -> F215
// 0x141f3e703  7357                           jae	0x141f3e75c <.text+0x1f3d75c>
// Frames: F218
// 0x141f3e705  4939d4                         cmpq	%rdx, %r12
// 0x141f3e708  0f8381000000                   jae	0x141f3e78f <.text+0x1f3d78f>
// 0x141f3e70e  4f8d1c22                       leaq	(%r10,%r12), %r11
// 0x141f3e712  4d39cb                         cmpq	%r9, %r11
// 0x141f3e715  0f8387000000                   jae	0x141f3e7a2 <.text+0x1f3d7a2>
// 0x141f3e71b  460fb61c27                     movzbl	(%rdi,%r12), %r11d
// 0x141f3e720  473a1c27                       cmpb	(%r15,%r12), %r11b
// Frames: F219 -> F220 -> F221 -> F217 -> F215
// 0x141f3e724  4d8d642401                     leaq	0x1(%r12), %r12
// Frames: F218
// 0x141f3e729  74d5                           je	0x141f3e700 <.text+0x1f3d700>
// Frames: F222
// 0x141f3e72b  488b742430                     movq	0x30(%rsp), %rsi
// 0x141f3e730  4829f5                         subq	%rsi, %rbp
// 0x141f3e733  48896828                       movq	%rbp, 0x28(%rax)
// 0x141f3e737  4989ea                         movq	%rbp, %r10
// 0x141f3e73a  4589f7                         movl	%r14d, %r15d
// 0x141f3e73d  4584f6                         testb	%r14b, %r14b
// 0x141f3e740  0f84cafeffff                   je	0x141f3e610 <.text+0x1f3d610>
// 0x141f3e746  e9ccfeffff                     jmp	0x141f3e617 <.text+0x1f3d617>
// Frames: F251
// 0x141f3e74b  48c7402800000000               movq	$0x0, 0x28(%rax)
// 0x141f3e753  31c0                           xorl	%eax, %eax
// 0x141f3e755  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x141f3e75a  eb1f                           jmp	0x141f3e77b <.text+0x1f3d77b>
// 0x141f3e75c  4c895028                       movq	%r10, 0x28(%rax)
// 0x141f3e760  4584f6                         testb	%r14b, %r14b
// 0x141f3e763  7504                           jne	0x141f3e769 <.text+0x1f3d769>
// 0x141f3e765  48895038                       movq	%rdx, 0x38(%rax)
// Frames: F224 -> F251
// 0x141f3e769  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x141f3e76e  4c895108                       movq	%r10, 0x8(%rcx)
// 0x141f3e772  48896910                       movq	%rbp, 0x10(%rcx)
// 0x141f3e776  b801000000                     movl	$0x1, %eax
// Frames: F251
// 0x141f3e77b  488901                         movq	%rax, (%rcx)
// 0x141f3e77e  4883c438                       addq	$0x38, %rsp
// 0x141f3e782  5b                             popq	%rbx
// 0x141f3e783  5d                             popq	%rbp
// 0x141f3e784  5f                             popq	%rdi
// 0x141f3e785  5e                             popq	%rsi
// 0x141f3e786  415c                           popq	%r12
// 0x141f3e788  415d                           popq	%r13
// 0x141f3e78a  415e                           popq	%r14
// 0x141f3e78c  415f                           popq	%r15
// 0x141f3e78e  c3                             retq
// Frames: F46 -> F202 -> F251
// 0x141f3e78f  4839d1                         cmpq	%rdx, %rcx
// 0x141f3e792  480f46ca                       cmovbeq	%rdx, %rcx
// Frames: F251
// 0x141f3e796  4c8d0533aa4d01                 leaq	0x14daa33(%rip), %r8    # 0x1434191d0
// 0x141f3e79d  e8202fe700                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f3e7a2  4901ca                         addq	%rcx, %r10
// 0x141f3e7a5  4d39d1                         cmpq	%r10, %r9
// 0x141f3e7a8  4d0f47d1                       cmovaq	%r9, %r10
// 0x141f3e7ac  4c8d0535aa4d01                 leaq	0x14daa35(%rip), %r8    # 0x1434191e8
// 0x141f3e7b3  4c89d1                         movq	%r10, %rcx
// 0x141f3e7b6  4c89ca                         movq	%r9, %rdx
// 0x141f3e7b9  e8042fe700                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f3e7be  4c8d0553aa4d01                 leaq	0x14daa53(%rip), %r8    # 0x143419218
// 0x141f3e7c5  4889f1                         movq	%rsi, %rcx
// 0x141f3e7c8  4c89ca                         movq	%r9, %rdx
// 0x141f3e7cb  e8f22ee700                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f3e7d0  4c8d0529aa4d01                 leaq	0x14daa29(%rip), %r8    # 0x143419200
// 0x141f3e7d7  4889f1                         movq	%rsi, %rcx
// 0x141f3e7da  e8e32ee700                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f3e7df  cc                             int3
//
// RANGE rva_1f42050_len_230: RVA 0x1f42050, 560 PDB bytes; 146 instructions
// Evidence: _assembly/roof-v1/C/pack-001/rva_1f42050_len_230.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2449:37572: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Frames: F199
// 0x141f42050  4157                           pushq	%r15
// 0x141f42052  4156                           pushq	%r14
// 0x141f42054  4155                           pushq	%r13
// 0x141f42056  4154                           pushq	%r12
// 0x141f42058  56                             pushq	%rsi
// 0x141f42059  57                             pushq	%rdi
// 0x141f4205a  55                             pushq	%rbp
// 0x141f4205b  53                             pushq	%rbx
// 0x141f4205c  4883ec38                       subq	$0x38, %rsp
// 0x141f42060  4889d0                         movq	%rdx, %rax
// 0x141f42063  48894c2428                     movq	%rcx, 0x28(%rsp)
// 0x141f42068  488b9424a8000000               movq	0xa8(%rsp), %rdx
// 0x141f42070  488b6828                       movq	0x28(%rax), %rbp
// Frames: F200 -> F201
// 0x141f42074  4989ea                         movq	%rbp, %r10
// 0x141f42077  4929d2                         subq	%rdx, %r10
// Frames: F46 -> F202 -> F201
// 0x141f4207a  4d39ca                         cmpq	%r9, %r10
// 0x141f4207d  0f8368010000                   jae	0x141f421eb <.text+0x1f411eb>
// 0x141f42083  440fb6bc24b0000000             movzbl	0xb0(%rsp), %r15d
// 0x141f4208c  488bbc24a0000000               movq	0xa0(%rsp), %rdi
// 0x141f42094  488b5818                       movq	0x18(%rax), %rbx
// 0x141f42098  488b4808                       movq	0x8(%rax), %rcx
// 0x141f4209c  4c8b5810                       movq	0x10(%rax), %r11
// 0x141f420a0  4c895c2430                     movq	%r11, 0x30(%rsp)
// 0x141f420a5  4c8b6838                       movq	0x38(%rax), %r13
// 0x141f420a9  eb1b                           jmp	0x141f420c6 <.text+0x1f410c6>
// 0x141f420ab  0f1f440000                     nopl	(%rax,%rax)
// 0x141f420b0  48897038                       movq	%rsi, 0x38(%rax)
// 0x141f420b4  4989f5                         movq	%rsi, %r13
// Frames: F200 -> F201
// 0x141f420b7  4c89d5                         movq	%r10, %rbp
// 0x141f420ba  4929d2                         subq	%rdx, %r10
// Frames: F46 -> F202 -> F201
// 0x141f420bd  4d39ca                         cmpq	%r9, %r10
// 0x141f420c0  0f8325010000                   jae	0x141f421eb <.text+0x1f411eb>
// Frames: F203
// 0x141f420c6  470fb61c10                     movzbl	(%r8,%r10), %r11d
// 0x141f420cb  4c0fa3db                       btq	%r11, %rbx
// 0x141f420cf  0f838b000000                   jae	0x141f42160 <.text+0x1f41160>
// Frames: F204
// 0x141f420d5  4939cd                         cmpq	%rcx, %r13
// 0x141f420d8  4989cb                         movq	%rcx, %r11
// 0x141f420db  4d0f42dd                       cmovbq	%r13, %r11
// 0x141f420df  4589fe                         movl	%r15d, %r14d
// 0x141f420e2  41f6c701                       testb	$0x1, %r15b
// 0x141f420e6  4c0f45d9                       cmovneq	%rcx, %r11
// Frames: F205
// 0x141f420ea  498d73ff                       leaq	-0x1(%r11), %rsi
// 0x141f420ee  4839d6                         cmpq	%rdx, %rsi
// 0x141f420f1  0f8389000000                   jae	0x141f42180 <.text+0x1f41180>
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f420f7  4d89dc                         movq	%r11, %r12
// 0x141f420fa  49f7dc                         negq	%r12
// 0x141f420fd  488d77ff                       leaq	-0x1(%rdi), %rsi
// 0x141f42101  4e8d3c1e                       leaq	(%rsi,%r11), %r15
// 0x141f42105  4b8d3413                       leaq	(%r11,%r10), %rsi
// 0x141f42109  48ffce                         decq	%rsi
// 0x141f4210c  0f1f4000                       nopl	(%rax)
// Frames: F209 -> F206 -> F207 -> F208 -> F205
// 0x141f42110  4d85e4                         testq	%r12, %r12
// Frames: F206 -> F207 -> F208 -> F205
// 0x141f42113  7474                           je	0x141f42189 <.text+0x1f41189>
// Frames: F210
// 0x141f42115  4c39ce                         cmpq	%r9, %rsi
// 0x141f42118  0f8340010000                   jae	0x141f4225e <.text+0x1f4125e>
// 0x141f4211e  450fb61f                       movzbl	(%r15), %r11d
// 0x141f42122  49ffc4                         incq	%r12
// 0x141f42125  49ffcf                         decq	%r15
// 0x141f42128  453a1c30                       cmpb	(%r8,%rsi), %r11b
// 0x141f4212c  488d76ff                       leaq	-0x1(%rsi), %rsi
// 0x141f42130  74de                           je	0x141f42110 <.text+0x1f41110>
// Frames: F211
// 0x141f42132  4829cd                         subq	%rcx, %rbp
// 0x141f42135  4c29e5                         subq	%r12, %rbp
// 0x141f42138  48896828                       movq	%rbp, 0x28(%rax)
// 0x141f4213c  4889d6                         movq	%rdx, %rsi
// 0x141f4213f  4989ea                         movq	%rbp, %r10
// 0x141f42142  4589f7                         movl	%r14d, %r15d
// 0x141f42145  4584f6                         testb	%r14b, %r14b
// 0x141f42148  0f8462ffffff                   je	0x141f420b0 <.text+0x1f410b0>
// 0x141f4214e  e964ffffff                     jmp	0x141f420b7 <.text+0x1f410b7>
// 0x141f42153  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// Frames: F212
// 0x141f42160  4c895028                       movq	%r10, 0x28(%rax)
// 0x141f42164  4889d6                         movq	%rdx, %rsi
// Frames: F213
// 0x141f42167  4584ff                         testb	%r15b, %r15b
// Frames: F248
// 0x141f4216a  0f8440ffffff                   je	0x141f420b0 <.text+0x1f410b0>
// 0x141f42170  e942ffffff                     jmp	0x141f420b7 <.text+0x1f410b7>
// 0x141f42175  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F209 -> F206 -> F207 -> F208 -> F248
// 0x141f42180  4d85db                         testq	%r11, %r11
// Frames: F206 -> F207 -> F208 -> F248
// 0x141f42183  0f85e7000000                   jne	0x141f42270 <.text+0x1f41270>
// Frames: F248
// 0x141f42189  4584f6                         testb	%r14b, %r14b
// 0x141f4218c  4c89ee                         movq	%r13, %rsi
// 0x141f4218f  480f45f2                       cmovneq	%rdx, %rsi
// 0x141f42193  4f8d3c10                       leaq	(%r8,%r10), %r15
// 0x141f42197  4989cc                         movq	%rcx, %r12
// 0x141f4219a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F209 -> F216 -> F217 -> F248
// 0x141f421a0  4939f4                         cmpq	%rsi, %r12
// Frames: F216 -> F217 -> F248
// 0x141f421a3  7357                           jae	0x141f421fc <.text+0x1f411fc>
// Frames: F248
// 0x141f421a5  4939d4                         cmpq	%rdx, %r12
// 0x141f421a8  0f8381000000                   jae	0x141f4222f <.text+0x1f4122f>
// 0x141f421ae  4f8d1c22                       leaq	(%r10,%r12), %r11
// 0x141f421b2  4d39cb                         cmpq	%r9, %r11
// 0x141f421b5  0f8387000000                   jae	0x141f42242 <.text+0x1f41242>
// 0x141f421bb  460fb61c27                     movzbl	(%rdi,%r12), %r11d
// 0x141f421c0  473a1c27                       cmpb	(%r15,%r12), %r11b
// Frames: F219 -> F220 -> F221 -> F217 -> F248
// 0x141f421c4  4d8d642401                     leaq	0x1(%r12), %r12
// Frames: F248
// 0x141f421c9  74d5                           je	0x141f421a0 <.text+0x1f411a0>
// 0x141f421cb  488b742430                     movq	0x30(%rsp), %rsi
// 0x141f421d0  4829f5                         subq	%rsi, %rbp
// 0x141f421d3  48896828                       movq	%rbp, 0x28(%rax)
// 0x141f421d7  4989ea                         movq	%rbp, %r10
// 0x141f421da  4589f7                         movl	%r14d, %r15d
// 0x141f421dd  4584f6                         testb	%r14b, %r14b
// 0x141f421e0  0f84cafeffff                   je	0x141f420b0 <.text+0x1f410b0>
// 0x141f421e6  e9ccfeffff                     jmp	0x141f420b7 <.text+0x1f410b7>
// 0x141f421eb  48c7402800000000               movq	$0x0, 0x28(%rax)
// 0x141f421f3  31c0                           xorl	%eax, %eax
// 0x141f421f5  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x141f421fa  eb1f                           jmp	0x141f4221b <.text+0x1f4121b>
// 0x141f421fc  4c895028                       movq	%r10, 0x28(%rax)
// Frames: F249
// 0x141f42200  4584f6                         testb	%r14b, %r14b
// 0x141f42203  7504                           jne	0x141f42209 <.text+0x1f41209>
// Frames: F250
// 0x141f42205  48895038                       movq	%rdx, 0x38(%rax)
// Frames: F224 -> F225
// 0x141f42209  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x141f4220e  4c895108                       movq	%r10, 0x8(%rcx)
// 0x141f42212  48896910                       movq	%rbp, 0x10(%rcx)
// 0x141f42216  b801000000                     movl	$0x1, %eax
// Frames: F226
// 0x141f4221b  488901                         movq	%rax, (%rcx)
// 0x141f4221e  4883c438                       addq	$0x38, %rsp
// 0x141f42222  5b                             popq	%rbx
// 0x141f42223  5d                             popq	%rbp
// 0x141f42224  5f                             popq	%rdi
// 0x141f42225  5e                             popq	%rsi
// 0x141f42226  415c                           popq	%r12
// 0x141f42228  415d                           popq	%r13
// 0x141f4222a  415e                           popq	%r14
// 0x141f4222c  415f                           popq	%r15
// 0x141f4222e  c3                             retq
// Frames: F46 -> F202 -> F201
// 0x141f4222f  4839d1                         cmpq	%rdx, %rcx
// 0x141f42232  480f46ca                       cmovbeq	%rdx, %rcx
// Frames: F218
// 0x141f42236  4c8d05e37a4d01                 leaq	0x14d7ae3(%rip), %r8    # 0x143419d20
// 0x141f4223d  e880f4e600                     callq	0x142db16c2 <.text+0x2db06c2>
// Frames: F215
// 0x141f42242  4901ca                         addq	%rcx, %r10
// 0x141f42245  4d39d1                         cmpq	%r10, %r9
// 0x141f42248  4d0f47d1                       cmovaq	%r9, %r10
// Frames: F218
// 0x141f4224c  4c8d05e57a4d01                 leaq	0x14d7ae5(%rip), %r8    # 0x143419d38
// 0x141f42253  4c89d1                         movq	%r10, %rcx
// 0x141f42256  4c89ca                         movq	%r9, %rdx
// 0x141f42259  e864f4e600                     callq	0x142db16c2 <.text+0x2db06c2>
// Frames: F210
// 0x141f4225e  4c8d05037b4d01                 leaq	0x14d7b03(%rip), %r8    # 0x143419d68
// 0x141f42265  4889f1                         movq	%rsi, %rcx
// 0x141f42268  4c89ca                         movq	%r9, %rdx
// 0x141f4226b  e852f4e600                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f42270  4c8d05d97a4d01                 leaq	0x14d7ad9(%rip), %r8    # 0x143419d50
// 0x141f42277  4889f1                         movq	%rsi, %rcx
// 0x141f4227a  e843f4e600                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x141f4227f  cc                             int3
//
// RANGE rva_281b220_len_230: RVA 0x281b220, 560 PDB bytes; 146 instructions
// Evidence: _assembly/roof-v1/C/pack-001/rva_281b220_len_230.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3750:7760: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Frames: F199
// 0x14281b220  4157                           pushq	%r15
// Frames: F248
// 0x14281b222  4156                           pushq	%r14
// 0x14281b224  4155                           pushq	%r13
// 0x14281b226  4154                           pushq	%r12
// 0x14281b228  56                             pushq	%rsi
// 0x14281b229  57                             pushq	%rdi
// 0x14281b22a  55                             pushq	%rbp
// 0x14281b22b  53                             pushq	%rbx
// 0x14281b22c  4883ec38                       subq	$0x38, %rsp
// 0x14281b230  4889d0                         movq	%rdx, %rax
// 0x14281b233  48894c2428                     movq	%rcx, 0x28(%rsp)
// 0x14281b238  488b9424a8000000               movq	0xa8(%rsp), %rdx
// 0x14281b240  488b6828                       movq	0x28(%rax), %rbp
// Frames: F200 -> F248
// 0x14281b244  4989ea                         movq	%rbp, %r10
// 0x14281b247  4929d2                         subq	%rdx, %r10
// Frames: F46 -> F202 -> F248
// 0x14281b24a  4d39ca                         cmpq	%r9, %r10
// 0x14281b24d  0f8368010000                   jae	0x14281b3bb <.text+0x281a3bb>
// 0x14281b253  440fb6bc24b0000000             movzbl	0xb0(%rsp), %r15d
// 0x14281b25c  488bbc24a0000000               movq	0xa0(%rsp), %rdi
// 0x14281b264  488b5818                       movq	0x18(%rax), %rbx
// 0x14281b268  488b4808                       movq	0x8(%rax), %rcx
// 0x14281b26c  4c8b5810                       movq	0x10(%rax), %r11
// 0x14281b270  4c895c2430                     movq	%r11, 0x30(%rsp)
// 0x14281b275  4c8b6838                       movq	0x38(%rax), %r13
// 0x14281b279  eb1b                           jmp	0x14281b296 <.text+0x281a296>
// 0x14281b27b  0f1f440000                     nopl	(%rax,%rax)
// 0x14281b280  48897038                       movq	%rsi, 0x38(%rax)
// 0x14281b284  4989f5                         movq	%rsi, %r13
// Frames: F200 -> F248
// 0x14281b287  4c89d5                         movq	%r10, %rbp
// 0x14281b28a  4929d2                         subq	%rdx, %r10
// Frames: F46 -> F202 -> F248
// 0x14281b28d  4d39ca                         cmpq	%r9, %r10
// 0x14281b290  0f8325010000                   jae	0x14281b3bb <.text+0x281a3bb>
// Frames: F248
// 0x14281b296  470fb61c10                     movzbl	(%r8,%r10), %r11d
// 0x14281b29b  4c0fa3db                       btq	%r11, %rbx
// 0x14281b29f  0f838b000000                   jae	0x14281b330 <.text+0x281a330>
// 0x14281b2a5  4939cd                         cmpq	%rcx, %r13
// 0x14281b2a8  4989cb                         movq	%rcx, %r11
// 0x14281b2ab  4d0f42dd                       cmovbq	%r13, %r11
// 0x14281b2af  4589fe                         movl	%r15d, %r14d
// 0x14281b2b2  41f6c701                       testb	$0x1, %r15b
// 0x14281b2b6  4c0f45d9                       cmovneq	%rcx, %r11
// 0x14281b2ba  498d73ff                       leaq	-0x1(%r11), %rsi
// 0x14281b2be  4839d6                         cmpq	%rdx, %rsi
// 0x14281b2c1  0f8389000000                   jae	0x14281b350 <.text+0x281a350>
// Frames: F206 -> F207 -> F208 -> F248
// 0x14281b2c7  4d89dc                         movq	%r11, %r12
// 0x14281b2ca  49f7dc                         negq	%r12
// 0x14281b2cd  488d77ff                       leaq	-0x1(%rdi), %rsi
// 0x14281b2d1  4e8d3c1e                       leaq	(%rsi,%r11), %r15
// 0x14281b2d5  4b8d3413                       leaq	(%r11,%r10), %rsi
// 0x14281b2d9  48ffce                         decq	%rsi
// 0x14281b2dc  0f1f4000                       nopl	(%rax)
// Frames: F209 -> F206 -> F207 -> F208 -> F248
// 0x14281b2e0  4d85e4                         testq	%r12, %r12
// Frames: F206 -> F207 -> F208 -> F248
// 0x14281b2e3  7474                           je	0x14281b359 <.text+0x281a359>
// Frames: F248
// 0x14281b2e5  4c39ce                         cmpq	%r9, %rsi
// 0x14281b2e8  0f8340010000                   jae	0x14281b42e <.text+0x281a42e>
// 0x14281b2ee  450fb61f                       movzbl	(%r15), %r11d
// 0x14281b2f2  49ffc4                         incq	%r12
// 0x14281b2f5  49ffcf                         decq	%r15
// 0x14281b2f8  453a1c30                       cmpb	(%r8,%rsi), %r11b
// 0x14281b2fc  488d76ff                       leaq	-0x1(%rsi), %rsi
// 0x14281b300  74de                           je	0x14281b2e0 <.text+0x281a2e0>
// 0x14281b302  4829cd                         subq	%rcx, %rbp
// 0x14281b305  4c29e5                         subq	%r12, %rbp
// 0x14281b308  48896828                       movq	%rbp, 0x28(%rax)
// 0x14281b30c  4889d6                         movq	%rdx, %rsi
// 0x14281b30f  4989ea                         movq	%rbp, %r10
// 0x14281b312  4589f7                         movl	%r14d, %r15d
// 0x14281b315  4584f6                         testb	%r14b, %r14b
// 0x14281b318  0f8462ffffff                   je	0x14281b280 <.text+0x281a280>
// 0x14281b31e  e964ffffff                     jmp	0x14281b287 <.text+0x281a287>
// 0x14281b323  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x14281b330  4c895028                       movq	%r10, 0x28(%rax)
// 0x14281b334  4889d6                         movq	%rdx, %rsi
// 0x14281b337  4584ff                         testb	%r15b, %r15b
// 0x14281b33a  0f8440ffffff                   je	0x14281b280 <.text+0x281a280>
// 0x14281b340  e942ffffff                     jmp	0x14281b287 <.text+0x281a287>
// 0x14281b345  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F209 -> F206 -> F207 -> F208 -> F248
// 0x14281b350  4d85db                         testq	%r11, %r11
// Frames: F206 -> F207 -> F208 -> F248
// 0x14281b353  0f85e7000000                   jne	0x14281b440 <.text+0x281a440>
// Frames: F248
// 0x14281b359  4584f6                         testb	%r14b, %r14b
// 0x14281b35c  4c89ee                         movq	%r13, %rsi
// 0x14281b35f  480f45f2                       cmovneq	%rdx, %rsi
// 0x14281b363  4f8d3c10                       leaq	(%r8,%r10), %r15
// 0x14281b367  4989cc                         movq	%rcx, %r12
// 0x14281b36a  660f1f440000                   nopw	(%rax,%rax)
// Frames: F209 -> F216 -> F217 -> F248
// 0x14281b370  4939f4                         cmpq	%rsi, %r12
// Frames: F216 -> F217 -> F248
// 0x14281b373  7357                           jae	0x14281b3cc <.text+0x281a3cc>
// Frames: F248
// 0x14281b375  4939d4                         cmpq	%rdx, %r12
// 0x14281b378  0f8381000000                   jae	0x14281b3ff <.text+0x281a3ff>
// 0x14281b37e  4f8d1c22                       leaq	(%r10,%r12), %r11
// 0x14281b382  4d39cb                         cmpq	%r9, %r11
// 0x14281b385  0f8387000000                   jae	0x14281b412 <.text+0x281a412>
// 0x14281b38b  460fb61c27                     movzbl	(%rdi,%r12), %r11d
// 0x14281b390  473a1c27                       cmpb	(%r15,%r12), %r11b
// Frames: F219 -> F220 -> F221 -> F217 -> F248
// 0x14281b394  4d8d642401                     leaq	0x1(%r12), %r12
// Frames: F248
// 0x14281b399  74d5                           je	0x14281b370 <.text+0x281a370>
// 0x14281b39b  488b742430                     movq	0x30(%rsp), %rsi
// 0x14281b3a0  4829f5                         subq	%rsi, %rbp
// 0x14281b3a3  48896828                       movq	%rbp, 0x28(%rax)
// 0x14281b3a7  4989ea                         movq	%rbp, %r10
// 0x14281b3aa  4589f7                         movl	%r14d, %r15d
// 0x14281b3ad  4584f6                         testb	%r14b, %r14b
// 0x14281b3b0  0f84cafeffff                   je	0x14281b280 <.text+0x281a280>
// 0x14281b3b6  e9ccfeffff                     jmp	0x14281b287 <.text+0x281a287>
// 0x14281b3bb  48c7402800000000               movq	$0x0, 0x28(%rax)
// 0x14281b3c3  31c0                           xorl	%eax, %eax
// 0x14281b3c5  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x14281b3ca  eb1f                           jmp	0x14281b3eb <.text+0x281a3eb>
// 0x14281b3cc  4c895028                       movq	%r10, 0x28(%rax)
// Frames: F249
// 0x14281b3d0  4584f6                         testb	%r14b, %r14b
// 0x14281b3d3  7504                           jne	0x14281b3d9 <.text+0x281a3d9>
// Frames: F250
// 0x14281b3d5  48895038                       movq	%rdx, 0x38(%rax)
// Frames: F224 -> F225
// 0x14281b3d9  488b4c2428                     movq	0x28(%rsp), %rcx
// 0x14281b3de  4c895108                       movq	%r10, 0x8(%rcx)
// 0x14281b3e2  48896910                       movq	%rbp, 0x10(%rcx)
// 0x14281b3e6  b801000000                     movl	$0x1, %eax
// Frames: F226
// 0x14281b3eb  488901                         movq	%rax, (%rcx)
// 0x14281b3ee  4883c438                       addq	$0x38, %rsp
// 0x14281b3f2  5b                             popq	%rbx
// 0x14281b3f3  5d                             popq	%rbp
// 0x14281b3f4  5f                             popq	%rdi
// 0x14281b3f5  5e                             popq	%rsi
// 0x14281b3f6  415c                           popq	%r12
// 0x14281b3f8  415d                           popq	%r13
// 0x14281b3fa  415e                           popq	%r14
// 0x14281b3fc  415f                           popq	%r15
// 0x14281b3fe  c3                             retq
// Frames: F46 -> F202 -> F201
// 0x14281b3ff  4839d1                         cmpq	%rdx, %rcx
// 0x14281b402  480f46ca                       cmovbeq	%rdx, %rcx
// Frames: F218
// 0x14281b406  4c8d0573bed600                 leaq	0xd6be73(%rip), %r8     # 0x143587280
// 0x14281b40d  e8b0625900                     callq	0x142db16c2 <.text+0x2db06c2>
// Frames: F215
// 0x14281b412  4901ca                         addq	%rcx, %r10
// 0x14281b415  4d39d1                         cmpq	%r10, %r9
// 0x14281b418  4d0f47d1                       cmovaq	%r9, %r10
// Frames: F218
// 0x14281b41c  4c8d0575bed600                 leaq	0xd6be75(%rip), %r8     # 0x143587298
// 0x14281b423  4c89d1                         movq	%r10, %rcx
// 0x14281b426  4c89ca                         movq	%r9, %rdx
// 0x14281b429  e894625900                     callq	0x142db16c2 <.text+0x2db06c2>
// Frames: F210
// 0x14281b42e  4c8d0593bed600                 leaq	0xd6be93(%rip), %r8     # 0x1435872c8
// 0x14281b435  4889f1                         movq	%rsi, %rcx
// 0x14281b438  4c89ca                         movq	%r9, %rdx
// 0x14281b43b  e882625900                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x14281b440  4c8d0569bed600                 leaq	0xd6be69(%rip), %r8     # 0x1435872b0
// 0x14281b447  4889f1                         movq	%rsi, %rcx
// 0x14281b44a  e873625900                     callq	0x142db16c2 <.text+0x2db06c2>
// 0x14281b44f  cc                             int3
//
// RANGE rva_281bb00_len_1dc: RVA 0x281bb00, 476 PDB bytes; 144 instructions
// Evidence: _assembly/roof-v1/C/pack-001/rva_281bb00_len_1dc.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3756:952: arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>,enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > > >
// Frames: F252
// 0x14281bb00  55                             pushq	%rbp
// 0x14281bb01  4157                           pushq	%r15
// 0x14281bb03  4156                           pushq	%r14
// 0x14281bb05  56                             pushq	%rsi
// 0x14281bb06  57                             pushq	%rdi
// 0x14281bb07  53                             pushq	%rbx
// 0x14281bb08  4883ec58                       subq	$0x58, %rsp
// 0x14281bb0c  488d6c2450                     leaq	0x50(%rsp), %rbp
// Frames: F253 -> F254 -> F255
// 0x14281bb11  48c74500feffffff               movq	$-0x2, (%rbp)
// 0x14281bb19  488b02                         movq	(%rdx), %rax
// Frames: F254 -> F255
// 0x14281bb1c  488945f8                       movq	%rax, -0x8(%rbp)
// 0x14281bb20  498b7108                       movq	0x8(%r9), %rsi
// Frames: F256
// 0x14281bb24  4189c1                         movl	%eax, %r9d
// 0x14281bb27  4183e103                       andl	$0x3, %r9d
// 0x14281bb2b  488d3d06bcd600                 leaq	0xd6bc06(%rip), %rdi    # 0x143587738
// 0x14281bb32  4e630c8f                       movslq	(%rdi,%r9,4), %r9
// 0x14281bb36  4901f9                         addq	%rdi, %r9
// 0x14281bb39  41ffe1                         jmpq	*%r9
// Frames: F257
// 0x14281bb3c  4885c0                         testq	%rax, %rax
// 0x14281bb3f  0f8531010000                   jne	0x14281bc76 <.text+0x281ac76>
// Frames: F258
// 0x14281bb45  4883c458                       addq	$0x58, %rsp
// 0x14281bb49  5b                             popq	%rbx
// 0x14281bb4a  5f                             popq	%rdi
// 0x14281bb4b  5e                             popq	%rsi
// 0x14281bb4c  415e                           popq	%r14
// 0x14281bb4e  415f                           popq	%r15
// 0x14281bb50  5d                             popq	%rbp
// 0x14281bb51  c3                             retq
// 0x14281bb52  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// Frames: F253 -> F254 -> F259
// 0x14281bb60  488b4210                       movq	0x10(%rdx), %rax
// Frames: F260
// 0x14281bb64  4c39c0                         cmpq	%r8, %rax
// 0x14281bb67  7577                           jne	0x14281bbe0 <.text+0x281abe0>
// Frames: F261 -> F262
// 0x14281bb69  4c89c3                         movq	%r8, %rbx
// 0x14281bb6c  488b06                         movq	(%rsi), %rax
// 0x14281bb6f  488945e8                       movq	%rax, -0x18(%rbp)
// 0x14281bb73  488d45e8                       leaq	-0x18(%rbp), %rax
// 0x14281bb77  4989ce                         movq	%rcx, %r14
// Frames: F263 -> F261 -> F262
// 0x14281bb7a  4889c1                         movq	%rax, %rcx
// 0x14281bb7d  4989d7                         movq	%rdx, %r15
// 0x14281bb80  e85bcc6eff                     callq	0x141f087e0 <.text+0x1f077e0>
// 0x14281bb85  4c89f1                         movq	%r14, %rcx
// 0x14281bb88  4989d1                         movq	%rdx, %r9
// 0x14281bb8b  4c89fa                         movq	%r15, %rdx
// 0x14281bb8e  4c894df0                       movq	%r9, -0x10(%rbp)
// Frames: F264 -> F265 -> F266 -> F261 -> F262
// 0x14281bb92  48c745e800000000               movq	$0x0, -0x18(%rbp)
// Frames: F266 -> F261 -> F262
// 0x14281bb9a  4885c0                         testq	%rax, %rax
// 0x14281bb9d  7470                           je	0x14281bc0f <.text+0x281ac0f>
// Frames: F261 -> F262
// 0x14281bb9f  4989c0                         movq	%rax, %r8
// Frames: F267 -> F268 -> F269 -> F261 -> F262
// 0x14281bba2  4d85c9                         testq	%r9, %r9
// 0x14281bba5  745b                           je	0x14281bc02 <.text+0x281ac02>
// Frames: F132 -> F270 -> F271 -> F272 -> F268 -> F269 -> F261 -> F262
// 0x14281bba7  f0                             lock
// 0x14281bba8  49ff01                         incq	(%r9)
// Frames: F273 -> F272 -> F268 -> F269 -> F261 -> F262
// 0x14281bbab  0f8ef8000000                   jle	0x14281bca9 <.text+0x281aca9>
// Frames: F274 -> F275 -> F276 -> F277 -> F261 -> F262
// 0x14281bbb1  4c89c8                         movq	%r9, %rax
// 0x14281bbb4  4883c010                       addq	$0x10, %rax
// 0x14281bbb8  41ba03000000                   movl	$0x3, %r10d
// 0x14281bbbe  f0                             lock
// 0x14281bbbf  4d0fb110                       cmpxchgq	%r10, (%r8)
// Frames: F277 -> F261 -> F262
// 0x14281bbc3  744a                           je	0x14281bc0f <.text+0x281ac0f>
// Frames: F278 -> F279 -> F261 -> F262
// 0x14281bbc5  4c894dd8                       movq	%r9, -0x28(%rbp)
// Frames: F280 -> F281 -> F282 -> F283 -> F283 -> F284 -> F278 -> F279 -> F261 -> F262
// 0x14281bbc9  f0                             lock
// 0x14281bbca  49ff09                         decq	(%r9)
// Frames: F282 -> F283 -> F283 -> F284 -> F278 -> F279 -> F261 -> F262
// 0x14281bbcd  7540                           jne	0x14281bc0f <.text+0x281ac0f>
// Frames: F285 -> F283 -> F283 -> F284 -> F278 -> F279 -> F261 -> F262
// 0x14281bbcf  488d4dd8                       leaq	-0x28(%rbp), %rcx
// 0x14281bbd3  e8389becfd                     callq	0x1406e5710 <.text+0x6e4710>
// 0x14281bbd8  4c89fa                         movq	%r15, %rdx
// 0x14281bbdb  4c89f1                         movq	%r14, %rcx
// 0x14281bbde  eb2f                           jmp	0x14281bc0f <.text+0x281ac0f>
// Frames: F253 -> F254 -> F286
// 0x14281bbe0  488b02                         movq	(%rdx), %rax
// Frames: F287
// 0x14281bbe3  483b45f8                       cmpq	-0x8(%rbp), %rax
// 0x14281bbe7  0f8458ffffff                   je	0x14281bb45 <.text+0x281ab45>
// Frames: F288
// 0x14281bbed  488945f8                       movq	%rax, -0x8(%rbp)
// Frames: F256
// 0x14281bbf1  4189c1                         movl	%eax, %r9d
// 0x14281bbf4  4183e103                       andl	$0x3, %r9d
// 0x14281bbf8  4e630c8f                       movslq	(%rdi,%r9,4), %r9
// 0x14281bbfc  4901f9                         addq	%rdi, %r9
// 0x14281bbff  41ffe1                         jmpq	*%r9
// Frames: F274 -> F275 -> F276 -> F277 -> F261 -> F262
// 0x14281bc02  41b903000000                   movl	$0x3, %r9d
// 0x14281bc08  31c0                           xorl	%eax, %eax
// 0x14281bc0a  f0                             lock
// 0x14281bc0b  4d0fb108                       cmpxchgq	%r9, (%r8)
// Frames: F289 -> F290 -> F261 -> F262
// 0x14281bc0f  488b45f0                       movq	-0x10(%rbp), %rax
// Frames: F262
// 0x14281bc13  488945e0                       movq	%rax, -0x20(%rbp)
// 0x14281bc17  4c8d4810                       leaq	0x10(%rax), %r9
// Frames: F54 -> F291 -> F292
// 0x14281bc1b  4885c0                         testq	%rax, %rax
// 0x14281bc1e  4c0f44c8                       cmoveq	%rax, %r9
// Frames: F253 -> F293 -> F294
// 0x14281bc22  4c8b4220                       movq	0x20(%rdx), %r8
// Frames: F253 -> F293 -> F295
// 0x14281bc26  4c8b5120                       movq	0x20(%rcx), %r10
// Frames: F296 -> F297 -> F298
// 0x14281bc2a  4d870a                         xchgq	%r9, (%r10)
// Frames: F299
// 0x14281bc2d  4983ca01                       orq	$0x1, %r10
// Frames: F300
// 0x14281bc31  488b45f8                       movq	-0x8(%rbp), %rax
// Frames: F301 -> F275 -> F300
// 0x14281bc35  f0                             lock
// 0x14281bc36  4c0fb112                       cmpxchgq	%r10, (%rdx)
// Frames: F302
// 0x14281bc3a  7464                           je	0x14281bca0 <.text+0x281aca0>
// Frames: F303
// 0x14281bc3c  488945f8                       movq	%rax, -0x8(%rbp)
// Frames: F283 -> F304
// 0x14281bc40  488b45e0                       movq	-0x20(%rbp), %rax
// 0x14281bc44  4885c0                         testq	%rax, %rax
// 0x14281bc47  7415                           je	0x14281bc5e <.text+0x281ac5e>
// Frames: F280 -> F281 -> F282 -> F283 -> F283 -> F304
// 0x14281bc49  f0                             lock
// 0x14281bc4a  48ff08                         decq	(%rax)
// Frames: F282 -> F283 -> F283 -> F304
// 0x14281bc4d  750f                           jne	0x14281bc5e <.text+0x281ac5e>
// Frames: F305 -> F306 -> F283 -> F283 -> F304
// 0x14281bc4f  488d4de0                       leaq	-0x20(%rbp), %rcx
// Frames: F285 -> F283 -> F283 -> F304
// 0x14281bc53  e8b89aecfd                     callq	0x1406e5710 <.text+0x6e4710>
// 0x14281bc58  4c89fa                         movq	%r15, %rdx
// 0x14281bc5b  4c89f1                         movq	%r14, %rcx
// Frames: F256
// 0x14281bc5e  488b45f8                       movq	-0x8(%rbp), %rax
// 0x14281bc62  4189c0                         movl	%eax, %r8d
// 0x14281bc65  4183e003                       andl	$0x3, %r8d
// 0x14281bc69  4e630c87                       movslq	(%rdi,%r8,4), %r9
// 0x14281bc6d  4901f9                         addq	%rdi, %r9
// 0x14281bc70  4989d8                         movq	%rbx, %r8
// 0x14281bc73  41ffe1                         jmpq	*%r9
// Frames: F307
// 0x14281bc76  488d45f8                       leaq	-0x8(%rbp), %rax
// 0x14281bc7a  488945e8                       movq	%rax, -0x18(%rbp)
// 0x14281bc7e  488d05cbc674fe                 leaq	-0x18b3935(%rip), %rax  # 0x140f68350 <.text+0xf67350>
// 0x14281bc85  488945f0                       movq	%rax, -0x10(%rbp)
// 0x14281bc89  488d0d48bad600                 leaq	0xd6ba48(%rip), %rcx    # 0x1435876d8
// 0x14281bc90  4c8d0589bad600                 leaq	0xd6ba89(%rip), %r8     # 0x143587720
// 0x14281bc97  488d55e8                       leaq	-0x18(%rbp), %rdx
// 0x14281bc9b  e8005c5900                     callq	0x142db18a0 <.text+0x2db08a0>
// Frames: F296 -> F308 -> F309
// 0x14281bca0  4c874120                       xchgq	%r8, 0x20(%rcx)
// Frames: F309
// 0x14281bca4  e99cfeffff                     jmp	0x14281bb45 <.text+0x281ab45>
// Frames: F310 -> F272 -> F268 -> F269 -> F261 -> F262
// 0x14281bca9  0f0b                           ud2
// 0x14281bcab  0f1f440000                     nopl	(%rax,%rax)
// 0x14281bcb0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x14281bcb5  55                             pushq	%rbp
// 0x14281bcb6  4157                           pushq	%r15
// 0x14281bcb8  4156                           pushq	%r14
// 0x14281bcba  56                             pushq	%rsi
// 0x14281bcbb  57                             pushq	%rdi
// 0x14281bcbc  53                             pushq	%rbx
// 0x14281bcbd  4883ec28                       subq	$0x28, %rsp
// 0x14281bcc1  488d6a50                       leaq	0x50(%rdx), %rbp
// 0x14281bcc5  488d4de8                       leaq	-0x18(%rbp), %rcx
// Frames: F311 -> F261 -> F262
// 0x14281bcc9  e8d2be92fd                     callq	0x140147ba0 <.text+0x146ba0>
// 0x14281bcce  90                             nop
// Frames: F312 -> F261 -> F262
// 0x14281bccf  4883c428                       addq	$0x28, %rsp
// 0x14281bcd3  5b                             popq	%rbx
// 0x14281bcd4  5f                             popq	%rdi
// 0x14281bcd5  5e                             popq	%rsi
// 0x14281bcd6  415e                           popq	%r14
// 0x14281bcd8  415f                           popq	%r15
// 0x14281bcda  5d                             popq	%rbp
// 0x14281bcdb  c3                             retq
//
// RANGE rva_2831600_len_581: RVA 0x2831600, 1409 PDB bytes; 390 instructions
// Evidence: _assembly/roof-v1/C/pack-003/rva_2831600_len_581.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3788:640: rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
// Frames: F313
// 0x142831600  55                             pushq	%rbp
// 0x142831601  4157                           pushq	%r15
// 0x142831603  4156                           pushq	%r14
// 0x142831605  4155                           pushq	%r13
// 0x142831607  4154                           pushq	%r12
// 0x142831609  56                             pushq	%rsi
// 0x14283160a  57                             pushq	%rdi
// 0x14283160b  53                             pushq	%rbx
// 0x14283160c  4881ec28010000                 subq	$0x128, %rsp            # imm = 0x128
// 0x142831613  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14283161b  48c785a0000000feffffff         movq	$-0x2, 0xa0(%rbp)
// 0x142831626  4d89cc                         movq	%r9, %r12
// 0x142831629  4c898590000000                 movq	%r8, 0x90(%rbp)
// 0x142831630  4989d7                         movq	%rdx, %r15
// 0x142831633  4889ce                         movq	%rcx, %rsi
// 0x142831636  488b8520010000                 movq	0x120(%rbp), %rax
// 0x14283163d  488b8d18010000                 movq	0x118(%rbp), %rcx
// 0x142831644  488bbd10010000                 movq	0x110(%rbp), %rdi
// 0x14283164b  4989fe                         movq	%rdi, %r14
// 0x14283164e  49c1ee20                       shrq	$0x20, %r14
// 0x142831652  49bd000000000000ffff           movabsq	$-0x1000000000000, %r13 # imm = 0xFFFF000000000000
// Frames: F314
// 0x14283165c  4921fd                         andq	%rdi, %r13
// 0x14283165f  48898d80000000                 movq	%rcx, 0x80(%rbp)
// 0x142831666  48898588000000                 movq	%rax, 0x88(%rbp)
// 0x14283166d  488d8d80000000                 leaq	0x80(%rbp), %rcx
// Frames: F315
// 0x142831674  e8270e5efe                     callq	0x140e124a0 <.text+0xe114a0>
// 0x142831679  4489f3                         movl	%r14d, %ebx
// 0x14283167c  83cb02                         orl	$0x2, %ebx
// 0x14283167f  4885c0                         testq	%rax, %rax
// 0x142831682  410f44de                       cmovel	%r14d, %ebx
// Frames: F316
// 0x142831686  4d8db790000000                 leaq	0x90(%r15), %r14
// Frames: F317
// 0x14283168d  4c89f9                         movq	%r15, %rcx
// 0x142831690  e83b0262fe                     callq	0x140e518d0 <.text+0xe508d0>
// Frames: F318 -> F319 -> F320 -> F321
// 0x142831695  4d8b8f60060000                 movq	0x660(%r15), %r9
// 0x14283169c  4c897d10                       movq	%r15, 0x10(%rbp)
// Frames: F318 -> F319 -> F320 -> F322
// 0x1428316a0  498b8fa8060000                 movq	0x6a8(%r15), %rcx
// Frames: F320 -> F321
// 0x1428316a7  4983c110                       addq	$0x10, %r9
// Frames: F320 -> F322
// 0x1428316ab  4883c110                       addq	$0x10, %rcx
// Frames: F314
// 0x1428316af  0fb7d3                         movzwl	%bx, %edx
// 0x1428316b2  48c1e220                       shlq	$0x20, %rdx
// 0x1428316b6  4c09ea                         orq	%r13, %rdx
// 0x1428316b9  89ff                           movl	%edi, %edi
// 0x1428316bb  4809fa                         orq	%rdi, %rdx
// 0x1428316be  4c89642438                     movq	%r12, 0x38(%rsp)
// 0x1428316c3  4c8b8590000000                 movq	0x90(%rbp), %r8
// 0x1428316ca  4c89442430                     movq	%r8, 0x30(%rsp)
// 0x1428316cf  48894c2420                     movq	%rcx, 0x20(%rsp)
// 0x1428316d4  4889542428                     movq	%rdx, 0x28(%rsp)
// 0x1428316d9  488d4d20                       leaq	0x20(%rbp), %rcx
// 0x1428316dd  4d89f5                         movq	%r14, %r13
// 0x1428316e0  4c89f2                         movq	%r14, %rdx
// 0x1428316e3  4989c0                         movq	%rax, %r8
// 0x1428316e6  e865e861fe                     callq	0x140e4ff50 <.text+0xe4ef50>
// Frames: F323 -> F314
// 0x1428316eb  488b4520                       movq	0x20(%rbp), %rax
// 0x1428316ef  488b4d28                       movq	0x28(%rbp), %rcx
// 0x1428316f3  4c8b6530                       movq	0x30(%rbp), %r12
// 0x1428316f7  4c8b7d38                       movq	0x38(%rbp), %r15
// 0x1428316fb  4883f80b                       cmpq	$0xb, %rax
// 0x1428316ff  7579                           jne	0x14283177a <.text+0x283077a>
// Frames: F314
// 0x142831701  48898d90000000                 movq	%rcx, 0x90(%rbp)
// 0x142831708  48894d78                       movq	%rcx, 0x78(%rbp)
// Frames: F324
// 0x14283170c  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x142831713  e8880d5efe                     callq	0x140e124a0 <.text+0xe114a0>
// 0x142831718  90                             nop
// 0x142831719  4c8b7510                       movq	0x10(%rbp), %r14
// 0x14283171d  4885c0                         testq	%rax, %rax
// 0x142831720  7444                           je	0x142831766 <.text+0x2830766>
// Frames: F325
// 0x142831722  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x142831729  e8720d5efe                     callq	0x140e124a0 <.text+0xe114a0>
// 0x14283172e  90                             nop
// 0x14283172f  4839f8                         cmpq	%rdi, %rax
// 0x142831732  0f87a4020000                   ja	0x1428319dc <.text+0x28309dc>
// Frames: F326
// 0x142831738  4d85e4                         testq	%r12, %r12
// 0x14283173b  747b                           je	0x1428317b8 <.text+0x28307b8>
// Frames: F327
// 0x14283173d  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x142831744  e8570d5efe                     callq	0x140e124a0 <.text+0xe114a0>
// 0x142831749  90                             nop
// Frames: F328 -> F329 -> F330 -> F327
// 0x14283174a  4c39f8                         cmpq	%r15, %rax
// 0x14283174d  0f876f020000                   ja	0x1428319c2 <.text+0x28309c2>
// Frames: F327
// 0x142831753  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x14283175a  4c89e2                         movq	%r12, %rdx
// 0x14283175d  4989c0                         movq	%rax, %r8
// 0x142831760  e84b0d5efe                     callq	0x140e124b0 <.text+0xe114b0>
// 0x142831765  90                             nop
// Frames: F331
// 0x142831766  488b8590000000                 movq	0x90(%rbp), %rax
// 0x14283176d  48894608                       movq	%rax, 0x8(%rsi)
// 0x142831771  48c7060b000000                 movq	$0xb, (%rsi)
// 0x142831778  eb27                           jmp	0x1428317a1 <.text+0x28307a1>
// Frames: F332 -> F314
// 0x14283177a  0f104540                       movups	0x40(%rbp), %xmm0
// 0x14283177e  0f104d50                       movups	0x50(%rbp), %xmm1
// 0x142831782  0f105560                       movups	0x60(%rbp), %xmm2
// 0x142831786  0f115640                       movups	%xmm2, 0x40(%rsi)
// 0x14283178a  0f114e30                       movups	%xmm1, 0x30(%rsi)
// 0x14283178e  0f114620                       movups	%xmm0, 0x20(%rsi)
// Frames: F246 -> F314
// 0x142831792  488906                         movq	%rax, (%rsi)
// 0x142831795  48894e08                       movq	%rcx, 0x8(%rsi)
// 0x142831799  4c896610                       movq	%r12, 0x10(%rsi)
// 0x14283179d  4c897e18                       movq	%r15, 0x18(%rsi)
// Frames: F333
// 0x1428317a1  4889f0                         movq	%rsi, %rax
// 0x1428317a4  4881c428010000                 addq	$0x128, %rsp            # imm = 0x128
// 0x1428317ab  5b                             popq	%rbx
// 0x1428317ac  5f                             popq	%rdi
// 0x1428317ad  5e                             popq	%rsi
// 0x1428317ae  415c                           popq	%r12
// 0x1428317b0  415d                           popq	%r13
// 0x1428317b2  415e                           popq	%r14
// 0x1428317b4  415f                           popq	%r15
// 0x1428317b6  5d                             popq	%rbp
// 0x1428317b7  c3                             retq
// Frames: F334
// 0x1428317b8  4889f9                         movq	%rdi, %rcx
// 0x1428317bb  66ba0100                       movw	$0x1, %dx
// 0x1428317bf  e80c6a5efe                     callq	0x140e181d0 <.text+0xe171d0>
// 0x1428317c4  90                             nop
// Frames: F335
// 0x1428317c5  4989c7                         movq	%rax, %r15
// 0x1428317c8  81ff00000400                   cmpl	$0x40000, %edi          # imm = 0x40000
// 0x1428317ce  0f8781000000                   ja	0x142831855 <.text+0x2830855>
// 0x1428317d4  81e300fd0000                   andl	$0xfd00, %ebx           # imm = 0xFD00
// 0x1428317da  7579                           jne	0x142831855 <.text+0x2830855>
// Frames: F54 -> F336 -> F337
// 0x1428317dc  41803e00                       cmpb	$0x0, (%r14)
// 0x1428317e0  0f842f020000                   je	0x142831a15 <.text+0x2830a15>
// Frames: F338 -> F336 -> F337
// 0x1428317e6  498d5e08                       leaq	0x8(%r14), %rbx
// 0x1428317ea  b101                           movb	$0x1, %cl
// Frames: F339 -> F340 -> F341 -> F342 -> F336 -> F337
// 0x1428317ec  31c0                           xorl	%eax, %eax
// 0x1428317ee  f0                             lock
// 0x1428317ef  410fb04e08                     cmpxchgb	%cl, 0x8(%r14)
// Frames: F343 -> F342 -> F336 -> F337
// 0x1428317f4  0f852a020000                   jne	0x142831a24 <.text+0x2830a24>
// Frames: F344 -> F345 -> F342 -> F336 -> F337
// 0x1428317fa  4889d9                         movq	%rbx, %rcx
// 0x1428317fd  e8cefe65fe                     callq	0x140e916d0 <.text+0xe906d0>
// 0x142831802  90                             nop
// Frames: F346 -> F347 -> F348
// 0x142831803  48895d18                       movq	%rbx, 0x18(%rbp)
// 0x142831807  498d5610                       leaq	0x10(%r14), %rdx
// Frames: F349
// 0x14283180b  48c744242000000000             movq	$0x0, 0x20(%rsp)
// 0x142831814  488d4d20                       leaq	0x20(%rbp), %rcx
// 0x142831818  4d89f0                         movq	%r14, %r8
// 0x14283181b  4989f9                         movq	%rdi, %r9
// 0x14283181e  e8edfc5efe                     callq	0x140e21510 <.text+0xe20510>
// 0x142831823  90                             nop
// Frames: F350 -> F351 -> F352 -> F283 -> F349
// 0x142831824  498d7e08                       leaq	0x8(%r14), %rdi
// 0x142831828  4889f9                         movq	%rdi, %rcx
// 0x14283182b  e8a00066fe                     callq	0x140e918d0 <.text+0xe908d0>
// 0x142831830  90                             nop
// Frames: F274 -> F353 -> F354 -> F352 -> F283 -> F349
// 0x142831831  31c9                           xorl	%ecx, %ecx
// 0x142831833  b001                           movb	$0x1, %al
// 0x142831835  f0                             lock
// 0x142831836  0fb00f                         cmpxchgb	%cl, (%rdi)
// Frames: F355 -> F352 -> F283 -> F349
// 0x142831839  0f85f9010000                   jne	0x142831a38 <.text+0x2830a38>
// Frames: F356
// 0x14283183f  4c8b7d30                       movq	0x30(%rbp), %r15
// 0x142831843  448b6548                       movl	0x48(%rbp), %r12d
// 0x142831847  488b7d20                       movq	0x20(%rbp), %rdi
// 0x14283184b  488b5d28                       movq	0x28(%rbp), %rbx
// Frames: F335
// 0x14283184f  4c897d70                       movq	%r15, 0x70(%rbp)
// Frames: F357 -> F358
// 0x142831853  eb78                           jmp	0x1428318cd <.text+0x28308cd>
// Frames: F359
// 0x142831855  4c89f1                         movq	%r14, %rcx
// Frames: F360
// 0x142831858  e8730062fe                     callq	0x140e518d0 <.text+0xe508d0>
// 0x14283185d  90                             nop
// Frames: F318 -> F319 -> F320 -> F360
// 0x14283185e  4d8b8e60060000                 movq	0x660(%r14), %r9
// 0x142831865  498b8ea8060000                 movq	0x6a8(%r14), %rcx
// Frames: F320 -> F360
// 0x14283186c  4983c110                       addq	$0x10, %r9
// 0x142831870  4883c110                       addq	$0x10, %rcx
// Frames: F360
// 0x142831874  488d15ed7dd500                 leaq	0xd57ded(%rip), %rdx    # 0x143589668
// 0x14283187b  4889542430                     movq	%rdx, 0x30(%rsp)
// 0x142831880  4c897c2428                     movq	%r15, 0x28(%rsp)
// 0x142831885  48894c2420                     movq	%rcx, 0x20(%rsp)
// 0x14283188a  48c744243813000000             movq	$0x13, 0x38(%rsp)
// 0x142831893  488d4d20                       leaq	0x20(%rbp), %rcx
// 0x142831897  4c89ea                         movq	%r13, %rdx
// 0x14283189a  4989c0                         movq	%rax, %r8
// 0x14283189d  e8aee661fe                     callq	0x140e4ff50 <.text+0xe4ef50>
// 0x1428318a2  90                             nop
// Frames: F323 -> F360
// 0x1428318a3  488b4520                       movq	0x20(%rbp), %rax
// 0x1428318a7  4c8b7d28                       movq	0x28(%rbp), %r15
// 0x1428318ab  488b7d30                       movq	0x30(%rbp), %rdi
// 0x1428318af  488b5d38                       movq	0x38(%rbp), %rbx
// 0x1428318b3  4883f80b                       cmpq	$0xb, %rax
// 0x1428318b7  0f85d0000000                   jne	0x14283198d <.text+0x283098d>
// Frames: F360
// 0x1428318bd  4c897d70                       movq	%r15, 0x70(%rbp)
// 0x1428318c1  4531e4                         xorl	%r12d, %r12d
// Frames: F357 -> F360
// 0x1428318c4  4885ff                         testq	%rdi, %rdi
// 0x1428318c7  0f847b010000                   je	0x142831a48 <.text+0x2830a48>
// Frames: F360
// 0x1428318cd  c6859f00000001                 movb	$0x1, 0x9f(%rbp)
// 0x1428318d4  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x1428318db  e8c00b5efe                     callq	0x140e124a0 <.text+0xe114a0>
// 0x1428318e0  90                             nop
// Frames: F328 -> F329 -> F330 -> F360
// 0x1428318e1  4839d8                         cmpq	%rbx, %rax
// 0x1428318e4  0f870d010000                   ja	0x1428319f7 <.text+0x28309f7>
// Frames: F360
// 0x1428318ea  c6859f00000001                 movb	$0x1, 0x9f(%rbp)
// 0x1428318f1  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x1428318f8  4889fa                         movq	%rdi, %rdx
// 0x1428318fb  4989c0                         movq	%rax, %r8
// 0x1428318fe  e8ad0b5efe                     callq	0x140e124b0 <.text+0xe114b0>
// 0x142831903  90                             nop
// Frames: F361 -> F362 -> F363 -> F364 -> F365 -> F366 -> F367 -> F360
// 0x142831904  488b8d90000000                 movq	0x90(%rbp), %rcx
// 0x14283190b  89c8                           movl	%ecx, %eax
// 0x14283190d  83e03f                         andl	$0x3f, %eax
// Frames: F368 -> F369 -> F370 -> F363 -> F364 -> F365 -> F366 -> F367 -> F360
// 0x142831910  4883e1c0                       andq	$-0x40, %rcx
// Frames: F360
// 0x142831914  488b84c110010000               movq	0x110(%rcx,%rax,8), %rax
// 0x14283191c  4c897520                       movq	%r14, 0x20(%rbp)
// 0x142831920  4c897d28                       movq	%r15, 0x28(%rbp)
// 0x142831924  48894530                       movq	%rax, 0x30(%rbp)
// 0x142831928  44896548                       movl	%r12d, 0x48(%rbp)
// 0x14283192c  488b8518010000                 movq	0x118(%rbp), %rax
// 0x142831933  48894538                       movq	%rax, 0x38(%rbp)
// 0x142831937  488b8520010000                 movq	0x120(%rbp), %rax
// 0x14283193e  48894540                       movq	%rax, 0x40(%rbp)
// 0x142831942  c6859f00000000                 movb	$0x0, 0x9f(%rbp)
// 0x142831949  488d4dc0                       leaq	-0x40(%rbp), %rcx
// 0x14283194d  4c8d4520                       leaq	0x20(%rbp), %r8
// 0x142831951  4c89f2                         movq	%r14, %rdx
// 0x142831954  e8170462fe                     callq	0x140e51d70 <.text+0xe50d70>
// 0x142831959  90                             nop
// Frames: F323 -> F360
// 0x14283195a  837dc00b                       cmpl	$0xb, -0x40(%rbp)
// 0x14283195e  0f8402feffff                   je	0x142831766 <.text+0x2830766>
// Frames: F332 -> F360
// 0x142831964  0f104500                       movups	(%rbp), %xmm0
// 0x142831968  0f114640                       movups	%xmm0, 0x40(%rsi)
// 0x14283196c  0f1045c0                       movups	-0x40(%rbp), %xmm0
// 0x142831970  0f104dd0                       movups	-0x30(%rbp), %xmm1
// 0x142831974  0f1055e0                       movups	-0x20(%rbp), %xmm2
// 0x142831978  0f105df0                       movups	-0x10(%rbp), %xmm3
// 0x14283197c  0f115e30                       movups	%xmm3, 0x30(%rsi)
// 0x142831980  0f115620                       movups	%xmm2, 0x20(%rsi)
// 0x142831984  0f114e10                       movups	%xmm1, 0x10(%rsi)
// 0x142831988  0f1106                         movups	%xmm0, (%rsi)
// 0x14283198b  eb27                           jmp	0x1428319b4 <.text+0x28309b4>
// 0x14283198d  0f104540                       movups	0x40(%rbp), %xmm0
// 0x142831991  0f104d50                       movups	0x50(%rbp), %xmm1
// 0x142831995  0f105560                       movups	0x60(%rbp), %xmm2
// 0x142831999  0f115640                       movups	%xmm2, 0x40(%rsi)
// 0x14283199d  0f114e30                       movups	%xmm1, 0x30(%rsi)
// 0x1428319a1  0f114620                       movups	%xmm0, 0x20(%rsi)
// Frames: F246 -> F360
// 0x1428319a5  488906                         movq	%rax, (%rsi)
// 0x1428319a8  4c897e08                       movq	%r15, 0x8(%rsi)
// 0x1428319ac  48897e10                       movq	%rdi, 0x10(%rsi)
// 0x1428319b0  48895e18                       movq	%rbx, 0x18(%rsi)
// Frames: F360
// 0x1428319b4  488d4d78                       leaq	0x78(%rbp), %rcx
// 0x1428319b8  e8f3e678fe                     callq	0x140fc00b0 <.text+0xfbf0b0>
// 0x1428319bd  e9dffdffff                     jmp	0x1428317a1 <.text+0x28307a1>
// Frames: F371 -> F330 -> F360
// 0x1428319c2  4c8d0d877cd500                 leaq	0xd57c87(%rip), %r9     # 0x143589650
// Frames: F371 -> F330 -> F327
// 0x1428319c9  31c9                           xorl	%ecx, %ecx
// 0x1428319cb  4889c2                         movq	%rax, %rdx
// 0x1428319ce  4d89f8                         movq	%r15, %r8
// 0x1428319d1  e81aff5700                     callq	0x142db18f0 <.text+0x2db08f0>
// 0x1428319d6  90                             nop
// 0x1428319d7  e98c000000                     jmp	0x142831a68 <.text+0x2830a68>
// Frames: F325
// 0x1428319dc  488d0dc17bd500                 leaq	0xd57bc1(%rip), %rcx    # 0x1435895a4
// 0x1428319e3  4c8d054e7cd500                 leaq	0xd57c4e(%rip), %r8     # 0x143589638
// 0x1428319ea  ba41000000                     movl	$0x41, %edx
// 0x1428319ef  e88cfe5700                     callq	0x142db1880 <.text+0x2db0880>
// 0x1428319f4  90                             nop
// 0x1428319f5  eb71                           jmp	0x142831a68 <.text+0x2830a68>
// Frames: F371 -> F330 -> F372
// 0x1428319f7  c6859f00000001                 movb	$0x1, 0x9f(%rbp)
// 0x1428319fe  4c8d0d137dd500                 leaq	0xd57d13(%rip), %r9     # 0x143589718
// 0x142831a05  31c9                           xorl	%ecx, %ecx
// 0x142831a07  4889c2                         movq	%rax, %rdx
// 0x142831a0a  4989d8                         movq	%rbx, %r8
// 0x142831a0d  e8defe5700                     callq	0x142db18f0 <.text+0x2db08f0>
// 0x142831a12  90                             nop
// 0x142831a13  eb53                           jmp	0x142831a68 <.text+0x2830a68>
// Frames: F373 -> F336 -> F337
// 0x142831a15  488d0de47cd500                 leaq	0xd57ce4(%rip), %rcx    # 0x143589700
// 0x142831a1c  e8fffb5700                     callq	0x142db1620 <.text+0x2db0620>
// 0x142831a21  90                             nop
// 0x142831a22  eb44                           jmp	0x142831a68 <.text+0x2830a68>
// Frames: F374 -> F342 -> F336 -> F337
// 0x142831a24  4889d9                         movq	%rbx, %rcx
// 0x142831a27  41b800ca9a3b                   movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
// 0x142831a2d  e80e4d5700                     callq	0x142da6740 <.text+0x2da5740>
// 0x142831a32  90                             nop
// 0x142831a33  e9c2fdffff                     jmp	0x1428317fa <.text+0x28307fa>
// Frames: F375 -> F352 -> F283 -> F349
// 0x142831a38  4889f9                         movq	%rdi, %rcx
// 0x142831a3b  31d2                           xorl	%edx, %edx
// 0x142831a3d  e84e4a5700                     callq	0x142da6490 <.text+0x2da5490>
// 0x142831a42  90                             nop
// 0x142831a43  e9f7fdffff                     jmp	0x14283183f <.text+0x283083f>
// Frames: F376 -> F358
// 0x142831a48  c6859f00000001                 movb	$0x1, 0x9f(%rbp)
// 0x142831a4f  488d0d257cd500                 leaq	0xd57c25(%rip), %rcx    # 0x14358967b
// 0x142831a56  4c8d05337cd500                 leaq	0xd57c33(%rip), %r8     # 0x143589690
// 0x142831a5d  ba14000000                     movl	$0x14, %edx
// 0x142831a62  e879fb5700                     callq	0x142db15e0 <.text+0x2db05e0>
// 0x142831a67  90                             nop
// 0x142831a68  0f0b                           ud2
// 0x142831a6a  660f1f440000                   nopw	(%rax,%rax)
// 0x142831a70  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x142831a75  55                             pushq	%rbp
// 0x142831a76  4157                           pushq	%r15
// 0x142831a78  4156                           pushq	%r14
// 0x142831a7a  4155                           pushq	%r13
// 0x142831a7c  4154                           pushq	%r12
// 0x142831a7e  56                             pushq	%rsi
// 0x142831a7f  57                             pushq	%rdi
// 0x142831a80  53                             pushq	%rbx
// 0x142831a81  4883ec48                       subq	$0x48, %rsp
// 0x142831a85  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x142831a8c  488d4d78                       leaq	0x78(%rbp), %rcx
// Frames: F333
// 0x142831a90  e81be678fe                     callq	0x140fc00b0 <.text+0xfbf0b0>
// 0x142831a95  90                             nop
// 0x142831a96  4883c448                       addq	$0x48, %rsp
// 0x142831a9a  5b                             popq	%rbx
// 0x142831a9b  5f                             popq	%rdi
// 0x142831a9c  5e                             popq	%rsi
// 0x142831a9d  415c                           popq	%r12
// 0x142831a9f  415d                           popq	%r13
// 0x142831aa1  415e                           popq	%r14
// 0x142831aa3  415f                           popq	%r15
// 0x142831aa5  5d                             popq	%rbp
// 0x142831aa6  c3                             retq
// 0x142831aa7  660f1f840000000000             nopw	(%rax,%rax)
// 0x142831ab0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x142831ab5  55                             pushq	%rbp
// 0x142831ab6  4157                           pushq	%r15
// 0x142831ab8  4156                           pushq	%r14
// 0x142831aba  4155                           pushq	%r13
// 0x142831abc  4154                           pushq	%r12
// 0x142831abe  56                             pushq	%rsi
// 0x142831abf  57                             pushq	%rdi
// 0x142831ac0  53                             pushq	%rbx
// 0x142831ac1  4883ec48                       subq	$0x48, %rsp
// 0x142831ac5  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x142831acc  488b4d18                       movq	0x18(%rbp), %rcx
// Frames: F349
// 0x142831ad0  e81bff85fd                     callq	0x1400919f0 <.text+0x909f0>
// 0x142831ad5  90                             nop
// 0x142831ad6  4883c448                       addq	$0x48, %rsp
// 0x142831ada  5b                             popq	%rbx
// 0x142831adb  5f                             popq	%rdi
// 0x142831adc  5e                             popq	%rsi
// 0x142831add  415c                           popq	%r12
// 0x142831adf  415d                           popq	%r13
// 0x142831ae1  415e                           popq	%r14
// 0x142831ae3  415f                           popq	%r15
// 0x142831ae5  5d                             popq	%rbp
// 0x142831ae6  c3                             retq
// 0x142831ae7  660f1f840000000000             nopw	(%rax,%rax)
// 0x142831af0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x142831af5  55                             pushq	%rbp
// 0x142831af6  4157                           pushq	%r15
// 0x142831af8  4156                           pushq	%r14
// 0x142831afa  4155                           pushq	%r13
// 0x142831afc  4154                           pushq	%r12
// 0x142831afe  56                             pushq	%rsi
// 0x142831aff  57                             pushq	%rdi
// 0x142831b00  53                             pushq	%rbx
// 0x142831b01  4883ec48                       subq	$0x48, %rsp
// 0x142831b05  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F283 -> F377
// 0x142831b0c  488d4d30                       leaq	0x30(%rbp), %rcx
// 0x142831b10  e89be578fe                     callq	0x140fc00b0 <.text+0xfbf0b0>
// 0x142831b15  90                             nop
// Frames: F377
// 0x142831b16  4883c448                       addq	$0x48, %rsp
// 0x142831b1a  5b                             popq	%rbx
// 0x142831b1b  5f                             popq	%rdi
// 0x142831b1c  5e                             popq	%rsi
// 0x142831b1d  415c                           popq	%r12
// 0x142831b1f  415d                           popq	%r13
// 0x142831b21  415e                           popq	%r14
// 0x142831b23  415f                           popq	%r15
// 0x142831b25  5d                             popq	%rbp
// 0x142831b26  c3                             retq
// 0x142831b27  660f1f840000000000             nopw	(%rax,%rax)
// 0x142831b30  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x142831b35  55                             pushq	%rbp
// 0x142831b36  4157                           pushq	%r15
// 0x142831b38  4156                           pushq	%r14
// 0x142831b3a  4155                           pushq	%r13
// 0x142831b3c  4154                           pushq	%r12
// 0x142831b3e  56                             pushq	%rsi
// 0x142831b3f  57                             pushq	%rdi
// 0x142831b40  53                             pushq	%rbx
// 0x142831b41  4883ec48                       subq	$0x48, %rsp
// 0x142831b45  488daa80000000                 leaq	0x80(%rdx), %rbp
// Frames: F378
// 0x142831b4c  80bd9f00000000                 cmpb	$0x0, 0x9f(%rbp)
// 0x142831b53  741b                           je	0x142831b70 <.text+0x2830b70>
// 0x142831b55  488d4d70                       leaq	0x70(%rbp), %rcx
// 0x142831b59  e852e578fe                     callq	0x140fc00b0 <.text+0xfbf0b0>
// 0x142831b5e  90                             nop
// 0x142831b5f  4883c448                       addq	$0x48, %rsp
// 0x142831b63  5b                             popq	%rbx
// 0x142831b64  5f                             popq	%rdi
// 0x142831b65  5e                             popq	%rsi
// 0x142831b66  415c                           popq	%r12
// 0x142831b68  415d                           popq	%r13
// 0x142831b6a  415e                           popq	%r14
// 0x142831b6c  415f                           popq	%r15
// 0x142831b6e  5d                             popq	%rbp
// 0x142831b6f  c3                             retq
// 0x142831b70  4883c448                       addq	$0x48, %rsp
// 0x142831b74  5b                             popq	%rbx
// 0x142831b75  5f                             popq	%rdi
// 0x142831b76  5e                             popq	%rsi
// 0x142831b77  415c                           popq	%r12
// 0x142831b79  415d                           popq	%r13
// 0x142831b7b  415e                           popq	%r14
// 0x142831b7d  415f                           popq	%r15
// 0x142831b7f  5d                             popq	%rbp
// 0x142831b80  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/lib.rs:61:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs:1309:0 hashbrown::map::HashMap::get
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs:942:0 bevy_platform::collections::hash_map::HashMap::get
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs:591:0 bevy_ecs::component::info::Components::get_valid_resource_id
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs:1841:0 bevy_ecs::world::World::contains_resource
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs:352:0 bevy_app::sub_app::SubApp::add_message
// F7: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs:401:0 bevy_app::app::App::add_message
// F8: crates/systems/roof/src/lib.rs:62:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F9: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:2492:0 hashbrown::raw::RawTableInner::probe_seq
// F10: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:2051:0 hashbrown::raw::RawTableInner::find_inner
// F11: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:1236:0 hashbrown::raw::RawTable::find
// F12: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:1251:0 hashbrown::raw::RawTable::get
// F13: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs:1311:0 hashbrown::map::HashMap::get
// F14: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:547:0 core::ptr::copy_nonoverlapping
// F15: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1335:0 core::core_arch::x86::sse2::_mm_loadu_si128
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs:52:0 hashbrown::control::group::sse2::Group::load
// F17: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:2068:0 hashbrown::raw::RawTableInner::find_inner
// F18: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1563:0 core::core_arch::x86::sse2::_mm_movemask_epi8
// F19: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs:87:0 hashbrown::control::group::sse2::Group::match_tag
// F20: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:2070:0 hashbrown::raw::RawTableInner::find_inner
// F21: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs:50:0 hashbrown::control::bitmask::BitMask::lowest_set_bit
// F22: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs:113:0 hashbrown::control::bitmask::impl$2::next
// F23: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/nonzero.rs:641:0 core::num::nonzero::NonZero::trailing_zeros
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs:80:0 hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
// F25: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs:51:0 hashbrown::control::bitmask::BitMask::lowest_set_bit
// F26: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:2073:0 hashbrown::raw::RawTableInner::find_inner
// F27: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:1072:0 core::ptr::mut_ptr::impl$0::sub
// F28: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:339:0 hashbrown::raw::Bucket::from_base_index
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:812:0 hashbrown::raw::RawTable::bucket
// F30: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:1236:0 hashbrown::raw::impl$8::find::closure$0
// F31: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:2075:0 hashbrown::raw::RawTableInner::find_inner
// F32: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/any.rs:764:0 core::any::impl$9::eq::runtime
// F33: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics/mod.rs:2447:0 core::any::impl$9::eq
// F34: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs:89:0 equivalent::impl$0::equivalent
// F35: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs:224:0 hashbrown::map::equivalent_key::closure$0
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs:13:0 hashbrown::util::likely
// F37: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/nonzero.rs:499:0 core::num::nonzero::NonZero::get
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs:95:0 hashbrown::control::group::sse2::Group::match_empty
// F39: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:2080:0 hashbrown::raw::RawTableInner::find_inner
// F40: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs:13:0 hashbrown::control::group::sse2::Group::match_empty
// F41: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:91:0 hashbrown::raw::ProbeSeq::move_next
// F42: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:2084:0 hashbrown::raw::RawTableInner::find_inner
// F43: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:90:0 hashbrown::raw::ProbeSeq::move_next
// F44: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs:2053:0 hashbrown::raw::RawTableInner::find_inner
// F45: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:2126:0 bevy_platform::collections::hash_map::HashMap::get
// F46: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:219:0 core::slice::index::impl$2::get
// F47: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 core::slice::impl$0::get
// F48: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::SparseArray::get
// F49: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:406:0 bevy_ecs::storage::sparse_set::SparseSet::get
// F50: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs:333:0 bevy_ecs::storage::resource::Resources::get
// F51: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs:1842:0 bevy_ecs::world::impl$2::contains_resource::closure$0
// F52: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1546:0 core::option::Option::and_then
// F53: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs:1842:0 bevy_ecs::world::World::contains_resource
// F54: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:745:0 core::option::Option::as_ref
// F55: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F56: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1546:0 core::slice::impl$0::get
// F57: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:612:0 alloc::raw_vec::RawVecInner::non_null
// F58: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:607:0 alloc::raw_vec::RawVecInner::ptr
// F59: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:295:0 alloc::raw_vec::RawVec::ptr
// F60: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1916:0 alloc::vec::Vec::as_ptr
// F61: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1815:0 alloc::vec::Vec::as_slice
// F62: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3729:0 alloc::vec::impl$9::deref
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:408:0 bevy_ecs::storage::sparse_set::impl$15::get::closure$0
// F64: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:0 core::option::Option::map
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 alloc::vec::impl$9::deref
// F66: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:253:0 core::slice::index::impl$2::get_unchecked
// F67: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 alloc::vec::impl$9::deref
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs:88:0 bevy_ecs::storage::blob_vec::BlobVec::is_empty
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs:89:0 bevy_ecs::storage::resource::ResourceData::is_present
// F70: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:664:0 core::option::Option::is_some_and
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs:1843:0 bevy_ecs::world::World::contains_resource
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs:353:0 bevy_app::sub_app::SubApp::add_message
// F73: crates/systems/roof/src/lib.rs:63:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F74: crates/systems/roof/src/lib.rs:64:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F75: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:93:0 alloc::alloc::alloc
// F76: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:190:0 alloc::alloc::Global::alloc_impl_runtime
// F77: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:312:0 alloc::alloc::Global::alloc_impl
// F78: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:429:0 alloc::alloc::impl$1::allocate
// F79: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:248:0 alloc::boxed::box_new_uninit
// F80: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:286:0 alloc::boxed::impl$0::new
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs:566:0 bevy_ecs::schedule::config::impl$4::into_configs
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs:449:0 bevy_ecs::schedule::config::IntoScheduleConfigs::run_if
// F83: crates/systems/roof/src/lib.rs:71:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F84: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:95:0 alloc::alloc::alloc
// F85: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:289:0 alloc::boxed::impl$0::new
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs:567:0 bevy_ecs::schedule::config::impl$4::into_configs
// F87: crates/systems/roof/src/lib.rs:73:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F88: crates/systems/roof/src/lib.rs:77:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F89: crates/systems/roof/src/lib.rs:80:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F90: crates/systems/roof/src/lib.rs:74:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F91: crates/systems/roof/src/lib.rs:85:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F92: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/label.rs:121:0 bevy_ecs::schedule::set::SystemSet::intern
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs:499:0 bevy_ecs::schedule::config::impl$3::in_set
// F94: crates/systems/roof/src/lib.rs:86:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F95: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs:501:0 bevy_ecs::schedule::config::impl$3::in_set
// F96: crates/systems/roof/src/lib.rs:96:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F97: crates/systems/roof/src/lib.rs:89:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F98: crates/systems/roof/src/lib.rs:102:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F99: crates/systems/roof/src/lib.rs:87:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F100: crates/systems/roof/src/lib.rs:108:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F101: crates/systems/roof/src/lib.rs:109:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F102: crates/systems/roof/src/lib.rs:116:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F103: crates/systems/roof/src/lib.rs:121:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F104: crates/systems/roof/src/lib.rs:122:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F105: crates/systems/roof/src/lib.rs:123:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F106: crates/systems/roof/src/lib.rs:124:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F107: crates/systems/roof/src/lib.rs:125:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F108: crates/systems/roof/src/lib.rs:67:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs:323:0 bevy_app::app::App::add_systems
// F110: crates/systems/roof/src/lib.rs:65:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F111: crates/systems/roof/src/lib.rs:128:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F112: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:250:0 alloc::boxed::box_new_uninit
// F113: crates/systems/roof/src/lib.rs:126:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F114: crates/systems/roof/src/lib.rs:84:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h9b99d94f090f9457
// F115: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs:502:0 bevy_ecs::schedule::config::impl$3::in_set
// F116: crates/systems/roof/src/lib.rs:130:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F117: crates/systems/roof/src/lib.rs:131:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F118: crates/systems/roof/src/lib.rs:132:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F119: crates/systems/roof/src/lib.rs:134:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F120: crates/systems/roof/src/lib.rs:135:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F121: crates/systems/roof/src/lib.rs:136:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F122: crates/systems/roof/src/lib.rs:138:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F123: crates/systems/roof/src/lib.rs:139:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F124: crates/systems/roof/src/lib.rs:140:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F125: crates/systems/roof/src/lib.rs:141:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F126: crates/systems/roof/src/lib.rs:142:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F127: crates/systems/roof/src/lib.rs:143:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F128: crates/systems/roof/src/lib.rs:144:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F129: crates/systems/roof/src/lib.rs:145:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F130: crates/systems/roof/src/lib.rs:146:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h72d3efe0680905fe
// F131: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:680:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h0e1a01db4faa1222 (.llvm.9631438182730803438)
// F132: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3923:0 core::sync::atomic::atomic_add
// F133: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3136:0 core::sync::atomic::AtomicU32::fetch_add
// F134: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:341:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::increment_change_tick
// F135: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:688:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h0e1a01db4faa1222 (.llvm.9631438182730803438)
// F136: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:767:0 core::option::Option::as_mut
// F137: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:690:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h0e1a01db4faa1222 (.llvm.9631438182730803438)
// F138: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs:184:0 bevy_ecs::world::World::id
// F139: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:251:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::id
// F140: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:691:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h0e1a01db4faa1222 (.llvm.9631438182730803438)
// F141: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/identifier.rs:12:0 bevy_ecs::world::identifier::impl$10::eq
// F142: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:697:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h0e1a01db4faa1222 (.llvm.9631438182730803438)
// F143: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:655:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
// F144: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:811:0 bevy_ecs::system::system_param::impl$7::get_param
// F145: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:2171:0 bevy_ecs::system::system_param::impl$106::get_param
// F146: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs:132:0 bevy_ecs::storage::resource::ResourceData::get_with_ticks
// F147: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:656:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
// F148: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/bool.rs:66:0 bevy_ecs::storage::resource::ResourceData::is_present
// F149: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:549:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F150: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:889:0 bevy_ecs::system::system_param::impl$8::get_param
// F151: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F152: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 nonmax::NonMaxUsize::get
// F153: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:550:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F154: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs:350:0 bevy_ecs::storage::blob_vec::BlobVec::get_ptr
// F155: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs:326:0 bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
// F156: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs:136:0 bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
// F157: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3891:0 core::sync::atomic::atomic_load
// F158: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:2844:0 core::sync::atomic::AtomicU32::load
// F159: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs:2869:0 bevy_ecs::world::World::read_change_tick
// F160: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:305:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
// F161: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:556:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F162: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F163: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F164: crates/systems/roof/src/lib.rs:153:0 system_roof::gpu_flush_buffers
// F165: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:166:0 core::ops::function::FnMut::call_mut
// F166: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:298:0 core::ops::function::impls::impl$3::call_mut
// F167: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:926:0 bevy_ecs::system::function_system::impl$36::run::call_inner
// F168: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:929:0 bevy_ecs::system::function_system::impl$36::run
// F169: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:711:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h0e1a01db4faa1222 (.llvm.9631438182730803438)
// F170: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3890:0 core::sync::atomic::atomic_load
// F171: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:729:0 core::sync::atomic::AtomicBool::load
// F172: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/poison.rs:141:0 std::sync::poison::Flag::get
// F173: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/poison.rs:113:0 std::sync::poison::Flag::borrow
// F174: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/poison/rwlock.rs:680:0 std::sync::poison::rwlock::RwLock::get_mut
// F175: crates/frozen/src/lib.rs:115:0 frozen::FrozenMap::as_mut
// F176: crates/country-core/src/asset_libraries/ssbo_library.rs:256:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut_dyn
// F177: crates/country-core/src/asset_libraries/ssbo_library.rs:297:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut
// F178: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:2469:0 core::cell::UnsafeCell::get_mut
// F179: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/poison/rwlock.rs:679:0 std::sync::poison::rwlock::RwLock::get_mut
// F180: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1231:0 core::result::Result::unwrap
// F181: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:1264:0 std::collections::hash::map::HashMap::get_mut
// F182: crates/country-core/src/asset_libraries/ssbo_library.rs:258:0 country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut_dyn
// F183: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:0 core::option::Option::unwrap
// F184: crates/systems/roof/src/lib.rs:154:0 system_roof::gpu_flush_buffers
// F185: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:713:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h0e1a01db4faa1222 (.llvm.9631438182730803438)
// F186: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:617:0 bevy_ecs::system::function_system::impl$6::into_result
// F187: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:714:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h0e1a01db4faa1222 (.llvm.9631438182730803438)
// F188: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:715:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::h0e1a01db4faa1222 (.llvm.9631438182730803438)
// F189: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:971:0 core::option::Option::as_mut
// F190: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1233:0 core::result::Result::unwrap
// F191: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:284:0 core::ptr::non_null::NonNull::from_ref
// F192: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/rt.rs:48:0 core::fmt::rt::Argument::new_display
// F193: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:813:0 bevy_ecs::system::system_param::impl$7::get_param::closure$0
// F194: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1067:0 core::option::Option::unwrap_or_else
// F195: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:812:0 bevy_ecs::system::system_param::impl$7::get_param
// F196: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:891:0 bevy_ecs::system::system_param::impl$8::get_param::closure$0
// F197: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:890:0 bevy_ecs::system::system_param::impl$8::get_param
// F198: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1016:0 core::option::Option::unwrap
// F199: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1544:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F200: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:2494:0 core::num::impl$11::wrapping_sub
// F201: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1556:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F202: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 core::num::impl$11::wrapping_sub
// F203: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1569:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F204: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1578:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F205: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1583:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F206: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/range.rs:814:0 core::iter::range::impl$5::spec_next_back
// F207: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/range.rs:979:0 core::iter::range::impl$7::next_back
// F208: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/rev.rs:53:0 core::iter::adapters::rev::impl$1::next
// F209: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:1916:0 core::cmp::impls::impl$58::lt
// F210: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1584:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F211: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1585:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F212: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1570:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F213: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1571:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F214: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1594:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F215: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1595:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F216: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/range.rs:772:0 core::iter::range::impl$5::spec_next
// F217: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/range.rs:857:0 core::iter::range::impl$6::next
// F218: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1596:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F219: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:844:0 core::num::impl$11::unchecked_add
// F220: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/range.rs:213:0 core::iter::range::impl$43::forward_unchecked
// F221: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/range.rs:775:0 core::iter::range::impl$5::spec_next
// F222: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1597:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F223: crates/systems/roof/src/lib.rs:1:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F224: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1741:0 core::str::pattern::impl$36::matching
// F225: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1613:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F226: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1615:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F227: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system.rs:127:0 bevy_ecs::system::system::System::run_without_applying_deferred::ha01d43df8641bc3d
// F228: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:744:0 bevy_ecs::system::function_system::impl$12::validate_param_unsafe
// F229: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system.rs:135:0 bevy_ecs::system::system::System::run_without_applying_deferred::ha01d43df8641bc3d
// F230: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:745:0 bevy_ecs::system::function_system::impl$12::validate_param_unsafe
// F231: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:750:0 bevy_ecs::system::function_system::impl$12::validate_param_unsafe
// F232: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:791:0 bevy_ecs::system::system_param::impl$7::validate_param
// F233: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:2154:0 bevy_ecs::system::system_param::impl$106::validate_param
// F234: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:792:0 bevy_ecs::system::system_param::impl$7::validate_param
// F235: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:789:0 bevy_ecs::system::system_param::impl$7::validate_param
// F236: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:870:0 bevy_ecs::system::system_param::impl$8::validate_param
// F237: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:871:0 bevy_ecs::system::system_param::impl$8::validate_param
// F238: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:868:0 bevy_ecs::system::system_param::impl$8::validate_param
// F239: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:688:0 bevy_ecs::system::function_system::impl$12::run_unsafe
// F240: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system.rs:139:0 bevy_ecs::system::system::System::run_without_applying_deferred::ha01d43df8641bc3d
// F241: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/bool.rs:66:0 core::bool::impl$0::then
// F242: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:697:0 bevy_ecs::system::function_system::impl$12::run_unsafe
// F243: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:711:0 bevy_ecs::system::function_system::impl$12::run_unsafe
// F244: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:713:0 bevy_ecs::system::function_system::impl$12::run_unsafe
// F245: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system.rs:452:0 bevy_ecs::system::system::impl$3::from
// F246: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:2189:0 core::result::impl$28::from_residual
// F247: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system.rs:140:0 bevy_ecs::system::system::System::run_without_applying_deferred::ha01d43df8641bc3d
// F248: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1608:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F249: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1609:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F250: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1610:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F251: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs:1559:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F252: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:215:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F253: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3892:0 core::sync::atomic::atomic_load
// F254: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:2844:0 core::sync::atomic::AtomicUsize::load
// F255: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:222:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F256: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:224:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F257: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:226:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F258: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:302:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F259: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:239:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F260: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:240:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F261: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs:201:0 arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure$0
// F262: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:258:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F263: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs:187:0 arc_swap::strategy::hybrid::impl$5::load
// F264: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/mem/mod.rs:902:0 core::mem::replace
// F265: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1886:0 core::option::Option::take
// F266: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs:138:0 arc_swap::strategy::hybrid::impl$2::into_inner
// F267: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:2262:0 core::option::impl$6::clone
// F268: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs:74:0 arc_swap::ref_cnt::RefCnt::inc
// F269: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs:141:0 arc_swap::strategy::hybrid::impl$2::into_inner
// F270: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3136:0 core::sync::atomic::AtomicUsize::fetch_add
// F271: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2388:0 alloc::sync::impl$32::clone
// F272: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:2263:0 core::option::impl$6::clone
// F273: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2405:0 alloc::sync::impl$32::clone
// F274: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3984:0 core::sync::atomic::atomic_compare_exchange
// F275: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3039:0 core::sync::atomic::AtomicUsize::compare_exchange
// F276: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs:76:0 arc_swap::debt::Debt::pay
// F277: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs:142:0 arc_swap::strategy::hybrid::impl$2::into_inner
// F278: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs:87:0 arc_swap::ref_cnt::RefCnt::dec
// F279: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs:143:0 arc_swap::strategy::hybrid::impl$2::into_inner
// F280: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3942:0 core::sync::atomic::atomic_sub
// F281: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3167:0 core::sync::atomic::AtomicUsize::fetch_sub
// F282: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2811:0 alloc::sync::impl$42::drop
// F283: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:805:0 core::ptr::drop_in_place
// F284: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/mem/mod.rs:975:0 core::mem::drop
// F285: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2854:0 alloc::sync::impl$42::drop
// F286: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:242:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F287: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:243:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F288: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:250:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F289: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1717:0 core::ptr::read
// F290: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs:150:0 arc_swap::strategy::hybrid::impl$2::into_inner
// F291: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs:167:0 arc_swap::ref_cnt::impl$2::as_ptr
// F292: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:259:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F293: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:1702:0 core::sync::atomic::AtomicPtr::load
// F294: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:263:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F295: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:265:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F296: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3877:0 core::sync::atomic::atomic_store
// F297: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:2872:0 core::sync::atomic::AtomicUsize::store
// F298: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:269:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F299: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:273:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F300: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:280:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F301: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:4008:0 core::sync::atomic::atomic_compare_exchange
// F302: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:278:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F303: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:295:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F304: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:298:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F305: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:4373:0 core::sync::atomic::fence
// F306: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:64:0 alloc::sync::impl$42::drop
// F307: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:299:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F308: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:1733:0 core::sync::atomic::AtomicPtr::store
// F309: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/helping.rs:285:0 arc_swap::debt::helping::Slots::help::hf24530cc0f815767
// F310: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2406:0 alloc::sync::impl$32::clone
// F311: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs:153:0 arc_swap::strategy::hybrid::impl$2::into_inner
// F312: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs:135:0 arc_swap::strategy::hybrid::impl$2::into_inner
// F313: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:529:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F314: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:539:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F315: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:535:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F316: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:540:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F317: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:541:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F318: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:444:0 core::ptr::non_null::NonNull::as_ref
// F319: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2104:0 alloc::sync::Arc::inner
// F320: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2422:0 alloc::sync::impl$34::deref
// F321: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:542:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F322: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:543:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F323: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:2173:0 core::result::impl$27::branch
// F324: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:548:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F325: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:549:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F326: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:552:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F327: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:553:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F328: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:923:0 core::num::impl$11::checked_sub
// F329: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:450:0 core::slice::index::impl$4::index_mut
// F330: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:31:0 core::slice::index::impl$1::index_mut
// F331: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:613:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F332: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:2175:0 core::result::impl$27::branch
// F333: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:614:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F334: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:555:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F335: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:560:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F336: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/device.rs:920:0 rhapsody::raw::device::GraphicsDevice::dynamic_constants
// F337: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:572:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F338: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:746:0 core::option::Option::as_ref
// F339: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:4040:0 core::sync::atomic::atomic_compare_exchange_weak
// F340: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3104:0 core::sync::atomic::AtomicU8::compare_exchange_weak
// F341: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:69:0 parking_lot::raw_mutex::impl$0::lock
// F342: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs:213:0 core::option::Option::as_ref
// F343: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:67:0 parking_lot::raw_mutex::impl$0::lock
// F344: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs:1114:0 parking_lot_core::parking_lot::deadlock::acquire_resource
// F345: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:74:0 parking_lot::raw_mutex::impl$0::lock
// F346: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:2447:0 core::cell::UnsafeCell::get
// F347: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs:697:0 lock_api::mutex::impl$14::deref_mut
// F348: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:571:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F349: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:573:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F350: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs:1130:0 parking_lot_core::parking_lot::deadlock::release_resource
// F351: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:101:0 parking_lot::raw_mutex::impl$0::unlock
// F352: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs:706:0 lock_api::mutex::impl$15::drop
// F353: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3039:0 core::sync::atomic::AtomicU8::compare_exchange
// F354: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:104:0 parking_lot::raw_mutex::impl$0::unlock
// F355: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:102:0 parking_lot::raw_mutex::impl$0::unlock
// F356: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:574:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F357: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:0 core::option::Option::expect
// F358: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:588:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F359: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:578:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F360: crates/systems/roof/src/lib.rs:1:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F361: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/int_macros.rs:2137:0 core::num::impl$5::wrapping_sub
// F362: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:201:0 core::ptr::mut_ptr::impl$0::with_addr
// F363: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:363:0 core::ptr::non_null::NonNull::with_addr
// F364: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:376:0 core::ptr::non_null::NonNull::map_addr
// F365: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs:475:0 rhapsody::raw::respool::TaggedSlot::chunk_and_index
// F366: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs:861:0 rhapsody::raw::respool::Handle::hot
// F367: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs:147:0 rhapsody::raw::buffer::Buffer::hot
// F368: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:470:0 core::ptr::mut_ptr::impl$0::wrapping_offset
// F369: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:488:0 core::ptr::mut_ptr::impl$0::wrapping_byte_offset
// F370: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:202:0 core::ptr::mut_ptr::impl$0::with_addr
// F371: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:456:0 core::slice::index::impl$4::index_mut
// F372: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:589:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F373: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1016:0 core::option::Option::as_ref
// F374: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:72:0 parking_lot::raw_mutex::impl$0::lock
// F375: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs:109:0 parking_lot::raw_mutex::impl$0::unlock
// F376: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:971:0 core::option::Option::expect
// F377: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:575:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// F378: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs:610:0 rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::he563ef9961b4976a
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 4aef09c7bb446a57383fbe0017658fd5b794ae9091f2e51e235900415b6f2f39
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 269, 270, 949, 956, 965, 2425, 2426, 2435, 2437, 2438, 2444, 2449, 2451, 3750, 3756, 3788, 4986
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arc_swap; // namespace candidate
//   package identities: [{"name":"arc-swap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.7.1"}]
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use frozen; // namespace candidate
//   package identities: [{"name":"frozen","source":"local_or_path","version":"0.1.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>,enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3756,"record_offset":952,"rva":42056448,"va":5410765568}
// 编译实例 RVAs: 0x281bb00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140147ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=15 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-40714542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410765568,"raw_hex":"e8d2be92fd","size":5,"soft_fail":false,"target_va":5370051488,"va":5410766025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-34825416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410765568,"raw_hex":"e8389becfd","size":5,"soft_fail":false,"target_va":5375940368,"va":5410765779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9515941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410765568,"raw_hex":"e85bcc6eff","size":5,"soft_fail":false,"target_va":5401249760,"va":5410765696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5856256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410765568,"raw_hex":"e8005c5900","size":5,"soft_fail":false,"target_va":5416622240,"va":5410765979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arc_swap::ref_cnt::impl$2::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":3756,"record_offset":2276,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":7027,"source_line":166,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":3756,"record_offset":1512,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":16674,"source_line":201,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3756,"record_offset":2556,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3756,"record_offset":2332,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3756,"record_offset":2736,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":6403,"source_line":1730,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3756,"record_offset":2500,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3756,"record_offset":1400,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3756,"record_offset":2444,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":5319,"source_line":2870,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3756,"record_offset":1748,"parent_record_offset":1716,"ancestor_record_offset":952,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3756,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":952,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":3756,"record_offset":1852,"parent_record_offset":1572,"ancestor_record_offset":952,"inlinee":7010,"source_line":65,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::RefCnt::dec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":3756,"record_offset":1948,"parent_record_offset":1572,"ancestor_record_offset":952,"inlinee":4848,"source_line":86,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::RefCnt::inc
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":3756,"record_offset":1688,"parent_record_offset":1572,"ancestor_record_offset":952,"inlinee":7017,"source_line":73,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$2::into_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":3756,"record_offset":1572,"parent_record_offset":1512,"ancestor_record_offset":952,"inlinee":7016,"source_line":135,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":3756,"record_offset":1544,"parent_record_offset":1512,"ancestor_record_offset":952,"inlinee":16675,"source_line":186,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":3756,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":952,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":3756,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":952,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3756,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":952,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3756,"record_offset":1632,"parent_record_offset":1572,"ancestor_record_offset":952,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3756,"record_offset":1716,"parent_record_offset":1688,"ancestor_record_offset":952,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3756,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":952,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3756,"record_offset":2184,"parent_record_offset":1572,"ancestor_record_offset":952,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3756,"record_offset":1880,"parent_record_offset":1852,"ancestor_record_offset":952,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3756,"record_offset":1784,"parent_record_offset":1748,"ancestor_record_offset":952,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3756,"record_offset":2080,"parent_record_offset":2044,"ancestor_record_offset":952,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3756,"record_offset":1808,"parent_record_offset":1784,"ancestor_record_offset":952,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3756,"record_offset":1908,"parent_record_offset":1880,"ancestor_record_offset":952,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3756,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":952,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3756,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":952,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3756,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":952,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3756,"record_offset":2136,"parent_record_offset":2044,"ancestor_record_offset":952,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/03756.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 记录形式: compiled_procedure; instances=7
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2425,"record_offset":27416,"rva":32640800,"va":5401349920}
// 编译实例 RVAs: 0x1f20f20, 0x1f36190, 0x1f37fc0, 0x1f3aba0, 0x1f3e5b0, 0x1f42050, 0x281b220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=28
//     evidence={"assembly":"\tcallq\t15271344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401349920,"raw_hex":"e8b005e900","size":5,"soft_fail":false,"target_va":5416621762,"va":5401350413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":2425,"record_offset":27692,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2425,"record_offset":27820,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2425,"record_offset":27600,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/02425.jsonl.gz"}
//   [pdb_inline_direct] core::str::pattern::impl$36::matching
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2425,"record_offset":27972,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":9307,"source_line":1740,"archive":"_pdb/C/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2425,"record_offset":27776,"parent_record_offset":27748,"ancestor_record_offset":27416,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2425,"record_offset":27908,"parent_record_offset":27848,"ancestor_record_offset":27416,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2425,"record_offset":27848,"parent_record_offset":27820,"ancestor_record_offset":27416,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2425,"record_offset":27748,"parent_record_offset":27720,"ancestor_record_offset":27416,"inlinee":9305,"source_line":813,"archive":"_pdb/C/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2425,"record_offset":27720,"parent_record_offset":27692,"ancestor_record_offset":27416,"inlinee":9304,"source_line":978,"archive":"_pdb/C/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2425,"record_offset":27932,"parent_record_offset":27908,"ancestor_record_offset":27416,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/02425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2425,"record_offset":27652,"parent_record_offset":27600,"ancestor_record_offset":27416,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02425.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3788,"record_offset":640,"rva":42145280,"va":5410854400}
// 编译实例 RVAs: 0x2831600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$core..option..Option$LT$$RF$$u5b$u8$u5d$$GT$$u20$as$u20$rhapsody..raw..buffer..BufferInitialData$GT$::byte_count
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-27390425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e8270e5efe","size":5,"soft_fail":false,"target_va":5383464096,"va":5410854516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$core..option..Option$LT$$RF$$u5b$u8$u5d$$GT$$u20$as$u20$rhapsody..raw..buffer..BufferInitialData$GT$::copy_to_slice
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-27390645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e84b0d5efe","size":5,"soft_fail":false,"target_va":5383464112,"va":5410854752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-41550053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e81bff85fd","size":5,"soft_fail":false,"target_va":5369305584,"va":5410855632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fc00b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25630989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e8f3e678fe","size":5,"soft_fail":false,"target_va":5385224368,"va":5410855352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5766009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e879fb5700","size":5,"soft_fail":false,"target_va":5416621536,"va":5410855522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5766143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e8fffb5700","size":5,"soft_fail":false,"target_va":5416621600,"va":5410855452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5766796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e88cfe5700","size":5,"soft_fail":false,"target_va":5416622208,"va":5410855407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5766938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e81aff5700","size":5,"soft_fail":false,"target_va":5416622320,"va":5410855377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5721358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e80e4d5700","size":5,"soft_fail":false,"target_va":5416576832,"va":5410855469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5720654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e84e4a5700","size":5,"soft_fail":false,"target_va":5416576144,"va":5410855485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-26870066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e8cefe65fe","size":5,"soft_fail":false,"target_va":5383984848,"va":5410854909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-26869600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e8a00066fe","size":5,"soft_fail":false,"target_va":5383985360,"va":5410854955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-27366900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e80c6a5efe","size":5,"soft_fail":false,"target_va":5383487952,"va":5410854847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-27137947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e865e861fe","size":5,"soft_fail":false,"target_va":5383716688,"va":5410854630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-27131333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e83b0262fe","size":5,"soft_fail":false,"target_va":5383723216,"va":5410854544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::with_setup_cb
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-27130857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e8170462fe","size":5,"soft_fail":false,"target_va":5383724400,"va":5410855252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-27329299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410854400,"raw_hex":"e8edfc5efe","size":5,"soft_fail":false,"target_va":5383525648,"va":5410854942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3788,"record_offset":864,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3788,"record_offset":1656,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3788,"record_offset":1468,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3788,"record_offset":1040,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3788,"record_offset":1172,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3788,"record_offset":1076,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":3788,"record_offset":1412,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":7519,"source_line":696,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::Buffer::hot
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3788,"record_offset":2004,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":8236,"source_line":146,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::dynamic_constants
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/device.rs
//     evidence={"module":3788,"record_offset":1200,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":16914,"source_line":919,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3788,"record_offset":892,"parent_record_offset":864,"ancestor_record_offset":640,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3788,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":640,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3788,"record_offset":1136,"parent_record_offset":1104,"ancestor_record_offset":640,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":3788,"record_offset":2152,"parent_record_offset":2124,"ancestor_record_offset":640,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3788,"record_offset":1228,"parent_record_offset":1200,"ancestor_record_offset":640,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3788,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":640,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3788,"record_offset":2180,"parent_record_offset":2124,"ancestor_record_offset":640,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3788,"record_offset":2204,"parent_record_offset":2180,"ancestor_record_offset":640,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3788,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":640,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3788,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":640,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3788,"record_offset":2100,"parent_record_offset":2076,"ancestor_record_offset":640,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3788,"record_offset":1104,"parent_record_offset":1076,"ancestor_record_offset":640,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3788,"record_offset":1588,"parent_record_offset":1524,"ancestor_record_offset":640,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3788,"record_offset":1316,"parent_record_offset":1280,"ancestor_record_offset":640,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3788,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":640,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3788,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":640,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":3788,"record_offset":1496,"parent_record_offset":1468,"ancestor_record_offset":640,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":3788,"record_offset":1280,"parent_record_offset":1228,"ancestor_record_offset":640,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":3788,"record_offset":1524,"parent_record_offset":1496,"ancestor_record_offset":640,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":3788,"record_offset":1372,"parent_record_offset":1280,"ancestor_record_offset":640,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":3788,"record_offset":1560,"parent_record_offset":1524,"ancestor_record_offset":640,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::Handle::hot
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3788,"record_offset":2028,"parent_record_offset":2004,"ancestor_record_offset":640,"inlinee":8238,"source_line":859,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":3788,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":640,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/03788.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::impl$0::add_roof_systems
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":270,"record_offset":1520,"rva":7063568,"va":5375772688}
// 编译实例 RVAs: 0x6bc810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..boxed..Box$LT$dyn$u20$bevy_ecs..system..system..System$u2b$In$u20$$u3d$$u20$$LP$$RP$$u2b$Out$u20$$u3d$$u20$$LP$$RP$$GT$$u20$as$u20$bevy_ecs..schedule..config..Schedulable$GT$::into_config
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t8445349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e8a5dd8000","size":5,"soft_fail":false,"target_va":5384218864,"va":5375773510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$bevy_ecs..schedule..config..ScheduleConfigs$LT$T$GT$$u20$as$u20$bevy_ecs..schedule..config..IntoScheduleConfigs$LT$T$C$$LP$$RP$$GT$$GT$::run_if
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t9496275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e8d3e69000","size":5,"soft_fail":false,"target_va":5385269808,"va":5375773528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-94766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e8d28dfeff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375773449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7051247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e8116894ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375773434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400793a0
//     owner=unresolved_namespace; count=11
//     target_aliases=118 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6569970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e80ec09bff","size":5,"soft_fail":false,"target_va":5369205664,"va":5375775629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd210
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5176108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e8d404b1ff","size":5,"soft_fail":false,"target_va":5370597904,"va":5375774007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fce600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9509113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e8f9189100","size":5,"soft_fail":false,"target_va":5385283072,"va":5375773954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fcec60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9511146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e8ea209100","size":5,"soft_fail":false,"target_va":5385284704,"va":5375773553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fd76f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9545508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e824a79100","size":5,"soft_fail":false,"target_va":5385320176,"va":5375774663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40845213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e89d3f6f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5375775551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9548140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e86cb19100","size":5,"soft_fail":false,"target_va":5385323664,"va":5375775519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9527783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e8e7619100","size":5,"soft_fail":false,"target_va":5385301776,"va":5375773988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_registry::MessageRegistry::register_message
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9478054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e8a69f9000","size":5,"soft_fail":false,"target_va":5385251008,"va":5375772949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9497919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e83fed9000","size":5,"soft_fail":false,"target_va":5385272176,"va":5375774252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9533981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e81d7a9100","size":5,"soft_fail":false,"target_va":5385308112,"va":5375774126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::in_set
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9537628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375772688,"raw_hex":"e85c889100","size":5,"soft_fail":false,"target_va":5385312192,"va":5375774559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_message
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":270,"record_offset":1656,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":11549,"source_line":400,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":270,"record_offset":7584,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":270,"record_offset":6656,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":4638,"source_line":448,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":270,"record_offset":7448,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":5917,"source_line":493,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":1520,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":6836,"parent_record_offset":6812,"ancestor_record_offset":1520,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":6860,"parent_record_offset":6836,"ancestor_record_offset":1520,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":6788,"parent_record_offset":6756,"ancestor_record_offset":1520,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":270,"record_offset":6756,"parent_record_offset":6720,"ancestor_record_offset":1520,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":270,"record_offset":6720,"parent_record_offset":6684,"ancestor_record_offset":1520,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":270,"record_offset":3048,"parent_record_offset":3024,"ancestor_record_offset":1520,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":270,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":1520,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":270,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":1520,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":3024,"parent_record_offset":3000,"ancestor_record_offset":1520,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":3000,"parent_record_offset":2960,"ancestor_record_offset":1520,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":1520,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_message
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs
//     evidence={"module":270,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":1520,"inlinee":9392,"source_line":348,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":270,"record_offset":1740,"parent_record_offset":1708,"ancestor_record_offset":1520,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$4::into_configs
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":270,"record_offset":6684,"parent_record_offset":6656,"ancestor_record_offset":1520,"inlinee":4259,"source_line":565,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/label.rs
//     evidence={"module":270,"record_offset":7484,"parent_record_offset":7448,"ancestor_record_offset":1520,"inlinee":4539,"source_line":119,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":270,"record_offset":3268,"parent_record_offset":3244,"ancestor_record_offset":1520,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":270,"record_offset":3244,"parent_record_offset":3220,"ancestor_record_offset":1520,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":270,"record_offset":2708,"parent_record_offset":2684,"ancestor_record_offset":1520,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":2756,"parent_record_offset":2732,"ancestor_record_offset":1520,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":2732,"parent_record_offset":2708,"ancestor_record_offset":1520,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":1520,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":270,"record_offset":1708,"parent_record_offset":1680,"ancestor_record_offset":1520,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":270,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":1520,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":270,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":1520,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":270,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":1520,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":270,"record_offset":2436,"parent_record_offset":2408,"ancestor_record_offset":1520,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":270,"record_offset":1980,"parent_record_offset":1956,"ancestor_record_offset":1520,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":270,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":1520,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":270,"record_offset":2212,"parent_record_offset":2184,"ancestor_record_offset":1520,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":270,"record_offset":2184,"parent_record_offset":2156,"ancestor_record_offset":1520,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":270,"record_offset":2848,"parent_record_offset":2784,"ancestor_record_offset":1520,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":270,"record_offset":2660,"parent_record_offset":1708,"ancestor_record_offset":1520,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":270,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":1520,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":270,"record_offset":3196,"parent_record_offset":1708,"ancestor_record_offset":1520,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":270,"record_offset":2912,"parent_record_offset":2732,"ancestor_record_offset":1520,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":2004,"parent_record_offset":1980,"ancestor_record_offset":1520,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":270,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":1520,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":270,"record_offset":2784,"parent_record_offset":2756,"ancestor_record_offset":1520,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":270,"record_offset":2820,"parent_record_offset":2784,"ancestor_record_offset":1520,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":270,"record_offset":3140,"parent_record_offset":2960,"ancestor_record_offset":1520,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":270,"record_offset":2384,"parent_record_offset":2360,"ancestor_record_offset":1520,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":270,"record_offset":2124,"parent_record_offset":2096,"ancestor_record_offset":1520,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":270,"record_offset":2156,"parent_record_offset":2124,"ancestor_record_offset":1520,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":270,"record_offset":2096,"parent_record_offset":1876,"ancestor_record_offset":1520,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":270,"record_offset":1956,"parent_record_offset":1876,"ancestor_record_offset":1520,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":270,"record_offset":2512,"parent_record_offset":1876,"ancestor_record_offset":1520,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":270,"record_offset":2040,"parent_record_offset":1876,"ancestor_record_offset":1520,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":270,"record_offset":1800,"parent_record_offset":1764,"ancestor_record_offset":1520,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":270,"record_offset":2360,"parent_record_offset":2256,"ancestor_record_offset":1520,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":1520,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":2604,"parent_record_offset":1876,"ancestor_record_offset":1520,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":2276,"parent_record_offset":2256,"ancestor_record_offset":1520,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":1520,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":1828,"parent_record_offset":1800,"ancestor_record_offset":1520,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":1876,"parent_record_offset":1852,"ancestor_record_offset":1520,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":1928,"parent_record_offset":1876,"ancestor_record_offset":1520,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":2256,"parent_record_offset":1876,"ancestor_record_offset":1520,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":270,"record_offset":2484,"parent_record_offset":1876,"ancestor_record_offset":1520,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::impl$0::add_roof_events_and_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":270,"record_offset":7616,"rva":7067008,"va":5375776128}
// 编译实例 RVAs: 0x6bd580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9547993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375776128,"raw_hex":"e8d9b09100","size":5,"soft_fail":false,"target_va":5385324144,"va":5375776146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_registry::MessageRegistry::register_message
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9468470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375776128,"raw_hex":"e8367a9000","size":5,"soft_fail":false,"target_va":5385244864,"va":5375776389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_message
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":270,"record_offset":7752,"parent_record_offset":7616,"ancestor_record_offset":7616,"inlinee":11549,"source_line":400,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":270,"record_offset":9148,"parent_record_offset":9124,"ancestor_record_offset":7616,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":270,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":7616,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":270,"record_offset":9172,"parent_record_offset":9148,"ancestor_record_offset":7616,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":9124,"parent_record_offset":9100,"ancestor_record_offset":7616,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":9100,"parent_record_offset":9060,"ancestor_record_offset":7616,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":9060,"parent_record_offset":9036,"ancestor_record_offset":7616,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_message
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs
//     evidence={"module":270,"record_offset":7776,"parent_record_offset":7752,"ancestor_record_offset":7616,"inlinee":9392,"source_line":348,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":270,"record_offset":7836,"parent_record_offset":7804,"ancestor_record_offset":7616,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":270,"record_offset":9368,"parent_record_offset":9344,"ancestor_record_offset":7616,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":270,"record_offset":9344,"parent_record_offset":9320,"ancestor_record_offset":7616,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":270,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":7616,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":8856,"parent_record_offset":8832,"ancestor_record_offset":7616,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":8832,"parent_record_offset":8808,"ancestor_record_offset":7616,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":270,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":7616,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":270,"record_offset":7804,"parent_record_offset":7776,"ancestor_record_offset":7616,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":270,"record_offset":8784,"parent_record_offset":8760,"ancestor_record_offset":7616,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":270,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":7616,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":270,"record_offset":8508,"parent_record_offset":8484,"ancestor_record_offset":7616,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":270,"record_offset":8536,"parent_record_offset":8508,"ancestor_record_offset":7616,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":270,"record_offset":8076,"parent_record_offset":8052,"ancestor_record_offset":7616,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":270,"record_offset":8160,"parent_record_offset":8136,"ancestor_record_offset":7616,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":270,"record_offset":8308,"parent_record_offset":8280,"ancestor_record_offset":7616,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":270,"record_offset":8280,"parent_record_offset":8252,"ancestor_record_offset":7616,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":270,"record_offset":8948,"parent_record_offset":8884,"ancestor_record_offset":7616,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":270,"record_offset":8760,"parent_record_offset":7804,"ancestor_record_offset":7616,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":270,"record_offset":8972,"parent_record_offset":8948,"ancestor_record_offset":7616,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":270,"record_offset":9296,"parent_record_offset":7804,"ancestor_record_offset":7616,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":270,"record_offset":9012,"parent_record_offset":8832,"ancestor_record_offset":7616,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":7616,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":270,"record_offset":8424,"parent_record_offset":8400,"ancestor_record_offset":7616,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":270,"record_offset":8884,"parent_record_offset":8856,"ancestor_record_offset":7616,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":270,"record_offset":8920,"parent_record_offset":8884,"ancestor_record_offset":7616,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":270,"record_offset":9240,"parent_record_offset":9060,"ancestor_record_offset":7616,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":270,"record_offset":8484,"parent_record_offset":8460,"ancestor_record_offset":7616,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":270,"record_offset":8220,"parent_record_offset":8192,"ancestor_record_offset":7616,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":270,"record_offset":8252,"parent_record_offset":8220,"ancestor_record_offset":7616,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":270,"record_offset":8192,"parent_record_offset":7972,"ancestor_record_offset":7616,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":270,"record_offset":8052,"parent_record_offset":7972,"ancestor_record_offset":7616,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":270,"record_offset":8612,"parent_record_offset":7972,"ancestor_record_offset":7616,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":270,"record_offset":8136,"parent_record_offset":7972,"ancestor_record_offset":7616,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":270,"record_offset":7896,"parent_record_offset":7860,"ancestor_record_offset":7616,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":270,"record_offset":8460,"parent_record_offset":8352,"ancestor_record_offset":7616,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":8400,"parent_record_offset":8376,"ancestor_record_offset":7616,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":8704,"parent_record_offset":7972,"ancestor_record_offset":7616,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":8376,"parent_record_offset":8352,"ancestor_record_offset":7616,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":7948,"parent_record_offset":7924,"ancestor_record_offset":7616,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":7924,"parent_record_offset":7896,"ancestor_record_offset":7616,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":7972,"parent_record_offset":7948,"ancestor_record_offset":7616,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":8024,"parent_record_offset":7972,"ancestor_record_offset":7616,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":270,"record_offset":8352,"parent_record_offset":7972,"ancestor_record_offset":7616,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":270,"record_offset":8584,"parent_record_offset":7972,"ancestor_record_offset":7616,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00270.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::gpu_flush_buffers
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFC42
// 首条位置: {"module":965,"inlinee":64578}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":965,"record_offset":4352,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00965.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":965,"record_offset":4752,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00965.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":965,"record_offset":4408,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":26382,"source_line":294,"archive":"_pdb/C/modules/00965.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":965,"record_offset":4376,"parent_record_offset":4352,"ancestor_record_offset":4312,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00965.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":965,"record_offset":4644,"parent_record_offset":4500,"ancestor_record_offset":4312,"inlinee":13529,"source_line":2468,"archive":"_pdb/C/modules/00965.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":965,"record_offset":4676,"parent_record_offset":4472,"ancestor_record_offset":4312,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00965.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":965,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":4312,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00965.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":965,"record_offset":4604,"parent_record_offset":4580,"ancestor_record_offset":4312,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00965.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":965,"record_offset":4436,"parent_record_offset":4408,"ancestor_record_offset":4312,"inlinee":26383,"source_line":253,"archive":"_pdb/C/modules/00965.jsonl.gz"}
//   [pdb_inline_transitive] frozen::FrozenMap::as_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"frozen","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/frozen/src/lib.rs
//     evidence={"module":965,"record_offset":4472,"parent_record_offset":4436,"ancestor_record_offset":4312,"inlinee":26384,"source_line":114,"archive":"_pdb/C/modules/00965.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":965,"record_offset":4716,"parent_record_offset":4436,"ancestor_record_offset":4312,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/00965.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":965,"record_offset":4528,"parent_record_offset":4500,"ancestor_record_offset":4312,"inlinee":4343,"source_line":112,"archive":"_pdb/C/modules/00965.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":965,"record_offset":4556,"parent_record_offset":4528,"ancestor_record_offset":4312,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00965.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\rwlock.rs
//     evidence={"module":965,"record_offset":4500,"parent_record_offset":4472,"ancestor_record_offset":4312,"inlinee":4341,"source_line":678,"archive":"_pdb/C/modules/00965.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_roof::utils::components::RoofSpire
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 53
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >", "constructor": "tuple$", "arguments": ["country_core::resources::roofs::roof_shape::Roof", "country_core::resources::roofs::roof_tile_sim::RoofTileSimulation", "country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume", "country_core::resources::roofs::roof_animation::RoofAnimation", "country_core::resources::roofs::roof_tiles::RoofTiles", "system_roof::utils::components::RoofSpire", "system_roof::utils::components::RoofSupportBeams", "country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::BundleInfo::write_components<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::info::BundleInfo::write_components", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>", "constructor": "bevy_ecs::query::filter::With", "arguments": ["system_roof::utils::components::RoofSpire"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::iter::QueryIter", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::impl$2::next<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>", "constructor": "core::iter::adapters::filter_map::impl$2::next", "arguments": ["country_core::resources::roof_tile_ssbo::RoofSpireSsbo", "bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "system_roof::visual::place_spires::place_roof_spires::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "system_roof::visual::place_spires::place_roof_spires::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::roof_tile_ssbo::RoofSpireSsbo", "core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::state::QueryState", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ptr::MovingPtr<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >,bevy_ptr::Aligned>", "constructor": "bevy_ptr::MovingPtr", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >", "bevy_ptr::Aligned"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ptr::MovingPtr<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >,bevy_ptr::Aligned> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ptr::MovingPtr<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >,bevy_ptr::Aligned>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::system::query::Query", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::ScheduleConfigTupleMarker", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["tuple$<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_component<system_roof::utils::components::RoofSpire>", "constructor": "bevy_ecs::world::World::register_component", "arguments": ["system_roof::utils::components::RoofSpire"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$108::init_access<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary> >", "constructor": "bevy_ecs::system::system_param::impl$108::init_access", "arguments": ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::new_archetype<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::state::QueryState::new_archetype", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::update_archetypes_unsafe_world_cell<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::state::QueryState::update_archetypes_unsafe_world_cell", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::new<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::state::QueryState::new", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$35::component_ids<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impls::impl$35::component_ids", "arguments": ["country_core::resources::roofs::roof_shape::Roof", "country_core::resources::roofs::roof_tile_sim::RoofTileSimulation", "country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume", "country_core::resources::roofs::roof_animation::RoofAnimation", "country_core::resources::roofs::roof_tiles::RoofTiles", "system_roof::utils::components::RoofSpire", "system_roof::utils::components::RoofSupportBeams", "country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::components::drawable::impl$32::component_ids<country_core::render::nani::instance_data::StoneFloorInstanceData,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "country_core::components::drawable::impl$32::component_ids", "arguments": ["country_core::render::nani::instance_data::StoneFloorInstanceData", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<country_core::resources::roofs::roof_animation::RoofAnimation,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["country_core::resources::roofs::roof_animation::RoofAnimation", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["country_core::resources::roofs::roof_tile_sim::RoofTileSimulation", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<country_core::resources::roofs::roof_tiles::RoofTiles,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["country_core::resources::roofs::roof_tiles::RoofTiles", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<system_roof::utils::components::RoofSpire,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["system_roof::utils::components::RoofSpire", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<system_roof::utils::components::RoofSupportBeams,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["system_roof::utils::components::RoofSupportBeams", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_roof::utils::components::RoofSupportBeams
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 18
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >", "constructor": "tuple$", "arguments": ["country_core::resources::roofs::roof_shape::Roof", "country_core::resources::roofs::roof_tile_sim::RoofTileSimulation", "country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume", "country_core::resources::roofs::roof_animation::RoofAnimation", "country_core::resources::roofs::roof_tiles::RoofTiles", "system_roof::utils::components::RoofSpire", "system_roof::utils::components::RoofSupportBeams", "country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::BundleInfo::write_components<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::info::BundleInfo::write_components", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ptr::MovingPtr<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >,bevy_ptr::Aligned>", "constructor": "bevy_ptr::MovingPtr", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >", "bevy_ptr::Aligned"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ptr::MovingPtr<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >,bevy_ptr::Aligned> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ptr::MovingPtr<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >,bevy_ptr::Aligned>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$35::component_ids<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impls::impl$35::component_ids", "arguments": ["country_core::resources::roofs::roof_shape::Roof", "country_core::resources::roofs::roof_tile_sim::RoofTileSimulation", "country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume", "country_core::resources::roofs::roof_animation::RoofAnimation", "country_core::resources::roofs::roof_tiles::RoofTiles", "system_roof::utils::components::RoofSpire", "system_roof::utils::components::RoofSupportBeams", "country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::components::drawable::impl$32::component_ids<country_core::render::nani::instance_data::StoneFloorInstanceData,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "country_core::components::drawable::impl$32::component_ids", "arguments": ["country_core::render::nani::instance_data::StoneFloorInstanceData", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<country_core::resources::roofs::roof_animation::RoofAnimation,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["country_core::resources::roofs::roof_animation::RoofAnimation", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["country_core::resources::roofs::roof_tile_sim::RoofTileSimulation", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<country_core::resources::roofs::roof_tiles::RoofTiles,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["country_core::resources::roofs::roof_tiles::RoofTiles", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<system_roof::utils::components::RoofSpire,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["system_roof::utils::components::RoofSpire", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<system_roof::utils::components::RoofSupportBeams,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["system_roof::utils::components::RoofSupportBeams", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/binary/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/roof/src/lib.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 27; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::main_camera::MainCameraMoved>,bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof::DoUiEditRoof>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::main_camera::MainCameraMoved>", "bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof::DoUiEditRoof>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof::DoUiEditRoof>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::message::message_reader::MessageReader<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::DeleteRoofCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>,enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > > >
// 信息 ID: 21a6e0d021d726aaa60b5899
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::sync::Arc -> ["alloc::string::String", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["alloc::sync::Arc<alloc::string::String,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0 -> ["enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::DefaultConfig"]; 字段布局 = unknown。
// [类型关系] alloc::sync::Arc -> ["alloc::string::String", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["alloc::sync::Arc<alloc::string::String,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] arc_swap::debt::helping::Slots::help -> ["arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>", "enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] arc_swap::debt::impl$2::pay_all::closure$0<enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> > @ 0x1f085e8 -> 0x281bb00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 信息 ID: 0538cde337678dd7ea2e895d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["core::str::pattern::MatchOnly"]; 字段布局 = unknown。
// [类型关系] core::str::pattern::TwoWaySearcher::next_back -> ["enum2$<core::str::pattern::MatchOnly>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1f20df3 -> 0x1f20f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1f36063 -> 0x1f36190 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1f37e97 -> 0x1f37fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1f3aa73 -> 0x1f3aba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1f3e487 -> 0x1f3e5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1f41f27 -> 0x1f42050 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x281b0f7 -> 0x281b220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
// 信息 ID: 7d215f4f047f611f4f35cf3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<ref$<slice2$<u8> > >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::create_buffer -> ["enum2$<core::option::Option<ref$<slice2$<u8> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roof_tile_ssbo::RoofTileSsbo> @ 0x1f3fd02 -> 0x2831600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::gpu_flush_buffers
// 信息 ID: 0d02d413c1443b9103d6ca5f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5d58aab8b83fbd13d0042c71: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>; module=965, inline_record=4312; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >; module=2426, inline_record=329348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::impl$0::add_roof_events_and_resources
// 信息 ID: e9747559ce6bb1b3ea0f6cd6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x41ea6d -> 0x6bd580 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::impl$0::add_roof_systems
// 信息 ID: d342f7b6562a3d218f000d5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x41e1f1 -> 0x6bc810 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
