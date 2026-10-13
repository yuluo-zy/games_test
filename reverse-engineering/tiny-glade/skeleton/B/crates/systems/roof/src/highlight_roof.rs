// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: highlight_roof.rs.assembly.json.gz

// FUNCTION system_roof::highlight_roof::highlight_roof
// Candidate interfaces: 129a18334caf95e5c2263778
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 2264:204028 -> host 2264:199500 / rva_1a284b0_len_36d; exact_symbolizer_function_frame
//   Function-frame RVA span [0x1a285ee, 0x1a28664), 31 instructions
// Inline site 2267:5292 -> host 2267:1324 / rva_1a31280_len_36e; exact_symbolizer_function_frame
//   Function-frame RVA span [0x1a3138d, 0x1a3148e), 66 instructions
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1a284b0_len_36d: RVA 0x1a284b0, 877 PDB bytes; 183 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_1a284b0_len_36d.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2264:199500: bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)> > >
// Frames: F1
// 0x141a284b0  4157                           pushq	%r15
// 0x141a284b2  4156                           pushq	%r14
// 0x141a284b4  4155                           pushq	%r13
// 0x141a284b6  4154                           pushq	%r12
// 0x141a284b8  56                             pushq	%rsi
// 0x141a284b9  57                             pushq	%rdi
// 0x141a284ba  55                             pushq	%rbp
// 0x141a284bb  53                             pushq	%rbx
// 0x141a284bc  4881ec88000000                 subq	$0x88, %rsp
// Frames: F2 -> F3 -> F4 -> F5
// 0x141a284c3  f60101                         testb	$0x1, (%rcx)
// 0x141a284c6  0f8438030000                   je	0x141a28804 <.text+0x1a27804>
// Frames: F6 -> F7 -> F8 -> F4 -> F5
// 0x141a284cc  4889ce                         movq	%rcx, %rsi
// 0x141a284cf  488b82a0050000                 movq	0x5a0(%rdx), %rax
// 0x141a284d6  4889442428                     movq	%rax, 0x28(%rsp)
// Frames: F9 -> F8 -> F4 -> F5
// 0x141a284db  48394128                       cmpq	%rax, 0x28(%rcx)
// Frames: F8 -> F4 -> F5
// 0x141a284df  0f859b010000                   jne	0x141a28680 <.text+0x1a27680>
// Frames: F10 -> F11 -> F12 -> F4 -> F5
// 0x141a284e5  488b8208010000                 movq	0x108(%rdx), %rax
// Frames: F13 -> F14 -> F12 -> F4 -> F5
// 0x141a284ec  8b8e68010000                   movl	0x168(%rsi), %ecx
// Frames: F15 -> F16 -> F12 -> F4 -> F5
// 0x141a284f2  898668010000                   movl	%eax, 0x168(%rsi)
// Frames: F17 -> F18 -> F19 -> F20 -> F21 -> F4 -> F5
// 0x141a284f8  4839c8                         cmpq	%rcx, %rax
// 0x141a284fb  0f82c9010000                   jb	0x141a286ca <.text+0x1a276ca>
// Frames: F22 -> F23 -> F24 -> F25 -> F26 -> F27
// 0x141a28501  bd01000000                     movl	$0x1, %ebp
// 0x141a28506  f0                             lock
// 0x141a28507  0fc1aab0050000                 xaddl	%ebp, 0x5b0(%rdx)
// Frames: F28 -> F29 -> F26 -> F27
// 0x141a2850e  488b4618                       movq	0x18(%rsi), %rax
// Frames: F30 -> F31 -> F32 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a28512  48398218020000                 cmpq	%rax, 0x218(%rdx)
// 0x141a28519  0f86ba010000                   jbe	0x141a286d9 <.text+0x1a276d9>
// Frames: F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a2851f  488b8a10020000                 movq	0x210(%rdx), %rcx
// Frames: F37 -> F38 -> F39 -> F32 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a28526  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x141a2852a  4885c0                         testq	%rax, %rax
// Frames: F39 -> F32 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a2852d  0f84a6010000                   je	0x141a286d9 <.text+0x1a276d9>
// Frames: F40 -> F41 -> F42 -> F43 -> F44 -> F45 -> F46 -> F47 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a28533  4c8bb2e0010000                 movq	0x1e0(%rdx), %r14
// Frames: F48 -> F46 -> F47 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a2853a  48f7d0                         notq	%rax
// Frames: F49 -> F50 -> F51 -> F46 -> F47 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a2853d  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x141a28541  48c1e005                       shlq	$0x5, %rax
// Frames: F52 -> F53 -> F54 -> F55 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a28545  49837c062000                   cmpq	$0x0, 0x20(%r14,%rax)
// Frames: F56 -> F54 -> F55 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a2854b  0f8488010000                   je	0x141a286d9 <.text+0x1a276d9>
// Frames: F57 -> F58 -> F59 -> F56 -> F54 -> F55 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a28551  4901c6                         addq	%rax, %r14
// 0x141a28554  4d8b7e10                       movq	0x10(%r14), %r15
// Frames: F60 -> F61 -> F62 -> F63 -> F64 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a28558  8b82b0050000                   movl	0x5b0(%rdx), %eax
// Frames: F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a2855e  488b4610                       movq	0x10(%rsi), %rax
// Frames: F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a28562  4c8b8218020000                 movq	0x218(%rdx), %r8
// Frames: F30 -> F31 -> F32 -> F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a28569  4939c0                         cmpq	%rax, %r8
// 0x141a2856c  0f8688010000                   jbe	0x141a286fa <.text+0x1a276fa>
// Frames: F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a28572  4c8b8a10020000                 movq	0x210(%rdx), %r9
// Frames: F37 -> F38 -> F39 -> F32 -> F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a28579  498b0cc1                       movq	(%r9,%rax,8), %rcx
// 0x141a2857d  4885c9                         testq	%rcx, %rcx
// Frames: F39 -> F32 -> F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a28580  0f8474010000                   je	0x141a286fa <.text+0x1a276fa>
// Frames: F40 -> F41 -> F42 -> F43 -> F44 -> F45 -> F46 -> F47 -> F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a28586  488b82e0010000                 movq	0x1e0(%rdx), %rax
// Frames: F48 -> F46 -> F47 -> F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a2858d  48f7d1                         notq	%rcx
// Frames: F49 -> F50 -> F51 -> F46 -> F47 -> F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a28590  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x141a28594  48c1e105                       shlq	$0x5, %rcx
// Frames: F52 -> F53 -> F54 -> F69 -> F68 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a28598  48837c082000                   cmpq	$0x0, 0x20(%rax,%rcx)
// Frames: F56 -> F54 -> F69 -> F68 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a2859e  0f8456010000                   je	0x141a286fa <.text+0x1a276fa>
// Frames: F28 -> F29 -> F26 -> F27
// 0x141a285a4  4c8b5620                       movq	0x20(%rsi), %r10
// Frames: F30 -> F31 -> F32 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a285a8  4d39d0                         cmpq	%r10, %r8
// 0x141a285ab  0f86ce010000                   jbe	0x141a2877f <.text+0x1a2777f>
// Frames: F37 -> F38 -> F39 -> F32 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a285b1  4f8b04d1                       movq	(%r9,%r10,8), %r8
// 0x141a285b5  4d85c0                         testq	%r8, %r8
// Frames: F39 -> F32 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a285b8  0f84c1010000                   je	0x141a2877f <.text+0x1a2777f>
// Frames: F70 -> F46 -> F47 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a285be  49f7d0                         notq	%r8
// Frames: F49 -> F50 -> F71 -> F46 -> F47 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a285c1  4f8d2440                       leaq	(%r8,%r8,2), %r12
// 0x141a285c5  49c1e405                       shlq	$0x5, %r12
// Frames: F52 -> F53 -> F54 -> F55 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a285c9  4a837c202000                   cmpq	$0x0, 0x20(%rax,%r12)
// Frames: F56 -> F54 -> F55 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a285cf  0f84aa010000                   je	0x141a2877f <.text+0x1a2777f>
// Frames: F72 -> F73 -> F26 -> F27
// 0x141a285d5  4801c1                         addq	%rax, %rcx
// 0x141a285d8  4c8b4110                       movq	0x10(%rcx), %r8
// 0x141a285dc  4901c4                         addq	%rax, %r12
// 0x141a285df  488d4608                       leaq	0x8(%rsi), %rax
// Frames: F57 -> F58 -> F59 -> F56 -> F54 -> F55 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a285e3  4d8b6c2410                     movq	0x10(%r12), %r13
// Frames: F60 -> F61 -> F62 -> F63 -> F64 -> F36 -> F28 -> F29 -> F26 -> F27
// 0x141a285e8  8b8ab0050000                   movl	0x5b0(%rdx), %ecx
// Frames: F74 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F26 -> F27
// 0x141a285ee  41896c245c                     movl	%ebp, 0x5c(%r12)
// Frames: F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F26 -> F27
// 0x141a285f3  41c6450800                     movb	$0x0, 0x8(%r13)
// 0x141a285f8  488d7c2458                     leaq	0x58(%rsp), %rdi
// Frames: F82 -> F83 -> F84 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F26 -> F27
// 0x141a285fd  4889f9                         movq	%rdi, %rcx
// 0x141a28600  4889c2                         movq	%rax, %rdx
// 0x141a28603  e8d8e57700                     callq	0x1421a6be0 <.text+0x21a5be0>
// Frames: F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F26 -> F27
// 0x141a28608  4889f9                         movq	%rdi, %rcx
// 0x141a2860b  e880e57700                     callq	0x1421a6b90 <.text+0x21a5b90>
// 0x141a28610  4885c0                         testq	%rax, %rax
// 0x141a28613  744f                           je	0x141a28664 <.text+0x1a27664>
// 0x141a28615  488d7c2428                     leaq	0x28(%rsp), %rdi
// 0x141a2861a  488d5c2458                     leaq	0x58(%rsp), %rbx
// 0x141a2861f  90                             nop
// Frames: F86 -> F77 -> F78 -> F79 -> F80 -> F81 -> F26 -> F27
// 0x141a28620  488b08                         movq	(%rax), %rcx
// 0x141a28623  0fb65008                       movzbl	0x8(%rax), %edx
// 0x141a28627  4883c008                       addq	$0x8, %rax
// Frames: F74 -> F75 -> F86 -> F77 -> F78 -> F79 -> F80 -> F81 -> F26 -> F27
// 0x141a2862b  41896c245c                     movl	%ebp, 0x5c(%r12)
// Frames: F86 -> F77 -> F78 -> F79 -> F80 -> F81 -> F26 -> F27
// 0x141a28630  49894d00                       movq	%rcx, (%r13)
// 0x141a28634  41885508                       movb	%dl, 0x8(%r13)
// Frames: F87 -> F77 -> F78 -> F79 -> F80 -> F81 -> F26 -> F27
// 0x141a28638  4889f9                         movq	%rdi, %rcx
// 0x141a2863b  4889c2                         movq	%rax, %rdx
// 0x141a2863e  e8edc810ff                     callq	0x140b34f30 <.text+0xb33f30>
// Frames: F74 -> F75 -> F87 -> F77 -> F78 -> F79 -> F80 -> F81 -> F26 -> F27
// 0x141a28643  41896e5c                       movl	%ebp, 0x5c(%r14)
// Frames: F87 -> F77 -> F78 -> F79 -> F80 -> F81 -> F26 -> F27
// 0x141a28647  8b442430                       movl	0x30(%rsp), %eax
// 0x141a2864b  41894708                       movl	%eax, 0x8(%r15)
// 0x141a2864f  488b442428                     movq	0x28(%rsp), %rax
// 0x141a28654  498907                         movq	%rax, (%r15)
// Frames: F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F26 -> F27
// 0x141a28657  4889d9                         movq	%rbx, %rcx
// 0x141a2865a  e831e57700                     callq	0x1421a6b90 <.text+0x21a5b90>
// 0x141a2865f  4885c0                         testq	%rax, %rax
// 0x141a28662  75bc                           jne	0x141a28620 <.text+0x1a27620>
// Frames: F88 -> F26 -> F27
// 0x141a28664  89ae60010000                   movl	%ebp, 0x160(%rsi)
// Frames: F89
// 0x141a2866a  31c0                           xorl	%eax, %eax
// 0x141a2866c  4881c488000000                 addq	$0x88, %rsp
// 0x141a28673  5b                             popq	%rbx
// 0x141a28674  5d                             popq	%rbp
// 0x141a28675  5f                             popq	%rdi
// 0x141a28676  5e                             popq	%rsi
// 0x141a28677  415c                           popq	%r12
// 0x141a28679  415d                           popq	%r13
// 0x141a2867b  415e                           popq	%r14
// 0x141a2867d  415f                           popq	%r15
// 0x141a2867f  c3                             retq
// Frames: F8 -> F4 -> F5
// 0x141a28680  488d05697d4801                 leaq	0x1487d69(%rip), %rax   # 0x142eb03f0
// 0x141a28687  4889442458                     movq	%rax, 0x58(%rsp)
// 0x141a2868c  48c744246001000000             movq	$0x1, 0x60(%rsp)
// 0x141a28695  48c744246808000000             movq	$0x8, 0x68(%rsp)
// 0x141a2869e  0f57c0                         xorps	%xmm0, %xmm0
// 0x141a286a1  0f11442470                     movups	%xmm0, 0x70(%rsp)
// 0x141a286a6  4883c628                       addq	$0x28, %rsi
// 0x141a286aa  488d054f7d4801                 leaq	0x1487d4f(%rip), %rax   # 0x142eb0400
// 0x141a286b1  4889442420                     movq	%rax, 0x20(%rsp)
// 0x141a286b6  4c8d442428                     leaq	0x28(%rsp), %r8
// 0x141a286bb  4c8d4c2458                     leaq	0x58(%rsp), %r9
// 0x141a286c0  31c9                           xorl	%ecx, %ecx
// 0x141a286c2  4889f2                         movq	%rsi, %rdx
// 0x141a286c5  e8175aea00                     callq	0x1428ce0e1 <.text+0x28cd0e1>
// Frames: F90 -> F18 -> F19 -> F20 -> F21 -> F4 -> F5
// 0x141a286ca  4c8d05477d4801                 leaq	0x1487d47(%rip), %r8    # 0x142eb0418
// 0x141a286d1  4889c2                         movq	%rax, %rdx
// 0x141a286d4  e85711eb00                     callq	0x1428d9830 <.text+0x28d8830>
// Frames: F91 -> F92 -> F93 -> F94 -> F95 -> F96 -> F28 -> F29 -> F26 -> F27
// 0x141a286d9  4881c6c8000000                 addq	$0xc8, %rsi
// Frames: F97 -> F98 -> F95 -> F96 -> F28 -> F29 -> F26 -> F27
// 0x141a286e0  488d05aa9e4801                 leaq	0x1489eaa(%rip), %rax   # 0x142eb2591
// 0x141a286e7  4889442448                     movq	%rax, 0x48(%rsp)
// 0x141a286ec  48c74424503c000000             movq	$0x3c, 0x50(%rsp)
// 0x141a286f5  e9a1000000                     jmp	0x141a2879b <.text+0x1a2779b>
// Frames: F91 -> F92 -> F93 -> F99 -> F95 -> F100 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a286fa  4881c6c8000000                 addq	$0xc8, %rsi
// Frames: F97 -> F101 -> F95 -> F100 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a28701  488d05c9954801                 leaq	0x14895c9(%rip), %rax   # 0x142eb1cd1
// 0x141a28708  4889442448                     movq	%rax, 0x48(%rsp)
// 0x141a2870d  48c744245060000000             movq	$0x60, 0x50(%rsp)
// Frames: F99 -> F95 -> F100 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a28716  4889742428                     movq	%rsi, 0x28(%rsp)
// 0x141a2871b  488d05ae3361fe                 leaq	-0x19ecc52(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
// 0x141a28722  4889442430                     movq	%rax, 0x30(%rsp)
// 0x141a28727  488d442448                     leaq	0x48(%rsp), %rax
// 0x141a2872c  4889442438                     movq	%rax, 0x38(%rsp)
// 0x141a28731  488d0558176efe                 leaq	-0x191e8a8(%rip), %rax  # 0x140109e90 <.text+0x108e90>
// 0x141a28738  4889442440                     movq	%rax, 0x40(%rsp)
// Frames: F102 -> F99 -> F95 -> F100 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a2873d  488d0554904801                 leaq	0x1489054(%rip), %rax   # 0x142eb1798
// 0x141a28744  4889442458                     movq	%rax, 0x58(%rsp)
// 0x141a28749  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x141a28752  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x141a2875b  488d442428                     leaq	0x28(%rsp), %rax
// 0x141a28760  4889442468                     movq	%rax, 0x68(%rsp)
// 0x141a28765  48c744247002000000             movq	$0x2, 0x70(%rsp)
// Frames: F99 -> F95 -> F100 -> F65 -> F66 -> F28 -> F29 -> F26 -> F27
// 0x141a2876e  488d15ab904801                 leaq	0x14890ab(%rip), %rdx   # 0x142eb1820
// 0x141a28775  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x141a2877a  e8110ceb00                     callq	0x1428d9390 <.text+0x28d8390>
// Frames: F91 -> F92 -> F93 -> F94 -> F95 -> F96 -> F28 -> F29 -> F26 -> F27
// 0x141a2877f  4881c6c8000000                 addq	$0xc8, %rsi
// Frames: F97 -> F98 -> F95 -> F96 -> F28 -> F29 -> F26 -> F27
// 0x141a28786  488d05879c4801                 leaq	0x1489c87(%rip), %rax   # 0x142eb2414
// 0x141a2878d  4889442448                     movq	%rax, 0x48(%rsp)
// 0x141a28792  48c74424503d000000             movq	$0x3d, 0x50(%rsp)
// Frames: F28 -> F29 -> F26 -> F27
// 0x141a2879b  4889742428                     movq	%rsi, 0x28(%rsp)
// 0x141a287a0  488d05293361fe                 leaq	-0x19eccd7(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
// 0x141a287a7  4889442430                     movq	%rax, 0x30(%rsp)
// 0x141a287ac  488d442448                     leaq	0x48(%rsp), %rax
// 0x141a287b1  4889442438                     movq	%rax, 0x38(%rsp)
// 0x141a287b6  488d05d3166efe                 leaq	-0x191e92d(%rip), %rax  # 0x140109e90 <.text+0x108e90>
// 0x141a287bd  4889442440                     movq	%rax, 0x40(%rsp)
// 0x141a287c2  488d05cf8f4801                 leaq	0x1488fcf(%rip), %rax   # 0x142eb1798
// 0x141a287c9  4889442458                     movq	%rax, 0x58(%rsp)
// 0x141a287ce  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x141a287d7  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x141a287e0  488d442428                     leaq	0x28(%rsp), %rax
// 0x141a287e5  4889442468                     movq	%rax, 0x68(%rsp)
// 0x141a287ea  48c744247002000000             movq	$0x2, 0x70(%rsp)
// 0x141a287f3  488d15f6984801                 leaq	0x14898f6(%rip), %rdx   # 0x142eb20f0
// 0x141a287fa  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x141a287ff  e88c0beb00                     callq	0x1428d9390 <.text+0x28d8390>
// Frames: F103 -> F3 -> F4 -> F5
// 0x141a28804  488d0d957a4801                 leaq	0x1487a95(%rip), %rcx   # 0x142eb02a0
// 0x141a2880b  4c8d05567b4801                 leaq	0x1487b56(%rip), %r8    # 0x142eb0368
// 0x141a28812  ba59000000                     movl	$0x59, %edx
// 0x141a28817  e8140beb00                     callq	0x1428d9330 <.text+0x28d8330>
// 0x141a2881c  cc                             int3
//
// RANGE rva_1a31280_len_36e: RVA 0x1a31280, 878 PDB bytes; 191 instructions
// Evidence: _assembly/roof-v1/B/pack-000/rva_1a31280_len_36e.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2267:1324: bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)> >
// Frames: F104
// 0x141a31280  4157                           pushq	%r15
// 0x141a31282  4156                           pushq	%r14
// 0x141a31284  4155                           pushq	%r13
// 0x141a31286  4154                           pushq	%r12
// 0x141a31288  56                             pushq	%rsi
// 0x141a31289  57                             pushq	%rdi
// 0x141a3128a  55                             pushq	%rbp
// 0x141a3128b  53                             pushq	%rbx
// 0x141a3128c  4881ec98000000                 subq	$0x98, %rsp
// 0x141a31293  bb01000000                     movl	$0x1, %ebx
// Frames: F22 -> F23 -> F24 -> F25 -> F105
// 0x141a31298  f0                             lock
// 0x141a31299  0fc19ab0050000                 xaddl	%ebx, 0x5b0(%rdx)
// Frames: F2 -> F73 -> F105
// 0x141a312a0  f60101                         testb	$0x1, (%rcx)
// 0x141a312a3  0f842c030000                   je	0x141a315d5 <.text+0x1a305d5>
// Frames: F28 -> F29 -> F105
// 0x141a312a9  488b4118                       movq	0x18(%rcx), %rax
// Frames: F30 -> F31 -> F32 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F105
// 0x141a312ad  48398218020000                 cmpq	%rax, 0x218(%rdx)
// 0x141a312b4  0f86f0010000                   jbe	0x141a314aa <.text+0x1a304aa>
// Frames: F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F105
// 0x141a312ba  4c8b8210020000                 movq	0x210(%rdx), %r8
// Frames: F37 -> F38 -> F39 -> F32 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F105
// 0x141a312c1  498b04c0                       movq	(%r8,%rax,8), %rax
// 0x141a312c5  4885c0                         testq	%rax, %rax
// Frames: F39 -> F32 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F105
// 0x141a312c8  0f84dc010000                   je	0x141a314aa <.text+0x1a304aa>
// Frames: F40 -> F41 -> F42 -> F43 -> F44 -> F45 -> F46 -> F47 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F105
// 0x141a312ce  4c8b92e0010000                 movq	0x1e0(%rdx), %r10
// Frames: F48 -> F46 -> F47 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F105
// 0x141a312d5  48f7d0                         notq	%rax
// Frames: F49 -> F50 -> F51 -> F46 -> F47 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F105
// 0x141a312d8  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x141a312dc  48c1e005                       shlq	$0x5, %rax
// Frames: F52 -> F53 -> F54 -> F55 -> F36 -> F28 -> F29 -> F105
// 0x141a312e0  49837c022000                   cmpq	$0x0, 0x20(%r10,%rax)
// Frames: F56 -> F54 -> F55 -> F36 -> F28 -> F29 -> F105
// 0x141a312e6  0f84be010000                   je	0x141a314aa <.text+0x1a304aa>
// Frames: F57 -> F58 -> F59 -> F56 -> F54 -> F55 -> F36 -> F28 -> F29 -> F105
// 0x141a312ec  4901c2                         addq	%rax, %r10
// 0x141a312ef  4d8b7a10                       movq	0x10(%r10), %r15
// Frames: F60 -> F61 -> F62 -> F63 -> F64 -> F36 -> F28 -> F29 -> F105
// 0x141a312f3  8b82b0050000                   movl	0x5b0(%rdx), %eax
// Frames: F65 -> F66 -> F28 -> F29 -> F105
// 0x141a312f9  488b4110                       movq	0x10(%rcx), %rax
// Frames: F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a312fd  4c8b8218020000                 movq	0x218(%rdx), %r8
// Frames: F30 -> F31 -> F32 -> F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a31304  4939c0                         cmpq	%rax, %r8
// 0x141a31307  0f86be010000                   jbe	0x141a314cb <.text+0x1a304cb>
// Frames: F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a3130d  4c8b8a10020000                 movq	0x210(%rdx), %r9
// Frames: F37 -> F38 -> F39 -> F32 -> F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a31314  498b04c1                       movq	(%r9,%rax,8), %rax
// 0x141a31318  4885c0                         testq	%rax, %rax
// Frames: F39 -> F32 -> F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a3131b  0f84aa010000                   je	0x141a314cb <.text+0x1a304cb>
// Frames: F40 -> F41 -> F42 -> F43 -> F44 -> F45 -> F46 -> F47 -> F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a31321  4c8b9ae0010000                 movq	0x1e0(%rdx), %r11
// Frames: F48 -> F46 -> F47 -> F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a31328  48f7d0                         notq	%rax
// Frames: F49 -> F50 -> F51 -> F46 -> F47 -> F33 -> F34 -> F67 -> F68 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a3132b  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x141a3132f  48c1e005                       shlq	$0x5, %rax
// Frames: F52 -> F53 -> F54 -> F69 -> F68 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a31333  49837c032000                   cmpq	$0x0, 0x20(%r11,%rax)
// Frames: F56 -> F54 -> F69 -> F68 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a31339  0f848c010000                   je	0x141a314cb <.text+0x1a304cb>
// Frames: F28 -> F29 -> F105
// 0x141a3133f  4c89942490000000               movq	%r10, 0x90(%rsp)
// 0x141a31347  4c8b5120                       movq	0x20(%rcx), %r10
// Frames: F30 -> F31 -> F32 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F105
// 0x141a3134b  4d39d0                         cmpq	%r10, %r8
// 0x141a3134e  0f86fc010000                   jbe	0x141a31550 <.text+0x1a30550>
// Frames: F37 -> F38 -> F39 -> F32 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F105
// 0x141a31354  4f8b04d1                       movq	(%r9,%r10,8), %r8
// 0x141a31358  4d85c0                         testq	%r8, %r8
// Frames: F39 -> F32 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F105
// 0x141a3135b  0f84ef010000                   je	0x141a31550 <.text+0x1a30550>
// Frames: F70 -> F46 -> F47 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F105
// 0x141a31361  49f7d0                         notq	%r8
// Frames: F49 -> F50 -> F71 -> F46 -> F47 -> F33 -> F34 -> F35 -> F36 -> F28 -> F29 -> F105
// 0x141a31364  4f8d2440                       leaq	(%r8,%r8,2), %r12
// 0x141a31368  49c1e405                       shlq	$0x5, %r12
// Frames: F52 -> F53 -> F54 -> F55 -> F36 -> F28 -> F29 -> F105
// 0x141a3136c  4b837c232000                   cmpq	$0x0, 0x20(%r11,%r12)
// Frames: F56 -> F54 -> F55 -> F36 -> F28 -> F29 -> F105
// 0x141a31372  0f84d8010000                   je	0x141a31550 <.text+0x1a30550>
// Frames: F28 -> F29 -> F105
// 0x141a31378  4c01d8                         addq	%r11, %rax
// 0x141a3137b  488b4010                       movq	0x10(%rax), %rax
// 0x141a3137f  4d01dc                         addq	%r11, %r12
// Frames: F57 -> F58 -> F59 -> F56 -> F54 -> F55 -> F36 -> F28 -> F29 -> F105
// 0x141a31382  4d8b6c2410                     movq	0x10(%r12), %r13
// Frames: F60 -> F61 -> F62 -> F63 -> F64 -> F36 -> F28 -> F29 -> F105
// 0x141a31387  8b92b0050000                   movl	0x5b0(%rdx), %edx
// Frames: F74 -> F75 -> F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a3138d  41895c245c                     movl	%ebx, 0x5c(%r12)
// Frames: F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a31392  41c6450800                     movb	$0x0, 0x8(%r13)
// Frames: F106 -> F82 -> F83 -> F84 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a31397  4c8b4108                       movq	0x8(%rcx), %r8
// 0x141a3139b  31f6                           xorl	%esi, %esi
// Frames: F107 -> F108 -> F82 -> F83 -> F84 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a3139d  4d89c1                         movq	%r8, %r9
// 0x141a313a0  4c2b4818                       subq	0x18(%rax), %r9
// 0x141a313a4  4c0f42ce                       cmovbq	%rsi, %r9
// Frames: F107 -> F109 -> F82 -> F83 -> F84 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a313a8  4c2b4038                       subq	0x38(%rax), %r8
// 0x141a313ac  4c0f42c6                       cmovbq	%rsi, %r8
// Frames: F110 -> F111 -> F112 -> F31 -> F113 -> F82 -> F83 -> F84 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a313b0  4b8d1449                       leaq	(%r9,%r9,2), %rdx
// 0x141a313b4  48c1e203                       shlq	$0x3, %rdx
// 0x141a313b8  48035008                       addq	0x8(%rax), %rdx
// Frames: F44 -> F45 -> F113 -> F82 -> F83 -> F84 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a313bc  4c8b5010                       movq	0x10(%rax), %r10
// Frames: F110 -> F111 -> F112 -> F31 -> F113 -> F82 -> F83 -> F84 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a313c0  4d29ca                         subq	%r9, %r10
// 0x141a313c3  4c0f42d6                       cmovbq	%rsi, %r10
// 0x141a313c7  41b908000000                   movl	$0x8, %r9d
// 0x141a313cd  490f42d1                       cmovbq	%r9, %rdx
// Frames: F110 -> F111 -> F112 -> F31 -> F114 -> F82 -> F83 -> F84 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a313d1  4b8d2c40                       leaq	(%r8,%r8,2), %rbp
// 0x141a313d5  48c1e503                       shlq	$0x3, %rbp
// 0x141a313d9  48036828                       addq	0x28(%rax), %rbp
// Frames: F44 -> F45 -> F114 -> F82 -> F83 -> F84 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a313dd  4c8b5830                       movq	0x30(%rax), %r11
// Frames: F110 -> F111 -> F112 -> F31 -> F114 -> F82 -> F83 -> F84 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a313e1  4d29c3                         subq	%r8, %r11
// 0x141a313e4  4c0f42de                       cmovbq	%rsi, %r11
// 0x141a313e8  490f42e9                       cmovbq	%r9, %rbp
// Frames: F115 -> F82 -> F83 -> F84 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a313ec  488b4040                       movq	0x40(%rax), %rax
// 0x141a313f0  4c29d8                         subq	%r11, %rax
// 0x141a313f3  4c29d0                         subq	%r10, %rax
// 0x141a313f6  4889cf                         movq	%rcx, %rdi
// 0x141a313f9  48894108                       movq	%rax, 0x8(%rcx)
// Frames: F116 -> F117 -> F118 -> F119 -> F82 -> F83 -> F84 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a313fd  4b8d0452                       leaq	(%r10,%r10,2), %rax
// 0x141a31401  488d34c2                       leaq	(%rdx,%rax,8), %rsi
// 0x141a31405  4b8d045b                       leaq	(%r11,%r11,2), %rax
// 0x141a31409  488d04c500000000               leaq	(,%rax,8), %rax
// 0x141a31411  4801e8                         addq	%rbp, %rax
// 0x141a31414  4889842488000000               movq	%rax, 0x88(%rsp)
// 0x141a3141c  eb4b                           jmp	0x141a31469 <.text+0x1a30469>
// 0x141a3141e  6690                           nop
// 0x141a31420  4c8d7218                       leaq	0x18(%rdx), %r14
// Frames: F120 -> F121 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a31424  48ff4708                       incq	0x8(%rdi)
// Frames: F86 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a31428  488b4208                       movq	0x8(%rdx), %rax
// 0x141a3142c  0fb64a10                       movzbl	0x10(%rdx), %ecx
// 0x141a31430  4883c210                       addq	$0x10, %rdx
// Frames: F74 -> F75 -> F86 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a31434  41895c245c                     movl	%ebx, 0x5c(%r12)
// Frames: F86 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a31439  49894500                       movq	%rax, (%r13)
// 0x141a3143d  41884d08                       movb	%cl, 0x8(%r13)
// Frames: F87 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a31441  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x141a31446  e8e53a10ff                     callq	0x140b34f30 <.text+0xb33f30>
// 0x141a3144b  488b842490000000               movq	0x90(%rsp), %rax
// Frames: F74 -> F75 -> F87 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a31453  89585c                         movl	%ebx, 0x5c(%rax)
// Frames: F87 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a31456  8b442460                       movl	0x60(%rsp), %eax
// 0x141a3145a  41894708                       movl	%eax, 0x8(%r15)
// 0x141a3145e  488b442458                     movq	0x58(%rsp), %rax
// 0x141a31463  498907                         movq	%rax, (%r15)
// 0x141a31466  4c89f2                         movq	%r14, %rdx
// Frames: F2 -> F122 -> F123 -> F124 -> F121 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a31469  4885d2                         testq	%rdx, %rdx
// 0x141a3146c  7405                           je	0x141a31473 <.text+0x1a30473>
// 0x141a3146e  4839f2                         cmpq	%rsi, %rdx
// 0x141a31471  75ad                           jne	0x141a31420 <.text+0x1a30420>
// Frames: F2 -> F125 -> F126 -> F123 -> F124 -> F121 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a31473  4885ed                         testq	%rbp, %rbp
// 0x141a31476  7416                           je	0x141a3148e <.text+0x1a3048e>
// 0x141a31478  483bac2488000000               cmpq	0x88(%rsp), %rbp
// 0x141a31480  740c                           je	0x141a3148e <.text+0x1a3048e>
// Frames: F127 -> F128 -> F129 -> F130 -> F125 -> F126 -> F123 -> F124 -> F121 -> F85 -> F77 -> F78 -> F79 -> F80 -> F81 -> F105
// 0x141a31482  4889ea                         movq	%rbp, %rdx
// 0x141a31485  4883c518                       addq	$0x18, %rbp
// 0x141a31489  4531f6                         xorl	%r14d, %r14d
// 0x141a3148c  eb96                           jmp	0x141a31424 <.text+0x1a30424>
// Frames: F88 -> F105
// 0x141a3148e  899f60010000                   movl	%ebx, 0x160(%rdi)
// Frames: F131
// 0x141a31494  31c0                           xorl	%eax, %eax
// 0x141a31496  4881c498000000                 addq	$0x98, %rsp
// 0x141a3149d  5b                             popq	%rbx
// 0x141a3149e  5d                             popq	%rbp
// 0x141a3149f  5f                             popq	%rdi
// 0x141a314a0  5e                             popq	%rsi
// 0x141a314a1  415c                           popq	%r12
// 0x141a314a3  415d                           popq	%r13
// 0x141a314a5  415e                           popq	%r14
// 0x141a314a7  415f                           popq	%r15
// 0x141a314a9  c3                             retq
// Frames: F91 -> F92 -> F93 -> F94 -> F95 -> F96 -> F28 -> F29 -> F105
// 0x141a314aa  4881c1c8000000                 addq	$0xc8, %rcx
// Frames: F97 -> F98 -> F95 -> F96 -> F28 -> F29 -> F105
// 0x141a314b1  488d05c86d4801                 leaq	0x1486dc8(%rip), %rax   # 0x142eb8280
// 0x141a314b8  4889442428                     movq	%rax, 0x28(%rsp)
// 0x141a314bd  48c74424303c000000             movq	$0x3c, 0x30(%rsp)
// 0x141a314c6  e9a1000000                     jmp	0x141a3156c <.text+0x1a3056c>
// Frames: F91 -> F92 -> F93 -> F99 -> F95 -> F100 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a314cb  4881c1c8000000                 addq	$0xc8, %rcx
// Frames: F97 -> F101 -> F95 -> F100 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a314d2  488d05226c4801                 leaq	0x1486c22(%rip), %rax   # 0x142eb80fb
// 0x141a314d9  4889442428                     movq	%rax, 0x28(%rsp)
// 0x141a314de  48c744243060000000             movq	$0x60, 0x30(%rsp)
// Frames: F99 -> F95 -> F100 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a314e7  48894c2438                     movq	%rcx, 0x38(%rsp)
// 0x141a314ec  488d05dda560fe                 leaq	-0x19f5a23(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
// 0x141a314f3  4889442440                     movq	%rax, 0x40(%rsp)
// 0x141a314f8  488d442428                     leaq	0x28(%rsp), %rax
// 0x141a314fd  4889442448                     movq	%rax, 0x48(%rsp)
// 0x141a31502  488d0587896dfe                 leaq	-0x1927679(%rip), %rax  # 0x140109e90 <.text+0x108e90>
// 0x141a31509  4889442450                     movq	%rax, 0x50(%rsp)
// Frames: F102 -> F99 -> F95 -> F100 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a3150e  488d05736c4801                 leaq	0x1486c73(%rip), %rax   # 0x142eb8188
// 0x141a31515  4889442458                     movq	%rax, 0x58(%rsp)
// 0x141a3151a  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x141a31523  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x141a3152c  488d442438                     leaq	0x38(%rsp), %rax
// 0x141a31531  4889442468                     movq	%rax, 0x68(%rsp)
// 0x141a31536  48c744247002000000             movq	$0x2, 0x70(%rsp)
// Frames: F99 -> F95 -> F100 -> F65 -> F66 -> F28 -> F29 -> F105
// 0x141a3153f  488d15ca6c4801                 leaq	0x1486cca(%rip), %rdx   # 0x142eb8210
// 0x141a31546  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x141a3154b  e8407eea00                     callq	0x1428d9390 <.text+0x28d8390>
// Frames: F91 -> F92 -> F93 -> F94 -> F95 -> F96 -> F28 -> F29 -> F105
// 0x141a31550  4881c1c8000000                 addq	$0xc8, %rcx
// Frames: F97 -> F98 -> F95 -> F96 -> F28 -> F29 -> F105
// 0x141a31557  488d05ca6c4801                 leaq	0x1486cca(%rip), %rax   # 0x142eb8228
// 0x141a3155e  4889442428                     movq	%rax, 0x28(%rsp)
// 0x141a31563  48c74424303d000000             movq	$0x3d, 0x30(%rsp)
// Frames: F28 -> F29 -> F105
// 0x141a3156c  48894c2438                     movq	%rcx, 0x38(%rsp)
// 0x141a31571  488d0558a560fe                 leaq	-0x19f5aa8(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
// 0x141a31578  4889442440                     movq	%rax, 0x40(%rsp)
// 0x141a3157d  488d442428                     leaq	0x28(%rsp), %rax
// 0x141a31582  4889442448                     movq	%rax, 0x48(%rsp)
// 0x141a31587  488d0502896dfe                 leaq	-0x19276fe(%rip), %rax  # 0x140109e90 <.text+0x108e90>
// 0x141a3158e  4889442450                     movq	%rax, 0x50(%rsp)
// 0x141a31593  488d05ee6b4801                 leaq	0x1486bee(%rip), %rax   # 0x142eb8188
// 0x141a3159a  4889442458                     movq	%rax, 0x58(%rsp)
// 0x141a3159f  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x141a315a8  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x141a315b1  488d442438                     leaq	0x38(%rsp), %rax
// 0x141a315b6  4889442468                     movq	%rax, 0x68(%rsp)
// 0x141a315bb  48c744247002000000             movq	$0x2, 0x70(%rsp)
// 0x141a315c4  488d159d6c4801                 leaq	0x1486c9d(%rip), %rdx   # 0x142eb8268
// 0x141a315cb  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x141a315d0  e8bb7dea00                     callq	0x1428d9390 <.text+0x28d8390>
// Frames: F103 -> F73 -> F105
// 0x141a315d5  488d0dbc6d4801                 leaq	0x1486dbc(%rip), %rcx   # 0x142eb8398
// 0x141a315dc  4c8d05156e4801                 leaq	0x1486e15(%rip), %r8    # 0x142eb83f8
// 0x141a315e3  ba59000000                     movl	$0x59, %edx
// 0x141a315e8  e8437dea00                     callq	0x1428d9330 <.text+0x28d8330>
// 0x141a315ed  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system.rs:96:0 bevy_ecs::system::system::System::run_without_applying_deferred::h68e99e4c52418f82
// F2: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:729:0 core::option::Option::as_mut
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:784:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/schedule_system.rs:93:0 bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system.rs:102:0 bevy_ecs::system::system::System::run_without_applying_deferred::h68e99e4c52418f82
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs:188:0 bevy_ecs::world::World::id
// F7: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:250:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::id
// F8: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:785:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F9: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/identifier.rs:11:0 bevy_ecs::world::identifier::impl$9::eq
// F10: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2681:0 alloc::vec::Vec::len
// F11: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:843:0 bevy_ecs::archetype::Archetypes::generation
// F12: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:789:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F13: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1380:0 core::ptr::read
// F14: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/mem/mod.rs:869:0 bevy_ecs::archetype::Archetypes::generation
// F15: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1582:0 core::ptr::write
// F16: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/mem/mod.rs:870:0 bevy_ecs::archetype::Archetypes::generation
// F17: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:560:0 core::slice::index::impl$7::index
// F18: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:16:0 core::slice::index::impl$0::index
// F19: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3358:0 alloc::vec::impl$13::index
// F20: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:1024:0 bevy_ecs::archetype::impl$11::index
// F21: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:791:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F22: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:3764:0 core::sync::atomic::atomic_add
// F23: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:2958:0 core::sync::atomic::AtomicU32::fetch_add
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:340:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::increment_change_tick
// F25: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:725:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F26: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/schedule_system.rs:64:0 bevy_ecs::system::schedule_system::impl$1::run_unsafe
// F27: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system.rs:106:0 bevy_ecs::system::system::System::run_without_applying_deferred::h68e99e4c52418f82
// F28: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:2117:0 bevy_ecs::system::system_param::impl$101::get_param
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:734:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F30: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:228:0 core::slice::index::impl$2::get
// F31: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 core::slice::impl$0::get
// F32: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:60:0 bevy_ecs::storage::sparse_set::SparseArray::get
// F33: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:396:0 bevy_ecs::storage::sparse_set::SparseSet::get
// F34: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs:342:0 bevy_ecs::storage::resource::Resources::get
// F35: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:524:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:1026:0 bevy_ecs::system::system_param::impl$12::get_param
// F37: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:707:0 core::option::Option::as_ref
// F38: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F39: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::slice::impl$0::get
// F40: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:508:0 alloc::raw_vec::RawVecInner::non_null
// F41: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:503:0 alloc::raw_vec::RawVecInner::ptr
// F42: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:282:0 alloc::raw_vec::RawVec::ptr
// F43: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1671:0 alloc::vec::Vec::as_ptr
// F44: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1575:0 alloc::vec::Vec::as_slice
// F45: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 alloc::vec::impl$8::deref
// F46: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:398:0 bevy_ecs::storage::sparse_set::impl$15::get::closure$0
// F47: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1119:0 core::option::Option::map
// F48: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 alloc::vec::impl$8::deref
// F49: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:90:0 core::slice::index::get_noubcheck
// F50: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:256:0 core::slice::index::impl$2::get_unchecked
// F51: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:663:0 alloc::vec::impl$8::deref
// F52: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs:88:0 bevy_ecs::storage::blob_vec::BlobVec::is_empty
// F53: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs:92:0 bevy_ecs::storage::resource::ResourceData::is_present
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs:141:0 bevy_ecs::storage::resource::ResourceData::get_with_ticks
// F55: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:522:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F56: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/bool.rs:62:0 bevy_ecs::storage::resource::ResourceData::is_present
// F57: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs:350:0 bevy_ecs::storage::blob_vec::BlobVec::get_ptr
// F58: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs:326:0 bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
// F59: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs:145:0 bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
// F60: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:3732:0 core::sync::atomic::atomic_load
// F61: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/sync/atomic.rs:2700:0 core::sync::atomic::AtomicU32::load
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs:2980:0 bevy_ecs::world::World::read_change_tick
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:304:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
// F64: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:531:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:2117:0 bevy_ecs::system::system_param::impl$99::get_param
// F66: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs:16:0 bevy_ecs::event::reader::_::impl$0::get_param
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:630:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:913:0 bevy_ecs::system::system_param::impl$9::get_param
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs:628:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F71: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:663:0 nonmax::NonMaxUsize::get
// F72: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:730:0 core::option::Option::as_mut
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:727:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F76: crates/systems/roof/src/highlight_roof.rs:12:0 system_roof::highlight_roof::highlight_roof
// F77: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:166:0 core::ops::function::FnMut::call_mut
// F78: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:294:0 core::ops::function::impls::impl$3::call_mut
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:945:0 bevy_ecs::system::function_system::impl$33::run::call_inner
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:948:0 bevy_ecs::system::function_system::impl$33::run
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:735:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs:89:0 bevy_ecs::event::event_cursor::EventCursor::read_with_id
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs:79:0 bevy_ecs::event::event_cursor::EventCursor::read
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs:28:0 bevy_ecs::event::reader::EventReader::read
// F85: crates/systems/roof/src/highlight_roof.rs:14:0 system_roof::highlight_roof::highlight_roof
// F86: crates/systems/roof/src/highlight_roof.rs:15:0 system_roof::highlight_roof::highlight_roof
// F87: crates/systems/roof/src/highlight_roof.rs:16:0 system_roof::highlight_roof::highlight_roof
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/function_system.rs:736:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system.rs:107:0 bevy_ecs::system::system::System::run_without_applying_deferred::h68e99e4c52418f82
// F90: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:561:0 core::slice::index::impl$7::index
// F91: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:270:0 core::ptr::non_null::NonNull::from_ref
// F92: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/fmt/rt.rs:104:0 core::fmt::rt::Argument::new
// F93: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/fmt/rt.rs:114:0 core::fmt::rt::Argument::new_display
// F94: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:1029:0 bevy_ecs::system::system_param::impl$12::get_param::closure$0
// F95: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1023:0 core::option::Option::unwrap_or_else
// F96: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:1028:0 bevy_ecs::system::system_param::impl$12::get_param
// F97: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/any.rs:813:0 core::any::type_name
// F98: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:1032:0 bevy_ecs::system::system_param::impl$12::get_param::closure$0
// F99: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:915:0 bevy_ecs::system::system_param::impl$9::get_param::closure$0
// F100: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:914:0 bevy_ecs::system::system_param::impl$9::get_param
// F101: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs:918:0 bevy_ecs::system::system_param::impl$9::get_param::closure$0
// F102: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/fmt/mod.rs:613:0 core::fmt::Arguments::new_v1
// F103: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:933:0 core::option::Option::as_mut
// F104: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/schedule_system.rs:59:0 _$LT$bevy_ecs..system..schedule_system..InfallibleSystemWrapper$LT$S$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::he6d2ed5e258acb89 (.llvm.5564871786334330908)
// F105: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/schedule_system.rs:64:0 _$LT$bevy_ecs..system..schedule_system..InfallibleSystemWrapper$LT$S$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::he6d2ed5e258acb89 (.llvm.5564871786334330908)
// F106: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:55:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F107: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:1860:0 core::num::impl$11::saturating_sub
// F108: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:57:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:60:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F110: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/uint_macros.rs:659:0 core::num::impl$11::checked_sub
// F111: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:366:0 core::slice::index::impl$4::get
// F112: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:538:0 core::slice::index::impl$7::get
// F113: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:61:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F114: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:62:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F115: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:67:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F116: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 core::ptr::mut_ptr::impl$0::add
// F117: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F118: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1051:0 core::slice::impl$0::iter
// F119: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:69:0 bevy_ecs::event::iterators::EventIteratorWithId::new
// F120: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:95:0 bevy_ecs::event::iterators::impl$3::next
// F121: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:15:0 bevy_ecs::event::iterators::impl$0::next
// F122: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:333:0 core::iter::adapters::chain::and_then_or_clear
// F123: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:84:0 core::iter::adapters::chain::impl$1::next
// F124: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs:89:0 bevy_ecs::event::iterators::impl$3::next
// F125: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/iter/adapters/chain.rs:84:0 core::iter::adapters::chain::impl$1::next::closure$0
// F126: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1552:0 core::option::Option::or_else
// F127: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:617:0 core::ptr::non_null::NonNull::add
// F128: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:108:0 core::slice::iter::Iter::post_inc_start
// F129: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:443:0 core::slice::iter::impl$186::next_unchecked
// F130: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:165:0 core::slice::iter::impl$182::next
// F131: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/schedule_system.rs:66:0 _$LT$bevy_ecs..system..schedule_system..InfallibleSystemWrapper$LT$S$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::he6d2ed5e258acb89 (.llvm.5564871786334330908)
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/highlight_roof.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6a4156bb723b897f90a7a87cd40004248f8711344331f2383ba713d31db0e014
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: highlight_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 2261, 2264, 2267
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::highlight_roof::highlight_roof
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16C3D
// 首条位置: {"module":2261,"inlinee":93245}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2264,"record_offset":204068,"parent_record_offset":204028,"ancestor_record_offset":204028,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02264.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2267,"record_offset":6084,"parent_record_offset":5292,"ancestor_record_offset":5292,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":2264,"record_offset":204124,"parent_record_offset":204028,"ancestor_record_offset":204028,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/02264.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2267,"record_offset":5712,"parent_record_offset":5688,"ancestor_record_offset":5292,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2267,"record_offset":5688,"parent_record_offset":5460,"ancestor_record_offset":5292,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2264,"record_offset":204092,"parent_record_offset":204068,"ancestor_record_offset":204028,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":2264,"record_offset":204148,"parent_record_offset":204124,"ancestor_record_offset":204028,"inlinee":8036,"source_line":78,"archive":"_pdb/B/modules/02264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":2264,"record_offset":204172,"parent_record_offset":204148,"ancestor_record_offset":204028,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/02264.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2267,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":5292,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2267,"record_offset":6112,"parent_record_offset":6084,"ancestor_record_offset":5292,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2267,"record_offset":6168,"parent_record_offset":6144,"ancestor_record_offset":5292,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2267,"record_offset":6144,"parent_record_offset":6112,"ancestor_record_offset":5292,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2267,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":5292,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2267,"record_offset":5644,"parent_record_offset":5616,"ancestor_record_offset":5292,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2267,"record_offset":5504,"parent_record_offset":5460,"ancestor_record_offset":5292,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2267,"record_offset":6192,"parent_record_offset":6168,"ancestor_record_offset":5292,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2267,"record_offset":6224,"parent_record_offset":6144,"ancestor_record_offset":5292,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2267,"record_offset":5976,"parent_record_offset":5952,"ancestor_record_offset":5292,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2267,"record_offset":6372,"parent_record_offset":6348,"ancestor_record_offset":5292,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2267,"record_offset":5560,"parent_record_offset":5460,"ancestor_record_offset":5292,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2267,"record_offset":5928,"parent_record_offset":5460,"ancestor_record_offset":5292,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2267,"record_offset":5616,"parent_record_offset":5588,"ancestor_record_offset":5292,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2267,"record_offset":5588,"parent_record_offset":5560,"ancestor_record_offset":5292,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2267,"record_offset":5952,"parent_record_offset":5928,"ancestor_record_offset":5292,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2267,"record_offset":6348,"parent_record_offset":6324,"ancestor_record_offset":5292,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2267,"record_offset":6300,"parent_record_offset":6248,"ancestor_record_offset":5292,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02267.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2267,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":5292,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/02267.jsonl.gz"}
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
// 完整结构及原始类型记录: highlight_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: highlight_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: highlight_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 26; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>,bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof::DoUiEditRoof>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >", "bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>", "bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof::DoUiEditRoof>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_roof::ui_edit_roof::DoUiEditRoof>
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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_roof::highlight_roof::highlight_roof
// 信息 ID: 3807607a6a7cae73339ec9b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 129a18334caf95e5c2263778: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)> > >; module=2264, inline_record=204028; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)> >; module=2267, inline_record=5292; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
