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
// Compiled procedure 271:1472 -> rva_62fcb0_len_935
//
// FUNCTION system_roof::impl$0::add_roof_events_and_resources
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 271:3888 -> rva_6305f0_len_42b
//
// FUNCTION system_roof::gpu_flush_buffers
// Candidate interfaces: 5d58aab8b83fbd13d0042c71
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 924:3712 -> host 924:1036 / rva_de50a0_len_21a; exact_symbolizer_function_frame
//   Function-frame RVA span [0xde514f, 0xde5177), 10 instructions
//   Function-frame RVA span [0xde52ad, 0xde52ba), 3 instructions
// Inline site 2264:199340 -> host 2264:196140 / rva_1a28200_len_2a5; exact_symbolizer_function_frame
//   Function-frame RVA span [0x1a282e1, 0x1a28309), 10 instructions
//   Function-frame RVA span [0x1a28498, 0x1a284a5), 3 instructions
//
// FUNCTION core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 7; complete records in enriched/assembly sidecars.
// Compiled procedure 2259:2088 -> rva_1a18a90_len_236
// Compiled procedure 2275:1452 -> rva_1a36400_len_236
// Compiled procedure 2279:1880 -> rva_1a38370_len_236
// Compiled procedure 2281:22296 -> rva_1a3c870_len_236
// Compiled procedure 3430:2460 -> rva_2196bd0_len_236
// Compiled procedure 3466:2164 -> rva_21a7ed0_len_236
// Compiled procedure 3470:1908 -> rva_21ad100_len_236
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_62fcb0_len_935: RVA 0x62fcb0, 2357 PDB bytes; 465 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_62fcb0_len_935.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 271:1472: system_roof::impl$0::add_roof_systems
// Frames: F1
// 0x14062fcb0  55                             pushq	%rbp
// 0x14062fcb1  56                             pushq	%rsi
// 0x14062fcb2  57                             pushq	%rdi
// 0x14062fcb3  4881ec10080000                 subq	$0x810, %rsp            # imm = 0x810
// 0x14062fcba  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14062fcc2  48c78588070000feffffff         movq	$-0x2, 0x788(%rbp)
// 0x14062fccd  4889ce                         movq	%rcx, %rsi
// Frames: F2 -> F3 -> F4 -> F5
// 0x14062fcd0  488d7938                       leaq	0x38(%rcx), %rdi
// 0x14062fcd4  48ba44f35fe2153da444           movabsq	$0x44a43d15e25ff344, %rdx # imm = 0x44A43D15E25FF344
// 0x14062fcde  49b8a350a9c1bfdce677           movabsq	$0x77e6dcbfc1a950a3, %r8 # imm = 0x77E6DCBFC1A950A3
// 0x14062fce8  4889f9                         movq	%rdi, %rcx
// 0x14062fceb  e800daaaff                     callq	0x1400dd6f0 <.text+0xdc6f0>
// Frames: F6 -> F7 -> F3 -> F4 -> F5
// 0x14062fcf0  4885c0                         testq	%rax, %rax
// 0x14062fcf3  7433                           je	0x14062fd28 <.text+0x62ed28>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F5
// 0x14062fcf5  48399618020000                 cmpq	%rdx, 0x218(%rsi)
// 0x14062fcfc  762a                           jbe	0x14062fd28 <.text+0x62ed28>
// Frames: F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F5
// 0x14062fcfe  488b8610020000                 movq	0x210(%rsi), %rax
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F5
// 0x14062fd05  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x14062fd09  4885c0                         testq	%rax, %rax
// Frames: F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F5
// 0x14062fd0c  741a                           je	0x14062fd28 <.text+0x62ed28>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F5
// 0x14062fd0e  488b8ee0010000                 movq	0x1e0(%rsi), %rcx
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F5
// 0x14062fd15  48f7d0                         notq	%rax
// Frames: F27 -> F28 -> F3 -> F4 -> F5
// 0x14062fd18  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x14062fd1c  48c1e005                       shlq	$0x5, %rax
// Frames: F29 -> F30 -> F16 -> F27 -> F28 -> F3 -> F4 -> F5
// 0x14062fd20  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F3 -> F4 -> F5
// 0x14062fd26  7508                           jne	0x14062fd30 <.text+0x62ed30>
// Frames: F31 -> F4 -> F5
// 0x14062fd28  4889f1                         movq	%rsi, %rcx
// 0x14062fd2b  e8c0727a00                     callq	0x140dd6ff0 <.text+0xdd5ff0>
// Frames: F2 -> F3 -> F4 -> F32
// 0x14062fd30  48ba7be2be49b9b29198           movabsq	$-0x676e4d46b6411d85, %rdx # imm = 0x9891B2B949BEE27B
// 0x14062fd3a  49b87a05dab82a5b0afa           movabsq	$-0x5f5a4d54725fa86, %r8 # imm = 0xFA0A5B2AB8DA057A
// 0x14062fd44  4889f9                         movq	%rdi, %rcx
// 0x14062fd47  e8a4d9aaff                     callq	0x1400dd6f0 <.text+0xdc6f0>
// Frames: F6 -> F7 -> F3 -> F4 -> F32
// 0x14062fd4c  4885c0                         testq	%rax, %rax
// 0x14062fd4f  7433                           je	0x14062fd84 <.text+0x62ed84>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F32
// 0x14062fd51  48399618020000                 cmpq	%rdx, 0x218(%rsi)
// 0x14062fd58  762a                           jbe	0x14062fd84 <.text+0x62ed84>
// Frames: F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F32
// 0x14062fd5a  488b8610020000                 movq	0x210(%rsi), %rax
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F32
// 0x14062fd61  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x14062fd65  4885c0                         testq	%rax, %rax
// Frames: F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F32
// 0x14062fd68  741a                           je	0x14062fd84 <.text+0x62ed84>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F32
// 0x14062fd6a  488b8ee0010000                 movq	0x1e0(%rsi), %rcx
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F32
// 0x14062fd71  48f7d0                         notq	%rax
// Frames: F27 -> F28 -> F3 -> F4 -> F32
// 0x14062fd74  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x14062fd78  48c1e005                       shlq	$0x5, %rax
// Frames: F29 -> F30 -> F16 -> F27 -> F28 -> F3 -> F4 -> F32
// 0x14062fd7c  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F3 -> F4 -> F32
// 0x14062fd82  7508                           jne	0x14062fd8c <.text+0x62ed8c>
// Frames: F31 -> F4 -> F32
// 0x14062fd84  4889f1                         movq	%rsi, %rcx
// 0x14062fd87  e8a4597a00                     callq	0x140dd5730 <.text+0xdd4730>
// Frames: F2 -> F3 -> F4 -> F33
// 0x14062fd8c  48baa1ba994f4b457955           movabsq	$0x5579454b4f99baa1, %rdx # imm = 0x5579454B4F99BAA1
// 0x14062fd96  49b855d935a44272902c           movabsq	$0x2c907242a435d955, %r8 # imm = 0x2C907242A435D955
// 0x14062fda0  4889f9                         movq	%rdi, %rcx
// 0x14062fda3  e848d9aaff                     callq	0x1400dd6f0 <.text+0xdc6f0>
// Frames: F6 -> F7 -> F3 -> F4 -> F33
// 0x14062fda8  4885c0                         testq	%rax, %rax
// 0x14062fdab  7433                           je	0x14062fde0 <.text+0x62ede0>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F33
// 0x14062fdad  48399618020000                 cmpq	%rdx, 0x218(%rsi)
// 0x14062fdb4  762a                           jbe	0x14062fde0 <.text+0x62ede0>
// Frames: F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F33
// 0x14062fdb6  488b8610020000                 movq	0x210(%rsi), %rax
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F33
// 0x14062fdbd  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x14062fdc1  4885c0                         testq	%rax, %rax
// Frames: F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F33
// 0x14062fdc4  741a                           je	0x14062fde0 <.text+0x62ede0>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F33
// 0x14062fdc6  488b8ee0010000                 movq	0x1e0(%rsi), %rcx
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F33
// 0x14062fdcd  48f7d0                         notq	%rax
// Frames: F27 -> F28 -> F3 -> F4 -> F33
// 0x14062fdd0  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x14062fdd4  48c1e005                       shlq	$0x5, %rax
// Frames: F29 -> F30 -> F16 -> F27 -> F28 -> F3 -> F4 -> F33
// 0x14062fdd8  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F3 -> F4 -> F33
// 0x14062fdde  7508                           jne	0x14062fde8 <.text+0x62ede8>
// Frames: F31 -> F4 -> F33
// 0x14062fde0  4889f1                         movq	%rsi, %rcx
// 0x14062fde3  e898847a00                     callq	0x140dd8280 <.text+0xdd7280>
// Frames: F34
// 0x14062fde8  4c8d05e9c93d02                 leaq	0x23dc9e9(%rip), %r8    # 0x142a0c7d8
// 0x14062fdef  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// 0x14062fdf6  b211                           movb	$0x11, %dl
// 0x14062fdf8  e893c17a00                     callq	0x140ddbf90 <.text+0xddaf90>
// Frames: F35
// 0x14062fdfd  488d8d40020000                 leaq	0x240(%rbp), %rcx
// 0x14062fe04  e807bc7a00                     callq	0x140ddba10 <.text+0xddaa10>
// Frames: F36 -> F37 -> F35
// 0x14062fe09  c6851007000012                 movb	$0x12, 0x710(%rbp)
// 0x14062fe10  488b0d59c5ff02                 movq	0x2ffc559(%rip), %rcx   # 0x14362c370
// 0x14062fe17  4c8d05224c5402                 leaq	0x2544c22(%rip), %r8    # 0x142b74a40
// 0x14062fe1e  488d9510070000                 leaq	0x710(%rbp), %rdx
// 0x14062fe25  e866cd7a00                     callq	0x140ddcb90 <.text+0xddbb90>
// Frames: F37 -> F35
// 0x14062fe2a  4989d0                         movq	%rdx, %r8
// 0x14062fe2d  488d8d40020000                 leaq	0x240(%rbp), %rcx
// 0x14062fe34  4889c2                         movq	%rax, %rdx
// 0x14062fe37  e844e47a00                     callq	0x140dde280 <.text+0xddd280>
// Frames: F38 -> F35
// 0x14062fe3c  0f1085a0020000                 movups	0x2a0(%rbp), %xmm0
// 0x14062fe43  0f298500070000                 movaps	%xmm0, 0x700(%rbp)
// 0x14062fe4a  0f108590020000                 movups	0x290(%rbp), %xmm0
// 0x14062fe51  0f2985f0060000                 movaps	%xmm0, 0x6f0(%rbp)
// 0x14062fe58  0f108580020000                 movups	0x280(%rbp), %xmm0
// 0x14062fe5f  0f2985e0060000                 movaps	%xmm0, 0x6e0(%rbp)
// 0x14062fe66  0f108540020000                 movups	0x240(%rbp), %xmm0
// 0x14062fe6d  0f108d50020000                 movups	0x250(%rbp), %xmm1
// 0x14062fe74  0f109560020000                 movups	0x260(%rbp), %xmm2
// 0x14062fe7b  0f109d70020000                 movups	0x270(%rbp), %xmm3
// 0x14062fe82  0f299dd0060000                 movaps	%xmm3, 0x6d0(%rbp)
// 0x14062fe89  0f2995c0060000                 movaps	%xmm2, 0x6c0(%rbp)
// 0x14062fe90  0f298db0060000                 movaps	%xmm1, 0x6b0(%rbp)
// 0x14062fe97  0f2985a0060000                 movaps	%xmm0, 0x6a0(%rbp)
// Frames: F39
// 0x14062fe9e  488d8d40020000                 leaq	0x240(%rbp), %rcx
// 0x14062fea5  e816f07a00                     callq	0x140ddeec0 <.text+0xdddec0>
// Frames: F40
// 0x14062feaa  0f1085a0020000                 movups	0x2a0(%rbp), %xmm0
// 0x14062feb1  0f298570070000                 movaps	%xmm0, 0x770(%rbp)
// 0x14062feb8  0f108590020000                 movups	0x290(%rbp), %xmm0
// 0x14062febf  0f298560070000                 movaps	%xmm0, 0x760(%rbp)
// 0x14062fec6  0f108580020000                 movups	0x280(%rbp), %xmm0
// 0x14062fecd  0f298550070000                 movaps	%xmm0, 0x750(%rbp)
// 0x14062fed4  0f108540020000                 movups	0x240(%rbp), %xmm0
// 0x14062fedb  0f108d50020000                 movups	0x250(%rbp), %xmm1
// 0x14062fee2  0f109560020000                 movups	0x260(%rbp), %xmm2
// 0x14062fee9  0f109d70020000                 movups	0x270(%rbp), %xmm3
// 0x14062fef0  0f299d40070000                 movaps	%xmm3, 0x740(%rbp)
// 0x14062fef7  0f299530070000                 movaps	%xmm2, 0x730(%rbp)
// 0x14062fefe  0f298d20070000                 movaps	%xmm1, 0x720(%rbp)
// 0x14062ff05  0f298510070000                 movaps	%xmm0, 0x710(%rbp)
// 0x14062ff0c  488d8d40020000                 leaq	0x240(%rbp), %rcx
// 0x14062ff13  488d9510070000                 leaq	0x710(%rbp), %rdx
// 0x14062ff1a  e8e1df7a00                     callq	0x140dddf00 <.text+0xddcf00>
// Frames: F41
// 0x14062ff1f  0f108540020000                 movups	0x240(%rbp), %xmm0
// 0x14062ff26  0f108d50020000                 movups	0x250(%rbp), %xmm1
// 0x14062ff2d  0f109560020000                 movups	0x260(%rbp), %xmm2
// 0x14062ff34  0f109d70020000                 movups	0x270(%rbp), %xmm3
// 0x14062ff3b  0f298510070000                 movaps	%xmm0, 0x710(%rbp)
// 0x14062ff42  0f298d20070000                 movaps	%xmm1, 0x720(%rbp)
// 0x14062ff49  0f299530070000                 movaps	%xmm2, 0x730(%rbp)
// 0x14062ff50  0f299d40070000                 movaps	%xmm3, 0x740(%rbp)
// 0x14062ff57  0f108580020000                 movups	0x280(%rbp), %xmm0
// 0x14062ff5e  0f298550070000                 movaps	%xmm0, 0x750(%rbp)
// 0x14062ff65  0f108590020000                 movups	0x290(%rbp), %xmm0
// 0x14062ff6c  0f298560070000                 movaps	%xmm0, 0x760(%rbp)
// 0x14062ff73  0f1085a0020000                 movups	0x2a0(%rbp), %xmm0
// 0x14062ff7a  0f298570070000                 movaps	%xmm0, 0x770(%rbp)
// 0x14062ff81  488d8d40020000                 leaq	0x240(%rbp), %rcx
// 0x14062ff88  488d9510070000                 leaq	0x710(%rbp), %rdx
// 0x14062ff8f  e8acda7a00                     callq	0x140ddda40 <.text+0xddca40>
// Frames: F36 -> F37 -> F41
// 0x14062ff94  c6851007000014                 movb	$0x14, 0x710(%rbp)
// 0x14062ff9b  488b0dcec3ff02                 movq	0x2ffc3ce(%rip), %rcx   # 0x14362c370
// 0x14062ffa2  4c8d05974a5402                 leaq	0x2544a97(%rip), %r8    # 0x142b74a40
// 0x14062ffa9  488d9510070000                 leaq	0x710(%rbp), %rdx
// 0x14062ffb0  e8dbcb7a00                     callq	0x140ddcb90 <.text+0xddbb90>
// Frames: F37 -> F41
// 0x14062ffb5  4989d0                         movq	%rdx, %r8
// 0x14062ffb8  488d8d40020000                 leaq	0x240(%rbp), %rcx
// 0x14062ffbf  4889c2                         movq	%rax, %rdx
// 0x14062ffc2  e8b9e27a00                     callq	0x140dde280 <.text+0xddd280>
// Frames: F38 -> F41
// 0x14062ffc7  0f1085a0020000                 movups	0x2a0(%rbp), %xmm0
// 0x14062ffce  0f298590060000                 movaps	%xmm0, 0x690(%rbp)
// 0x14062ffd5  0f108590020000                 movups	0x290(%rbp), %xmm0
// 0x14062ffdc  0f298580060000                 movaps	%xmm0, 0x680(%rbp)
// 0x14062ffe3  0f108580020000                 movups	0x280(%rbp), %xmm0
// 0x14062ffea  0f298570060000                 movaps	%xmm0, 0x670(%rbp)
// 0x14062fff1  0f108540020000                 movups	0x240(%rbp), %xmm0
// 0x14062fff8  0f108d50020000                 movups	0x250(%rbp), %xmm1
// 0x14062ffff  0f109560020000                 movups	0x260(%rbp), %xmm2
// 0x140630006  0f109d70020000                 movups	0x270(%rbp), %xmm3
// 0x14063000d  0f299d60060000                 movaps	%xmm3, 0x660(%rbp)
// 0x140630014  0f299550060000                 movaps	%xmm2, 0x650(%rbp)
// 0x14063001b  0f298d40060000                 movaps	%xmm1, 0x640(%rbp)
// 0x140630022  0f298530060000                 movaps	%xmm0, 0x630(%rbp)
// Frames: F42
// 0x140630029  4c8d05c0c73d02                 leaq	0x23dc7c0(%rip), %r8    # 0x142a0c7f0
// 0x140630030  488d8d60010000                 leaq	0x160(%rbp), %rcx
// 0x140630037  b21d                           movb	$0x1d, %dl
// 0x140630039  e872ff7a00                     callq	0x140ddffb0 <.text+0xddefb0>
// Frames: F43
// 0x14063003e  4c8d05c3c73d02                 leaq	0x23dc7c3(%rip), %r8    # 0x142a0c808
// 0x140630045  488d8df0000000                 leaq	0xf0(%rbp), %rcx
// 0x14063004c  b215                           movb	$0x15, %dl
// 0x14063004e  e8adf37a00                     callq	0x140ddf400 <.text+0xdde400>
// Frames: F44
// 0x140630053  4c8d05c6c73d02                 leaq	0x23dc7c6(%rip), %r8    # 0x142a0c820
// 0x14063005a  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x140630061  b228                           movb	$0x28, %dl
// 0x140630063  e848107b00                     callq	0x140de10b0 <.text+0xde00b0>
// Frames: F45
// 0x140630068  4c8d05c9c73d02                 leaq	0x23dc7c9(%rip), %r8    # 0x142a0c838
// 0x14063006f  488d4d10                       leaq	0x10(%rbp), %rcx
// 0x140630073  b222                           movb	$0x22, %dl
// 0x140630075  e836197b00                     callq	0x140de19b0 <.text+0xde09b0>
// Frames: F46
// 0x14063007a  4c8d05cfc73d02                 leaq	0x23dc7cf(%rip), %r8    # 0x142a0c850
// 0x140630081  488d4da0                       leaq	-0x60(%rbp), %rcx
// 0x140630085  b222                           movb	$0x22, %dl
// 0x140630087  e8d44c7b00                     callq	0x140de4d60 <.text+0xde3d60>
// Frames: F47
// 0x14063008c  4c8d05d5c73d02                 leaq	0x23dc7d5(%rip), %r8    # 0x142a0c868
// 0x140630093  488d8d10070000                 leaq	0x710(%rbp), %rcx
// 0x14063009a  b22c                           movb	$0x2c, %dl
// 0x14063009c  e82f557b00                     callq	0x140de55d0 <.text+0xde45d0>
// Frames: F48
// 0x1406300a1  0f108530020000                 movups	0x230(%rbp), %xmm0
// 0x1406300a8  0f2985a0020000                 movaps	%xmm0, 0x2a0(%rbp)
// 0x1406300af  0f108520020000                 movups	0x220(%rbp), %xmm0
// 0x1406300b6  0f298590020000                 movaps	%xmm0, 0x290(%rbp)
// 0x1406300bd  0f108510020000                 movups	0x210(%rbp), %xmm0
// 0x1406300c4  0f298580020000                 movaps	%xmm0, 0x280(%rbp)
// 0x1406300cb  0f1085d0010000                 movups	0x1d0(%rbp), %xmm0
// 0x1406300d2  0f108de0010000                 movups	0x1e0(%rbp), %xmm1
// 0x1406300d9  0f1095f0010000                 movups	0x1f0(%rbp), %xmm2
// 0x1406300e0  0f109d00020000                 movups	0x200(%rbp), %xmm3
// 0x1406300e7  0f299d70020000                 movaps	%xmm3, 0x270(%rbp)
// 0x1406300ee  0f299560020000                 movaps	%xmm2, 0x260(%rbp)
// 0x1406300f5  0f298d50020000                 movaps	%xmm1, 0x250(%rbp)
// 0x1406300fc  0f298540020000                 movaps	%xmm0, 0x240(%rbp)
// 0x140630103  0f2885a0060000                 movaps	0x6a0(%rbp), %xmm0
// 0x14063010a  0f288db0060000                 movaps	0x6b0(%rbp), %xmm1
// 0x140630111  0f2895c0060000                 movaps	0x6c0(%rbp), %xmm2
// 0x140630118  0f289dd0060000                 movaps	0x6d0(%rbp), %xmm3
// 0x14063011f  0f2985b0020000                 movaps	%xmm0, 0x2b0(%rbp)
// 0x140630126  0f298dc0020000                 movaps	%xmm1, 0x2c0(%rbp)
// 0x14063012d  0f2995d0020000                 movaps	%xmm2, 0x2d0(%rbp)
// 0x140630134  0f299de0020000                 movaps	%xmm3, 0x2e0(%rbp)
// 0x14063013b  0f2885e0060000                 movaps	0x6e0(%rbp), %xmm0
// 0x140630142  0f2985f0020000                 movaps	%xmm0, 0x2f0(%rbp)
// 0x140630149  0f2885f0060000                 movaps	0x6f0(%rbp), %xmm0
// 0x140630150  0f298500030000                 movaps	%xmm0, 0x300(%rbp)
// 0x140630157  0f288500070000                 movaps	0x700(%rbp), %xmm0
// 0x14063015e  0f298510030000                 movaps	%xmm0, 0x310(%rbp)
// 0x140630165  0f288590060000                 movaps	0x690(%rbp), %xmm0
// 0x14063016c  0f298580030000                 movaps	%xmm0, 0x380(%rbp)
// 0x140630173  0f288580060000                 movaps	0x680(%rbp), %xmm0
// 0x14063017a  0f298570030000                 movaps	%xmm0, 0x370(%rbp)
// 0x140630181  0f288570060000                 movaps	0x670(%rbp), %xmm0
// 0x140630188  0f298560030000                 movaps	%xmm0, 0x360(%rbp)
// 0x14063018f  0f288530060000                 movaps	0x630(%rbp), %xmm0
// 0x140630196  0f288d40060000                 movaps	0x640(%rbp), %xmm1
// 0x14063019d  0f289550060000                 movaps	0x650(%rbp), %xmm2
// 0x1406301a4  0f289d60060000                 movaps	0x660(%rbp), %xmm3
// 0x1406301ab  0f299d50030000                 movaps	%xmm3, 0x350(%rbp)
// 0x1406301b2  0f299540030000                 movaps	%xmm2, 0x340(%rbp)
// 0x1406301b9  0f298d30030000                 movaps	%xmm1, 0x330(%rbp)
// 0x1406301c0  0f298520030000                 movaps	%xmm0, 0x320(%rbp)
// 0x1406301c7  0f1085c0010000                 movups	0x1c0(%rbp), %xmm0
// 0x1406301ce  0f2985f0030000                 movaps	%xmm0, 0x3f0(%rbp)
// 0x1406301d5  0f1085b0010000                 movups	0x1b0(%rbp), %xmm0
// 0x1406301dc  0f2985e0030000                 movaps	%xmm0, 0x3e0(%rbp)
// 0x1406301e3  0f1085a0010000                 movups	0x1a0(%rbp), %xmm0
// 0x1406301ea  0f2985d0030000                 movaps	%xmm0, 0x3d0(%rbp)
// 0x1406301f1  0f108560010000                 movups	0x160(%rbp), %xmm0
// 0x1406301f8  0f108d70010000                 movups	0x170(%rbp), %xmm1
// 0x1406301ff  0f109580010000                 movups	0x180(%rbp), %xmm2
// 0x140630206  0f109d90010000                 movups	0x190(%rbp), %xmm3
// 0x14063020d  0f299dc0030000                 movaps	%xmm3, 0x3c0(%rbp)
// 0x140630214  0f2995b0030000                 movaps	%xmm2, 0x3b0(%rbp)
// 0x14063021b  0f298da0030000                 movaps	%xmm1, 0x3a0(%rbp)
// 0x140630222  0f298590030000                 movaps	%xmm0, 0x390(%rbp)
// 0x140630229  0f108550010000                 movups	0x150(%rbp), %xmm0
// 0x140630230  0f298560040000                 movaps	%xmm0, 0x460(%rbp)
// 0x140630237  0f108540010000                 movups	0x140(%rbp), %xmm0
// 0x14063023e  0f298550040000                 movaps	%xmm0, 0x450(%rbp)
// 0x140630245  0f108530010000                 movups	0x130(%rbp), %xmm0
// 0x14063024c  0f298540040000                 movaps	%xmm0, 0x440(%rbp)
// 0x140630253  0f1085f0000000                 movups	0xf0(%rbp), %xmm0
// 0x14063025a  0f108d00010000                 movups	0x100(%rbp), %xmm1
// 0x140630261  0f109510010000                 movups	0x110(%rbp), %xmm2
// 0x140630268  0f109d20010000                 movups	0x120(%rbp), %xmm3
// 0x14063026f  0f299d30040000                 movaps	%xmm3, 0x430(%rbp)
// 0x140630276  0f299520040000                 movaps	%xmm2, 0x420(%rbp)
// 0x14063027d  0f298d10040000                 movaps	%xmm1, 0x410(%rbp)
// 0x140630284  0f298500040000                 movaps	%xmm0, 0x400(%rbp)
// 0x14063028b  0f1085e0000000                 movups	0xe0(%rbp), %xmm0
// 0x140630292  0f2985d0040000                 movaps	%xmm0, 0x4d0(%rbp)
// 0x140630299  0f1085d0000000                 movups	0xd0(%rbp), %xmm0
// 0x1406302a0  0f2985c0040000                 movaps	%xmm0, 0x4c0(%rbp)
// 0x1406302a7  0f1085c0000000                 movups	0xc0(%rbp), %xmm0
// 0x1406302ae  0f2985b0040000                 movaps	%xmm0, 0x4b0(%rbp)
// 0x1406302b5  0f108580000000                 movups	0x80(%rbp), %xmm0
// 0x1406302bc  0f108d90000000                 movups	0x90(%rbp), %xmm1
// 0x1406302c3  0f1095a0000000                 movups	0xa0(%rbp), %xmm2
// 0x1406302ca  0f109db0000000                 movups	0xb0(%rbp), %xmm3
// 0x1406302d1  0f299da0040000                 movaps	%xmm3, 0x4a0(%rbp)
// 0x1406302d8  0f299590040000                 movaps	%xmm2, 0x490(%rbp)
// 0x1406302df  0f298d80040000                 movaps	%xmm1, 0x480(%rbp)
// 0x1406302e6  0f298570040000                 movaps	%xmm0, 0x470(%rbp)
// 0x1406302ed  0f104570                       movups	0x70(%rbp), %xmm0
// 0x1406302f1  0f298540050000                 movaps	%xmm0, 0x540(%rbp)
// 0x1406302f8  0f104560                       movups	0x60(%rbp), %xmm0
// 0x1406302fc  0f298530050000                 movaps	%xmm0, 0x530(%rbp)
// 0x140630303  0f104550                       movups	0x50(%rbp), %xmm0
// 0x140630307  0f298520050000                 movaps	%xmm0, 0x520(%rbp)
// 0x14063030e  0f104510                       movups	0x10(%rbp), %xmm0
// 0x140630312  0f104d20                       movups	0x20(%rbp), %xmm1
// 0x140630316  0f105530                       movups	0x30(%rbp), %xmm2
// 0x14063031a  0f105d40                       movups	0x40(%rbp), %xmm3
// 0x14063031e  0f299d10050000                 movaps	%xmm3, 0x510(%rbp)
// 0x140630325  0f299500050000                 movaps	%xmm2, 0x500(%rbp)
// 0x14063032c  0f298df0040000                 movaps	%xmm1, 0x4f0(%rbp)
// 0x140630333  0f2985e0040000                 movaps	%xmm0, 0x4e0(%rbp)
// 0x14063033a  0f104500                       movups	(%rbp), %xmm0
// 0x14063033e  0f2985b0050000                 movaps	%xmm0, 0x5b0(%rbp)
// 0x140630345  0f1045f0                       movups	-0x10(%rbp), %xmm0
// 0x140630349  0f2985a0050000                 movaps	%xmm0, 0x5a0(%rbp)
// 0x140630350  0f1045e0                       movups	-0x20(%rbp), %xmm0
// 0x140630354  0f298590050000                 movaps	%xmm0, 0x590(%rbp)
// 0x14063035b  0f1045a0                       movups	-0x60(%rbp), %xmm0
// 0x14063035f  0f104db0                       movups	-0x50(%rbp), %xmm1
// 0x140630363  0f1055c0                       movups	-0x40(%rbp), %xmm2
// 0x140630367  0f105dd0                       movups	-0x30(%rbp), %xmm3
// 0x14063036b  0f299d80050000                 movaps	%xmm3, 0x580(%rbp)
// 0x140630372  0f299570050000                 movaps	%xmm2, 0x570(%rbp)
// 0x140630379  0f298d60050000                 movaps	%xmm1, 0x560(%rbp)
// 0x140630380  0f298550050000                 movaps	%xmm0, 0x550(%rbp)
// 0x140630387  0f108570070000                 movups	0x770(%rbp), %xmm0
// 0x14063038e  0f298520060000                 movaps	%xmm0, 0x620(%rbp)
// 0x140630395  0f108560070000                 movups	0x760(%rbp), %xmm0
// 0x14063039c  0f298510060000                 movaps	%xmm0, 0x610(%rbp)
// 0x1406303a3  0f108550070000                 movups	0x750(%rbp), %xmm0
// 0x1406303aa  0f298500060000                 movaps	%xmm0, 0x600(%rbp)
// 0x1406303b1  0f108510070000                 movups	0x710(%rbp), %xmm0
// 0x1406303b8  0f108d20070000                 movups	0x720(%rbp), %xmm1
// 0x1406303bf  0f109530070000                 movups	0x730(%rbp), %xmm2
// 0x1406303c6  0f109d40070000                 movups	0x740(%rbp), %xmm3
// 0x1406303cd  0f299df0050000                 movaps	%xmm3, 0x5f0(%rbp)
// 0x1406303d4  0f2995e0050000                 movaps	%xmm2, 0x5e0(%rbp)
// 0x1406303db  0f298dd0050000                 movaps	%xmm1, 0x5d0(%rbp)
// 0x1406303e2  0f2985c0050000                 movaps	%xmm0, 0x5c0(%rbp)
// 0x1406303e9  488d9540020000                 leaq	0x240(%rbp), %rdx
// Frames: F49 -> F50
// 0x1406303f0  4889f1                         movq	%rsi, %rcx
// 0x1406303f3  e868387a00                     callq	0x140dd3c60 <.text+0xdd2c60>
// Frames: F51
// 0x1406303f8  4889f0                         movq	%rsi, %rax
// 0x1406303fb  4881c410080000                 addq	$0x810, %rsp            # imm = 0x810
// 0x140630402  5f                             popq	%rdi
// 0x140630403  5e                             popq	%rsi
// 0x140630404  5d                             popq	%rbp
// 0x140630405  c3                             retq
// 0x140630406  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// 0x140630410  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x140630415  55                             pushq	%rbp
// 0x140630416  56                             pushq	%rsi
// 0x140630417  57                             pushq	%rdi
// 0x140630418  4883ec20                       subq	$0x20, %rsp
// 0x14063041c  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x140630423  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// Frames: F52
// 0x14063042a  e851949fff                     callq	0x140029880 <.text+0x28880>
// Frames: F1
// 0x14063042f  90                             nop
// 0x140630430  4883c420                       addq	$0x20, %rsp
// 0x140630434  5f                             popq	%rdi
// 0x140630435  5e                             popq	%rsi
// 0x140630436  5d                             popq	%rbp
// 0x140630437  c3                             retq
// 0x140630438  0f1f840000000000               nopl	(%rax,%rax)
// 0x140630440  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x140630445  55                             pushq	%rbp
// 0x140630446  56                             pushq	%rsi
// 0x140630447  57                             pushq	%rdi
// 0x140630448  4883ec20                       subq	$0x20, %rsp
// 0x14063044c  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x140630453  488d8d40020000                 leaq	0x240(%rbp), %rcx
// Frames: F53 -> F35
// 0x14063045a  e821949fff                     callq	0x140029880 <.text+0x28880>
// 0x14063045f  90                             nop
// 0x140630460  4883c420                       addq	$0x20, %rsp
// 0x140630464  5f                             popq	%rdi
// 0x140630465  5e                             popq	%rsi
// 0x140630466  5d                             popq	%rbp
// 0x140630467  c3                             retq
// 0x140630468  0f1f840000000000               nopl	(%rax,%rax)
// 0x140630470  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x140630475  55                             pushq	%rbp
// 0x140630476  56                             pushq	%rsi
// 0x140630477  57                             pushq	%rdi
// 0x140630478  4883ec20                       subq	$0x20, %rsp
// 0x14063047c  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x140630483  488d8da0060000                 leaq	0x6a0(%rbp), %rcx
// Frames: F52
// 0x14063048a  e8f1939fff                     callq	0x140029880 <.text+0x28880>
// 0x14063048f  90                             nop
// 0x140630490  4883c420                       addq	$0x20, %rsp
// 0x140630494  5f                             popq	%rdi
// 0x140630495  5e                             popq	%rsi
// 0x140630496  5d                             popq	%rbp
// 0x140630497  c3                             retq
// 0x140630498  0f1f840000000000               nopl	(%rax,%rax)
// 0x1406304a0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406304a5  55                             pushq	%rbp
// 0x1406304a6  56                             pushq	%rsi
// 0x1406304a7  57                             pushq	%rdi
// 0x1406304a8  4883ec20                       subq	$0x20, %rsp
// 0x1406304ac  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406304b3  488d8d40020000                 leaq	0x240(%rbp), %rcx
// Frames: F53 -> F41
// 0x1406304ba  e8c1939fff                     callq	0x140029880 <.text+0x28880>
// 0x1406304bf  90                             nop
// 0x1406304c0  4883c420                       addq	$0x20, %rsp
// 0x1406304c4  5f                             popq	%rdi
// 0x1406304c5  5e                             popq	%rsi
// 0x1406304c6  5d                             popq	%rbp
// 0x1406304c7  c3                             retq
// 0x1406304c8  0f1f840000000000               nopl	(%rax,%rax)
// 0x1406304d0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406304d5  55                             pushq	%rbp
// 0x1406304d6  56                             pushq	%rsi
// 0x1406304d7  57                             pushq	%rdi
// 0x1406304d8  4883ec20                       subq	$0x20, %rsp
// 0x1406304dc  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406304e3  488d8d30060000                 leaq	0x630(%rbp), %rcx
// Frames: F52
// 0x1406304ea  e891939fff                     callq	0x140029880 <.text+0x28880>
// 0x1406304ef  90                             nop
// 0x1406304f0  4883c420                       addq	$0x20, %rsp
// 0x1406304f4  5f                             popq	%rdi
// 0x1406304f5  5e                             popq	%rsi
// 0x1406304f6  5d                             popq	%rbp
// 0x1406304f7  c3                             retq
// 0x1406304f8  0f1f840000000000               nopl	(%rax,%rax)
// 0x140630500  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x140630505  55                             pushq	%rbp
// 0x140630506  56                             pushq	%rsi
// 0x140630507  57                             pushq	%rdi
// 0x140630508  4883ec20                       subq	$0x20, %rsp
// 0x14063050c  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x140630513  488d8d60010000                 leaq	0x160(%rbp), %rcx
// 0x14063051a  e861939fff                     callq	0x140029880 <.text+0x28880>
// 0x14063051f  90                             nop
// 0x140630520  4883c420                       addq	$0x20, %rsp
// 0x140630524  5f                             popq	%rdi
// 0x140630525  5e                             popq	%rsi
// 0x140630526  5d                             popq	%rbp
// 0x140630527  c3                             retq
// 0x140630528  0f1f840000000000               nopl	(%rax,%rax)
// 0x140630530  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x140630535  55                             pushq	%rbp
// 0x140630536  56                             pushq	%rsi
// 0x140630537  57                             pushq	%rdi
// 0x140630538  4883ec20                       subq	$0x20, %rsp
// 0x14063053c  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x140630543  488d8df0000000                 leaq	0xf0(%rbp), %rcx
// 0x14063054a  e831939fff                     callq	0x140029880 <.text+0x28880>
// 0x14063054f  90                             nop
// 0x140630550  4883c420                       addq	$0x20, %rsp
// 0x140630554  5f                             popq	%rdi
// 0x140630555  5e                             popq	%rsi
// 0x140630556  5d                             popq	%rbp
// 0x140630557  c3                             retq
// 0x140630558  0f1f840000000000               nopl	(%rax,%rax)
// 0x140630560  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x140630565  55                             pushq	%rbp
// 0x140630566  56                             pushq	%rsi
// 0x140630567  57                             pushq	%rdi
// 0x140630568  4883ec20                       subq	$0x20, %rsp
// 0x14063056c  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x140630573  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x14063057a  e801939fff                     callq	0x140029880 <.text+0x28880>
// 0x14063057f  90                             nop
// 0x140630580  4883c420                       addq	$0x20, %rsp
// 0x140630584  5f                             popq	%rdi
// 0x140630585  5e                             popq	%rsi
// 0x140630586  5d                             popq	%rbp
// 0x140630587  c3                             retq
// 0x140630588  0f1f840000000000               nopl	(%rax,%rax)
// 0x140630590  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x140630595  55                             pushq	%rbp
// 0x140630596  56                             pushq	%rsi
// 0x140630597  57                             pushq	%rdi
// 0x140630598  4883ec20                       subq	$0x20, %rsp
// 0x14063059c  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406305a3  488d4d10                       leaq	0x10(%rbp), %rcx
// 0x1406305a7  e8d4929fff                     callq	0x140029880 <.text+0x28880>
// 0x1406305ac  90                             nop
// 0x1406305ad  4883c420                       addq	$0x20, %rsp
// 0x1406305b1  5f                             popq	%rdi
// 0x1406305b2  5e                             popq	%rsi
// 0x1406305b3  5d                             popq	%rbp
// 0x1406305b4  c3                             retq
// 0x1406305b5  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// 0x1406305c0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1406305c5  55                             pushq	%rbp
// 0x1406305c6  56                             pushq	%rsi
// 0x1406305c7  57                             pushq	%rdi
// 0x1406305c8  4883ec20                       subq	$0x20, %rsp
// 0x1406305cc  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1406305d3  488d4da0                       leaq	-0x60(%rbp), %rcx
// 0x1406305d7  e8a4929fff                     callq	0x140029880 <.text+0x28880>
// 0x1406305dc  90                             nop
// 0x1406305dd  4883c420                       addq	$0x20, %rsp
// 0x1406305e1  5f                             popq	%rdi
// 0x1406305e2  5e                             popq	%rsi
// 0x1406305e3  5d                             popq	%rbp
// 0x1406305e4  c3                             retq
//
// RANGE rva_6305f0_len_42b: RVA 0x6305f0, 1067 PDB bytes; 237 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_6305f0_len_42b.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 271:3888: system_roof::impl$0::add_roof_events_and_resources
// Frames: F54
// 0x1406305f0  56                             pushq	%rsi
// 0x1406305f1  57                             pushq	%rdi
// 0x1406305f2  4883ec58                       subq	$0x58, %rsp
// Frames: F55
// 0x1406305f6  c744242002000000               movl	$0x2, 0x20(%rsp)
// 0x1406305fe  488d542420                     leaq	0x20(%rsp), %rdx
// 0x140630603  e878577b00                     callq	0x140de5d80 <.text+0xde4d80>
// 0x140630608  4889c1                         movq	%rax, %rcx
// 0x14063060b  4531c0                         xorl	%r8d, %r8d
// 0x14063060e  e82d527b00                     callq	0x140de5840 <.text+0xde4840>
// 0x140630613  4889c6                         movq	%rax, %rsi
// Frames: F2 -> F3 -> F4 -> F55
// 0x140630616  4889c7                         movq	%rax, %rdi
// 0x140630619  4883c738                       addq	$0x38, %rdi
// 0x14063061d  48ba704fde58fb12f876           movabsq	$0x76f812fb58de4f70, %rdx # imm = 0x76F812FB58DE4F70
// 0x140630627  49b865a809dd511da512           movabsq	$0x12a51d51dd09a865, %r8 # imm = 0x12A51D51DD09A865
// 0x140630631  4889f9                         movq	%rdi, %rcx
// 0x140630634  e8b7d0aaff                     callq	0x1400dd6f0 <.text+0xdc6f0>
// Frames: F6 -> F7 -> F3 -> F4 -> F55
// 0x140630639  4885c0                         testq	%rax, %rax
// 0x14063063c  7433                           je	0x140630671 <.text+0x62f671>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063063e  48399618020000                 cmpq	%rdx, 0x218(%rsi)
// 0x140630645  762a                           jbe	0x140630671 <.text+0x62f671>
// Frames: F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630647  488b8610020000                 movq	0x210(%rsi), %rax
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063064e  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x140630652  4885c0                         testq	%rax, %rax
// Frames: F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630655  741a                           je	0x140630671 <.text+0x62f671>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630657  488b8ee0010000                 movq	0x1e0(%rsi), %rcx
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063065e  48f7d0                         notq	%rax
// Frames: F27 -> F28 -> F3 -> F4 -> F55
// 0x140630661  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x140630665  48c1e005                       shlq	$0x5, %rax
// Frames: F29 -> F30 -> F16 -> F27 -> F28 -> F3 -> F4 -> F55
// 0x140630669  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F3 -> F4 -> F55
// 0x14063066f  7508                           jne	0x140630679 <.text+0x62f679>
// Frames: F31 -> F4 -> F55
// 0x140630671  4889f1                         movq	%rsi, %rcx
// 0x140630674  e8a76f7a00                     callq	0x140dd7620 <.text+0xdd6620>
// Frames: F2 -> F3 -> F4 -> F55
// 0x140630679  48ba4e8a052f216bf17d           movabsq	$0x7df16b212f058a4e, %rdx # imm = 0x7DF16B212F058A4E
// 0x140630683  49b82d55ea0ecbaad681           movabsq	$-0x7e295534f115aad3, %r8 # imm = 0x81D6AACB0EEA552D
// 0x14063068d  4889f9                         movq	%rdi, %rcx
// 0x140630690  e85bd0aaff                     callq	0x1400dd6f0 <.text+0xdc6f0>
// Frames: F6 -> F7 -> F3 -> F4 -> F55
// 0x140630695  4885c0                         testq	%rax, %rax
// 0x140630698  7433                           je	0x1406306cd <.text+0x62f6cd>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063069a  48399618020000                 cmpq	%rdx, 0x218(%rsi)
// 0x1406306a1  762a                           jbe	0x1406306cd <.text+0x62f6cd>
// Frames: F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406306a3  488b8610020000                 movq	0x210(%rsi), %rax
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406306aa  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x1406306ae  4885c0                         testq	%rax, %rax
// Frames: F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406306b1  741a                           je	0x1406306cd <.text+0x62f6cd>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406306b3  488b8ee0010000                 movq	0x1e0(%rsi), %rcx
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406306ba  48f7d0                         notq	%rax
// Frames: F27 -> F28 -> F3 -> F4 -> F55
// 0x1406306bd  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1406306c1  48c1e005                       shlq	$0x5, %rax
// Frames: F29 -> F30 -> F16 -> F27 -> F28 -> F3 -> F4 -> F55
// 0x1406306c5  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F3 -> F4 -> F55
// 0x1406306cb  7508                           jne	0x1406306d5 <.text+0x62f6d5>
// Frames: F31 -> F4 -> F55
// 0x1406306cd  4889f1                         movq	%rsi, %rcx
// 0x1406306d0  e82b4a7a00                     callq	0x140dd5100 <.text+0xdd4100>
// Frames: F2 -> F3 -> F4 -> F55
// 0x1406306d5  48ba1039721173b4d6c9           movabsq	$-0x36294b8cee8dc6f0, %rdx # imm = 0xC9D6B47311723910
// 0x1406306df  49b867a8d265a7abb918           movabsq	$0x18b9aba765d2a867, %r8 # imm = 0x18B9ABA765D2A867
// 0x1406306e9  4889f9                         movq	%rdi, %rcx
// 0x1406306ec  e8ffcfaaff                     callq	0x1400dd6f0 <.text+0xdc6f0>
// Frames: F6 -> F7 -> F3 -> F4 -> F55
// 0x1406306f1  4885c0                         testq	%rax, %rax
// 0x1406306f4  7433                           je	0x140630729 <.text+0x62f729>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406306f6  48399618020000                 cmpq	%rdx, 0x218(%rsi)
// 0x1406306fd  762a                           jbe	0x140630729 <.text+0x62f729>
// Frames: F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406306ff  488b8610020000                 movq	0x210(%rsi), %rax
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630706  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x14063070a  4885c0                         testq	%rax, %rax
// Frames: F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063070d  741a                           je	0x140630729 <.text+0x62f729>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063070f  488b8ee0010000                 movq	0x1e0(%rsi), %rcx
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630716  48f7d0                         notq	%rax
// Frames: F27 -> F28 -> F3 -> F4 -> F55
// 0x140630719  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x14063071d  48c1e005                       shlq	$0x5, %rax
// Frames: F29 -> F30 -> F16 -> F27 -> F28 -> F3 -> F4 -> F55
// 0x140630721  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F3 -> F4 -> F55
// 0x140630727  7508                           jne	0x140630731 <.text+0x62f731>
// Frames: F31 -> F4 -> F55
// 0x140630729  4889f1                         movq	%rsi, %rcx
// 0x14063072c  e88f627a00                     callq	0x140dd69c0 <.text+0xdd59c0>
// Frames: F2 -> F3 -> F4 -> F55
// 0x140630731  48baf4dd0f8046275179           movabsq	$0x79512746800fddf4, %rdx # imm = 0x79512746800FDDF4
// 0x14063073b  49b8a71b37aadb1b43c4           movabsq	$-0x3bbce42455c8e459, %r8 # imm = 0xC4431BDBAA371BA7
// 0x140630745  4889f9                         movq	%rdi, %rcx
// 0x140630748  e8a3cfaaff                     callq	0x1400dd6f0 <.text+0xdc6f0>
// Frames: F6 -> F7 -> F3 -> F4 -> F55
// 0x14063074d  4885c0                         testq	%rax, %rax
// 0x140630750  7433                           je	0x140630785 <.text+0x62f785>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630752  48399618020000                 cmpq	%rdx, 0x218(%rsi)
// 0x140630759  762a                           jbe	0x140630785 <.text+0x62f785>
// Frames: F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063075b  488b8610020000                 movq	0x210(%rsi), %rax
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630762  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x140630766  4885c0                         testq	%rax, %rax
// Frames: F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630769  741a                           je	0x140630785 <.text+0x62f785>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063076b  488b8ee0010000                 movq	0x1e0(%rsi), %rcx
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630772  48f7d0                         notq	%rax
// Frames: F27 -> F28 -> F3 -> F4 -> F55
// 0x140630775  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x140630779  48c1e005                       shlq	$0x5, %rax
// Frames: F29 -> F30 -> F16 -> F27 -> F28 -> F3 -> F4 -> F55
// 0x14063077d  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F3 -> F4 -> F55
// 0x140630783  7508                           jne	0x14063078d <.text+0x62f78d>
// Frames: F31 -> F4 -> F55
// 0x140630785  4889f1                         movq	%rsi, %rcx
// 0x140630788  e8035c7a00                     callq	0x140dd6390 <.text+0xdd5390>
// Frames: F2 -> F3 -> F4 -> F55
// 0x14063078d  48ba15d7a2eace822ed7           movabsq	$-0x28d17d31155d28eb, %rdx # imm = 0xD72E82CEEAA2D715
// 0x140630797  49b8c410350ce2225122           movabsq	$0x225122e20c3510c4, %r8 # imm = 0x225122E20C3510C4
// 0x1406307a1  4889f9                         movq	%rdi, %rcx
// 0x1406307a4  e847cfaaff                     callq	0x1400dd6f0 <.text+0xdc6f0>
// Frames: F6 -> F7 -> F3 -> F4 -> F55
// 0x1406307a9  4885c0                         testq	%rax, %rax
// 0x1406307ac  7433                           je	0x1406307e1 <.text+0x62f7e1>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406307ae  48399618020000                 cmpq	%rdx, 0x218(%rsi)
// 0x1406307b5  762a                           jbe	0x1406307e1 <.text+0x62f7e1>
// Frames: F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406307b7  488b8610020000                 movq	0x210(%rsi), %rax
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406307be  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x1406307c2  4885c0                         testq	%rax, %rax
// Frames: F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406307c5  741a                           je	0x1406307e1 <.text+0x62f7e1>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406307c7  488b8ee0010000                 movq	0x1e0(%rsi), %rcx
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406307ce  48f7d0                         notq	%rax
// Frames: F27 -> F28 -> F3 -> F4 -> F55
// 0x1406307d1  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1406307d5  48c1e005                       shlq	$0x5, %rax
// Frames: F29 -> F30 -> F16 -> F27 -> F28 -> F3 -> F4 -> F55
// 0x1406307d9  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F3 -> F4 -> F55
// 0x1406307df  7508                           jne	0x1406307e9 <.text+0x62f7e9>
// Frames: F31 -> F4 -> F55
// 0x1406307e1  4889f1                         movq	%rsi, %rcx
// 0x1406307e4  e867747a00                     callq	0x140dd7c50 <.text+0xdd6c50>
// Frames: F2 -> F3 -> F4 -> F55
// 0x1406307e9  48ba416ea47d363f519b           movabsq	$-0x64aec0c9825b91bf, %rdx # imm = 0x9B513F367DA46E41
// 0x1406307f3  49b8503d24b830340661           movabsq	$0x61063430b8243d50, %r8 # imm = 0x61063430B8243D50
// 0x1406307fd  4889f9                         movq	%rdi, %rcx
// 0x140630800  e8ebceaaff                     callq	0x1400dd6f0 <.text+0xdc6f0>
// Frames: F6 -> F7 -> F3 -> F4 -> F55
// 0x140630805  4885c0                         testq	%rax, %rax
// 0x140630808  7433                           je	0x14063083d <.text+0x62f83d>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063080a  48399618020000                 cmpq	%rdx, 0x218(%rsi)
// 0x140630811  762a                           jbe	0x14063083d <.text+0x62f83d>
// Frames: F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630813  488b8610020000                 movq	0x210(%rsi), %rax
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063081a  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x14063081e  4885c0                         testq	%rax, %rax
// Frames: F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630821  741a                           je	0x14063083d <.text+0x62f83d>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630823  488b8ee0010000                 movq	0x1e0(%rsi), %rcx
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063082a  48f7d0                         notq	%rax
// Frames: F27 -> F28 -> F3 -> F4 -> F55
// 0x14063082d  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x140630831  48c1e005                       shlq	$0x5, %rax
// Frames: F29 -> F30 -> F16 -> F27 -> F28 -> F3 -> F4 -> F55
// 0x140630835  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F3 -> F4 -> F55
// 0x14063083b  7508                           jne	0x140630845 <.text+0x62f845>
// Frames: F31 -> F4 -> F55
// 0x14063083d  4889f1                         movq	%rsi, %rcx
// 0x140630840  e88b427a00                     callq	0x140dd4ad0 <.text+0xdd3ad0>
// Frames: F2 -> F3 -> F4 -> F55
// 0x140630845  48ba215754ba194b7044           movabsq	$0x44704b19ba545721, %rdx # imm = 0x44704B19BA545721
// 0x14063084f  49b8ae9208dcda3273b7           movabsq	$-0x488ccd2523f76d52, %r8 # imm = 0xB77332DADC0892AE
// 0x140630859  4889f9                         movq	%rdi, %rcx
// 0x14063085c  e88fceaaff                     callq	0x1400dd6f0 <.text+0xdc6f0>
// Frames: F6 -> F7 -> F3 -> F4 -> F55
// 0x140630861  4885c0                         testq	%rax, %rax
// 0x140630864  7433                           je	0x140630899 <.text+0x62f899>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630866  48399618020000                 cmpq	%rdx, 0x218(%rsi)
// 0x14063086d  762a                           jbe	0x140630899 <.text+0x62f899>
// Frames: F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063086f  488b8610020000                 movq	0x210(%rsi), %rax
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630876  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x14063087a  4885c0                         testq	%rax, %rax
// Frames: F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063087d  741a                           je	0x140630899 <.text+0x62f899>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063087f  488b8ee0010000                 movq	0x1e0(%rsi), %rcx
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630886  48f7d0                         notq	%rax
// Frames: F27 -> F28 -> F3 -> F4 -> F55
// 0x140630889  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x14063088d  48c1e005                       shlq	$0x5, %rax
// Frames: F29 -> F30 -> F16 -> F27 -> F28 -> F3 -> F4 -> F55
// 0x140630891  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F3 -> F4 -> F55
// 0x140630897  7508                           jne	0x1406308a1 <.text+0x62f8a1>
// Frames: F31 -> F4 -> F55
// 0x140630899  4889f1                         movq	%rsi, %rcx
// 0x14063089c  e80f807a00                     callq	0x140dd88b0 <.text+0xdd78b0>
// Frames: F2 -> F3 -> F4 -> F55
// 0x1406308a1  48ba2b76e859c667d230           movabsq	$0x30d267c659e8762b, %rdx # imm = 0x30D267C659E8762B
// 0x1406308ab  49b8aaf67d0acba78b4c           movabsq	$0x4c8ba7cb0a7df6aa, %r8 # imm = 0x4C8BA7CB0A7DF6AA
// 0x1406308b5  4889f9                         movq	%rdi, %rcx
// 0x1406308b8  e833ceaaff                     callq	0x1400dd6f0 <.text+0xdc6f0>
// Frames: F6 -> F7 -> F3 -> F4 -> F55
// 0x1406308bd  4885c0                         testq	%rax, %rax
// 0x1406308c0  7433                           je	0x1406308f5 <.text+0x62f8f5>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406308c2  48399618020000                 cmpq	%rdx, 0x218(%rsi)
// 0x1406308c9  762a                           jbe	0x1406308f5 <.text+0x62f8f5>
// Frames: F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406308cb  488b8610020000                 movq	0x210(%rsi), %rax
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406308d2  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x1406308d6  4885c0                         testq	%rax, %rax
// Frames: F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406308d9  741a                           je	0x1406308f5 <.text+0x62f8f5>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406308db  488b8ee0010000                 movq	0x1e0(%rsi), %rcx
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406308e2  48f7d0                         notq	%rax
// Frames: F27 -> F28 -> F3 -> F4 -> F55
// 0x1406308e5  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1406308e9  48c1e005                       shlq	$0x5, %rax
// Frames: F29 -> F30 -> F16 -> F27 -> F28 -> F3 -> F4 -> F55
// 0x1406308ed  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F3 -> F4 -> F55
// 0x1406308f3  7508                           jne	0x1406308fd <.text+0x62f8fd>
// Frames: F31 -> F4 -> F55
// 0x1406308f5  4889f1                         movq	%rsi, %rcx
// 0x1406308f8  e873357a00                     callq	0x140dd3e70 <.text+0xdd2e70>
// Frames: F2 -> F3 -> F4 -> F55
// 0x1406308fd  48ba664ffdd63b0f1379           movabsq	$0x79130f3bd6fd4f66, %rdx # imm = 0x79130F3BD6FD4F66
// 0x140630907  49b8f7994a6d808a3074           movabsq	$0x74308a806d4a99f7, %r8 # imm = 0x74308A806D4A99F7
// 0x140630911  4889f9                         movq	%rdi, %rcx
// 0x140630914  e8d7cdaaff                     callq	0x1400dd6f0 <.text+0xdc6f0>
// Frames: F6 -> F7 -> F3 -> F4 -> F55
// 0x140630919  4885c0                         testq	%rax, %rax
// 0x14063091c  7433                           je	0x140630951 <.text+0x62f951>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063091e  48399618020000                 cmpq	%rdx, 0x218(%rsi)
// 0x140630925  762a                           jbe	0x140630951 <.text+0x62f951>
// Frames: F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630927  488b8610020000                 movq	0x210(%rsi), %rax
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063092e  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x140630932  4885c0                         testq	%rax, %rax
// Frames: F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630935  741a                           je	0x140630951 <.text+0x62f951>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630937  488b8ee0010000                 movq	0x1e0(%rsi), %rcx
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063093e  48f7d0                         notq	%rax
// Frames: F27 -> F28 -> F3 -> F4 -> F55
// 0x140630941  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x140630945  48c1e005                       shlq	$0x5, %rax
// Frames: F29 -> F30 -> F16 -> F27 -> F28 -> F3 -> F4 -> F55
// 0x140630949  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F3 -> F4 -> F55
// 0x14063094f  7508                           jne	0x140630959 <.text+0x62f959>
// Frames: F31 -> F4 -> F55
// 0x140630951  4889f1                         movq	%rsi, %rcx
// 0x140630954  e887857a00                     callq	0x140dd8ee0 <.text+0xdd7ee0>
// Frames: F2 -> F3 -> F4 -> F55
// 0x140630959  48ba3a00b4314d5248e1           movabsq	$-0x1eb7adb2ce4bffc6, %rdx # imm = 0xE148524D31B4003A
// 0x140630963  49b89ba8c83e6e1ce04f           movabsq	$0x4fe01c6e3ec8a89b, %r8 # imm = 0x4FE01C6E3EC8A89B
// 0x14063096d  4889f9                         movq	%rdi, %rcx
// 0x140630970  e87bcdaaff                     callq	0x1400dd6f0 <.text+0xdc6f0>
// Frames: F6 -> F7 -> F3 -> F4 -> F55
// 0x140630975  4885c0                         testq	%rax, %rax
// 0x140630978  7433                           je	0x1406309ad <.text+0x62f9ad>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063097a  48399618020000                 cmpq	%rdx, 0x218(%rsi)
// 0x140630981  762a                           jbe	0x1406309ad <.text+0x62f9ad>
// Frames: F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630983  488b8610020000                 movq	0x210(%rsi), %rax
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063098a  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x14063098e  4885c0                         testq	%rax, %rax
// Frames: F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630991  741a                           je	0x1406309ad <.text+0x62f9ad>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x140630993  488b8ee0010000                 movq	0x1e0(%rsi), %rcx
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x14063099a  48f7d0                         notq	%rax
// Frames: F27 -> F28 -> F3 -> F4 -> F55
// 0x14063099d  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1406309a1  48c1e005                       shlq	$0x5, %rax
// Frames: F29 -> F30 -> F16 -> F27 -> F28 -> F3 -> F4 -> F55
// 0x1406309a5  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F3 -> F4 -> F55
// 0x1406309ab  7508                           jne	0x1406309b5 <.text+0x62f9b5>
// Frames: F31 -> F4 -> F55
// 0x1406309ad  4889f1                         movq	%rsi, %rcx
// 0x1406309b0  e8eb3a7a00                     callq	0x140dd44a0 <.text+0xdd34a0>
// Frames: F2 -> F3 -> F4 -> F55
// 0x1406309b5  48ba925141826e34764e           movabsq	$0x4e76346e82415192, %rdx # imm = 0x4E76346E82415192
// 0x1406309bf  49b8175656704a9c703e           movabsq	$0x3e709c4a70565617, %r8 # imm = 0x3E709C4A70565617
// 0x1406309c9  4889f9                         movq	%rdi, %rcx
// 0x1406309cc  e81fcdaaff                     callq	0x1400dd6f0 <.text+0xdc6f0>
// Frames: F6 -> F7 -> F3 -> F4 -> F55
// 0x1406309d1  4885c0                         testq	%rax, %rax
// 0x1406309d4  7433                           je	0x140630a09 <.text+0x62fa09>
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406309d6  48399618020000                 cmpq	%rdx, 0x218(%rsi)
// 0x1406309dd  762a                           jbe	0x140630a09 <.text+0x62fa09>
// Frames: F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406309df  488b8610020000                 movq	0x210(%rsi), %rax
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406309e6  488b04d0                       movq	(%rax,%rdx,8), %rax
// 0x1406309ea  4885c0                         testq	%rax, %rax
// Frames: F17 -> F10 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406309ed  741a                           je	0x140630a09 <.text+0x62fa09>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406309ef  488b8ee0010000                 movq	0x1e0(%rsi), %rcx
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F13 -> F14 -> F7 -> F3 -> F4 -> F55
// 0x1406309f6  48f7d0                         notq	%rax
// Frames: F27 -> F28 -> F3 -> F4 -> F55
// 0x1406309f9  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1406309fd  48c1e005                       shlq	$0x5, %rax
// Frames: F29 -> F30 -> F16 -> F27 -> F28 -> F3 -> F4 -> F55
// 0x140630a01  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F3 -> F4 -> F55
// 0x140630a07  7508                           jne	0x140630a11 <.text+0x62fa11>
// Frames: F31 -> F4 -> F55
// 0x140630a09  4889f1                         movq	%rsi, %rcx
// 0x140630a0c  e84f537a00                     callq	0x140dd5d60 <.text+0xdd4d60>
// Frames: F56
// 0x140630a11  4889f0                         movq	%rsi, %rax
// 0x140630a14  4883c458                       addq	$0x58, %rsp
// 0x140630a18  5f                             popq	%rdi
// 0x140630a19  5e                             popq	%rsi
// 0x140630a1a  c3                             retq
//
// RANGE rva_de50a0_len_21a: RVA 0xde50a0, 538 PDB bytes; 108 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_de50a0_len_21a.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 924:1036: bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >
// Frames: F57
// 0x140de50a0  56                             pushq	%rsi
// 0x140de50a1  57                             pushq	%rdi
// 0x140de50a2  55                             pushq	%rbp
// 0x140de50a3  53                             pushq	%rbx
// 0x140de50a4  4881ec88000000                 subq	$0x88, %rsp
// 0x140de50ab  bd01000000                     movl	$0x1, %ebp
// Frames: F58 -> F59 -> F60 -> F61 -> F62
// 0x140de50b0  f0                             lock
// 0x140de50b1  0fc1aab0050000                 xaddl	%ebp, 0x5b0(%rdx)
// Frames: F63 -> F64 -> F62
// 0x140de50b8  f60101                         testb	$0x1, (%rcx)
// 0x140de50bb  0f84d4010000                   je	0x140de5295 <.text+0xde4295>
// Frames: F65 -> F62
// 0x140de50c1  4889ce                         movq	%rcx, %rsi
// 0x140de50c4  488b4108                       movq	0x8(%rcx), %rax
// Frames: F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F62
// 0x140de50c8  4c8b8218020000                 movq	0x218(%rdx), %r8
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F62
// 0x140de50cf  4939c0                         cmpq	%rax, %r8
// 0x140de50d2  0f86b3000000                   jbe	0x140de518b <.text+0xde418b>
// Frames: F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F62
// 0x140de50d8  4c8b8a10020000                 movq	0x210(%rdx), %r9
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F62
// 0x140de50df  498b0cc1                       movq	(%r9,%rax,8), %rcx
// 0x140de50e3  4885c9                         testq	%rcx, %rcx
// Frames: F17 -> F10 -> F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F62
// 0x140de50e6  0f849f000000                   je	0x140de518b <.text+0xde418b>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F62
// 0x140de50ec  488b82e0010000                 movq	0x1e0(%rdx), %rax
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F62
// 0x140de50f3  48f7d1                         notq	%rcx
// Frames: F69 -> F70 -> F71 -> F24 -> F25 -> F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F62
// 0x140de50f6  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x140de50fa  48c1e105                       shlq	$0x5, %rcx
// Frames: F29 -> F30 -> F72 -> F73 -> F67 -> F68 -> F65 -> F62
// 0x140de50fe  48837c082000                   cmpq	$0x0, 0x20(%rax,%rcx)
// Frames: F74 -> F72 -> F73 -> F67 -> F68 -> F65 -> F62
// 0x140de5104  0f8481000000                   je	0x140de518b <.text+0xde418b>
// Frames: F65 -> F62
// 0x140de510a  4c8b5610                       movq	0x10(%rsi), %r10
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F75 -> F76 -> F68 -> F65 -> F62
// 0x140de510e  4d39d0                         cmpq	%r10, %r8
// 0x140de5111  0f86f9000000                   jbe	0x140de5210 <.text+0xde4210>
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F75 -> F76 -> F68 -> F65 -> F62
// 0x140de5117  4f8b04d1                       movq	(%r9,%r10,8), %r8
// 0x140de511b  4d85c0                         testq	%r8, %r8
// Frames: F17 -> F10 -> F11 -> F12 -> F75 -> F76 -> F68 -> F65 -> F62
// 0x140de511e  0f84ec000000                   je	0x140de5210 <.text+0xde4210>
// Frames: F77 -> F24 -> F25 -> F11 -> F12 -> F75 -> F76 -> F68 -> F65 -> F62
// 0x140de5124  49f7d0                         notq	%r8
// Frames: F69 -> F70 -> F78 -> F24 -> F25 -> F11 -> F12 -> F75 -> F76 -> F68 -> F65 -> F62
// 0x140de5127  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x140de512b  49c1e005                       shlq	$0x5, %r8
// Frames: F29 -> F30 -> F72 -> F79 -> F76 -> F68 -> F65 -> F62
// 0x140de512f  4a837c002000                   cmpq	$0x0, 0x20(%rax,%r8)
// Frames: F74 -> F72 -> F79 -> F76 -> F68 -> F65 -> F62
// 0x140de5135  0f84d5000000                   je	0x140de5210 <.text+0xde4210>
// Frames: F68 -> F65 -> F62
// 0x140de513b  4801c1                         addq	%rax, %rcx
// 0x140de513e  488b7910                       movq	0x10(%rcx), %rdi
// 0x140de5142  4901c0                         addq	%rax, %r8
// Frames: F80 -> F81 -> F82 -> F74 -> F72 -> F79 -> F76 -> F68 -> F65 -> F62
// 0x140de5145  498b4810                       movq	0x10(%r8), %rcx
// Frames: F83 -> F84 -> F85 -> F86 -> F87 -> F76 -> F68 -> F65 -> F62
// 0x140de5149  8b82b0050000                   movl	0x5b0(%rdx), %eax
// Frames: F88 -> F89 -> F90 -> F91 -> F92 -> F93 -> F94 -> F95 -> F62
// 0x140de514f  4189685c                       movl	%ebp, 0x5c(%r8)
// Frames: F90 -> F91 -> F92 -> F93 -> F94 -> F95 -> F62
// 0x140de5153  e8585cc500                     callq	0x141a3adb0 <.text+0x1a39db0>
// Frames: F96 -> F90 -> F91 -> F92 -> F93 -> F94 -> F95 -> F62
// 0x140de5158  4885c0                         testq	%rax, %rax
// 0x140de515b  0f844c010000                   je	0x140de52ad <.text+0xde42ad>
// Frames: F97 -> F91 -> F92 -> F93 -> F94 -> F95 -> F62
// 0x140de5161  4889c3                         movq	%rax, %rbx
// 0x140de5164  4889f9                         movq	%rdi, %rcx
// 0x140de5167  e8342bbaff                     callq	0x140987ca0 <.text+0x986ca0>
// 0x140de516c  4889d9                         movq	%rbx, %rcx
// 0x140de516f  4889c2                         movq	%rax, %rdx
// 0x140de5172  e8095ec500                     callq	0x141a3af80 <.text+0x1a39f80>
// Frames: F98 -> F62
// 0x140de5177  89ae50010000                   movl	%ebp, 0x150(%rsi)
// Frames: F99
// 0x140de517d  31c0                           xorl	%eax, %eax
// 0x140de517f  4881c488000000                 addq	$0x88, %rsp
// 0x140de5186  5b                             popq	%rbx
// 0x140de5187  5d                             popq	%rbp
// 0x140de5188  5f                             popq	%rdi
// 0x140de5189  5e                             popq	%rsi
// 0x140de518a  c3                             retq
// Frames: F100 -> F101 -> F102 -> F103 -> F104 -> F105 -> F68 -> F65 -> F62
// 0x140de518b  4881c6b8000000                 addq	$0xb8, %rsi
// Frames: F106 -> F107 -> F104 -> F105 -> F68 -> F65 -> F62
// 0x140de5192  488d05711ad901                 leaq	0x1d91a71(%rip), %rax   # 0x142b76c0a
// 0x140de5199  4889442428                     movq	%rax, 0x28(%rsp)
// 0x140de519e  48c74424302c000000             movq	$0x2c, 0x30(%rsp)
// Frames: F103 -> F104 -> F105 -> F68 -> F65 -> F62
// 0x140de51a7  4889742438                     movq	%rsi, 0x38(%rsp)
// 0x140de51ac  488d051d6925ff                 leaq	-0xda96e3(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
// 0x140de51b3  4889442440                     movq	%rax, 0x40(%rsp)
// 0x140de51b8  488d442428                     leaq	0x28(%rsp), %rax
// 0x140de51bd  4889442448                     movq	%rax, 0x48(%rsp)
// 0x140de51c2  488d05c74c32ff                 leaq	-0xcdb339(%rip), %rax   # 0x140109e90 <.text+0x108e90>
// 0x140de51c9  4889442450                     movq	%rax, 0x50(%rsp)
// Frames: F108 -> F103 -> F104 -> F105 -> F68 -> F65 -> F62
// 0x140de51ce  488d058b1ad901                 leaq	0x1d91a8b(%rip), %rax   # 0x142b76c60
// 0x140de51d5  4889442458                     movq	%rax, 0x58(%rsp)
// 0x140de51da  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x140de51e3  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x140de51ec  488d442438                     leaq	0x38(%rsp), %rax
// 0x140de51f1  4889442468                     movq	%rax, 0x68(%rsp)
// 0x140de51f6  48c744247002000000             movq	$0x2, 0x70(%rsp)
// Frames: F103 -> F104 -> F105 -> F68 -> F65 -> F62
// 0x140de51ff  488d15e21ad901                 leaq	0x1d91ae2(%rip), %rdx   # 0x142b76ce8
// 0x140de5206  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x140de520b  e88041af01                     callq	0x1428d9390 <.text+0x28d8390>
// Frames: F100 -> F101 -> F102 -> F109 -> F104 -> F110 -> F68 -> F65 -> F62
// 0x140de5210  4881c6b8000000                 addq	$0xb8, %rsi
// Frames: F106 -> F111 -> F104 -> F110 -> F68 -> F65 -> F62
// 0x140de5217  488d05e21ad901                 leaq	0x1d91ae2(%rip), %rax   # 0x142b76d00
// 0x140de521e  4889442428                     movq	%rax, 0x28(%rsp)
// 0x140de5223  48c74424303d000000             movq	$0x3d, 0x30(%rsp)
// Frames: F109 -> F104 -> F110 -> F68 -> F65 -> F62
// 0x140de522c  4889742438                     movq	%rsi, 0x38(%rsp)
// 0x140de5231  488d05986825ff                 leaq	-0xda9768(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
// 0x140de5238  4889442440                     movq	%rax, 0x40(%rsp)
// 0x140de523d  488d442428                     leaq	0x28(%rsp), %rax
// 0x140de5242  4889442448                     movq	%rax, 0x48(%rsp)
// 0x140de5247  488d05424c32ff                 leaq	-0xcdb3be(%rip), %rax   # 0x140109e90 <.text+0x108e90>
// 0x140de524e  4889442450                     movq	%rax, 0x50(%rsp)
// Frames: F108 -> F109 -> F104 -> F110 -> F68 -> F65 -> F62
// 0x140de5253  488d05061ad901                 leaq	0x1d91a06(%rip), %rax   # 0x142b76c60
// 0x140de525a  4889442458                     movq	%rax, 0x58(%rsp)
// 0x140de525f  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x140de5268  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x140de5271  488d442438                     leaq	0x38(%rsp), %rax
// 0x140de5276  4889442468                     movq	%rax, 0x68(%rsp)
// 0x140de527b  48c744247002000000             movq	$0x2, 0x70(%rsp)
// Frames: F109 -> F104 -> F110 -> F68 -> F65 -> F62
// 0x140de5284  488d15b51ad901                 leaq	0x1d91ab5(%rip), %rdx   # 0x142b76d40
// 0x140de528b  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x140de5290  e8fb40af01                     callq	0x1428d9390 <.text+0x28d8390>
// Frames: F112 -> F64 -> F62
// 0x140de5295  488d0d9c1bd901                 leaq	0x1d91b9c(%rip), %rcx   # 0x142b76e38
// 0x140de529c  4c8d05f51bd901                 leaq	0x1d91bf5(%rip), %r8    # 0x142b76e98
// 0x140de52a3  ba59000000                     movl	$0x59, %edx
// 0x140de52a8  e88340af01                     callq	0x1428d9330 <.text+0x28d8330>
// Frames: F113 -> F90 -> F91 -> F92 -> F93 -> F94 -> F95 -> F62
// 0x140de52ad  488d0de4650d02                 leaq	0x20d65e4(%rip), %rcx   # 0x142ebb898
// 0x140de52b4  e85740af01                     callq	0x1428d9310 <.text+0x28d8310>
// 0x140de52b9  cc                             int3
//
// RANGE rva_1a18a90_len_236: RVA 0x1a18a90, 566 PDB bytes; 148 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_1a18a90_len_236.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2259:2088: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Frames: F114
// 0x141a18a90  4157                           pushq	%r15
// 0x141a18a92  4156                           pushq	%r14
// 0x141a18a94  4155                           pushq	%r13
// 0x141a18a96  4154                           pushq	%r12
// 0x141a18a98  56                             pushq	%rsi
// 0x141a18a99  57                             pushq	%rdi
// 0x141a18a9a  55                             pushq	%rbp
// 0x141a18a9b  53                             pushq	%rbx
// 0x141a18a9c  4883ec48                       subq	$0x48, %rsp
// 0x141a18aa0  4889d0                         movq	%rdx, %rax
// 0x141a18aa3  48894c2430                     movq	%rcx, 0x30(%rsp)
// 0x141a18aa8  488b9424b8000000               movq	0xb8(%rsp), %rdx
// 0x141a18ab0  488b6828                       movq	0x28(%rax), %rbp
// Frames: F115 -> F116
// 0x141a18ab4  4989ea                         movq	%rbp, %r10
// 0x141a18ab7  4929d2                         subq	%rdx, %r10
// Frames: F8 -> F117 -> F116
// 0x141a18aba  4d39ca                         cmpq	%r9, %r10
// 0x141a18abd  0f836f010000                   jae	0x141a18c32 <.text+0x1a17c32>
// 0x141a18ac3  440fb6b424c0000000             movzbl	0xc0(%rsp), %r14d
// 0x141a18acc  488bbc24b0000000               movq	0xb0(%rsp), %rdi
// 0x141a18ad4  488b5818                       movq	0x18(%rax), %rbx
// 0x141a18ad8  488b4808                       movq	0x8(%rax), %rcx
// 0x141a18adc  4c8b5810                       movq	0x10(%rax), %r11
// 0x141a18ae0  4c895c2438                     movq	%r11, 0x38(%rsp)
// 0x141a18ae5  4c8b6838                       movq	0x38(%rax), %r13
// 0x141a18ae9  4c8d5fff                       leaq	-0x1(%rdi), %r11
// 0x141a18aed  4c895c2440                     movq	%r11, 0x40(%rsp)
// 0x141a18af2  48895c2428                     movq	%rbx, 0x28(%rsp)
// 0x141a18af7  eb1d                           jmp	0x141a18b16 <.text+0x1a17b16>
// 0x141a18af9  0f1f8000000000                 nopl	(%rax)
// 0x141a18b00  48897038                       movq	%rsi, 0x38(%rax)
// 0x141a18b04  4989f5                         movq	%rsi, %r13
// Frames: F115 -> F116
// 0x141a18b07  4c89d5                         movq	%r10, %rbp
// 0x141a18b0a  4929d2                         subq	%rdx, %r10
// Frames: F8 -> F117 -> F116
// 0x141a18b0d  4d39ca                         cmpq	%r9, %r10
// 0x141a18b10  0f831c010000                   jae	0x141a18c32 <.text+0x1a17c32>
// Frames: F118
// 0x141a18b16  470fb61c10                     movzbl	(%r8,%r10), %r11d
// 0x141a18b1b  4c0fa3db                       btq	%r11, %rbx
// 0x141a18b1f  737f                           jae	0x141a18ba0 <.text+0x1a17ba0>
// Frames: F119
// 0x141a18b21  4939cd                         cmpq	%rcx, %r13
// 0x141a18b24  4989cb                         movq	%rcx, %r11
// 0x141a18b27  4d0f42dd                       cmovbq	%r13, %r11
// 0x141a18b2b  4489f3                         movl	%r14d, %ebx
// 0x141a18b2e  4584f6                         testb	%r14b, %r14b
// 0x141a18b31  4c0f45d9                       cmovneq	%rcx, %r11
// Frames: F120
// 0x141a18b35  498d73ff                       leaq	-0x1(%r11), %rsi
// 0x141a18b39  4839d6                         cmpq	%rdx, %rsi
// 0x141a18b3c  0f837e000000                   jae	0x141a18bc0 <.text+0x1a17bc0>
// Frames: F121 -> F122 -> F123 -> F120
// 0x141a18b42  4d89dc                         movq	%r11, %r12
// 0x141a18b45  49f7dc                         negq	%r12
// 0x141a18b48  488b742440                     movq	0x40(%rsp), %rsi
// 0x141a18b4d  4e8d3c1e                       leaq	(%rsi,%r11), %r15
// 0x141a18b51  4b8d3413                       leaq	(%r11,%r10), %rsi
// 0x141a18b55  48ffce                         decq	%rsi
// 0x141a18b58  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F124 -> F121 -> F122 -> F123 -> F120
// 0x141a18b60  4d85e4                         testq	%r12, %r12
// Frames: F121 -> F122 -> F123 -> F120
// 0x141a18b63  7464                           je	0x141a18bc9 <.text+0x1a17bc9>
// Frames: F125
// 0x141a18b65  4c39ce                         cmpq	%r9, %rsi
// 0x141a18b68  0f8336010000                   jae	0x141a18ca4 <.text+0x1a17ca4>
// 0x141a18b6e  450fb61f                       movzbl	(%r15), %r11d
// 0x141a18b72  49ffc4                         incq	%r12
// 0x141a18b75  49ffcf                         decq	%r15
// 0x141a18b78  4c8d76ff                       leaq	-0x1(%rsi), %r14
// 0x141a18b7c  453a1c30                       cmpb	(%r8,%rsi), %r11b
// 0x141a18b80  4c89f6                         movq	%r14, %rsi
// 0x141a18b83  74db                           je	0x141a18b60 <.text+0x1a17b60>
// Frames: F126
// 0x141a18b85  4829cd                         subq	%rcx, %rbp
// 0x141a18b88  4c29e5                         subq	%r12, %rbp
// 0x141a18b8b  48896828                       movq	%rbp, 0x28(%rax)
// 0x141a18b8f  4889d6                         movq	%rdx, %rsi
// 0x141a18b92  e983000000                     jmp	0x141a18c1a <.text+0x1a17c1a>
// 0x141a18b97  660f1f840000000000             nopw	(%rax,%rax)
// Frames: F127
// 0x141a18ba0  4c895028                       movq	%r10, 0x28(%rax)
// 0x141a18ba4  4889d6                         movq	%rdx, %rsi
// Frames: F128
// 0x141a18ba7  4584f6                         testb	%r14b, %r14b
// 0x141a18baa  0f8450ffffff                   je	0x141a18b00 <.text+0x1a17b00>
// 0x141a18bb0  e952ffffff                     jmp	0x141a18b07 <.text+0x1a17b07>
// 0x141a18bb5  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F124 -> F121 -> F122 -> F123 -> F120
// 0x141a18bc0  4d85db                         testq	%r11, %r11
// Frames: F121 -> F122 -> F123 -> F120
// 0x141a18bc3  0f85ed000000                   jne	0x141a18cb6 <.text+0x1a17cb6>
// Frames: F129
// 0x141a18bc9  84db                           testb	%bl, %bl
// 0x141a18bcb  4c89ee                         movq	%r13, %rsi
// 0x141a18bce  480f45f2                       cmovneq	%rdx, %rsi
// Frames: F130
// 0x141a18bd2  4f8d3c10                       leaq	(%r8,%r10), %r15
// 0x141a18bd6  4989cc                         movq	%rcx, %r12
// 0x141a18bd9  0f1f8000000000                 nopl	(%rax)
// Frames: F124 -> F131 -> F132 -> F130
// 0x141a18be0  4939f4                         cmpq	%rsi, %r12
// Frames: F131 -> F132 -> F130
// 0x141a18be3  735e                           jae	0x141a18c43 <.text+0x1a17c43>
// Frames: F133
// 0x141a18be5  4939d4                         cmpq	%rdx, %r12
// 0x141a18be8  0f8387000000                   jae	0x141a18c75 <.text+0x1a17c75>
// 0x141a18bee  4f8d1c22                       leaq	(%r10,%r12), %r11
// 0x141a18bf2  4d39cb                         cmpq	%r9, %r11
// 0x141a18bf5  0f838d000000                   jae	0x141a18c88 <.text+0x1a17c88>
// 0x141a18bfb  460fb61c27                     movzbl	(%rdi,%r12), %r11d
// Frames: F134 -> F135 -> F136 -> F132 -> F130
// 0x141a18c00  4d8d742401                     leaq	0x1(%r12), %r14
// Frames: F133
// 0x141a18c05  473a1c27                       cmpb	(%r15,%r12), %r11b
// 0x141a18c09  4d89f4                         movq	%r14, %r12
// 0x141a18c0c  74d2                           je	0x141a18be0 <.text+0x1a17be0>
// Frames: F137
// 0x141a18c0e  488b742438                     movq	0x38(%rsp), %rsi
// 0x141a18c13  4829f5                         subq	%rsi, %rbp
// 0x141a18c16  48896828                       movq	%rbp, 0x28(%rax)
// 0x141a18c1a  4989ea                         movq	%rbp, %r10
// 0x141a18c1d  4189de                         movl	%ebx, %r14d
// 0x141a18c20  84db                           testb	%bl, %bl
// 0x141a18c22  488b5c2428                     movq	0x28(%rsp), %rbx
// 0x141a18c27  0f84d3feffff                   je	0x141a18b00 <.text+0x1a17b00>
// 0x141a18c2d  e9d5feffff                     jmp	0x141a18b07 <.text+0x1a17b07>
// Frames: F138
// 0x141a18c32  48c7402800000000               movq	$0x0, 0x28(%rax)
// Frames: F139
// 0x141a18c3a  31c0                           xorl	%eax, %eax
// 0x141a18c3c  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x141a18c41  eb1e                           jmp	0x141a18c61 <.text+0x1a17c61>
// Frames: F140
// 0x141a18c43  4c895028                       movq	%r10, 0x28(%rax)
// Frames: F141
// 0x141a18c47  84db                           testb	%bl, %bl
// 0x141a18c49  7504                           jne	0x141a18c4f <.text+0x1a17c4f>
// Frames: F142
// 0x141a18c4b  48895038                       movq	%rdx, 0x38(%rax)
// Frames: F143 -> F144
// 0x141a18c4f  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x141a18c54  4c895108                       movq	%r10, 0x8(%rcx)
// 0x141a18c58  48896910                       movq	%rbp, 0x10(%rcx)
// 0x141a18c5c  b801000000                     movl	$0x1, %eax
// Frames: F145
// 0x141a18c61  488901                         movq	%rax, (%rcx)
// 0x141a18c64  4883c448                       addq	$0x48, %rsp
// 0x141a18c68  5b                             popq	%rbx
// 0x141a18c69  5d                             popq	%rbp
// 0x141a18c6a  5f                             popq	%rdi
// 0x141a18c6b  5e                             popq	%rsi
// 0x141a18c6c  415c                           popq	%r12
// 0x141a18c6e  415d                           popq	%r13
// 0x141a18c70  415e                           popq	%r14
// 0x141a18c72  415f                           popq	%r15
// 0x141a18c74  c3                             retq
// Frames: F8 -> F117 -> F116
// 0x141a18c75  4839d1                         cmpq	%rdx, %rcx
// 0x141a18c78  480f46ca                       cmovbeq	%rdx, %rcx
// Frames: F133
// 0x141a18c7c  4c8d0575654901                 leaq	0x1496575(%rip), %r8    # 0x142eaf1f8
// 0x141a18c83  e89008ec00                     callq	0x1428d9518 <.text+0x28d8518>
// Frames: F130
// 0x141a18c88  4901ca                         addq	%rcx, %r10
// 0x141a18c8b  4d39d1                         cmpq	%r10, %r9
// 0x141a18c8e  4d0f47d1                       cmovaq	%r9, %r10
// Frames: F133
// 0x141a18c92  4c8d0577654901                 leaq	0x1496577(%rip), %r8    # 0x142eaf210
// 0x141a18c99  4c89d1                         movq	%r10, %rcx
// 0x141a18c9c  4c89ca                         movq	%r9, %rdx
// 0x141a18c9f  e87408ec00                     callq	0x1428d9518 <.text+0x28d8518>
// Frames: F125
// 0x141a18ca4  4c8d0595654901                 leaq	0x1496595(%rip), %r8    # 0x142eaf240
// 0x141a18cab  4889f1                         movq	%rsi, %rcx
// 0x141a18cae  4c89ca                         movq	%r9, %rdx
// 0x141a18cb1  e86208ec00                     callq	0x1428d9518 <.text+0x28d8518>
// 0x141a18cb6  4c8d056b654901                 leaq	0x149656b(%rip), %r8    # 0x142eaf228
// 0x141a18cbd  4889f1                         movq	%rsi, %rcx
// 0x141a18cc0  e85308ec00                     callq	0x1428d9518 <.text+0x28d8518>
// 0x141a18cc5  cc                             int3
//
// RANGE rva_1a28200_len_2a5: RVA 0x1a28200, 677 PDB bytes; 134 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_1a28200_len_2a5.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2264:196140: bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >
// Frames: F146
// 0x141a28200  56                             pushq	%rsi
// 0x141a28201  57                             pushq	%rdi
// 0x141a28202  55                             pushq	%rbp
// 0x141a28203  53                             pushq	%rbx
// 0x141a28204  4881ec88000000                 subq	$0x88, %rsp
// Frames: F63 -> F147 -> F148 -> F149
// 0x141a2820b  f60101                         testb	$0x1, (%rcx)
// 0x141a2820e  0f846c020000                   je	0x141a28480 <.text+0x1a27480>
// Frames: F150 -> F151 -> F152 -> F148 -> F149
// 0x141a28214  4889ce                         movq	%rcx, %rsi
// 0x141a28217  488b82a0050000                 movq	0x5a0(%rdx), %rax
// 0x141a2821e  4889442428                     movq	%rax, 0x28(%rsp)
// Frames: F153 -> F152 -> F148 -> F149
// 0x141a28223  48394118                       cmpq	%rax, 0x18(%rcx)
// Frames: F152 -> F148 -> F149
// 0x141a28227  0f85f0000000                   jne	0x141a2831d <.text+0x1a2731d>
// Frames: F154 -> F155 -> F156 -> F148 -> F149
// 0x141a2822d  488b8208010000                 movq	0x108(%rdx), %rax
// Frames: F157 -> F158 -> F156 -> F148 -> F149
// 0x141a28234  8b8e58010000                   movl	0x158(%rsi), %ecx
// Frames: F159 -> F160 -> F156 -> F148 -> F149
// 0x141a2823a  898658010000                   movl	%eax, 0x158(%rsi)
// Frames: F161 -> F162 -> F163 -> F164 -> F165 -> F148 -> F149
// 0x141a28240  4839c8                         cmpq	%rcx, %rax
// 0x141a28243  0f821e010000                   jb	0x141a28367 <.text+0x1a27367>
// Frames: F58 -> F59 -> F60 -> F61 -> F166 -> F167
// 0x141a28249  bd01000000                     movl	$0x1, %ebp
// 0x141a2824e  f0                             lock
// 0x141a2824f  0fc1aab0050000                 xaddl	%ebp, 0x5b0(%rdx)
// Frames: F65 -> F166 -> F167
// 0x141a28256  488b4608                       movq	0x8(%rsi), %rax
// Frames: F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F166 -> F167
// 0x141a2825a  4c8b8218020000                 movq	0x218(%rdx), %r8
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F166 -> F167
// 0x141a28261  4939c0                         cmpq	%rax, %r8
// 0x141a28264  0f860c010000                   jbe	0x141a28376 <.text+0x1a27376>
// Frames: F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F166 -> F167
// 0x141a2826a  4c8b8a10020000                 movq	0x210(%rdx), %r9
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F166 -> F167
// 0x141a28271  498b0cc1                       movq	(%r9,%rax,8), %rcx
// 0x141a28275  4885c9                         testq	%rcx, %rcx
// Frames: F17 -> F10 -> F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F166 -> F167
// 0x141a28278  0f84f8000000                   je	0x141a28376 <.text+0x1a27376>
// Frames: F18 -> F19 -> F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F166 -> F167
// 0x141a2827e  488b82e0010000                 movq	0x1e0(%rdx), %rax
// Frames: F26 -> F24 -> F25 -> F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F166 -> F167
// 0x141a28285  48f7d1                         notq	%rcx
// Frames: F69 -> F70 -> F71 -> F24 -> F25 -> F11 -> F12 -> F66 -> F67 -> F68 -> F65 -> F166 -> F167
// 0x141a28288  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x141a2828c  48c1e105                       shlq	$0x5, %rcx
// Frames: F29 -> F30 -> F72 -> F73 -> F67 -> F68 -> F65 -> F166 -> F167
// 0x141a28290  48837c082000                   cmpq	$0x0, 0x20(%rax,%rcx)
// Frames: F74 -> F72 -> F73 -> F67 -> F68 -> F65 -> F166 -> F167
// 0x141a28296  0f84da000000                   je	0x141a28376 <.text+0x1a27376>
// Frames: F65 -> F166 -> F167
// 0x141a2829c  4c8b5610                       movq	0x10(%rsi), %r10
// Frames: F8 -> F9 -> F10 -> F11 -> F12 -> F75 -> F76 -> F68 -> F65 -> F166 -> F167
// 0x141a282a0  4d39d0                         cmpq	%r10, %r8
// 0x141a282a3  0f8652010000                   jbe	0x141a283fb <.text+0x1a273fb>
// Frames: F15 -> F16 -> F17 -> F10 -> F11 -> F12 -> F75 -> F76 -> F68 -> F65 -> F166 -> F167
// 0x141a282a9  4f8b04d1                       movq	(%r9,%r10,8), %r8
// 0x141a282ad  4d85c0                         testq	%r8, %r8
// Frames: F17 -> F10 -> F11 -> F12 -> F75 -> F76 -> F68 -> F65 -> F166 -> F167
// 0x141a282b0  0f8445010000                   je	0x141a283fb <.text+0x1a273fb>
// Frames: F77 -> F24 -> F25 -> F11 -> F12 -> F75 -> F76 -> F68 -> F65 -> F166 -> F167
// 0x141a282b6  49f7d0                         notq	%r8
// Frames: F69 -> F70 -> F78 -> F24 -> F25 -> F11 -> F12 -> F75 -> F76 -> F68 -> F65 -> F166 -> F167
// 0x141a282b9  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x141a282bd  49c1e005                       shlq	$0x5, %r8
// Frames: F29 -> F30 -> F72 -> F79 -> F76 -> F68 -> F65 -> F166 -> F167
// 0x141a282c1  4a837c002000                   cmpq	$0x0, 0x20(%rax,%r8)
// Frames: F74 -> F72 -> F79 -> F76 -> F68 -> F65 -> F166 -> F167
// 0x141a282c7  0f842e010000                   je	0x141a283fb <.text+0x1a273fb>
// Frames: F68 -> F65 -> F166 -> F167
// 0x141a282cd  4801c1                         addq	%rax, %rcx
// 0x141a282d0  488b7910                       movq	0x10(%rcx), %rdi
// 0x141a282d4  4901c0                         addq	%rax, %r8
// Frames: F80 -> F81 -> F82 -> F74 -> F72 -> F79 -> F76 -> F68 -> F65 -> F166 -> F167
// 0x141a282d7  498b4810                       movq	0x10(%r8), %rcx
// Frames: F83 -> F84 -> F85 -> F86 -> F87 -> F76 -> F68 -> F65 -> F166 -> F167
// 0x141a282db  8b82b0050000                   movl	0x5b0(%rdx), %eax
// Frames: F88 -> F89 -> F90 -> F91 -> F92 -> F93 -> F94 -> F95 -> F166 -> F167
// 0x141a282e1  4189685c                       movl	%ebp, 0x5c(%r8)
// Frames: F90 -> F91 -> F92 -> F93 -> F94 -> F95 -> F166 -> F167
// 0x141a282e5  e8c62a0100                     callq	0x141a3adb0 <.text+0x1a39db0>
// Frames: F96 -> F90 -> F91 -> F92 -> F93 -> F94 -> F95 -> F166 -> F167
// 0x141a282ea  4885c0                         testq	%rax, %rax
// 0x141a282ed  0f84a5010000                   je	0x141a28498 <.text+0x1a27498>
// Frames: F97 -> F91 -> F92 -> F93 -> F94 -> F95 -> F166 -> F167
// 0x141a282f3  4889c3                         movq	%rax, %rbx
// 0x141a282f6  4889f9                         movq	%rdi, %rcx
// 0x141a282f9  e8a2f9f5fe                     callq	0x140987ca0 <.text+0x986ca0>
// 0x141a282fe  4889d9                         movq	%rbx, %rcx
// 0x141a28301  4889c2                         movq	%rax, %rdx
// 0x141a28304  e8772c0100                     callq	0x141a3af80 <.text+0x1a39f80>
// Frames: F98 -> F166 -> F167
// 0x141a28309  89ae50010000                   movl	%ebp, 0x150(%rsi)
// Frames: F168
// 0x141a2830f  31c0                           xorl	%eax, %eax
// 0x141a28311  4881c488000000                 addq	$0x88, %rsp
// 0x141a28318  5b                             popq	%rbx
// 0x141a28319  5d                             popq	%rbp
// 0x141a2831a  5f                             popq	%rdi
// 0x141a2831b  5e                             popq	%rsi
// 0x141a2831c  c3                             retq
// Frames: F152 -> F148 -> F149
// 0x141a2831d  488d05cc804801                 leaq	0x14880cc(%rip), %rax   # 0x142eb03f0
// 0x141a28324  4889442458                     movq	%rax, 0x58(%rsp)
// 0x141a28329  48c744246001000000             movq	$0x1, 0x60(%rsp)
// 0x141a28332  48c744246808000000             movq	$0x8, 0x68(%rsp)
// 0x141a2833b  0f57c0                         xorps	%xmm0, %xmm0
// 0x141a2833e  0f11442470                     movups	%xmm0, 0x70(%rsp)
// 0x141a28343  4883c618                       addq	$0x18, %rsi
// 0x141a28347  488d05b2804801                 leaq	0x14880b2(%rip), %rax   # 0x142eb0400
// 0x141a2834e  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141a28353  4c8d442428                     leaq	0x28(%rsp), %r8
// 0x141a28358  4c8d4c2458                     leaq	0x58(%rsp), %r9
// 0x141a2835d  31c9                           xorl	%ecx, %ecx
// 0x141a2835f  4889f2                         movq	%rsi, %rdx
// 0x141a28362  e87a5dea00                     callq	0x1428ce0e1 <.text+0x28cd0e1>
// Frames: F169 -> F162 -> F163 -> F164 -> F165 -> F148 -> F149
// 0x141a28367  4c8d05aa804801                 leaq	0x14880aa(%rip), %r8    # 0x142eb0418
// 0x141a2836e  4889c2                         movq	%rax, %rdx
// 0x141a28371  e8ba14eb00                     callq	0x1428d9830 <.text+0x28d8830>
// Frames: F100 -> F101 -> F102 -> F103 -> F104 -> F105 -> F68 -> F65 -> F166 -> F167
// 0x141a28376  4881c6b8000000                 addq	$0xb8, %rsi
// Frames: F106 -> F107 -> F104 -> F105 -> F68 -> F65 -> F166 -> F167
// 0x141a2837d  488d05e1994801                 leaq	0x14899e1(%rip), %rax   # 0x142eb1d65
// 0x141a28384  4889442448                     movq	%rax, 0x48(%rsp)
// 0x141a28389  48c74424502c000000             movq	$0x2c, 0x50(%rsp)
// Frames: F103 -> F104 -> F105 -> F68 -> F65 -> F166 -> F167
// 0x141a28392  4889742428                     movq	%rsi, 0x28(%rsp)
// 0x141a28397  488d05323761fe                 leaq	-0x19ec8ce(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
// 0x141a2839e  4889442430                     movq	%rax, 0x30(%rsp)
// 0x141a283a3  488d442448                     leaq	0x48(%rsp), %rax
// 0x141a283a8  4889442438                     movq	%rax, 0x38(%rsp)
// 0x141a283ad  488d05dc1a6efe                 leaq	-0x191e524(%rip), %rax  # 0x140109e90 <.text+0x108e90>
// 0x141a283b4  4889442440                     movq	%rax, 0x40(%rsp)
// Frames: F108 -> F103 -> F104 -> F105 -> F68 -> F65 -> F166 -> F167
// 0x141a283b9  488d05d8934801                 leaq	0x14893d8(%rip), %rax   # 0x142eb1798
// 0x141a283c0  4889442458                     movq	%rax, 0x58(%rsp)
// 0x141a283c5  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x141a283ce  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x141a283d7  488d442428                     leaq	0x28(%rsp), %rax
// 0x141a283dc  4889442468                     movq	%rax, 0x68(%rsp)
// 0x141a283e1  48c744247002000000             movq	$0x2, 0x70(%rsp)
// Frames: F103 -> F104 -> F105 -> F68 -> F65 -> F166 -> F167
// 0x141a283ea  488d152f944801                 leaq	0x148942f(%rip), %rdx   # 0x142eb1820
// 0x141a283f1  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x141a283f6  e8950feb00                     callq	0x1428d9390 <.text+0x28d8390>
// Frames: F100 -> F101 -> F102 -> F109 -> F104 -> F110 -> F68 -> F65 -> F166 -> F167
// 0x141a283fb  4881c6b8000000                 addq	$0xb8, %rsi
// Frames: F106 -> F111 -> F104 -> F110 -> F68 -> F65 -> F166 -> F167
// 0x141a28402  488d05bb964801                 leaq	0x14896bb(%rip), %rax   # 0x142eb1ac4
// 0x141a28409  4889442448                     movq	%rax, 0x48(%rsp)
// 0x141a2840e  48c74424503d000000             movq	$0x3d, 0x50(%rsp)
// Frames: F109 -> F104 -> F110 -> F68 -> F65 -> F166 -> F167
// 0x141a28417  4889742428                     movq	%rsi, 0x28(%rsp)
// 0x141a2841c  488d05ad3661fe                 leaq	-0x19ec953(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
// 0x141a28423  4889442430                     movq	%rax, 0x30(%rsp)
// 0x141a28428  488d442448                     leaq	0x48(%rsp), %rax
// 0x141a2842d  4889442438                     movq	%rax, 0x38(%rsp)
// 0x141a28432  488d05571a6efe                 leaq	-0x191e5a9(%rip), %rax  # 0x140109e90 <.text+0x108e90>
// 0x141a28439  4889442440                     movq	%rax, 0x40(%rsp)
// Frames: F108 -> F109 -> F104 -> F110 -> F68 -> F65 -> F166 -> F167
// 0x141a2843e  488d0553934801                 leaq	0x1489353(%rip), %rax   # 0x142eb1798
// 0x141a28445  4889442458                     movq	%rax, 0x58(%rsp)
// 0x141a2844a  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x141a28453  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x141a2845c  488d442428                     leaq	0x28(%rsp), %rax
// 0x141a28461  4889442468                     movq	%rax, 0x68(%rsp)
// 0x141a28466  48c744247002000000             movq	$0x2, 0x70(%rsp)
// Frames: F109 -> F104 -> F110 -> F68 -> F65 -> F166 -> F167
// 0x141a2846f  488d157a9c4801                 leaq	0x1489c7a(%rip), %rdx   # 0x142eb20f0
// 0x141a28476  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x141a2847b  e8100feb00                     callq	0x1428d9390 <.text+0x28d8390>
// Frames: F112 -> F147 -> F148 -> F149
// 0x141a28480  488d0d197e4801                 leaq	0x1487e19(%rip), %rcx   # 0x142eb02a0
// 0x141a28487  4c8d05da7e4801                 leaq	0x1487eda(%rip), %r8    # 0x142eb0368
// 0x141a2848e  ba59000000                     movl	$0x59, %edx
// 0x141a28493  e8980eeb00                     callq	0x1428d9330 <.text+0x28d8330>
// Frames: F113 -> F90 -> F91 -> F92 -> F93 -> F94 -> F95 -> F166 -> F167
// 0x141a28498  488d0df9334901                 leaq	0x14933f9(%rip), %rcx   # 0x142ebb898
// 0x141a2849f  e86c0eeb00                     callq	0x1428d9310 <.text+0x28d8310>
// 0x141a284a4  cc                             int3
//
// RANGE rva_1a36400_len_236: RVA 0x1a36400, 566 PDB bytes; 148 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_1a36400_len_236.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2275:1452: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Frames: F114
// 0x141a36400  4157                           pushq	%r15
// 0x141a36402  4156                           pushq	%r14
// 0x141a36404  4155                           pushq	%r13
// 0x141a36406  4154                           pushq	%r12
// 0x141a36408  56                             pushq	%rsi
// 0x141a36409  57                             pushq	%rdi
// 0x141a3640a  55                             pushq	%rbp
// 0x141a3640b  53                             pushq	%rbx
// 0x141a3640c  4883ec48                       subq	$0x48, %rsp
// 0x141a36410  4889d0                         movq	%rdx, %rax
// 0x141a36413  48894c2430                     movq	%rcx, 0x30(%rsp)
// 0x141a36418  488b9424b8000000               movq	0xb8(%rsp), %rdx
// 0x141a36420  488b6828                       movq	0x28(%rax), %rbp
// Frames: F115 -> F116
// 0x141a36424  4989ea                         movq	%rbp, %r10
// 0x141a36427  4929d2                         subq	%rdx, %r10
// Frames: F8 -> F117 -> F116
// 0x141a3642a  4d39ca                         cmpq	%r9, %r10
// 0x141a3642d  0f836f010000                   jae	0x141a365a2 <.text+0x1a355a2>
// 0x141a36433  440fb6b424c0000000             movzbl	0xc0(%rsp), %r14d
// 0x141a3643c  488bbc24b0000000               movq	0xb0(%rsp), %rdi
// 0x141a36444  488b5818                       movq	0x18(%rax), %rbx
// 0x141a36448  488b4808                       movq	0x8(%rax), %rcx
// 0x141a3644c  4c8b5810                       movq	0x10(%rax), %r11
// 0x141a36450  4c895c2438                     movq	%r11, 0x38(%rsp)
// 0x141a36455  4c8b6838                       movq	0x38(%rax), %r13
// 0x141a36459  4c8d5fff                       leaq	-0x1(%rdi), %r11
// 0x141a3645d  4c895c2440                     movq	%r11, 0x40(%rsp)
// 0x141a36462  48895c2428                     movq	%rbx, 0x28(%rsp)
// 0x141a36467  eb1d                           jmp	0x141a36486 <.text+0x1a35486>
// 0x141a36469  0f1f8000000000                 nopl	(%rax)
// 0x141a36470  48897038                       movq	%rsi, 0x38(%rax)
// 0x141a36474  4989f5                         movq	%rsi, %r13
// Frames: F115 -> F116
// 0x141a36477  4c89d5                         movq	%r10, %rbp
// 0x141a3647a  4929d2                         subq	%rdx, %r10
// Frames: F8 -> F117 -> F116
// 0x141a3647d  4d39ca                         cmpq	%r9, %r10
// 0x141a36480  0f831c010000                   jae	0x141a365a2 <.text+0x1a355a2>
// Frames: F118
// 0x141a36486  470fb61c10                     movzbl	(%r8,%r10), %r11d
// 0x141a3648b  4c0fa3db                       btq	%r11, %rbx
// 0x141a3648f  737f                           jae	0x141a36510 <.text+0x1a35510>
// Frames: F119
// 0x141a36491  4939cd                         cmpq	%rcx, %r13
// 0x141a36494  4989cb                         movq	%rcx, %r11
// 0x141a36497  4d0f42dd                       cmovbq	%r13, %r11
// 0x141a3649b  4489f3                         movl	%r14d, %ebx
// 0x141a3649e  4584f6                         testb	%r14b, %r14b
// 0x141a364a1  4c0f45d9                       cmovneq	%rcx, %r11
// Frames: F120
// 0x141a364a5  498d73ff                       leaq	-0x1(%r11), %rsi
// 0x141a364a9  4839d6                         cmpq	%rdx, %rsi
// 0x141a364ac  0f837e000000                   jae	0x141a36530 <.text+0x1a35530>
// Frames: F121 -> F122 -> F123 -> F120
// 0x141a364b2  4d89dc                         movq	%r11, %r12
// 0x141a364b5  49f7dc                         negq	%r12
// 0x141a364b8  488b742440                     movq	0x40(%rsp), %rsi
// 0x141a364bd  4e8d3c1e                       leaq	(%rsi,%r11), %r15
// 0x141a364c1  4b8d3413                       leaq	(%r11,%r10), %rsi
// 0x141a364c5  48ffce                         decq	%rsi
// 0x141a364c8  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F124 -> F121 -> F122 -> F123 -> F120
// 0x141a364d0  4d85e4                         testq	%r12, %r12
// Frames: F121 -> F122 -> F123 -> F120
// 0x141a364d3  7464                           je	0x141a36539 <.text+0x1a35539>
// Frames: F125
// 0x141a364d5  4c39ce                         cmpq	%r9, %rsi
// 0x141a364d8  0f8336010000                   jae	0x141a36614 <.text+0x1a35614>
// 0x141a364de  450fb61f                       movzbl	(%r15), %r11d
// 0x141a364e2  49ffc4                         incq	%r12
// 0x141a364e5  49ffcf                         decq	%r15
// 0x141a364e8  4c8d76ff                       leaq	-0x1(%rsi), %r14
// 0x141a364ec  453a1c30                       cmpb	(%r8,%rsi), %r11b
// 0x141a364f0  4c89f6                         movq	%r14, %rsi
// 0x141a364f3  74db                           je	0x141a364d0 <.text+0x1a354d0>
// Frames: F126
// 0x141a364f5  4829cd                         subq	%rcx, %rbp
// 0x141a364f8  4c29e5                         subq	%r12, %rbp
// 0x141a364fb  48896828                       movq	%rbp, 0x28(%rax)
// 0x141a364ff  4889d6                         movq	%rdx, %rsi
// 0x141a36502  e983000000                     jmp	0x141a3658a <.text+0x1a3558a>
// 0x141a36507  660f1f840000000000             nopw	(%rax,%rax)
// Frames: F127
// 0x141a36510  4c895028                       movq	%r10, 0x28(%rax)
// 0x141a36514  4889d6                         movq	%rdx, %rsi
// Frames: F128
// 0x141a36517  4584f6                         testb	%r14b, %r14b
// 0x141a3651a  0f8450ffffff                   je	0x141a36470 <.text+0x1a35470>
// 0x141a36520  e952ffffff                     jmp	0x141a36477 <.text+0x1a35477>
// 0x141a36525  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F124 -> F121 -> F122 -> F123 -> F120
// 0x141a36530  4d85db                         testq	%r11, %r11
// Frames: F121 -> F122 -> F123 -> F120
// 0x141a36533  0f85ed000000                   jne	0x141a36626 <.text+0x1a35626>
// Frames: F129
// 0x141a36539  84db                           testb	%bl, %bl
// 0x141a3653b  4c89ee                         movq	%r13, %rsi
// 0x141a3653e  480f45f2                       cmovneq	%rdx, %rsi
// Frames: F130
// 0x141a36542  4f8d3c10                       leaq	(%r8,%r10), %r15
// 0x141a36546  4989cc                         movq	%rcx, %r12
// 0x141a36549  0f1f8000000000                 nopl	(%rax)
// Frames: F124 -> F131 -> F132 -> F130
// 0x141a36550  4939f4                         cmpq	%rsi, %r12
// Frames: F131 -> F132 -> F130
// 0x141a36553  735e                           jae	0x141a365b3 <.text+0x1a355b3>
// Frames: F133
// 0x141a36555  4939d4                         cmpq	%rdx, %r12
// 0x141a36558  0f8387000000                   jae	0x141a365e5 <.text+0x1a355e5>
// 0x141a3655e  4f8d1c22                       leaq	(%r10,%r12), %r11
// 0x141a36562  4d39cb                         cmpq	%r9, %r11
// 0x141a36565  0f838d000000                   jae	0x141a365f8 <.text+0x1a355f8>
// 0x141a3656b  460fb61c27                     movzbl	(%rdi,%r12), %r11d
// Frames: F134 -> F135 -> F136 -> F132 -> F130
// 0x141a36570  4d8d742401                     leaq	0x1(%r12), %r14
// Frames: F133
// 0x141a36575  473a1c27                       cmpb	(%r15,%r12), %r11b
// 0x141a36579  4d89f4                         movq	%r14, %r12
// 0x141a3657c  74d2                           je	0x141a36550 <.text+0x1a35550>
// Frames: F137
// 0x141a3657e  488b742438                     movq	0x38(%rsp), %rsi
// 0x141a36583  4829f5                         subq	%rsi, %rbp
// 0x141a36586  48896828                       movq	%rbp, 0x28(%rax)
// 0x141a3658a  4989ea                         movq	%rbp, %r10
// 0x141a3658d  4189de                         movl	%ebx, %r14d
// 0x141a36590  84db                           testb	%bl, %bl
// 0x141a36592  488b5c2428                     movq	0x28(%rsp), %rbx
// 0x141a36597  0f84d3feffff                   je	0x141a36470 <.text+0x1a35470>
// 0x141a3659d  e9d5feffff                     jmp	0x141a36477 <.text+0x1a35477>
// Frames: F138
// 0x141a365a2  48c7402800000000               movq	$0x0, 0x28(%rax)
// 0x141a365aa  31c0                           xorl	%eax, %eax
// 0x141a365ac  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x141a365b1  eb1e                           jmp	0x141a365d1 <.text+0x1a355d1>
// 0x141a365b3  4c895028                       movq	%r10, 0x28(%rax)
// 0x141a365b7  84db                           testb	%bl, %bl
// 0x141a365b9  7504                           jne	0x141a365bf <.text+0x1a355bf>
// 0x141a365bb  48895038                       movq	%rdx, 0x38(%rax)
// Frames: F143 -> F138
// 0x141a365bf  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x141a365c4  4c895108                       movq	%r10, 0x8(%rcx)
// 0x141a365c8  48896910                       movq	%rbp, 0x10(%rcx)
// 0x141a365cc  b801000000                     movl	$0x1, %eax
// Frames: F138
// 0x141a365d1  488901                         movq	%rax, (%rcx)
// Frames: F145
// 0x141a365d4  4883c448                       addq	$0x48, %rsp
// 0x141a365d8  5b                             popq	%rbx
// 0x141a365d9  5d                             popq	%rbp
// 0x141a365da  5f                             popq	%rdi
// 0x141a365db  5e                             popq	%rsi
// 0x141a365dc  415c                           popq	%r12
// 0x141a365de  415d                           popq	%r13
// 0x141a365e0  415e                           popq	%r14
// 0x141a365e2  415f                           popq	%r15
// 0x141a365e4  c3                             retq
// Frames: F8 -> F117 -> F116
// 0x141a365e5  4839d1                         cmpq	%rdx, %rcx
// 0x141a365e8  480f46ca                       cmovbeq	%rdx, %rcx
// Frames: F133
// 0x141a365ec  4c8d056d3f4801                 leaq	0x1483f6d(%rip), %r8    # 0x142eba560
// 0x141a365f3  e8202fea00                     callq	0x1428d9518 <.text+0x28d8518>
// Frames: F130
// 0x141a365f8  4901ca                         addq	%rcx, %r10
// 0x141a365fb  4d39d1                         cmpq	%r10, %r9
// 0x141a365fe  4d0f47d1                       cmovaq	%r9, %r10
// Frames: F133
// 0x141a36602  4c8d056f3f4801                 leaq	0x1483f6f(%rip), %r8    # 0x142eba578
// 0x141a36609  4c89d1                         movq	%r10, %rcx
// 0x141a3660c  4c89ca                         movq	%r9, %rdx
// 0x141a3660f  e8042fea00                     callq	0x1428d9518 <.text+0x28d8518>
// Frames: F125
// 0x141a36614  4c8d058d3f4801                 leaq	0x1483f8d(%rip), %r8    # 0x142eba5a8
// 0x141a3661b  4889f1                         movq	%rsi, %rcx
// 0x141a3661e  4c89ca                         movq	%r9, %rdx
// 0x141a36621  e8f22eea00                     callq	0x1428d9518 <.text+0x28d8518>
// 0x141a36626  4c8d05633f4801                 leaq	0x1483f63(%rip), %r8    # 0x142eba590
// 0x141a3662d  4889f1                         movq	%rsi, %rcx
// 0x141a36630  e8e32eea00                     callq	0x1428d9518 <.text+0x28d8518>
// 0x141a36635  cc                             int3
//
// RANGE rva_1a38370_len_236: RVA 0x1a38370, 566 PDB bytes; 148 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_1a38370_len_236.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2279:1880: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Frames: F114
// 0x141a38370  4157                           pushq	%r15
// 0x141a38372  4156                           pushq	%r14
// 0x141a38374  4155                           pushq	%r13
// 0x141a38376  4154                           pushq	%r12
// 0x141a38378  56                             pushq	%rsi
// 0x141a38379  57                             pushq	%rdi
// 0x141a3837a  55                             pushq	%rbp
// 0x141a3837b  53                             pushq	%rbx
// 0x141a3837c  4883ec48                       subq	$0x48, %rsp
// 0x141a38380  4889d0                         movq	%rdx, %rax
// 0x141a38383  48894c2430                     movq	%rcx, 0x30(%rsp)
// 0x141a38388  488b9424b8000000               movq	0xb8(%rsp), %rdx
// 0x141a38390  488b6828                       movq	0x28(%rax), %rbp
// Frames: F115 -> F116
// 0x141a38394  4989ea                         movq	%rbp, %r10
// 0x141a38397  4929d2                         subq	%rdx, %r10
// Frames: F8 -> F117 -> F116
// 0x141a3839a  4d39ca                         cmpq	%r9, %r10
// 0x141a3839d  0f836f010000                   jae	0x141a38512 <.text+0x1a37512>
// 0x141a383a3  440fb6b424c0000000             movzbl	0xc0(%rsp), %r14d
// 0x141a383ac  488bbc24b0000000               movq	0xb0(%rsp), %rdi
// 0x141a383b4  488b5818                       movq	0x18(%rax), %rbx
// 0x141a383b8  488b4808                       movq	0x8(%rax), %rcx
// 0x141a383bc  4c8b5810                       movq	0x10(%rax), %r11
// 0x141a383c0  4c895c2438                     movq	%r11, 0x38(%rsp)
// 0x141a383c5  4c8b6838                       movq	0x38(%rax), %r13
// 0x141a383c9  4c8d5fff                       leaq	-0x1(%rdi), %r11
// 0x141a383cd  4c895c2440                     movq	%r11, 0x40(%rsp)
// 0x141a383d2  48895c2428                     movq	%rbx, 0x28(%rsp)
// 0x141a383d7  eb1d                           jmp	0x141a383f6 <.text+0x1a373f6>
// 0x141a383d9  0f1f8000000000                 nopl	(%rax)
// 0x141a383e0  48897038                       movq	%rsi, 0x38(%rax)
// 0x141a383e4  4989f5                         movq	%rsi, %r13
// Frames: F115 -> F116
// 0x141a383e7  4c89d5                         movq	%r10, %rbp
// 0x141a383ea  4929d2                         subq	%rdx, %r10
// Frames: F8 -> F117 -> F116
// 0x141a383ed  4d39ca                         cmpq	%r9, %r10
// 0x141a383f0  0f831c010000                   jae	0x141a38512 <.text+0x1a37512>
// Frames: F118
// 0x141a383f6  470fb61c10                     movzbl	(%r8,%r10), %r11d
// 0x141a383fb  4c0fa3db                       btq	%r11, %rbx
// 0x141a383ff  737f                           jae	0x141a38480 <.text+0x1a37480>
// Frames: F119
// 0x141a38401  4939cd                         cmpq	%rcx, %r13
// 0x141a38404  4989cb                         movq	%rcx, %r11
// 0x141a38407  4d0f42dd                       cmovbq	%r13, %r11
// 0x141a3840b  4489f3                         movl	%r14d, %ebx
// 0x141a3840e  4584f6                         testb	%r14b, %r14b
// 0x141a38411  4c0f45d9                       cmovneq	%rcx, %r11
// Frames: F120
// 0x141a38415  498d73ff                       leaq	-0x1(%r11), %rsi
// 0x141a38419  4839d6                         cmpq	%rdx, %rsi
// 0x141a3841c  0f837e000000                   jae	0x141a384a0 <.text+0x1a374a0>
// Frames: F121 -> F122 -> F123 -> F120
// 0x141a38422  4d89dc                         movq	%r11, %r12
// 0x141a38425  49f7dc                         negq	%r12
// 0x141a38428  488b742440                     movq	0x40(%rsp), %rsi
// 0x141a3842d  4e8d3c1e                       leaq	(%rsi,%r11), %r15
// 0x141a38431  4b8d3413                       leaq	(%r11,%r10), %rsi
// 0x141a38435  48ffce                         decq	%rsi
// 0x141a38438  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F124 -> F121 -> F122 -> F123 -> F120
// 0x141a38440  4d85e4                         testq	%r12, %r12
// Frames: F121 -> F122 -> F123 -> F120
// 0x141a38443  7464                           je	0x141a384a9 <.text+0x1a374a9>
// Frames: F125
// 0x141a38445  4c39ce                         cmpq	%r9, %rsi
// 0x141a38448  0f8336010000                   jae	0x141a38584 <.text+0x1a37584>
// 0x141a3844e  450fb61f                       movzbl	(%r15), %r11d
// 0x141a38452  49ffc4                         incq	%r12
// 0x141a38455  49ffcf                         decq	%r15
// 0x141a38458  4c8d76ff                       leaq	-0x1(%rsi), %r14
// 0x141a3845c  453a1c30                       cmpb	(%r8,%rsi), %r11b
// 0x141a38460  4c89f6                         movq	%r14, %rsi
// 0x141a38463  74db                           je	0x141a38440 <.text+0x1a37440>
// Frames: F126
// 0x141a38465  4829cd                         subq	%rcx, %rbp
// 0x141a38468  4c29e5                         subq	%r12, %rbp
// 0x141a3846b  48896828                       movq	%rbp, 0x28(%rax)
// 0x141a3846f  4889d6                         movq	%rdx, %rsi
// 0x141a38472  e983000000                     jmp	0x141a384fa <.text+0x1a374fa>
// 0x141a38477  660f1f840000000000             nopw	(%rax,%rax)
// Frames: F127
// 0x141a38480  4c895028                       movq	%r10, 0x28(%rax)
// 0x141a38484  4889d6                         movq	%rdx, %rsi
// Frames: F128
// 0x141a38487  4584f6                         testb	%r14b, %r14b
// 0x141a3848a  0f8450ffffff                   je	0x141a383e0 <.text+0x1a373e0>
// 0x141a38490  e952ffffff                     jmp	0x141a383e7 <.text+0x1a373e7>
// 0x141a38495  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F124 -> F121 -> F122 -> F123 -> F120
// 0x141a384a0  4d85db                         testq	%r11, %r11
// Frames: F121 -> F122 -> F123 -> F120
// 0x141a384a3  0f85ed000000                   jne	0x141a38596 <.text+0x1a37596>
// Frames: F129
// 0x141a384a9  84db                           testb	%bl, %bl
// 0x141a384ab  4c89ee                         movq	%r13, %rsi
// 0x141a384ae  480f45f2                       cmovneq	%rdx, %rsi
// Frames: F130
// 0x141a384b2  4f8d3c10                       leaq	(%r8,%r10), %r15
// 0x141a384b6  4989cc                         movq	%rcx, %r12
// 0x141a384b9  0f1f8000000000                 nopl	(%rax)
// Frames: F124 -> F131 -> F132 -> F130
// 0x141a384c0  4939f4                         cmpq	%rsi, %r12
// Frames: F131 -> F132 -> F130
// 0x141a384c3  735e                           jae	0x141a38523 <.text+0x1a37523>
// Frames: F133
// 0x141a384c5  4939d4                         cmpq	%rdx, %r12
// 0x141a384c8  0f8387000000                   jae	0x141a38555 <.text+0x1a37555>
// 0x141a384ce  4f8d1c22                       leaq	(%r10,%r12), %r11
// 0x141a384d2  4d39cb                         cmpq	%r9, %r11
// 0x141a384d5  0f838d000000                   jae	0x141a38568 <.text+0x1a37568>
// 0x141a384db  460fb61c27                     movzbl	(%rdi,%r12), %r11d
// Frames: F134 -> F135 -> F136 -> F132 -> F130
// 0x141a384e0  4d8d742401                     leaq	0x1(%r12), %r14
// Frames: F140
// 0x141a384e5  473a1c27                       cmpb	(%r15,%r12), %r11b
// 0x141a384e9  4d89f4                         movq	%r14, %r12
// 0x141a384ec  74d2                           je	0x141a384c0 <.text+0x1a374c0>
// 0x141a384ee  488b742438                     movq	0x38(%rsp), %rsi
// 0x141a384f3  4829f5                         subq	%rsi, %rbp
// 0x141a384f6  48896828                       movq	%rbp, 0x28(%rax)
// 0x141a384fa  4989ea                         movq	%rbp, %r10
// 0x141a384fd  4189de                         movl	%ebx, %r14d
// 0x141a38500  84db                           testb	%bl, %bl
// 0x141a38502  488b5c2428                     movq	0x28(%rsp), %rbx
// 0x141a38507  0f84d3feffff                   je	0x141a383e0 <.text+0x1a373e0>
// 0x141a3850d  e9d5feffff                     jmp	0x141a383e7 <.text+0x1a373e7>
// 0x141a38512  48c7402800000000               movq	$0x0, 0x28(%rax)
// 0x141a3851a  31c0                           xorl	%eax, %eax
// 0x141a3851c  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x141a38521  eb1e                           jmp	0x141a38541 <.text+0x1a37541>
// 0x141a38523  4c895028                       movq	%r10, 0x28(%rax)
// Frames: F141
// 0x141a38527  84db                           testb	%bl, %bl
// 0x141a38529  7504                           jne	0x141a3852f <.text+0x1a3752f>
// Frames: F142
// 0x141a3852b  48895038                       movq	%rdx, 0x38(%rax)
// Frames: F143 -> F144
// 0x141a3852f  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x141a38534  4c895108                       movq	%r10, 0x8(%rcx)
// 0x141a38538  48896910                       movq	%rbp, 0x10(%rcx)
// 0x141a3853c  b801000000                     movl	$0x1, %eax
// Frames: F145
// 0x141a38541  488901                         movq	%rax, (%rcx)
// 0x141a38544  4883c448                       addq	$0x48, %rsp
// 0x141a38548  5b                             popq	%rbx
// 0x141a38549  5d                             popq	%rbp
// 0x141a3854a  5f                             popq	%rdi
// 0x141a3854b  5e                             popq	%rsi
// 0x141a3854c  415c                           popq	%r12
// 0x141a3854e  415d                           popq	%r13
// 0x141a38550  415e                           popq	%r14
// 0x141a38552  415f                           popq	%r15
// 0x141a38554  c3                             retq
// Frames: F8 -> F117 -> F116
// 0x141a38555  4839d1                         cmpq	%rdx, %rcx
// 0x141a38558  480f46ca                       cmovbeq	%rdx, %rcx
// Frames: F133
// 0x141a3855c  4c8d052d284801                 leaq	0x148282d(%rip), %r8    # 0x142ebad90
// 0x141a38563  e8b00fea00                     callq	0x1428d9518 <.text+0x28d8518>
// Frames: F130
// 0x141a38568  4901ca                         addq	%rcx, %r10
// 0x141a3856b  4d39d1                         cmpq	%r10, %r9
// 0x141a3856e  4d0f47d1                       cmovaq	%r9, %r10
// Frames: F133
// 0x141a38572  4c8d052f284801                 leaq	0x148282f(%rip), %r8    # 0x142ebada8
// 0x141a38579  4c89d1                         movq	%r10, %rcx
// 0x141a3857c  4c89ca                         movq	%r9, %rdx
// 0x141a3857f  e8940fea00                     callq	0x1428d9518 <.text+0x28d8518>
// Frames: F125
// 0x141a38584  4c8d054d284801                 leaq	0x148284d(%rip), %r8    # 0x142ebadd8
// 0x141a3858b  4889f1                         movq	%rsi, %rcx
// 0x141a3858e  4c89ca                         movq	%r9, %rdx
// 0x141a38591  e8820fea00                     callq	0x1428d9518 <.text+0x28d8518>
// 0x141a38596  4c8d0523284801                 leaq	0x1482823(%rip), %r8    # 0x142ebadc0
// 0x141a3859d  4889f1                         movq	%rsi, %rcx
// 0x141a385a0  e8730fea00                     callq	0x1428d9518 <.text+0x28d8518>
// 0x141a385a5  cc                             int3
//
// RANGE rva_1a3c870_len_236: RVA 0x1a3c870, 566 PDB bytes; 148 instructions
// Evidence: _assembly/roof-v1/B/pack-001/rva_1a3c870_len_236.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2281:22296: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Frames: F114
// 0x141a3c870  4157                           pushq	%r15
// 0x141a3c872  4156                           pushq	%r14
// 0x141a3c874  4155                           pushq	%r13
// 0x141a3c876  4154                           pushq	%r12
// 0x141a3c878  56                             pushq	%rsi
// 0x141a3c879  57                             pushq	%rdi
// 0x141a3c87a  55                             pushq	%rbp
// 0x141a3c87b  53                             pushq	%rbx
// 0x141a3c87c  4883ec48                       subq	$0x48, %rsp
// 0x141a3c880  4889d0                         movq	%rdx, %rax
// 0x141a3c883  48894c2430                     movq	%rcx, 0x30(%rsp)
// 0x141a3c888  488b9424b8000000               movq	0xb8(%rsp), %rdx
// 0x141a3c890  488b6828                       movq	0x28(%rax), %rbp
// Frames: F115 -> F116
// 0x141a3c894  4989ea                         movq	%rbp, %r10
// 0x141a3c897  4929d2                         subq	%rdx, %r10
// Frames: F8 -> F117 -> F116
// 0x141a3c89a  4d39ca                         cmpq	%r9, %r10
// 0x141a3c89d  0f836f010000                   jae	0x141a3ca12 <.text+0x1a3ba12>
// 0x141a3c8a3  440fb6b424c0000000             movzbl	0xc0(%rsp), %r14d
// 0x141a3c8ac  488bbc24b0000000               movq	0xb0(%rsp), %rdi
// 0x141a3c8b4  488b5818                       movq	0x18(%rax), %rbx
// 0x141a3c8b8  488b4808                       movq	0x8(%rax), %rcx
// 0x141a3c8bc  4c8b5810                       movq	0x10(%rax), %r11
// 0x141a3c8c0  4c895c2438                     movq	%r11, 0x38(%rsp)
// 0x141a3c8c5  4c8b6838                       movq	0x38(%rax), %r13
// 0x141a3c8c9  4c8d5fff                       leaq	-0x1(%rdi), %r11
// 0x141a3c8cd  4c895c2440                     movq	%r11, 0x40(%rsp)
// 0x141a3c8d2  48895c2428                     movq	%rbx, 0x28(%rsp)
// 0x141a3c8d7  eb1d                           jmp	0x141a3c8f6 <.text+0x1a3b8f6>
// 0x141a3c8d9  0f1f8000000000                 nopl	(%rax)
// 0x141a3c8e0  48897038                       movq	%rsi, 0x38(%rax)
// 0x141a3c8e4  4989f5                         movq	%rsi, %r13
// Frames: F115 -> F116
// 0x141a3c8e7  4c89d5                         movq	%r10, %rbp
// 0x141a3c8ea  4929d2                         subq	%rdx, %r10
// Frames: F8 -> F117 -> F116
// 0x141a3c8ed  4d39ca                         cmpq	%r9, %r10
// 0x141a3c8f0  0f831c010000                   jae	0x141a3ca12 <.text+0x1a3ba12>
// Frames: F118
// 0x141a3c8f6  470fb61c10                     movzbl	(%r8,%r10), %r11d
// 0x141a3c8fb  4c0fa3db                       btq	%r11, %rbx
// 0x141a3c8ff  737f                           jae	0x141a3c980 <.text+0x1a3b980>
// Frames: F119
// 0x141a3c901  4939cd                         cmpq	%rcx, %r13
// 0x141a3c904  4989cb                         movq	%rcx, %r11
// 0x141a3c907  4d0f42dd                       cmovbq	%r13, %r11
// 0x141a3c90b  4489f3                         movl	%r14d, %ebx
// 0x141a3c90e  4584f6                         testb	%r14b, %r14b
// 0x141a3c911  4c0f45d9                       cmovneq	%rcx, %r11
// Frames: F120
// 0x141a3c915  498d73ff                       leaq	-0x1(%r11), %rsi
// 0x141a3c919  4839d6                         cmpq	%rdx, %rsi
// 0x141a3c91c  0f837e000000                   jae	0x141a3c9a0 <.text+0x1a3b9a0>
// Frames: F121 -> F122 -> F123 -> F120
// 0x141a3c922  4d89dc                         movq	%r11, %r12
// 0x141a3c925  49f7dc                         negq	%r12
// 0x141a3c928  488b742440                     movq	0x40(%rsp), %rsi
// 0x141a3c92d  4e8d3c1e                       leaq	(%rsi,%r11), %r15
// 0x141a3c931  4b8d3413                       leaq	(%r11,%r10), %rsi
// 0x141a3c935  48ffce                         decq	%rsi
// 0x141a3c938  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F124 -> F121 -> F122 -> F123 -> F120
// 0x141a3c940  4d85e4                         testq	%r12, %r12
// Frames: F121 -> F122 -> F123 -> F120
// 0x141a3c943  7464                           je	0x141a3c9a9 <.text+0x1a3b9a9>
// Frames: F125
// 0x141a3c945  4c39ce                         cmpq	%r9, %rsi
// 0x141a3c948  0f8336010000                   jae	0x141a3ca84 <.text+0x1a3ba84>
// 0x141a3c94e  450fb61f                       movzbl	(%r15), %r11d
// 0x141a3c952  49ffc4                         incq	%r12
// 0x141a3c955  49ffcf                         decq	%r15
// 0x141a3c958  4c8d76ff                       leaq	-0x1(%rsi), %r14
// 0x141a3c95c  453a1c30                       cmpb	(%r8,%rsi), %r11b
// 0x141a3c960  4c89f6                         movq	%r14, %rsi
// 0x141a3c963  74db                           je	0x141a3c940 <.text+0x1a3b940>
// Frames: F126
// 0x141a3c965  4829cd                         subq	%rcx, %rbp
// 0x141a3c968  4c29e5                         subq	%r12, %rbp
// 0x141a3c96b  48896828                       movq	%rbp, 0x28(%rax)
// 0x141a3c96f  4889d6                         movq	%rdx, %rsi
// 0x141a3c972  e983000000                     jmp	0x141a3c9fa <.text+0x1a3b9fa>
// 0x141a3c977  660f1f840000000000             nopw	(%rax,%rax)
// Frames: F127
// 0x141a3c980  4c895028                       movq	%r10, 0x28(%rax)
// 0x141a3c984  4889d6                         movq	%rdx, %rsi
// Frames: F128
// 0x141a3c987  4584f6                         testb	%r14b, %r14b
// 0x141a3c98a  0f8450ffffff                   je	0x141a3c8e0 <.text+0x1a3b8e0>
// 0x141a3c990  e952ffffff                     jmp	0x141a3c8e7 <.text+0x1a3b8e7>
// 0x141a3c995  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F124 -> F121 -> F122 -> F123 -> F120
// 0x141a3c9a0  4d85db                         testq	%r11, %r11
// Frames: F121 -> F122 -> F123 -> F120
// 0x141a3c9a3  0f85ed000000                   jne	0x141a3ca96 <.text+0x1a3ba96>
// Frames: F129
// 0x141a3c9a9  84db                           testb	%bl, %bl
// 0x141a3c9ab  4c89ee                         movq	%r13, %rsi
// 0x141a3c9ae  480f45f2                       cmovneq	%rdx, %rsi
// Frames: F130
// 0x141a3c9b2  4f8d3c10                       leaq	(%r8,%r10), %r15
// 0x141a3c9b6  4989cc                         movq	%rcx, %r12
// 0x141a3c9b9  0f1f8000000000                 nopl	(%rax)
// Frames: F124 -> F131 -> F132 -> F130
// 0x141a3c9c0  4939f4                         cmpq	%rsi, %r12
// Frames: F131 -> F132 -> F130
// 0x141a3c9c3  735e                           jae	0x141a3ca23 <.text+0x1a3ba23>
// Frames: F133
// 0x141a3c9c5  4939d4                         cmpq	%rdx, %r12
// 0x141a3c9c8  0f8387000000                   jae	0x141a3ca55 <.text+0x1a3ba55>
// 0x141a3c9ce  4f8d1c22                       leaq	(%r10,%r12), %r11
// 0x141a3c9d2  4d39cb                         cmpq	%r9, %r11
// 0x141a3c9d5  0f838d000000                   jae	0x141a3ca68 <.text+0x1a3ba68>
// 0x141a3c9db  460fb61c27                     movzbl	(%rdi,%r12), %r11d
// Frames: F134 -> F135 -> F136 -> F132 -> F130
// 0x141a3c9e0  4d8d742401                     leaq	0x1(%r12), %r14
// Frames: F133
// 0x141a3c9e5  473a1c27                       cmpb	(%r15,%r12), %r11b
// 0x141a3c9e9  4d89f4                         movq	%r14, %r12
// 0x141a3c9ec  74d2                           je	0x141a3c9c0 <.text+0x1a3b9c0>
// Frames: F137
// 0x141a3c9ee  488b742438                     movq	0x38(%rsp), %rsi
// 0x141a3c9f3  4829f5                         subq	%rsi, %rbp
// 0x141a3c9f6  48896828                       movq	%rbp, 0x28(%rax)
// 0x141a3c9fa  4989ea                         movq	%rbp, %r10
// 0x141a3c9fd  4189de                         movl	%ebx, %r14d
// 0x141a3ca00  84db                           testb	%bl, %bl
// 0x141a3ca02  488b5c2428                     movq	0x28(%rsp), %rbx
// 0x141a3ca07  0f84d3feffff                   je	0x141a3c8e0 <.text+0x1a3b8e0>
// 0x141a3ca0d  e9d5feffff                     jmp	0x141a3c8e7 <.text+0x1a3b8e7>
// Frames: F138
// 0x141a3ca12  48c7402800000000               movq	$0x0, 0x28(%rax)
// 0x141a3ca1a  31c0                           xorl	%eax, %eax
// 0x141a3ca1c  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x141a3ca21  eb1e                           jmp	0x141a3ca41 <.text+0x1a3ba41>
// 0x141a3ca23  4c895028                       movq	%r10, 0x28(%rax)
// 0x141a3ca27  84db                           testb	%bl, %bl
// 0x141a3ca29  7504                           jne	0x141a3ca2f <.text+0x1a3ba2f>
// 0x141a3ca2b  48895038                       movq	%rdx, 0x38(%rax)
// Frames: F143 -> F138
// 0x141a3ca2f  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x141a3ca34  4c895108                       movq	%r10, 0x8(%rcx)
// 0x141a3ca38  48896910                       movq	%rbp, 0x10(%rcx)
// 0x141a3ca3c  b801000000                     movl	$0x1, %eax
// Frames: F138
// 0x141a3ca41  488901                         movq	%rax, (%rcx)
// 0x141a3ca44  4883c448                       addq	$0x48, %rsp
// 0x141a3ca48  5b                             popq	%rbx
// 0x141a3ca49  5d                             popq	%rbp
// 0x141a3ca4a  5f                             popq	%rdi
// 0x141a3ca4b  5e                             popq	%rsi
// 0x141a3ca4c  415c                           popq	%r12
// 0x141a3ca4e  415d                           popq	%r13
// 0x141a3ca50  415e                           popq	%r14
// 0x141a3ca52  415f                           popq	%r15
// 0x141a3ca54  c3                             retq
// Frames: F8 -> F117 -> F138
// 0x141a3ca55  4839d1                         cmpq	%rdx, %rcx
// Frames: F8 -> F117 -> F116
// 0x141a3ca58  480f46ca                       cmovbeq	%rdx, %rcx
// Frames: F133
// 0x141a3ca5c  4c8d0555ec4701                 leaq	0x147ec55(%rip), %r8    # 0x142ebb6b8
// 0x141a3ca63  e8b0cae900                     callq	0x1428d9518 <.text+0x28d8518>
// Frames: F130
// 0x141a3ca68  4901ca                         addq	%rcx, %r10
// 0x141a3ca6b  4d39d1                         cmpq	%r10, %r9
// 0x141a3ca6e  4d0f47d1                       cmovaq	%r9, %r10
// Frames: F133
// 0x141a3ca72  4c8d0557ec4701                 leaq	0x147ec57(%rip), %r8    # 0x142ebb6d0
// 0x141a3ca79  4c89d1                         movq	%r10, %rcx
// 0x141a3ca7c  4c89ca                         movq	%r9, %rdx
// 0x141a3ca7f  e894cae900                     callq	0x1428d9518 <.text+0x28d8518>
// Frames: F125
// 0x141a3ca84  4c8d0575ec4701                 leaq	0x147ec75(%rip), %r8    # 0x142ebb700
// 0x141a3ca8b  4889f1                         movq	%rsi, %rcx
// 0x141a3ca8e  4c89ca                         movq	%r9, %rdx
// 0x141a3ca91  e882cae900                     callq	0x1428d9518 <.text+0x28d8518>
// 0x141a3ca96  4c8d054bec4701                 leaq	0x147ec4b(%rip), %r8    # 0x142ebb6e8
// 0x141a3ca9d  4889f1                         movq	%rsi, %rcx
// 0x141a3caa0  e873cae900                     callq	0x1428d9518 <.text+0x28d8518>
// 0x141a3caa5  cc                             int3
//
// RANGE rva_2196bd0_len_236: RVA 0x2196bd0, 566 PDB bytes; 148 instructions
// Evidence: _assembly/roof-v1/B/pack-001/rva_2196bd0_len_236.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3430:2460: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Frames: F114
// 0x142196bd0  4157                           pushq	%r15
// 0x142196bd2  4156                           pushq	%r14
// 0x142196bd4  4155                           pushq	%r13
// 0x142196bd6  4154                           pushq	%r12
// 0x142196bd8  56                             pushq	%rsi
// 0x142196bd9  57                             pushq	%rdi
// 0x142196bda  55                             pushq	%rbp
// 0x142196bdb  53                             pushq	%rbx
// 0x142196bdc  4883ec48                       subq	$0x48, %rsp
// 0x142196be0  4889d0                         movq	%rdx, %rax
// 0x142196be3  48894c2430                     movq	%rcx, 0x30(%rsp)
// 0x142196be8  488b9424b8000000               movq	0xb8(%rsp), %rdx
// 0x142196bf0  488b6828                       movq	0x28(%rax), %rbp
// Frames: F115 -> F116
// 0x142196bf4  4989ea                         movq	%rbp, %r10
// 0x142196bf7  4929d2                         subq	%rdx, %r10
// Frames: F8 -> F117 -> F116
// 0x142196bfa  4d39ca                         cmpq	%r9, %r10
// 0x142196bfd  0f836f010000                   jae	0x142196d72 <.text+0x2195d72>
// 0x142196c03  440fb6b424c0000000             movzbl	0xc0(%rsp), %r14d
// 0x142196c0c  488bbc24b0000000               movq	0xb0(%rsp), %rdi
// 0x142196c14  488b5818                       movq	0x18(%rax), %rbx
// 0x142196c18  488b4808                       movq	0x8(%rax), %rcx
// 0x142196c1c  4c8b5810                       movq	0x10(%rax), %r11
// 0x142196c20  4c895c2438                     movq	%r11, 0x38(%rsp)
// 0x142196c25  4c8b6838                       movq	0x38(%rax), %r13
// 0x142196c29  4c8d5fff                       leaq	-0x1(%rdi), %r11
// 0x142196c2d  4c895c2440                     movq	%r11, 0x40(%rsp)
// 0x142196c32  48895c2428                     movq	%rbx, 0x28(%rsp)
// 0x142196c37  eb1d                           jmp	0x142196c56 <.text+0x2195c56>
// 0x142196c39  0f1f8000000000                 nopl	(%rax)
// 0x142196c40  48897038                       movq	%rsi, 0x38(%rax)
// 0x142196c44  4989f5                         movq	%rsi, %r13
// Frames: F115 -> F116
// 0x142196c47  4c89d5                         movq	%r10, %rbp
// 0x142196c4a  4929d2                         subq	%rdx, %r10
// Frames: F8 -> F117 -> F116
// 0x142196c4d  4d39ca                         cmpq	%r9, %r10
// 0x142196c50  0f831c010000                   jae	0x142196d72 <.text+0x2195d72>
// Frames: F118
// 0x142196c56  470fb61c10                     movzbl	(%r8,%r10), %r11d
// 0x142196c5b  4c0fa3db                       btq	%r11, %rbx
// 0x142196c5f  737f                           jae	0x142196ce0 <.text+0x2195ce0>
// Frames: F119
// 0x142196c61  4939cd                         cmpq	%rcx, %r13
// 0x142196c64  4989cb                         movq	%rcx, %r11
// 0x142196c67  4d0f42dd                       cmovbq	%r13, %r11
// 0x142196c6b  4489f3                         movl	%r14d, %ebx
// 0x142196c6e  4584f6                         testb	%r14b, %r14b
// 0x142196c71  4c0f45d9                       cmovneq	%rcx, %r11
// Frames: F120
// 0x142196c75  498d73ff                       leaq	-0x1(%r11), %rsi
// 0x142196c79  4839d6                         cmpq	%rdx, %rsi
// 0x142196c7c  0f837e000000                   jae	0x142196d00 <.text+0x2195d00>
// Frames: F121 -> F122 -> F123 -> F120
// 0x142196c82  4d89dc                         movq	%r11, %r12
// 0x142196c85  49f7dc                         negq	%r12
// 0x142196c88  488b742440                     movq	0x40(%rsp), %rsi
// 0x142196c8d  4e8d3c1e                       leaq	(%rsi,%r11), %r15
// 0x142196c91  4b8d3413                       leaq	(%r11,%r10), %rsi
// 0x142196c95  48ffce                         decq	%rsi
// 0x142196c98  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F124 -> F121 -> F122 -> F123 -> F120
// 0x142196ca0  4d85e4                         testq	%r12, %r12
// Frames: F121 -> F122 -> F123 -> F120
// 0x142196ca3  7464                           je	0x142196d09 <.text+0x2195d09>
// Frames: F125
// 0x142196ca5  4c39ce                         cmpq	%r9, %rsi
// 0x142196ca8  0f8336010000                   jae	0x142196de4 <.text+0x2195de4>
// 0x142196cae  450fb61f                       movzbl	(%r15), %r11d
// 0x142196cb2  49ffc4                         incq	%r12
// 0x142196cb5  49ffcf                         decq	%r15
// 0x142196cb8  4c8d76ff                       leaq	-0x1(%rsi), %r14
// 0x142196cbc  453a1c30                       cmpb	(%r8,%rsi), %r11b
// 0x142196cc0  4c89f6                         movq	%r14, %rsi
// 0x142196cc3  74db                           je	0x142196ca0 <.text+0x2195ca0>
// Frames: F126
// 0x142196cc5  4829cd                         subq	%rcx, %rbp
// 0x142196cc8  4c29e5                         subq	%r12, %rbp
// 0x142196ccb  48896828                       movq	%rbp, 0x28(%rax)
// 0x142196ccf  4889d6                         movq	%rdx, %rsi
// 0x142196cd2  e983000000                     jmp	0x142196d5a <.text+0x2195d5a>
// 0x142196cd7  660f1f840000000000             nopw	(%rax,%rax)
// Frames: F127
// 0x142196ce0  4c895028                       movq	%r10, 0x28(%rax)
// 0x142196ce4  4889d6                         movq	%rdx, %rsi
// Frames: F128
// 0x142196ce7  4584f6                         testb	%r14b, %r14b
// 0x142196cea  0f8450ffffff                   je	0x142196c40 <.text+0x2195c40>
// 0x142196cf0  e952ffffff                     jmp	0x142196c47 <.text+0x2195c47>
// 0x142196cf5  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F124 -> F121 -> F122 -> F123 -> F120
// 0x142196d00  4d85db                         testq	%r11, %r11
// Frames: F121 -> F122 -> F123 -> F120
// 0x142196d03  0f85ed000000                   jne	0x142196df6 <.text+0x2195df6>
// Frames: F129
// 0x142196d09  84db                           testb	%bl, %bl
// 0x142196d0b  4c89ee                         movq	%r13, %rsi
// 0x142196d0e  480f45f2                       cmovneq	%rdx, %rsi
// Frames: F130
// 0x142196d12  4f8d3c10                       leaq	(%r8,%r10), %r15
// 0x142196d16  4989cc                         movq	%rcx, %r12
// 0x142196d19  0f1f8000000000                 nopl	(%rax)
// Frames: F124 -> F131 -> F132 -> F130
// 0x142196d20  4939f4                         cmpq	%rsi, %r12
// Frames: F131 -> F132 -> F130
// 0x142196d23  735e                           jae	0x142196d83 <.text+0x2195d83>
// Frames: F133
// 0x142196d25  4939d4                         cmpq	%rdx, %r12
// 0x142196d28  0f8387000000                   jae	0x142196db5 <.text+0x2195db5>
// 0x142196d2e  4f8d1c22                       leaq	(%r10,%r12), %r11
// 0x142196d32  4d39cb                         cmpq	%r9, %r11
// 0x142196d35  0f838d000000                   jae	0x142196dc8 <.text+0x2195dc8>
// 0x142196d3b  460fb61c27                     movzbl	(%rdi,%r12), %r11d
// Frames: F134 -> F135 -> F136 -> F132 -> F130
// 0x142196d40  4d8d742401                     leaq	0x1(%r12), %r14
// Frames: F133
// 0x142196d45  473a1c27                       cmpb	(%r15,%r12), %r11b
// 0x142196d49  4d89f4                         movq	%r14, %r12
// 0x142196d4c  74d2                           je	0x142196d20 <.text+0x2195d20>
// Frames: F137
// 0x142196d4e  488b742438                     movq	0x38(%rsp), %rsi
// Frames: F139
// 0x142196d53  4829f5                         subq	%rsi, %rbp
// 0x142196d56  48896828                       movq	%rbp, 0x28(%rax)
// 0x142196d5a  4989ea                         movq	%rbp, %r10
// 0x142196d5d  4189de                         movl	%ebx, %r14d
// 0x142196d60  84db                           testb	%bl, %bl
// 0x142196d62  488b5c2428                     movq	0x28(%rsp), %rbx
// 0x142196d67  0f84d3feffff                   je	0x142196c40 <.text+0x2195c40>
// 0x142196d6d  e9d5feffff                     jmp	0x142196c47 <.text+0x2195c47>
// 0x142196d72  48c7402800000000               movq	$0x0, 0x28(%rax)
// 0x142196d7a  31c0                           xorl	%eax, %eax
// 0x142196d7c  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x142196d81  eb1e                           jmp	0x142196da1 <.text+0x2195da1>
// 0x142196d83  4c895028                       movq	%r10, 0x28(%rax)
// 0x142196d87  84db                           testb	%bl, %bl
// 0x142196d89  7504                           jne	0x142196d8f <.text+0x2195d8f>
// 0x142196d8b  48895038                       movq	%rdx, 0x38(%rax)
// Frames: F143 -> F139
// 0x142196d8f  488b4c2430                     movq	0x30(%rsp), %rcx
// Frames: F143 -> F144
// 0x142196d94  4c895108                       movq	%r10, 0x8(%rcx)
// 0x142196d98  48896910                       movq	%rbp, 0x10(%rcx)
// 0x142196d9c  b801000000                     movl	$0x1, %eax
// Frames: F145
// 0x142196da1  488901                         movq	%rax, (%rcx)
// 0x142196da4  4883c448                       addq	$0x48, %rsp
// 0x142196da8  5b                             popq	%rbx
// 0x142196da9  5d                             popq	%rbp
// 0x142196daa  5f                             popq	%rdi
// 0x142196dab  5e                             popq	%rsi
// 0x142196dac  415c                           popq	%r12
// 0x142196dae  415d                           popq	%r13
// 0x142196db0  415e                           popq	%r14
// 0x142196db2  415f                           popq	%r15
// 0x142196db4  c3                             retq
// Frames: F8 -> F117 -> F116
// 0x142196db5  4839d1                         cmpq	%rdx, %rcx
// 0x142196db8  480f46ca                       cmovbeq	%rdx, %rcx
// Frames: F133
// 0x142196dbc  4c8d059dd1e500                 leaq	0xe5d19d(%rip), %r8     # 0x142ff3f60
// 0x142196dc3  e850277400                     callq	0x1428d9518 <.text+0x28d8518>
// Frames: F130
// 0x142196dc8  4901ca                         addq	%rcx, %r10
// 0x142196dcb  4d39d1                         cmpq	%r10, %r9
// 0x142196dce  4d0f47d1                       cmovaq	%r9, %r10
// Frames: F133
// 0x142196dd2  4c8d059fd1e500                 leaq	0xe5d19f(%rip), %r8     # 0x142ff3f78
// 0x142196dd9  4c89d1                         movq	%r10, %rcx
// 0x142196ddc  4c89ca                         movq	%r9, %rdx
// 0x142196ddf  e834277400                     callq	0x1428d9518 <.text+0x28d8518>
// Frames: F125
// 0x142196de4  4c8d05bdd1e500                 leaq	0xe5d1bd(%rip), %r8     # 0x142ff3fa8
// 0x142196deb  4889f1                         movq	%rsi, %rcx
// 0x142196dee  4c89ca                         movq	%r9, %rdx
// 0x142196df1  e822277400                     callq	0x1428d9518 <.text+0x28d8518>
// 0x142196df6  4c8d0593d1e500                 leaq	0xe5d193(%rip), %r8     # 0x142ff3f90
// 0x142196dfd  4889f1                         movq	%rsi, %rcx
// 0x142196e00  e813277400                     callq	0x1428d9518 <.text+0x28d8518>
// 0x142196e05  cc                             int3
//
// RANGE rva_21a7ed0_len_236: RVA 0x21a7ed0, 566 PDB bytes; 148 instructions
// Evidence: _assembly/roof-v1/B/pack-002/rva_21a7ed0_len_236.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3466:2164: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Frames: F114
// 0x1421a7ed0  4157                           pushq	%r15
// 0x1421a7ed2  4156                           pushq	%r14
// 0x1421a7ed4  4155                           pushq	%r13
// 0x1421a7ed6  4154                           pushq	%r12
// 0x1421a7ed8  56                             pushq	%rsi
// 0x1421a7ed9  57                             pushq	%rdi
// 0x1421a7eda  55                             pushq	%rbp
// 0x1421a7edb  53                             pushq	%rbx
// 0x1421a7edc  4883ec48                       subq	$0x48, %rsp
// 0x1421a7ee0  4889d0                         movq	%rdx, %rax
// 0x1421a7ee3  48894c2430                     movq	%rcx, 0x30(%rsp)
// 0x1421a7ee8  488b9424b8000000               movq	0xb8(%rsp), %rdx
// 0x1421a7ef0  488b6828                       movq	0x28(%rax), %rbp
// Frames: F115 -> F116
// 0x1421a7ef4  4989ea                         movq	%rbp, %r10
// 0x1421a7ef7  4929d2                         subq	%rdx, %r10
// Frames: F8 -> F117 -> F116
// 0x1421a7efa  4d39ca                         cmpq	%r9, %r10
// 0x1421a7efd  0f836f010000                   jae	0x1421a8072 <.text+0x21a7072>
// 0x1421a7f03  440fb6b424c0000000             movzbl	0xc0(%rsp), %r14d
// 0x1421a7f0c  488bbc24b0000000               movq	0xb0(%rsp), %rdi
// 0x1421a7f14  488b5818                       movq	0x18(%rax), %rbx
// 0x1421a7f18  488b4808                       movq	0x8(%rax), %rcx
// 0x1421a7f1c  4c8b5810                       movq	0x10(%rax), %r11
// 0x1421a7f20  4c895c2438                     movq	%r11, 0x38(%rsp)
// 0x1421a7f25  4c8b6838                       movq	0x38(%rax), %r13
// 0x1421a7f29  4c8d5fff                       leaq	-0x1(%rdi), %r11
// 0x1421a7f2d  4c895c2440                     movq	%r11, 0x40(%rsp)
// 0x1421a7f32  48895c2428                     movq	%rbx, 0x28(%rsp)
// 0x1421a7f37  eb1d                           jmp	0x1421a7f56 <.text+0x21a6f56>
// 0x1421a7f39  0f1f8000000000                 nopl	(%rax)
// 0x1421a7f40  48897038                       movq	%rsi, 0x38(%rax)
// 0x1421a7f44  4989f5                         movq	%rsi, %r13
// Frames: F115 -> F116
// 0x1421a7f47  4c89d5                         movq	%r10, %rbp
// 0x1421a7f4a  4929d2                         subq	%rdx, %r10
// Frames: F8 -> F117 -> F116
// 0x1421a7f4d  4d39ca                         cmpq	%r9, %r10
// 0x1421a7f50  0f831c010000                   jae	0x1421a8072 <.text+0x21a7072>
// Frames: F118
// 0x1421a7f56  470fb61c10                     movzbl	(%r8,%r10), %r11d
// 0x1421a7f5b  4c0fa3db                       btq	%r11, %rbx
// 0x1421a7f5f  737f                           jae	0x1421a7fe0 <.text+0x21a6fe0>
// Frames: F119
// 0x1421a7f61  4939cd                         cmpq	%rcx, %r13
// 0x1421a7f64  4989cb                         movq	%rcx, %r11
// 0x1421a7f67  4d0f42dd                       cmovbq	%r13, %r11
// 0x1421a7f6b  4489f3                         movl	%r14d, %ebx
// 0x1421a7f6e  4584f6                         testb	%r14b, %r14b
// 0x1421a7f71  4c0f45d9                       cmovneq	%rcx, %r11
// Frames: F120
// 0x1421a7f75  498d73ff                       leaq	-0x1(%r11), %rsi
// 0x1421a7f79  4839d6                         cmpq	%rdx, %rsi
// 0x1421a7f7c  0f837e000000                   jae	0x1421a8000 <.text+0x21a7000>
// Frames: F121 -> F122 -> F123 -> F120
// 0x1421a7f82  4d89dc                         movq	%r11, %r12
// 0x1421a7f85  49f7dc                         negq	%r12
// 0x1421a7f88  488b742440                     movq	0x40(%rsp), %rsi
// 0x1421a7f8d  4e8d3c1e                       leaq	(%rsi,%r11), %r15
// 0x1421a7f91  4b8d3413                       leaq	(%r11,%r10), %rsi
// 0x1421a7f95  48ffce                         decq	%rsi
// 0x1421a7f98  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F124 -> F121 -> F122 -> F123 -> F120
// 0x1421a7fa0  4d85e4                         testq	%r12, %r12
// Frames: F121 -> F122 -> F123 -> F120
// 0x1421a7fa3  7464                           je	0x1421a8009 <.text+0x21a7009>
// Frames: F125
// 0x1421a7fa5  4c39ce                         cmpq	%r9, %rsi
// 0x1421a7fa8  0f8336010000                   jae	0x1421a80e4 <.text+0x21a70e4>
// 0x1421a7fae  450fb61f                       movzbl	(%r15), %r11d
// 0x1421a7fb2  49ffc4                         incq	%r12
// 0x1421a7fb5  49ffcf                         decq	%r15
// 0x1421a7fb8  4c8d76ff                       leaq	-0x1(%rsi), %r14
// 0x1421a7fbc  453a1c30                       cmpb	(%r8,%rsi), %r11b
// 0x1421a7fc0  4c89f6                         movq	%r14, %rsi
// 0x1421a7fc3  74db                           je	0x1421a7fa0 <.text+0x21a6fa0>
// Frames: F126
// 0x1421a7fc5  4829cd                         subq	%rcx, %rbp
// 0x1421a7fc8  4c29e5                         subq	%r12, %rbp
// 0x1421a7fcb  48896828                       movq	%rbp, 0x28(%rax)
// 0x1421a7fcf  4889d6                         movq	%rdx, %rsi
// 0x1421a7fd2  e983000000                     jmp	0x1421a805a <.text+0x21a705a>
// 0x1421a7fd7  660f1f840000000000             nopw	(%rax,%rax)
// Frames: F127
// 0x1421a7fe0  4c895028                       movq	%r10, 0x28(%rax)
// 0x1421a7fe4  4889d6                         movq	%rdx, %rsi
// Frames: F128
// 0x1421a7fe7  4584f6                         testb	%r14b, %r14b
// 0x1421a7fea  0f8450ffffff                   je	0x1421a7f40 <.text+0x21a6f40>
// 0x1421a7ff0  e952ffffff                     jmp	0x1421a7f47 <.text+0x21a6f47>
// 0x1421a7ff5  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F124 -> F121 -> F122 -> F123 -> F120
// 0x1421a8000  4d85db                         testq	%r11, %r11
// Frames: F121 -> F122 -> F123 -> F120
// 0x1421a8003  0f85ed000000                   jne	0x1421a80f6 <.text+0x21a70f6>
// Frames: F129
// 0x1421a8009  84db                           testb	%bl, %bl
// 0x1421a800b  4c89ee                         movq	%r13, %rsi
// 0x1421a800e  480f45f2                       cmovneq	%rdx, %rsi
// Frames: F130
// 0x1421a8012  4f8d3c10                       leaq	(%r8,%r10), %r15
// 0x1421a8016  4989cc                         movq	%rcx, %r12
// 0x1421a8019  0f1f8000000000                 nopl	(%rax)
// Frames: F124 -> F131 -> F132 -> F130
// 0x1421a8020  4939f4                         cmpq	%rsi, %r12
// Frames: F131 -> F132 -> F130
// 0x1421a8023  735e                           jae	0x1421a8083 <.text+0x21a7083>
// Frames: F133
// 0x1421a8025  4939d4                         cmpq	%rdx, %r12
// 0x1421a8028  0f8387000000                   jae	0x1421a80b5 <.text+0x21a70b5>
// 0x1421a802e  4f8d1c22                       leaq	(%r10,%r12), %r11
// 0x1421a8032  4d39cb                         cmpq	%r9, %r11
// 0x1421a8035  0f838d000000                   jae	0x1421a80c8 <.text+0x21a70c8>
// 0x1421a803b  460fb61c27                     movzbl	(%rdi,%r12), %r11d
// Frames: F134 -> F135 -> F136 -> F132 -> F130
// 0x1421a8040  4d8d742401                     leaq	0x1(%r12), %r14
// Frames: F133
// 0x1421a8045  473a1c27                       cmpb	(%r15,%r12), %r11b
// Frames: F139
// 0x1421a8049  4d89f4                         movq	%r14, %r12
// 0x1421a804c  74d2                           je	0x1421a8020 <.text+0x21a7020>
// 0x1421a804e  488b742438                     movq	0x38(%rsp), %rsi
// 0x1421a8053  4829f5                         subq	%rsi, %rbp
// 0x1421a8056  48896828                       movq	%rbp, 0x28(%rax)
// 0x1421a805a  4989ea                         movq	%rbp, %r10
// 0x1421a805d  4189de                         movl	%ebx, %r14d
// 0x1421a8060  84db                           testb	%bl, %bl
// 0x1421a8062  488b5c2428                     movq	0x28(%rsp), %rbx
// 0x1421a8067  0f84d3feffff                   je	0x1421a7f40 <.text+0x21a6f40>
// 0x1421a806d  e9d5feffff                     jmp	0x1421a7f47 <.text+0x21a6f47>
// 0x1421a8072  48c7402800000000               movq	$0x0, 0x28(%rax)
// 0x1421a807a  31c0                           xorl	%eax, %eax
// 0x1421a807c  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x1421a8081  eb1e                           jmp	0x1421a80a1 <.text+0x21a70a1>
// 0x1421a8083  4c895028                       movq	%r10, 0x28(%rax)
// Frames: F141
// 0x1421a8087  84db                           testb	%bl, %bl
// 0x1421a8089  7504                           jne	0x1421a808f <.text+0x21a708f>
// Frames: F142
// 0x1421a808b  48895038                       movq	%rdx, 0x38(%rax)
// Frames: F143 -> F144
// 0x1421a808f  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x1421a8094  4c895108                       movq	%r10, 0x8(%rcx)
// 0x1421a8098  48896910                       movq	%rbp, 0x10(%rcx)
// 0x1421a809c  b801000000                     movl	$0x1, %eax
// Frames: F145
// 0x1421a80a1  488901                         movq	%rax, (%rcx)
// 0x1421a80a4  4883c448                       addq	$0x48, %rsp
// 0x1421a80a8  5b                             popq	%rbx
// 0x1421a80a9  5d                             popq	%rbp
// 0x1421a80aa  5f                             popq	%rdi
// 0x1421a80ab  5e                             popq	%rsi
// 0x1421a80ac  415c                           popq	%r12
// 0x1421a80ae  415d                           popq	%r13
// 0x1421a80b0  415e                           popq	%r14
// 0x1421a80b2  415f                           popq	%r15
// 0x1421a80b4  c3                             retq
// Frames: F8 -> F117 -> F116
// 0x1421a80b5  4839d1                         cmpq	%rdx, %rcx
// 0x1421a80b8  480f46ca                       cmovbeq	%rdx, %rcx
// Frames: F133
// 0x1421a80bc  4c8d055dede400                 leaq	0xe4ed5d(%rip), %r8     # 0x142ff6e20
// 0x1421a80c3  e850147300                     callq	0x1428d9518 <.text+0x28d8518>
// Frames: F130
// 0x1421a80c8  4901ca                         addq	%rcx, %r10
// 0x1421a80cb  4d39d1                         cmpq	%r10, %r9
// 0x1421a80ce  4d0f47d1                       cmovaq	%r9, %r10
// Frames: F133
// 0x1421a80d2  4c8d055fede400                 leaq	0xe4ed5f(%rip), %r8     # 0x142ff6e38
// 0x1421a80d9  4c89d1                         movq	%r10, %rcx
// 0x1421a80dc  4c89ca                         movq	%r9, %rdx
// 0x1421a80df  e834147300                     callq	0x1428d9518 <.text+0x28d8518>
// Frames: F125
// 0x1421a80e4  4c8d057dede400                 leaq	0xe4ed7d(%rip), %r8     # 0x142ff6e68
// 0x1421a80eb  4889f1                         movq	%rsi, %rcx
// 0x1421a80ee  4c89ca                         movq	%r9, %rdx
// 0x1421a80f1  e822147300                     callq	0x1428d9518 <.text+0x28d8518>
// 0x1421a80f6  4c8d0553ede400                 leaq	0xe4ed53(%rip), %r8     # 0x142ff6e50
// 0x1421a80fd  4889f1                         movq	%rsi, %rcx
// 0x1421a8100  e813147300                     callq	0x1428d9518 <.text+0x28d8518>
// 0x1421a8105  cc                             int3
//
// RANGE rva_21ad100_len_236: RVA 0x21ad100, 566 PDB bytes; 148 instructions
// Evidence: _assembly/roof-v1/B/pack-002/rva_21ad100_len_236.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3470:1908: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// Frames: F114
// 0x1421ad100  4157                           pushq	%r15
// 0x1421ad102  4156                           pushq	%r14
// 0x1421ad104  4155                           pushq	%r13
// 0x1421ad106  4154                           pushq	%r12
// 0x1421ad108  56                             pushq	%rsi
// 0x1421ad109  57                             pushq	%rdi
// 0x1421ad10a  55                             pushq	%rbp
// 0x1421ad10b  53                             pushq	%rbx
// 0x1421ad10c  4883ec48                       subq	$0x48, %rsp
// 0x1421ad110  4889d0                         movq	%rdx, %rax
// 0x1421ad113  48894c2430                     movq	%rcx, 0x30(%rsp)
// 0x1421ad118  488b9424b8000000               movq	0xb8(%rsp), %rdx
// 0x1421ad120  488b6828                       movq	0x28(%rax), %rbp
// Frames: F115 -> F116
// 0x1421ad124  4989ea                         movq	%rbp, %r10
// 0x1421ad127  4929d2                         subq	%rdx, %r10
// Frames: F8 -> F117 -> F116
// 0x1421ad12a  4d39ca                         cmpq	%r9, %r10
// 0x1421ad12d  0f836f010000                   jae	0x1421ad2a2 <.text+0x21ac2a2>
// 0x1421ad133  440fb6b424c0000000             movzbl	0xc0(%rsp), %r14d
// 0x1421ad13c  488bbc24b0000000               movq	0xb0(%rsp), %rdi
// 0x1421ad144  488b5818                       movq	0x18(%rax), %rbx
// 0x1421ad148  488b4808                       movq	0x8(%rax), %rcx
// 0x1421ad14c  4c8b5810                       movq	0x10(%rax), %r11
// 0x1421ad150  4c895c2438                     movq	%r11, 0x38(%rsp)
// 0x1421ad155  4c8b6838                       movq	0x38(%rax), %r13
// 0x1421ad159  4c8d5fff                       leaq	-0x1(%rdi), %r11
// 0x1421ad15d  4c895c2440                     movq	%r11, 0x40(%rsp)
// 0x1421ad162  48895c2428                     movq	%rbx, 0x28(%rsp)
// 0x1421ad167  eb1d                           jmp	0x1421ad186 <.text+0x21ac186>
// 0x1421ad169  0f1f8000000000                 nopl	(%rax)
// 0x1421ad170  48897038                       movq	%rsi, 0x38(%rax)
// 0x1421ad174  4989f5                         movq	%rsi, %r13
// Frames: F115 -> F116
// 0x1421ad177  4c89d5                         movq	%r10, %rbp
// 0x1421ad17a  4929d2                         subq	%rdx, %r10
// Frames: F8 -> F117 -> F116
// 0x1421ad17d  4d39ca                         cmpq	%r9, %r10
// 0x1421ad180  0f831c010000                   jae	0x1421ad2a2 <.text+0x21ac2a2>
// Frames: F118
// 0x1421ad186  470fb61c10                     movzbl	(%r8,%r10), %r11d
// 0x1421ad18b  4c0fa3db                       btq	%r11, %rbx
// 0x1421ad18f  737f                           jae	0x1421ad210 <.text+0x21ac210>
// Frames: F119
// 0x1421ad191  4939cd                         cmpq	%rcx, %r13
// 0x1421ad194  4989cb                         movq	%rcx, %r11
// 0x1421ad197  4d0f42dd                       cmovbq	%r13, %r11
// 0x1421ad19b  4489f3                         movl	%r14d, %ebx
// 0x1421ad19e  4584f6                         testb	%r14b, %r14b
// 0x1421ad1a1  4c0f45d9                       cmovneq	%rcx, %r11
// Frames: F120
// 0x1421ad1a5  498d73ff                       leaq	-0x1(%r11), %rsi
// 0x1421ad1a9  4839d6                         cmpq	%rdx, %rsi
// 0x1421ad1ac  0f837e000000                   jae	0x1421ad230 <.text+0x21ac230>
// Frames: F121 -> F122 -> F123 -> F120
// 0x1421ad1b2  4d89dc                         movq	%r11, %r12
// 0x1421ad1b5  49f7dc                         negq	%r12
// 0x1421ad1b8  488b742440                     movq	0x40(%rsp), %rsi
// 0x1421ad1bd  4e8d3c1e                       leaq	(%rsi,%r11), %r15
// 0x1421ad1c1  4b8d3413                       leaq	(%r11,%r10), %rsi
// 0x1421ad1c5  48ffce                         decq	%rsi
// 0x1421ad1c8  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F124 -> F121 -> F122 -> F123 -> F120
// 0x1421ad1d0  4d85e4                         testq	%r12, %r12
// Frames: F121 -> F122 -> F123 -> F120
// 0x1421ad1d3  7464                           je	0x1421ad239 <.text+0x21ac239>
// Frames: F125
// 0x1421ad1d5  4c39ce                         cmpq	%r9, %rsi
// 0x1421ad1d8  0f8336010000                   jae	0x1421ad314 <.text+0x21ac314>
// 0x1421ad1de  450fb61f                       movzbl	(%r15), %r11d
// 0x1421ad1e2  49ffc4                         incq	%r12
// 0x1421ad1e5  49ffcf                         decq	%r15
// 0x1421ad1e8  4c8d76ff                       leaq	-0x1(%rsi), %r14
// 0x1421ad1ec  453a1c30                       cmpb	(%r8,%rsi), %r11b
// 0x1421ad1f0  4c89f6                         movq	%r14, %rsi
// 0x1421ad1f3  74db                           je	0x1421ad1d0 <.text+0x21ac1d0>
// Frames: F126
// 0x1421ad1f5  4829cd                         subq	%rcx, %rbp
// 0x1421ad1f8  4c29e5                         subq	%r12, %rbp
// 0x1421ad1fb  48896828                       movq	%rbp, 0x28(%rax)
// 0x1421ad1ff  4889d6                         movq	%rdx, %rsi
// 0x1421ad202  e983000000                     jmp	0x1421ad28a <.text+0x21ac28a>
// 0x1421ad207  660f1f840000000000             nopw	(%rax,%rax)
// Frames: F127
// 0x1421ad210  4c895028                       movq	%r10, 0x28(%rax)
// 0x1421ad214  4889d6                         movq	%rdx, %rsi
// Frames: F128
// 0x1421ad217  4584f6                         testb	%r14b, %r14b
// 0x1421ad21a  0f8450ffffff                   je	0x1421ad170 <.text+0x21ac170>
// 0x1421ad220  e952ffffff                     jmp	0x1421ad177 <.text+0x21ac177>
// 0x1421ad225  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F124 -> F121 -> F122 -> F123 -> F120
// 0x1421ad230  4d85db                         testq	%r11, %r11
// Frames: F121 -> F122 -> F123 -> F120
// 0x1421ad233  0f85ed000000                   jne	0x1421ad326 <.text+0x21ac326>
// Frames: F129
// 0x1421ad239  84db                           testb	%bl, %bl
// 0x1421ad23b  4c89ee                         movq	%r13, %rsi
// 0x1421ad23e  480f45f2                       cmovneq	%rdx, %rsi
// Frames: F130
// 0x1421ad242  4f8d3c10                       leaq	(%r8,%r10), %r15
// 0x1421ad246  4989cc                         movq	%rcx, %r12
// 0x1421ad249  0f1f8000000000                 nopl	(%rax)
// Frames: F124 -> F131 -> F132 -> F130
// 0x1421ad250  4939f4                         cmpq	%rsi, %r12
// Frames: F131 -> F132 -> F130
// 0x1421ad253  735e                           jae	0x1421ad2b3 <.text+0x21ac2b3>
// Frames: F133
// 0x1421ad255  4939d4                         cmpq	%rdx, %r12
// 0x1421ad258  0f8387000000                   jae	0x1421ad2e5 <.text+0x21ac2e5>
// 0x1421ad25e  4f8d1c22                       leaq	(%r10,%r12), %r11
// 0x1421ad262  4d39cb                         cmpq	%r9, %r11
// 0x1421ad265  0f838d000000                   jae	0x1421ad2f8 <.text+0x21ac2f8>
// 0x1421ad26b  460fb61c27                     movzbl	(%rdi,%r12), %r11d
// Frames: F134 -> F135 -> F136 -> F132 -> F130
// 0x1421ad270  4d8d742401                     leaq	0x1(%r12), %r14
// Frames: F133
// 0x1421ad275  473a1c27                       cmpb	(%r15,%r12), %r11b
// 0x1421ad279  4d89f4                         movq	%r14, %r12
// 0x1421ad27c  74d2                           je	0x1421ad250 <.text+0x21ac250>
// Frames: F137
// 0x1421ad27e  488b742438                     movq	0x38(%rsp), %rsi
// 0x1421ad283  4829f5                         subq	%rsi, %rbp
// 0x1421ad286  48896828                       movq	%rbp, 0x28(%rax)
// 0x1421ad28a  4989ea                         movq	%rbp, %r10
// 0x1421ad28d  4189de                         movl	%ebx, %r14d
// 0x1421ad290  84db                           testb	%bl, %bl
// 0x1421ad292  488b5c2428                     movq	0x28(%rsp), %rbx
// 0x1421ad297  0f84d3feffff                   je	0x1421ad170 <.text+0x21ac170>
// 0x1421ad29d  e9d5feffff                     jmp	0x1421ad177 <.text+0x21ac177>
// Frames: F138
// 0x1421ad2a2  48c7402800000000               movq	$0x0, 0x28(%rax)
// Frames: F139
// 0x1421ad2aa  31c0                           xorl	%eax, %eax
// 0x1421ad2ac  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x1421ad2b1  eb1e                           jmp	0x1421ad2d1 <.text+0x21ac2d1>
// Frames: F140
// 0x1421ad2b3  4c895028                       movq	%r10, 0x28(%rax)
// Frames: F141
// 0x1421ad2b7  84db                           testb	%bl, %bl
// 0x1421ad2b9  7504                           jne	0x1421ad2bf <.text+0x21ac2bf>
// Frames: F142
// 0x1421ad2bb  48895038                       movq	%rdx, 0x38(%rax)
// Frames: F143 -> F144
// 0x1421ad2bf  488b4c2430                     movq	0x30(%rsp), %rcx
// 0x1421ad2c4  4c895108                       movq	%r10, 0x8(%rcx)
// 0x1421ad2c8  48896910                       movq	%rbp, 0x10(%rcx)
// 0x1421ad2cc  b801000000                     movl	$0x1, %eax
// Frames: F145
// 0x1421ad2d1  488901                         movq	%rax, (%rcx)
// 0x1421ad2d4  4883c448                       addq	$0x48, %rsp
// 0x1421ad2d8  5b                             popq	%rbx
// 0x1421ad2d9  5d                             popq	%rbp
// 0x1421ad2da  5f                             popq	%rdi
// 0x1421ad2db  5e                             popq	%rsi
// 0x1421ad2dc  415c                           popq	%r12
// 0x1421ad2de  415d                           popq	%r13
// 0x1421ad2e0  415e                           popq	%r14
// 0x1421ad2e2  415f                           popq	%r15
// 0x1421ad2e4  c3                             retq
// Frames: F8 -> F117 -> F116
// 0x1421ad2e5  4839d1                         cmpq	%rdx, %rcx
// 0x1421ad2e8  480f46ca                       cmovbeq	%rdx, %rcx
// Frames: F133
// 0x1421ad2ec  4c8d059da5e400                 leaq	0xe4a59d(%rip), %r8     # 0x142ff7890
// 0x1421ad2f3  e820c27200                     callq	0x1428d9518 <.text+0x28d8518>
// Frames: F130
// 0x1421ad2f8  4901ca                         addq	%rcx, %r10
// 0x1421ad2fb  4d39d1                         cmpq	%r10, %r9
// 0x1421ad2fe  4d0f47d1                       cmovaq	%r9, %r10
// Frames: F133
// 0x1421ad302  4c8d059fa5e400                 leaq	0xe4a59f(%rip), %r8     # 0x142ff78a8
// 0x1421ad309  4c89d1                         movq	%r10, %rcx
// 0x1421ad30c  4c89ca                         movq	%r9, %rdx
// 0x1421ad30f  e804c27200                     callq	0x1428d9518 <.text+0x28d8518>
// Frames: F125
// 0x1421ad314  4c8d05bda5e400                 leaq	0xe4a5bd(%rip), %r8     # 0x142ff78d8
// 0x1421ad31b  4889f1                         movq	%rsi, %rcx
// 0x1421ad31e  4c89ca                         movq	%r9, %rdx
// 0x1421ad321  e8f2c17200                     callq	0x1428d9518 <.text+0x28d8518>
// 0x1421ad326  4c8d0593a5e400                 leaq	0xe4a593(%rip), %r8     # 0x142ff78c0
// 0x1421ad32d  4889f1                         movq	%rsi, %rcx
// 0x1421ad330  e8e3c17200                     callq	0x1428d9518 <.text+0x28d8518>
// 0x1421ad335  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/lib.rs:61:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs:1826:0 bevy_ecs::world::World::contains_resource
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs:343:0 bevy_app::sub_app::SubApp::add_event
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs:362:0 bevy_app::app::App::add_event
// F5: crates/systems/roof/src/lib.rs:62:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F6: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1451:0 core::option::Option::and_then
// F7: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs:1828:0 bevy_ecs::world::World::contains_resource
// F8: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:228:0 core::slice::index::impl$2::get
// F9: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 core::slice::impl$0::get
// F10: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:60:0 bevy_ecs::storage::sparse_set::SparseArray::get
// F11: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:396:0 bevy_ecs::storage::sparse_set::SparseSet::get
// F12: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs:342:0 bevy_ecs::storage::resource::Resources::get
// F13: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs:1828:0 bevy_ecs::world::impl$2::contains_resource::closure$0
// F14: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::option::Option::and_then
// F15: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:707:0 core::option::Option::as_ref
// F16: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F17: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::slice::impl$0::get
// F18: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:508:0 alloc::raw_vec::RawVecInner::non_null
// F19: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:503:0 alloc::raw_vec::RawVecInner::ptr
// F20: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:282:0 alloc::raw_vec::RawVec::ptr
// F21: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1671:0 alloc::vec::Vec::as_ptr
// F22: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1575:0 alloc::vec::Vec::as_slice
// F23: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 alloc::vec::impl$8::deref
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:398:0 bevy_ecs::storage::sparse_set::impl$15::get::closure$0
// F25: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1119:0 core::option::Option::map
// F26: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 alloc::vec::impl$8::deref
// F27: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:631:0 core::option::Option::is_some_and
// F28: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs:1829:0 bevy_ecs::world::World::contains_resource
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs:88:0 bevy_ecs::storage::blob_vec::BlobVec::is_empty
// F30: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs:92:0 bevy_ecs::storage::resource::ResourceData::is_present
// F31: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs:344:0 bevy_app::sub_app::SubApp::add_event
// F32: crates/systems/roof/src/lib.rs:63:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F33: crates/systems/roof/src/lib.rs:64:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F34: crates/systems/roof/src/lib.rs:68:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F35: crates/systems/roof/src/lib.rs:74:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/label.rs:141:0 bevy_ecs::schedule::set::SystemSet::intern
// F37: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/schedule/config.rs:498:0 bevy_ecs::schedule::config::impl$3::in_set
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/schedule/config.rs:500:0 bevy_ecs::schedule::config::impl$3::in_set
// F39: crates/systems/roof/src/lib.rs:90:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F40: crates/systems/roof/src/lib.rs:89:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F41: crates/systems/roof/src/lib.rs:87:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F42: crates/systems/roof/src/lib.rs:110:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F43: crates/systems/roof/src/lib.rs:121:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F44: crates/systems/roof/src/lib.rs:122:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F45: crates/systems/roof/src/lib.rs:123:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F46: crates/systems/roof/src/lib.rs:124:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F47: crates/systems/roof/src/lib.rs:125:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F48: crates/systems/roof/src/lib.rs:67:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F49: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs:306:0 bevy_app::app::App::add_systems
// F50: crates/systems/roof/src/lib.rs:65:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F51: crates/systems/roof/src/lib.rs:128:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F52: crates/systems/roof/src/lib.rs:126:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::h5c95c29a17d5b74f
// F53: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/schedule/config.rs:501:0 bevy_ecs::schedule::config::impl$3::in_set
// F54: crates/systems/roof/src/lib.rs:130:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::hc45b56d7536671e7
// F55: crates/systems/roof/src/lib.rs:131:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::hc45b56d7536671e7
// F56: crates/systems/roof/src/lib.rs:146:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::hc45b56d7536671e7
// F57: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/schedule_system.rs:59:0 _$LT$bevy_ecs..system..schedule_system..InfallibleSystemWrapper$LT$S$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hde196c088b9756e8 (.llvm.3857403289431776973)
// F58: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:3764:0 core::sync::atomic::atomic_add
// F59: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:2958:0 core::sync::atomic::AtomicU32::fetch_add
// F60: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:340:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::increment_change_tick
// F61: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:725:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/schedule_system.rs:64:0 _$LT$bevy_ecs..system..schedule_system..InfallibleSystemWrapper$LT$S$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hde196c088b9756e8 (.llvm.3857403289431776973)
// F63: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:729:0 core::option::Option::as_mut
// F64: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:727:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:734:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F66: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:630:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:913:0 bevy_ecs::system::system_param::impl$9::get_param
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:2117:0 bevy_ecs::system::system_param::impl$99::get_param
// F69: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:90:0 core::slice::index::get_noubcheck
// F70: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:256:0 core::slice::index::impl$2::get_unchecked
// F71: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:663:0 alloc::vec::impl$8::deref
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs:141:0 bevy_ecs::storage::resource::ResourceData::get_with_ticks
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:628:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
// F74: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/bool.rs:62:0 bevy_ecs::storage::resource::ResourceData::is_present
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:524:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:1026:0 bevy_ecs::system::system_param::impl$12::get_param
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F78: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:663:0 nonmax::NonMaxUsize::get
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:522:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs:350:0 bevy_ecs::storage::blob_vec::BlobVec::get_ptr
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs:326:0 bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs:145:0 bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
// F83: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:3732:0 core::sync::atomic::atomic_load
// F84: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:2700:0 core::sync::atomic::AtomicU32::load
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs:2980:0 bevy_ecs::world::World::read_change_tick
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:304:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:531:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F90: crates/systems/roof/src/lib.rs:153:0 system_roof::gpu_flush_buffers
// F91: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:166:0 core::ops::function::FnMut::call_mut
// F92: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:294:0 core::ops::function::impls::impl$3::call_mut
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:945:0 bevy_ecs::system::function_system::impl$31::run::call_inner
// F94: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:948:0 bevy_ecs::system::function_system::impl$31::run
// F95: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:735:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F96: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:976:0 core::option::Option::unwrap
// F97: crates/systems/roof/src/lib.rs:154:0 system_roof::gpu_flush_buffers
// F98: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:736:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F99: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/schedule_system.rs:66:0 _$LT$bevy_ecs..system..schedule_system..InfallibleSystemWrapper$LT$S$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hde196c088b9756e8 (.llvm.3857403289431776973)
// F100: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:270:0 core::ptr::non_null::NonNull::from_ref
// F101: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/fmt/rt.rs:104:0 core::fmt::rt::Argument::new
// F102: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/fmt/rt.rs:114:0 core::fmt::rt::Argument::new_display
// F103: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:915:0 bevy_ecs::system::system_param::impl$9::get_param::closure$0
// F104: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1023:0 core::option::Option::unwrap_or_else
// F105: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:914:0 bevy_ecs::system::system_param::impl$9::get_param
// F106: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/any.rs:813:0 core::any::type_name
// F107: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:918:0 bevy_ecs::system::system_param::impl$9::get_param::closure$0
// F108: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/fmt/mod.rs:613:0 core::fmt::Arguments::new_v1
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:1029:0 bevy_ecs::system::system_param::impl$12::get_param::closure$0
// F110: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:1028:0 bevy_ecs::system::system_param::impl$12::get_param
// F111: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:1032:0 bevy_ecs::system::system_param::impl$12::get_param::closure$0
// F112: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:933:0 core::option::Option::as_mut
// F113: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:978:0 core::option::Option::unwrap
// F114: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1540:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F115: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:2022:0 core::num::impl$11::wrapping_sub
// F116: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1552:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F117: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 core::num::impl$11::wrapping_sub
// F118: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1565:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F119: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1574:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F120: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1579:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F121: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/range.rs:806:0 core::iter::range::impl$5::spec_next_back
// F122: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/range.rs:971:0 core::iter::range::impl$7::next_back
// F123: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/rev.rs:34:0 core::iter::adapters::rev::impl$1::next
// F124: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/cmp.rs:1824:0 core::cmp::impls::impl$58::lt
// F125: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1580:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F126: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1581:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F127: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1566:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F128: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1567:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F129: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1590:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F130: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1591:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F131: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/range.rs:764:0 core::iter::range::impl$5::spec_next
// F132: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/range.rs:849:0 core::iter::range::impl$6::next
// F133: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1592:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F134: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:572:0 core::num::impl$11::unchecked_add
// F135: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/range.rs:205:0 core::iter::range::impl$43::forward_unchecked
// F136: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/range.rs:767:0 core::iter::range::impl$5::spec_next
// F137: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1593:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F138: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1555:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F139: crates/systems/roof/src/lib.rs:1:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F140: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1604:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F141: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1605:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F142: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1606:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F143: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1737:0 core::str::pattern::impl$36::matching
// F144: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1609:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F145: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/str/pattern.rs:1611:0 core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// F146: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system.rs:96:0 bevy_ecs::system::system::System::run_without_applying_deferred::h5b150d1f2bbe93a2
// F147: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:784:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F148: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/schedule_system.rs:93:0 bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access
// F149: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system.rs:102:0 bevy_ecs::system::system::System::run_without_applying_deferred::h5b150d1f2bbe93a2
// F150: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs:188:0 bevy_ecs::world::World::id
// F151: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:250:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::id
// F152: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:785:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F153: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/identifier.rs:11:0 bevy_ecs::world::identifier::impl$9::eq
// F154: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2681:0 alloc::vec::Vec::len
// F155: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:843:0 bevy_ecs::archetype::Archetypes::generation
// F156: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:789:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F157: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1380:0 core::ptr::read
// F158: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/mem/mod.rs:869:0 bevy_ecs::archetype::Archetypes::generation
// F159: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1582:0 core::ptr::write
// F160: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/mem/mod.rs:870:0 bevy_ecs::archetype::Archetypes::generation
// F161: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:560:0 core::slice::index::impl$7::index
// F162: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:16:0 core::slice::index::impl$0::index
// F163: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3358:0 alloc::vec::impl$13::index
// F164: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:1024:0 bevy_ecs::archetype::impl$11::index
// F165: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:791:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F166: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/schedule_system.rs:64:0 bevy_ecs::system::schedule_system::impl$1::run_unsafe
// F167: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system.rs:106:0 bevy_ecs::system::system::System::run_without_applying_deferred::h5b150d1f2bbe93a2
// F168: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system.rs:107:0 bevy_ecs::system::system::System::run_without_applying_deferred::h5b150d1f2bbe93a2
// F169: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:561:0 core::slice::index::impl$7::index
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 2906e28a92de42173adb7e2e04380f4e3d66b441e376e33e93dd82f29e57fc11
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 271, 910, 912, 923, 924, 2259, 2261, 2264, 2275, 2279, 2281, 2282, 3430, 3438, 3466, 3470
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 记录形式: compiled_procedure; instances=7
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2259,"record_offset":2088,"rva":27363984,"va":5396073104}
// 编译实例 RVAs: 0x1a18a90, 0x1a36400, 0x1a38370, 0x1a3c870, 0x2196bd0, 0x21a7ed0, 0x21ad100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=28
//     evidence={"assembly":"\tcallq\t15468688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396073104,"raw_hex":"e89008ec00","size":5,"soft_fail":false,"target_va":5411542296,"va":5396073603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":2259,"record_offset":2364,"parent_record_offset":2088,"ancestor_record_offset":2088,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/02259.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2259,"record_offset":2492,"parent_record_offset":2088,"ancestor_record_offset":2088,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/02259.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2259,"record_offset":2272,"parent_record_offset":2088,"ancestor_record_offset":2088,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/02259.jsonl.gz"}
//   [pdb_inline_direct] core::str::pattern::impl$36::matching
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2259,"record_offset":2644,"parent_record_offset":2088,"ancestor_record_offset":2088,"inlinee":5012,"source_line":1736,"archive":"_pdb/B/modules/02259.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2259,"record_offset":2448,"parent_record_offset":2420,"ancestor_record_offset":2088,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/02259.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2259,"record_offset":2580,"parent_record_offset":2520,"ancestor_record_offset":2088,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/02259.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2259,"record_offset":2520,"parent_record_offset":2492,"ancestor_record_offset":2088,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/02259.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2259,"record_offset":2420,"parent_record_offset":2392,"ancestor_record_offset":2088,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/02259.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2259,"record_offset":2392,"parent_record_offset":2364,"ancestor_record_offset":2088,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/02259.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2259,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":2088,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/02259.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2259,"record_offset":2324,"parent_record_offset":2272,"ancestor_record_offset":2088,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/02259.jsonl.gz"}
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
// 首条位置: {"module":271,"record_offset":1472,"rva":6487216,"va":5375196336}
// 编译实例 RVAs: 0x62fcb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140029880
//     owner=unresolved_namespace; count=10
//     target_aliases=53 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6319023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375196336,"raw_hex":"e851949fff","size":5,"soft_fail":false,"target_va":5368879232,"va":5375198250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=3
//     target_aliases=19 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5580288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375196336,"raw_hex":"e800daaaff","size":5,"soft_fail":false,"target_va":5369616112,"va":5375196395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddba10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8043527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375196336,"raw_hex":"e807bc7a00","size":5,"soft_fail":false,"target_va":5383240208,"va":5375196676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddbf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8044947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375196336,"raw_hex":"e893c17a00","size":5,"soft_fail":false,"target_va":5383241616,"va":5375196664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de55d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8082735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375196336,"raw_hex":"e82f557b00","size":5,"soft_fail":false,"target_va":5383280080,"va":5375197340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8009832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375196336,"raw_hex":"e868387a00","size":5,"soft_fail":false,"target_va":5383208032,"va":5375198195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8024768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375196336,"raw_hex":"e8c0727a00","size":5,"soft_fail":false,"target_va":5383221232,"va":5375196459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8047974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375196336,"raw_hex":"e866cd7a00","size":5,"soft_fail":false,"target_va":5383244688,"va":5375196709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8052705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375196336,"raw_hex":"e8e1df7a00","size":5,"soft_fail":false,"target_va":5383249664,"va":5375196954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8056854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375196336,"raw_hex":"e816f07a00","size":5,"soft_fail":false,"target_va":5383253696,"va":5375196837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::in_set
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8060786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375196336,"raw_hex":"e872ff7a00","size":5,"soft_fail":false,"target_va":5383258032,"va":5375197241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::ScheduleConfigs$LT$T$GT$::in_set_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8053828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375196336,"raw_hex":"e844e47a00","size":5,"soft_fail":false,"target_va":5383250560,"va":5375196727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":271,"record_offset":1604,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":9230,"source_line":358,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":271,"record_offset":3856,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/schedule/config.rs
//     evidence={"module":271,"record_offset":3720,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":7908,"source_line":492,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":271,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":1472,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":271,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":1472,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":271,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":1472,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":271,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":1472,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":271,"record_offset":2008,"parent_record_offset":1976,"ancestor_record_offset":1472,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":271,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":1472,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":271,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":1472,"inlinee":9217,"source_line":339,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/label.rs
//     evidence={"module":271,"record_offset":3756,"parent_record_offset":3720,"ancestor_record_offset":1472,"inlinee":5551,"source_line":139,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":271,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":1472,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":271,"record_offset":2220,"parent_record_offset":2200,"ancestor_record_offset":1472,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":271,"record_offset":1732,"parent_record_offset":1712,"ancestor_record_offset":1472,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":271,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":1472,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":271,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":1472,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":271,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":1472,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":271,"record_offset":1656,"parent_record_offset":1628,"ancestor_record_offset":1472,"inlinee":9218,"source_line":1825,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":271,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":1472,"inlinee":9220,"source_line":1828,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":271,"record_offset":1868,"parent_record_offset":1808,"ancestor_record_offset":1472,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":271,"record_offset":1688,"parent_record_offset":1656,"ancestor_record_offset":1472,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":271,"record_offset":1888,"parent_record_offset":1868,"ancestor_record_offset":1472,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":271,"record_offset":2176,"parent_record_offset":1656,"ancestor_record_offset":1472,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":271,"record_offset":1928,"parent_record_offset":1756,"ancestor_record_offset":1472,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":271,"record_offset":1808,"parent_record_offset":1780,"ancestor_record_offset":1472,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":271,"record_offset":1840,"parent_record_offset":1808,"ancestor_record_offset":1472,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00271.jsonl.gz"}
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
// 首条位置: {"module":271,"record_offset":3888,"rva":6489584,"va":5375198704}
// 编译实例 RVAs: 0x6305f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=11
//     target_aliases=19 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5582665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375198704,"raw_hex":"e8b7d0aaff","size":5,"soft_fail":false,"target_va":5369616112,"va":5375198772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8083320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375198704,"raw_hex":"e878577b00","size":5,"soft_fail":false,"target_va":5383282048,"va":5375198723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8023975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375198704,"raw_hex":"e8a76f7a00","size":5,"soft_fail":false,"target_va":5383222816,"va":5375198836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":271,"record_offset":4024,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":9230,"source_line":358,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":271,"record_offset":4476,"parent_record_offset":4452,"ancestor_record_offset":3888,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":271,"record_offset":4524,"parent_record_offset":4500,"ancestor_record_offset":3888,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":271,"record_offset":4500,"parent_record_offset":4476,"ancestor_record_offset":3888,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":271,"record_offset":4452,"parent_record_offset":4428,"ancestor_record_offset":3888,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":271,"record_offset":4428,"parent_record_offset":4396,"ancestor_record_offset":3888,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":271,"record_offset":4396,"parent_record_offset":4372,"ancestor_record_offset":3888,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":271,"record_offset":4048,"parent_record_offset":4024,"ancestor_record_offset":3888,"inlinee":9217,"source_line":339,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":271,"record_offset":4664,"parent_record_offset":4640,"ancestor_record_offset":3888,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":271,"record_offset":4640,"parent_record_offset":4620,"ancestor_record_offset":3888,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":271,"record_offset":4152,"parent_record_offset":4132,"ancestor_record_offset":3888,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":271,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":3888,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":271,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":3888,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":271,"record_offset":4372,"parent_record_offset":4348,"ancestor_record_offset":3888,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":271,"record_offset":4076,"parent_record_offset":4048,"ancestor_record_offset":3888,"inlinee":9218,"source_line":1825,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":271,"record_offset":4132,"parent_record_offset":4108,"ancestor_record_offset":3888,"inlinee":9220,"source_line":1828,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":271,"record_offset":4288,"parent_record_offset":4228,"ancestor_record_offset":3888,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":271,"record_offset":4108,"parent_record_offset":4076,"ancestor_record_offset":3888,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":271,"record_offset":4308,"parent_record_offset":4288,"ancestor_record_offset":3888,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":271,"record_offset":4596,"parent_record_offset":4076,"ancestor_record_offset":3888,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":271,"record_offset":4348,"parent_record_offset":4176,"ancestor_record_offset":3888,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":271,"record_offset":4228,"parent_record_offset":4200,"ancestor_record_offset":3888,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":271,"record_offset":4260,"parent_record_offset":4228,"ancestor_record_offset":3888,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00271.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::gpu_flush_buffers
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xDA09
// 首条位置: {"module":924,"inlinee":55817}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":924,"record_offset":3748,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00924.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":924,"record_offset":3804,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00924.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":924,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":3712,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00924.jsonl.gz"}
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
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 26; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof::DoUiEditRoof>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >", "bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>", "bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof::DoUiEditRoof>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof::DoUiEditRoof>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_length::DoUiEditRoofRidgeLength>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_length::DoUiEditRoofRidgeLength>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_length::DoUiEditRoofRidgeLength>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 信息 ID: 935fd15b7fefbfb868c4a7b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["core::str::pattern::MatchOnly"]; 字段布局 = unknown。
// [类型关系] core::str::pattern::TwoWaySearcher::next_back -> ["enum2$<core::str::pattern::MatchOnly>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1a18a47 -> 0x1a18a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1a363b7 -> 0x1a36400 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1a38327 -> 0x1a38370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1a3c827 -> 0x1a3c870 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x2196b87 -> 0x2196bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x21a7e87 -> 0x21a7ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x21ad0b7 -> 0x21ad100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::gpu_flush_buffers
// 信息 ID: 04fdd585c080556fcc597663
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5d58aab8b83fbd13d0042c71: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >; module=924, inline_record=3712; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >; module=2264, inline_record=199340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::impl$0::add_roof_events_and_resources
// 信息 ID: 5f6fcf57fdfefc781c825255
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x5a0147 -> 0x6305f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::impl$0::add_roof_systems
// 信息 ID: 1082eaa60791b95a0d289ed2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x59f944 -> 0x62fcb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
