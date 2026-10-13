// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: highlight_roof.rs.assembly.json.gz

// FUNCTION system_roof::highlight_roof::highlight_roof
// Candidate interfaces: 85732dd58efbaa0a26968afc
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 2426:424440 -> host 2426:419580 / rva_1f2d720_len_3aa; exact_symbolizer_function_frame
//   Function-frame RVA span [0x1f2d8c1, 0x1f2d983), 42 instructions
// Inline site 2430:4856 -> host 2430:1400 / rva_1f30c60_len_2fe; exact_symbolizer_function_frame
//   Function-frame RVA span [0x1f30d7c, 0x1f30e78), 65 instructions
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1f2d720_len_3aa: RVA 0x1f2d720, 938 PDB bytes; 200 instructions
// Evidence: _assembly/roof-v1/C/pack-000/rva_1f2d720_len_3aa.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2426:419580: bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)> >
// Frames: F1
// 0x141f2d720  4157                           pushq	%r15
// 0x141f2d722  4156                           pushq	%r14
// 0x141f2d724  4155                           pushq	%r13
// 0x141f2d726  4154                           pushq	%r12
// 0x141f2d728  56                             pushq	%rsi
// 0x141f2d729  57                             pushq	%rdi
// 0x141f2d72a  55                             pushq	%rbp
// 0x141f2d72b  53                             pushq	%rbx
// 0x141f2d72c  4881eca8000000                 subq	$0xa8, %rsp
// Frames: F2 -> F3 -> F4
// 0x141f2d733  833a01                         cmpl	$0x1, (%rdx)
// 0x141f2d736  0f85d1020000                   jne	0x141f2da0d <.text+0x1f2ca0d>
// Frames: F5 -> F6 -> F7 -> F4
// 0x141f2d73c  498b8098050000                 movq	0x598(%r8), %rax
// 0x141f2d743  4889442440                     movq	%rax, 0x40(%rsp)
// Frames: F8 -> F7 -> F4
// 0x141f2d748  48394228                       cmpq	%rax, 0x28(%rdx)
// Frames: F7 -> F4
// 0x141f2d74c  0f85d3020000                   jne	0x141f2da25 <.text+0x1f2ca25>
// Frames: F9 -> F10 -> F4
// 0x141f2d752  4889cb                         movq	%rcx, %rbx
// 0x141f2d755  488b4a18                       movq	0x18(%rdx), %rcx
// Frames: F11 -> F12 -> F13 -> F9 -> F10 -> F4
// 0x141f2d759  4d8b9810020000                 movq	0x210(%r8), %r11
// 0x141f2d760  488d05e14c4e01                 leaq	0x14e4ce1(%rip), %rax   # 0x143412448
// Frames: F14 -> F15 -> F16 -> F11 -> F12 -> F13 -> F9 -> F10 -> F4
// 0x141f2d767  4c39d9                         cmpq	%r11, %rcx
// 0x141f2d76a  0f832f020000                   jae	0x141f2d99f <.text+0x1f2c99f>
// Frames: F11 -> F12 -> F13 -> F9 -> F10 -> F4
// 0x141f2d770  498bb008020000                 movq	0x208(%r8), %rsi
// Frames: F17 -> F18 -> F19 -> F16 -> F11 -> F12 -> F13 -> F9 -> F10 -> F4
// 0x141f2d777  4c8b0cce                       movq	(%rsi,%rcx,8), %r9
// 0x141f2d77b  4d85c9                         testq	%r9, %r9
// 0x141f2d77e  0f841b020000                   je	0x141f2d99f <.text+0x1f2c99f>
// Frames: F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F11 -> F12 -> F13 -> F9 -> F10 -> F4
// 0x141f2d784  4d8ba0d8010000                 movq	0x1d8(%r8), %r12
// Frames: F28 -> F26 -> F27 -> F11 -> F12 -> F13 -> F9 -> F10 -> F4
// 0x141f2d78b  49f7d1                         notq	%r9
// Frames: F29 -> F30 -> F26 -> F27 -> F11 -> F12 -> F13 -> F9 -> F10 -> F4
// 0x141f2d78e  49c1e106                       shlq	$0x6, %r9
// Frames: F31 -> F32 -> F18 -> F33 -> F34 -> F9 -> F10 -> F4
// 0x141f2d792  4b837c0c2000                   cmpq	$0x0, 0x20(%r12,%r9)
// Frames: F35 -> F9 -> F10 -> F4
// 0x141f2d798  0f8401020000                   je	0x141f2d99f <.text+0x1f2c99f>
// Frames: F9 -> F10 -> F4
// 0x141f2d79e  488b7a10                       movq	0x10(%rdx), %rdi
// 0x141f2d7a2  4c8d159f684e01                 leaq	0x14e689f(%rip), %r10   # 0x143414048
// 0x141f2d7a9  4c8d35af684e01                 leaq	0x14e68af(%rip), %r14   # 0x14341405f
// 0x141f2d7b0  b90a000000                     movl	$0xa, %ecx
// Frames: F14 -> F15 -> F16 -> F11 -> F12 -> F36 -> F37 -> F9 -> F10 -> F4
// 0x141f2d7b5  4c39df                         cmpq	%r11, %rdi
// 0x141f2d7b8  0f83ec010000                   jae	0x141f2d9aa <.text+0x1f2c9aa>
// Frames: F17 -> F18 -> F19 -> F16 -> F11 -> F12 -> F36 -> F37 -> F9 -> F10 -> F4
// 0x141f2d7be  488b3cfe                       movq	(%rsi,%rdi,8), %rdi
// 0x141f2d7c2  4885ff                         testq	%rdi, %rdi
// 0x141f2d7c5  0f84df010000                   je	0x141f2d9aa <.text+0x1f2c9aa>
// Frames: F38 -> F26 -> F27 -> F11 -> F12 -> F36 -> F37 -> F9 -> F10 -> F4
// 0x141f2d7cb  48f7d7                         notq	%rdi
// Frames: F29 -> F39 -> F26 -> F27 -> F11 -> F12 -> F36 -> F37 -> F9 -> F10 -> F4
// 0x141f2d7ce  48c1e706                       shlq	$0x6, %rdi
// Frames: F31 -> F32 -> F18 -> F33 -> F40 -> F37 -> F9 -> F10 -> F4
// 0x141f2d7d2  49837c3c2000                   cmpq	$0x0, 0x20(%r12,%rdi)
// Frames: F41 -> F37 -> F9 -> F10 -> F4
// 0x141f2d7d8  0f84cc010000                   je	0x141f2d9aa <.text+0x1f2c9aa>
// Frames: F9 -> F10 -> F4
// 0x141f2d7de  488b4a20                       movq	0x20(%rdx), %rcx
// Frames: F14 -> F15 -> F16 -> F11 -> F12 -> F13 -> F9 -> F10 -> F4
// 0x141f2d7e2  4c39d9                         cmpq	%r11, %rcx
// 0x141f2d7e5  0f83b4010000                   jae	0x141f2d99f <.text+0x1f2c99f>
// Frames: F17 -> F18 -> F19 -> F16 -> F11 -> F12 -> F13 -> F9 -> F10 -> F4
// 0x141f2d7eb  488b0cce                       movq	(%rsi,%rcx,8), %rcx
// 0x141f2d7ef  4885c9                         testq	%rcx, %rcx
// 0x141f2d7f2  0f84a7010000                   je	0x141f2d99f <.text+0x1f2c99f>
// Frames: F38 -> F26 -> F27 -> F11 -> F12 -> F13 -> F9 -> F10 -> F4
// 0x141f2d7f8  48f7d1                         notq	%rcx
// Frames: F29 -> F39 -> F26 -> F27 -> F11 -> F12 -> F13 -> F9 -> F10 -> F4
// 0x141f2d7fb  48c1e106                       shlq	$0x6, %rcx
// Frames: F31 -> F32 -> F18 -> F33 -> F34 -> F9 -> F10 -> F4
// 0x141f2d7ff  49837c0c2000                   cmpq	$0x0, 0x20(%r12,%rcx)
// Frames: F35 -> F9 -> F10 -> F4
// 0x141f2d805  0f8494010000                   je	0x141f2d99f <.text+0x1f2c99f>
// Frames: F42 -> F43 -> F44 -> F45 -> F46
// 0x141f2d80b  4d01cc                         addq	%r9, %r12
// 0x141f2d80e  bd01000000                     movl	$0x1, %ebp
// 0x141f2d813  f0                             lock
// 0x141f2d814  410fc1a8a8050000               xaddl	%ebp, 0x5a8(%r8)
// Frames: F47 -> F48 -> F49 -> F50 -> F51 -> F52 -> F53 -> F54 -> F55 -> F46
// 0x141f2d81c  4d8b7c2410                     movq	0x10(%r12), %r15
// Frames: F56 -> F57 -> F58 -> F59 -> F60 -> F53 -> F54 -> F55 -> F46
// 0x141f2d821  418b80a8050000                 movl	0x5a8(%r8), %eax
// Frames: F61 -> F62 -> F54 -> F55 -> F46
// 0x141f2d828  488b4210                       movq	0x10(%rdx), %rax
// Frames: F11 -> F12 -> F63 -> F64 -> F61 -> F62 -> F54 -> F55 -> F46
// 0x141f2d82c  4d8b9810020000                 movq	0x210(%r8), %r11
// Frames: F14 -> F15 -> F16 -> F11 -> F12 -> F63 -> F64 -> F61 -> F62 -> F54 -> F55 -> F46
// 0x141f2d833  4c39d8                         cmpq	%r11, %rax
// 0x141f2d836  0f8315020000                   jae	0x141f2da51 <.text+0x1f2ca51>
// Frames: F11 -> F12 -> F63 -> F64 -> F61 -> F62 -> F54 -> F55 -> F46
// 0x141f2d83c  4d8b8808020000                 movq	0x208(%r8), %r9
// Frames: F17 -> F18 -> F19 -> F16 -> F11 -> F12 -> F63 -> F64 -> F61 -> F62 -> F54 -> F55 -> F46
// 0x141f2d843  498b04c1                       movq	(%r9,%rax,8), %rax
// 0x141f2d847  4885c0                         testq	%rax, %rax
// 0x141f2d84a  0f8401020000                   je	0x141f2da51 <.text+0x1f2ca51>
// Frames: F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F11 -> F12 -> F63 -> F64 -> F61 -> F62 -> F54 -> F55 -> F46
// 0x141f2d850  498b88d8010000                 movq	0x1d8(%r8), %rcx
// Frames: F28 -> F26 -> F27 -> F11 -> F12 -> F63 -> F64 -> F61 -> F62 -> F54 -> F55 -> F46
// 0x141f2d857  48f7d0                         notq	%rax
// Frames: F29 -> F30 -> F26 -> F27 -> F11 -> F12 -> F63 -> F64 -> F61 -> F62 -> F54 -> F55 -> F46
// 0x141f2d85a  48c1e006                       shlq	$0x6, %rax
// Frames: F31 -> F32 -> F51 -> F65 -> F64 -> F61 -> F62 -> F54 -> F55 -> F46
// 0x141f2d85e  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F66 -> F51 -> F65 -> F64 -> F61 -> F62 -> F54 -> F55 -> F46
// 0x141f2d864  0f84e7010000                   je	0x141f2da51 <.text+0x1f2ca51>
// Frames: F54 -> F55 -> F46
// 0x141f2d86a  48899c2480000000               movq	%rbx, 0x80(%rsp)
// 0x141f2d872  4c8b5220                       movq	0x20(%rdx), %r10
// Frames: F14 -> F15 -> F16 -> F11 -> F12 -> F67 -> F53 -> F54 -> F55 -> F46
// 0x141f2d876  4d39da                         cmpq	%r11, %r10
// 0x141f2d879  0f830e020000                   jae	0x141f2da8d <.text+0x1f2ca8d>
// Frames: F17 -> F18 -> F19 -> F16 -> F11 -> F12 -> F67 -> F53 -> F54 -> F55 -> F46
// 0x141f2d87f  4c89e7                         movq	%r12, %rdi
// 0x141f2d882  4f8b24d1                       movq	(%r9,%r10,8), %r12
// 0x141f2d886  4d85e4                         testq	%r12, %r12
// 0x141f2d889  0f84fe010000                   je	0x141f2da8d <.text+0x1f2ca8d>
// Frames: F38 -> F26 -> F27 -> F11 -> F12 -> F67 -> F53 -> F54 -> F55 -> F46
// 0x141f2d88f  49f7d4                         notq	%r12
// Frames: F29 -> F39 -> F26 -> F27 -> F11 -> F12 -> F67 -> F53 -> F54 -> F55 -> F46
// 0x141f2d892  49c1e406                       shlq	$0x6, %r12
// Frames: F31 -> F32 -> F51 -> F52 -> F53 -> F54 -> F55 -> F46
// 0x141f2d896  4a837c212000                   cmpq	$0x0, 0x20(%rcx,%r12)
// Frames: F66 -> F51 -> F52 -> F53 -> F54 -> F55 -> F46
// 0x141f2d89c  0f84eb010000                   je	0x141f2da8d <.text+0x1f2ca8d>
// Frames: F68 -> F69 -> F46
// 0x141f2d8a2  4801c8                         addq	%rcx, %rax
// 0x141f2d8a5  488b4010                       movq	0x10(%rax), %rax
// 0x141f2d8a9  4901cc                         addq	%rcx, %r12
// 0x141f2d8ac  4889542478                     movq	%rdx, 0x78(%rsp)
// 0x141f2d8b1  4883c208                       addq	$0x8, %rdx
// Frames: F47 -> F48 -> F49 -> F66 -> F51 -> F52 -> F53 -> F54 -> F55 -> F46
// 0x141f2d8b5  4d8b6c2410                     movq	0x10(%r12), %r13
// Frames: F56 -> F57 -> F58 -> F59 -> F60 -> F53 -> F54 -> F55 -> F46
// 0x141f2d8ba  418b88a8050000                 movl	0x5a8(%r8), %ecx
// Frames: F70 -> F71 -> F72 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d8c1  41896c243c                     movl	%ebp, 0x3c(%r12)
// Frames: F72 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d8c6  41c6450800                     movb	$0x0, 0x8(%r13)
// 0x141f2d8cb  488d4c2440                     leaq	0x40(%rsp), %rcx
// Frames: F78 -> F79 -> F80 -> F81 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d8d0  4989c0                         movq	%rax, %r8
// 0x141f2d8d3  e838408f00                     callq	0x142821910 <.text+0x2820910>
// Frames: F81 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d8d8  488b742440                     movq	0x40(%rsp), %rsi
// 0x141f2d8dd  488b542448                     movq	0x48(%rsp), %rdx
// 0x141f2d8e2  488b442450                     movq	0x50(%rsp), %rax
// 0x141f2d8e7  4889842490000000               movq	%rax, 0x90(%rsp)
// 0x141f2d8ef  488b5c2458                     movq	0x58(%rsp), %rbx
// 0x141f2d8f4  488b442460                     movq	0x60(%rsp), %rax
// 0x141f2d8f9  4889842488000000               movq	%rax, 0x88(%rsp)
// 0x141f2d901  eb56                           jmp	0x141f2d959 <.text+0x1f2c959>
// 0x141f2d903  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x141f2d910  4c8d7218                       leaq	0x18(%rdx), %r14
// Frames: F82 -> F83 -> F81 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d914  48ff06                         incq	(%rsi)
// Frames: F84 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d917  488b4208                       movq	0x8(%rdx), %rax
// 0x141f2d91b  0fb64a10                       movzbl	0x10(%rdx), %ecx
// 0x141f2d91f  4883c210                       addq	$0x10, %rdx
// Frames: F70 -> F71 -> F84 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d923  41896c243c                     movl	%ebp, 0x3c(%r12)
// Frames: F84 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d928  49894500                       movq	%rax, (%r13)
// 0x141f2d92c  41884d08                       movb	%cl, 0x8(%r13)
// Frames: F85 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d930  488d8c249c000000               leaq	0x9c(%rsp), %rcx
// 0x141f2d938  e8d35f9efe                     callq	0x140913910 <.text+0x912910>
// Frames: F70 -> F71 -> F85 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d93d  896f3c                         movl	%ebp, 0x3c(%rdi)
// Frames: F85 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d940  8b8424a4000000                 movl	0xa4(%rsp), %eax
// 0x141f2d947  41894708                       movl	%eax, 0x8(%r15)
// 0x141f2d94b  488b84249c000000               movq	0x9c(%rsp), %rax
// 0x141f2d953  498907                         movq	%rax, (%r15)
// 0x141f2d956  4c89f2                         movq	%r14, %rdx
// Frames: F2 -> F86 -> F87 -> F88 -> F83 -> F81 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d959  4885d2                         testq	%rdx, %rdx
// 0x141f2d95c  740a                           je	0x141f2d968 <.text+0x1f2c968>
// Frames: F89 -> F90 -> F18 -> F86 -> F87 -> F88 -> F83 -> F81 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d95e  483b942490000000               cmpq	0x90(%rsp), %rdx
// 0x141f2d966  75a8                           jne	0x141f2d910 <.text+0x1f2c910>
// Frames: F2 -> F91 -> F92 -> F87 -> F88 -> F83 -> F81 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d968  4885db                         testq	%rbx, %rbx
// 0x141f2d96b  7416                           je	0x141f2d983 <.text+0x1f2c983>
// 0x141f2d96d  483b9c2488000000               cmpq	0x88(%rsp), %rbx
// 0x141f2d975  740c                           je	0x141f2d983 <.text+0x1f2c983>
// Frames: F93 -> F94 -> F91 -> F92 -> F87 -> F88 -> F83 -> F81 -> F73 -> F74 -> F75 -> F76 -> F77 -> F46
// 0x141f2d977  4889da                         movq	%rbx, %rdx
// 0x141f2d97a  4883c318                       addq	$0x18, %rbx
// 0x141f2d97e  4531f6                         xorl	%r14d, %r14d
// 0x141f2d981  eb91                           jmp	0x141f2d914 <.text+0x1f2c914>
// Frames: F95 -> F46
// 0x141f2d983  488b442478                     movq	0x78(%rsp), %rax
// 0x141f2d988  896830                         movl	%ebp, 0x30(%rax)
// 0x141f2d98b  48b80200000000000080           movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
// 0x141f2d995  488b9c2480000000               movq	0x80(%rsp), %rbx
// 0x141f2d99d  eb54                           jmp	0x141f2d9f3 <.text+0x1f2c9f3>
// 0x141f2d99f  31c9                           xorl	%ecx, %ecx
// 0x141f2d9a1  41be01000000                   movl	$0x1, %r14d
// 0x141f2d9a7  4989c2                         movq	%rax, %r10
// Frames: F96 -> F97 -> F4
// 0x141f2d9aa  48b80000000000000080           movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
// 0x141f2d9b4  4889442440                     movq	%rax, 0x40(%rsp)
// 0x141f2d9b9  4c89542448                     movq	%r10, 0x48(%rsp)
// 0x141f2d9be  48c744245017000000             movq	$0x17, 0x50(%rsp)
// 0x141f2d9c7  4889442458                     movq	%rax, 0x58(%rsp)
// 0x141f2d9cc  4c89742460                     movq	%r14, 0x60(%rsp)
// 0x141f2d9d1  48894c2468                     movq	%rcx, 0x68(%rsp)
// 0x141f2d9d6  c644247000                     movb	$0x0, 0x70(%rsp)
// 0x141f2d9db  488d4c2440                     leaq	0x40(%rsp), %rcx
// 0x141f2d9e0  e87b01ed00                     callq	0x142dfdb60 <.text+0x2dfcb60>
// 0x141f2d9e5  48894308                       movq	%rax, 0x8(%rbx)
// 0x141f2d9e9  48b80100000000000080           movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
// Frames: F98
// 0x141f2d9f3  488903                         movq	%rax, (%rbx)
// 0x141f2d9f6  4889d8                         movq	%rbx, %rax
// 0x141f2d9f9  4881c4a8000000                 addq	$0xa8, %rsp
// 0x141f2da00  5b                             popq	%rbx
// 0x141f2da01  5d                             popq	%rbp
// 0x141f2da02  5f                             popq	%rdi
// 0x141f2da03  5e                             popq	%rsi
// 0x141f2da04  415c                           popq	%r12
// 0x141f2da06  415d                           popq	%r13
// 0x141f2da08  415e                           popq	%r14
// 0x141f2da0a  415f                           popq	%r15
// 0x141f2da0c  c3                             retq
// Frames: F99 -> F3 -> F4
// 0x141f2da0d  488d0d144b4e01                 leaq	0x14e4b14(%rip), %rcx   # 0x143412528
// 0x141f2da14  4c8d052d644e01                 leaq	0x14e642d(%rip), %r8    # 0x143413e48
// 0x141f2da1b  ba59000000                     movl	$0x59, %edx
// 0x141f2da20  e8bb3be800                     callq	0x142db15e0 <.text+0x2db05e0>
// Frames: F7 -> F4
// 0x141f2da25  4883c228                       addq	$0x28, %rdx
// 0x141f2da29  488d0530644e01                 leaq	0x14e6430(%rip), %rax   # 0x143413e60
// 0x141f2da30  4889442428                     movq	%rax, 0x28(%rsp)
// 0x141f2da35  48c7442420df000000             movq	$0xdf, 0x20(%rsp)
// 0x141f2da3e  4c8d0dc34b4e01                 leaq	0x14e4bc3(%rip), %r9    # 0x143412608
// 0x141f2da45  4c8d442440                     leaq	0x40(%rsp), %r8
// 0x141f2da4a  31c9                           xorl	%ecx, %ecx
// 0x141f2da4c  e83c56e700                     callq	0x142da308d <.text+0x2da208d>
// Frames: F100 -> F101 -> F102 -> F103 -> F104 -> F61 -> F62 -> F54 -> F55 -> F46
// 0x141f2da51  4883c234                       addq	$0x34, %rdx
// Frames: F102 -> F103 -> F104 -> F61 -> F62 -> F54 -> F55 -> F46
// 0x141f2da55  4889542440                     movq	%rdx, 0x40(%rsp)
// 0x141f2da5a  488d05efcffffe                 leaq	-0x1003011(%rip), %rax  # 0x140f2aa50 <.text+0xf29a50>
// 0x141f2da61  4889442448                     movq	%rax, 0x48(%rsp)
// 0x141f2da66  488d4c243f                     leaq	0x3f(%rsp), %rcx
// 0x141f2da6b  48894c2450                     movq	%rcx, 0x50(%rsp)
// 0x141f2da70  4889442458                     movq	%rax, 0x58(%rsp)
// 0x141f2da75  488d0de3494e01                 leaq	0x14e49e3(%rip), %rcx   # 0x14341245f
// 0x141f2da7c  4c8d05754a4e01                 leaq	0x14e4a75(%rip), %r8    # 0x1434124f8
// 0x141f2da83  488d542440                     leaq	0x40(%rsp), %rdx
// 0x141f2da88  e8133ee800                     callq	0x142db18a0 <.text+0x2db08a0>
// Frames: F100 -> F101 -> F105 -> F103 -> F106 -> F54 -> F55 -> F46
// 0x141f2da8d  4883c234                       addq	$0x34, %rdx
// Frames: F105 -> F103 -> F106 -> F54 -> F55 -> F46
// 0x141f2da91  4889542440                     movq	%rdx, 0x40(%rsp)
// 0x141f2da96  488d05b3cffffe                 leaq	-0x100304d(%rip), %rax  # 0x140f2aa50 <.text+0xf29a50>
// 0x141f2da9d  4889442448                     movq	%rax, 0x48(%rsp)
// 0x141f2daa2  488d4c243f                     leaq	0x3f(%rsp), %rcx
// 0x141f2daa7  48894c2450                     movq	%rcx, 0x50(%rsp)
// 0x141f2daac  4889442458                     movq	%rax, 0x58(%rsp)
// 0x141f2dab1  488d0da7494e01                 leaq	0x14e49a7(%rip), %rcx   # 0x14341245f
// 0x141f2dab8  4c8d05514a4e01                 leaq	0x14e4a51(%rip), %r8    # 0x143412510
// 0x141f2dabf  488d542440                     leaq	0x40(%rsp), %rdx
// 0x141f2dac4  e8d73de800                     callq	0x142db18a0 <.text+0x2db08a0>
// 0x141f2dac9  cc                             int3
//
// RANGE rva_1f30c60_len_2fe: RVA 0x1f30c60, 766 PDB bytes; 179 instructions
// Evidence: _assembly/roof-v1/C/pack-000/rva_1f30c60_len_2fe.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 2430:1400: bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)>
// Frames: F107
// 0x141f30c60  4157                           pushq	%r15
// 0x141f30c62  4156                           pushq	%r14
// 0x141f30c64  4155                           pushq	%r13
// 0x141f30c66  4154                           pushq	%r12
// 0x141f30c68  56                             pushq	%rsi
// 0x141f30c69  57                             pushq	%rdi
// 0x141f30c6a  55                             pushq	%rbp
// 0x141f30c6b  53                             pushq	%rbx
// 0x141f30c6c  4883ec78                       subq	$0x78, %rsp
// 0x141f30c70  bd01000000                     movl	$0x1, %ebp
// Frames: F42 -> F43 -> F44 -> F108
// 0x141f30c75  f0                             lock
// 0x141f30c76  410fc1a8a8050000               xaddl	%ebp, 0x5a8(%r8)
// Frames: F2 -> F109
// 0x141f30c7e  833a01                         cmpl	$0x1, (%rdx)
// 0x141f30c81  0f8517020000                   jne	0x141f30e9e <.text+0x1f2fe9e>
// Frames: F5 -> F6 -> F110
// 0x141f30c87  4889d3                         movq	%rdx, %rbx
// 0x141f30c8a  498b8098050000                 movq	0x598(%r8), %rax
// 0x141f30c91  4889442470                     movq	%rax, 0x70(%rsp)
// Frames: F8 -> F110
// 0x141f30c96  48394228                       cmpq	%rax, 0x28(%rdx)
// Frames: F110
// 0x141f30c9a  0f8516020000                   jne	0x141f30eb6 <.text+0x1f2feb6>
// Frames: F54 -> F111
// 0x141f30ca0  488b4318                       movq	0x18(%rbx), %rax
// Frames: F14 -> F15 -> F16 -> F11 -> F12 -> F67 -> F53 -> F54 -> F111
// 0x141f30ca4  493b8010020000                 cmpq	0x210(%r8), %rax
// 0x141f30cab  0f8334020000                   jae	0x141f30ee5 <.text+0x1f2fee5>
// Frames: F11 -> F12 -> F67 -> F53 -> F54 -> F111
// 0x141f30cb1  498b9008020000                 movq	0x208(%r8), %rdx
// Frames: F17 -> F18 -> F19 -> F16 -> F11 -> F12 -> F67 -> F53 -> F54 -> F111
// 0x141f30cb8  488b04c2                       movq	(%rdx,%rax,8), %rax
// 0x141f30cbc  4885c0                         testq	%rax, %rax
// 0x141f30cbf  0f8420020000                   je	0x141f30ee5 <.text+0x1f2fee5>
// Frames: F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F11 -> F12 -> F67 -> F53 -> F54 -> F111
// 0x141f30cc5  4d8b90d8010000                 movq	0x1d8(%r8), %r10
// Frames: F28 -> F26 -> F27 -> F11 -> F12 -> F67 -> F53 -> F54 -> F111
// 0x141f30ccc  48f7d0                         notq	%rax
// Frames: F29 -> F30 -> F26 -> F27 -> F11 -> F12 -> F67 -> F53 -> F54 -> F111
// 0x141f30ccf  48c1e006                       shlq	$0x6, %rax
// Frames: F31 -> F32 -> F51 -> F52 -> F53 -> F54 -> F111
// 0x141f30cd3  49837c022000                   cmpq	$0x0, 0x20(%r10,%rax)
// Frames: F66 -> F51 -> F52 -> F53 -> F54 -> F111
// 0x141f30cd9  0f8406020000                   je	0x141f30ee5 <.text+0x1f2fee5>
// Frames: F47 -> F48 -> F49 -> F66 -> F51 -> F52 -> F53 -> F54 -> F111
// 0x141f30cdf  4901c2                         addq	%rax, %r10
// 0x141f30ce2  4d8b7a10                       movq	0x10(%r10), %r15
// Frames: F56 -> F57 -> F58 -> F59 -> F60 -> F53 -> F54 -> F111
// 0x141f30ce6  418b80a8050000                 movl	0x5a8(%r8), %eax
// Frames: F61 -> F62 -> F54 -> F111
// 0x141f30ced  488b4310                       movq	0x10(%rbx), %rax
// Frames: F11 -> F12 -> F63 -> F64 -> F61 -> F62 -> F54 -> F111
// 0x141f30cf1  498b9010020000                 movq	0x210(%r8), %rdx
// Frames: F14 -> F15 -> F16 -> F11 -> F12 -> F63 -> F64 -> F61 -> F62 -> F54 -> F111
// 0x141f30cf8  4839d0                         cmpq	%rdx, %rax
// 0x141f30cfb  0f8320020000                   jae	0x141f30f21 <.text+0x1f2ff21>
// Frames: F11 -> F12 -> F63 -> F64 -> F61 -> F62 -> F54 -> F111
// 0x141f30d01  4d8b8808020000                 movq	0x208(%r8), %r9
// Frames: F17 -> F18 -> F19 -> F16 -> F11 -> F12 -> F63 -> F64 -> F61 -> F62 -> F54 -> F111
// 0x141f30d08  498b04c1                       movq	(%r9,%rax,8), %rax
// 0x141f30d0c  4885c0                         testq	%rax, %rax
// 0x141f30d0f  0f840c020000                   je	0x141f30f21 <.text+0x1f2ff21>
// Frames: F20 -> F21 -> F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F11 -> F12 -> F63 -> F64 -> F61 -> F62 -> F54 -> F111
// 0x141f30d15  4d8b98d8010000                 movq	0x1d8(%r8), %r11
// Frames: F28 -> F26 -> F27 -> F11 -> F12 -> F63 -> F64 -> F61 -> F62 -> F54 -> F111
// 0x141f30d1c  48f7d0                         notq	%rax
// Frames: F29 -> F30 -> F26 -> F27 -> F11 -> F12 -> F63 -> F64 -> F61 -> F62 -> F54 -> F111
// 0x141f30d1f  48c1e006                       shlq	$0x6, %rax
// Frames: F31 -> F32 -> F51 -> F65 -> F64 -> F61 -> F62 -> F54 -> F111
// 0x141f30d23  49837c032000                   cmpq	$0x0, 0x20(%r11,%rax)
// Frames: F66 -> F51 -> F65 -> F64 -> F61 -> F62 -> F54 -> F111
// 0x141f30d29  0f84f2010000                   je	0x141f30f21 <.text+0x1f2ff21>
// Frames: F54 -> F111
// 0x141f30d2f  4c89542468                     movq	%r10, 0x68(%rsp)
// 0x141f30d34  48894c2458                     movq	%rcx, 0x58(%rsp)
// 0x141f30d39  4c8b5320                       movq	0x20(%rbx), %r10
// Frames: F14 -> F15 -> F16 -> F11 -> F12 -> F67 -> F53 -> F54 -> F111
// 0x141f30d3d  4939d2                         cmpq	%rdx, %r10
// 0x141f30d40  0f839f010000                   jae	0x141f30ee5 <.text+0x1f2fee5>
// Frames: F17 -> F18 -> F19 -> F16 -> F11 -> F12 -> F67 -> F53 -> F54 -> F111
// 0x141f30d46  4f8b24d1                       movq	(%r9,%r10,8), %r12
// 0x141f30d4a  4d85e4                         testq	%r12, %r12
// 0x141f30d4d  0f8492010000                   je	0x141f30ee5 <.text+0x1f2fee5>
// Frames: F38 -> F26 -> F27 -> F11 -> F12 -> F67 -> F53 -> F54 -> F111
// 0x141f30d53  49f7d4                         notq	%r12
// Frames: F29 -> F39 -> F26 -> F27 -> F11 -> F12 -> F67 -> F53 -> F54 -> F111
// 0x141f30d56  49c1e406                       shlq	$0x6, %r12
// Frames: F31 -> F32 -> F51 -> F52 -> F53 -> F54 -> F111
// 0x141f30d5a  4b837c232000                   cmpq	$0x0, 0x20(%r11,%r12)
// Frames: F66 -> F51 -> F52 -> F53 -> F54 -> F111
// 0x141f30d60  0f847f010000                   je	0x141f30ee5 <.text+0x1f2fee5>
// Frames: F54 -> F111
// 0x141f30d66  4c01d8                         addq	%r11, %rax
// 0x141f30d69  488b4010                       movq	0x10(%rax), %rax
// 0x141f30d6d  4d01dc                         addq	%r11, %r12
// Frames: F47 -> F48 -> F49 -> F66 -> F51 -> F52 -> F53 -> F54 -> F111
// 0x141f30d70  4d8b6c2410                     movq	0x10(%r12), %r13
// Frames: F56 -> F57 -> F58 -> F59 -> F60 -> F53 -> F54 -> F111
// 0x141f30d75  418b88a8050000                 movl	0x5a8(%r8), %ecx
// Frames: F70 -> F71 -> F72 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30d7c  41896c243c                     movl	%ebp, 0x3c(%r12)
// Frames: F72 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30d81  41c6450800                     movb	$0x0, 0x8(%r13)
// Frames: F113 -> F78 -> F79 -> F80 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30d86  4c8b4308                       movq	0x8(%rbx), %r8
// 0x141f30d8a  31c9                           xorl	%ecx, %ecx
// Frames: F114 -> F115 -> F78 -> F79 -> F80 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30d8c  4d89c1                         movq	%r8, %r9
// 0x141f30d8f  4c2b4818                       subq	0x18(%rax), %r9
// 0x141f30d93  4c0f42c9                       cmovbq	%rcx, %r9
// Frames: F114 -> F116 -> F78 -> F79 -> F80 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30d97  4c2b4038                       subq	0x38(%rax), %r8
// 0x141f30d9b  4c0f42c1                       cmovbq	%rcx, %r8
// Frames: F117 -> F118 -> F119 -> F15 -> F120 -> F78 -> F79 -> F80 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30d9f  4b8d1449                       leaq	(%r9,%r9,2), %rdx
// 0x141f30da3  48c1e203                       shlq	$0x3, %rdx
// 0x141f30da7  48035008                       addq	0x8(%rax), %rdx
// Frames: F24 -> F25 -> F120 -> F78 -> F79 -> F80 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30dab  4c8b5010                       movq	0x10(%rax), %r10
// Frames: F117 -> F118 -> F119 -> F15 -> F120 -> F78 -> F79 -> F80 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30daf  4d29ca                         subq	%r9, %r10
// 0x141f30db2  4c0f42d1                       cmovbq	%rcx, %r10
// 0x141f30db6  41b908000000                   movl	$0x8, %r9d
// 0x141f30dbc  490f42d1                       cmovbq	%r9, %rdx
// Frames: F117 -> F118 -> F119 -> F15 -> F121 -> F78 -> F79 -> F80 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30dc0  4b8d3c40                       leaq	(%r8,%r8,2), %rdi
// 0x141f30dc4  48c1e703                       shlq	$0x3, %rdi
// 0x141f30dc8  48037828                       addq	0x28(%rax), %rdi
// Frames: F24 -> F25 -> F121 -> F78 -> F79 -> F80 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30dcc  4c8b5830                       movq	0x30(%rax), %r11
// Frames: F117 -> F118 -> F119 -> F15 -> F121 -> F78 -> F79 -> F80 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30dd0  4d29c3                         subq	%r8, %r11
// 0x141f30dd3  490f42f9                       cmovbq	%r9, %rdi
// 0x141f30dd7  4c0f42d9                       cmovbq	%rcx, %r11
// Frames: F122 -> F78 -> F79 -> F80 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30ddb  488b4040                       movq	0x40(%rax), %rax
// 0x141f30ddf  4c29d8                         subq	%r11, %rax
// 0x141f30de2  4c29d0                         subq	%r10, %rax
// 0x141f30de5  4889de                         movq	%rbx, %rsi
// 0x141f30de8  48894308                       movq	%rax, 0x8(%rbx)
// Frames: F123 -> F124 -> F125 -> F126 -> F78 -> F79 -> F80 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30dec  4b8d0452                       leaq	(%r10,%r10,2), %rax
// 0x141f30df0  488d1cc2                       leaq	(%rdx,%rax,8), %rbx
// 0x141f30df4  4b8d045b                       leaq	(%r11,%r11,2), %rax
// 0x141f30df8  488d04c7                       leaq	(%rdi,%rax,8), %rax
// 0x141f30dfc  4889442460                     movq	%rax, 0x60(%rsp)
// 0x141f30e01  eb53                           jmp	0x141f30e56 <.text+0x1f2fe56>
// 0x141f30e03  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x141f30e10  4c8d7218                       leaq	0x18(%rdx), %r14
// Frames: F82 -> F83 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30e14  48ff4608                       incq	0x8(%rsi)
// Frames: F84 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30e18  488b4208                       movq	0x8(%rdx), %rax
// 0x141f30e1c  0fb64a10                       movzbl	0x10(%rdx), %ecx
// 0x141f30e20  4883c210                       addq	$0x10, %rdx
// Frames: F70 -> F71 -> F84 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30e24  41896c243c                     movl	%ebp, 0x3c(%r12)
// Frames: F84 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30e29  49894500                       movq	%rax, (%r13)
// 0x141f30e2d  41884d08                       movb	%cl, 0x8(%r13)
// Frames: F85 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30e31  488d4c2438                     leaq	0x38(%rsp), %rcx
// 0x141f30e36  e8d52a9efe                     callq	0x140913910 <.text+0x912910>
// 0x141f30e3b  488b442468                     movq	0x68(%rsp), %rax
// Frames: F70 -> F71 -> F85 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30e40  89683c                         movl	%ebp, 0x3c(%rax)
// Frames: F85 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30e43  8b442440                       movl	0x40(%rsp), %eax
// 0x141f30e47  41894708                       movl	%eax, 0x8(%r15)
// 0x141f30e4b  488b442438                     movq	0x38(%rsp), %rax
// 0x141f30e50  498907                         movq	%rax, (%r15)
// 0x141f30e53  4c89f2                         movq	%r14, %rdx
// Frames: F2 -> F86 -> F87 -> F88 -> F83 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30e56  4885d2                         testq	%rdx, %rdx
// 0x141f30e59  7405                           je	0x141f30e60 <.text+0x1f2fe60>
// 0x141f30e5b  4839da                         cmpq	%rbx, %rdx
// 0x141f30e5e  75b0                           jne	0x141f30e10 <.text+0x1f2fe10>
// Frames: F2 -> F91 -> F92 -> F87 -> F88 -> F83 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30e60  4885ff                         testq	%rdi, %rdi
// 0x141f30e63  7413                           je	0x141f30e78 <.text+0x1f2fe78>
// 0x141f30e65  483b7c2460                     cmpq	0x60(%rsp), %rdi
// 0x141f30e6a  740c                           je	0x141f30e78 <.text+0x1f2fe78>
// Frames: F93 -> F94 -> F91 -> F92 -> F87 -> F88 -> F83 -> F81 -> F73 -> F74 -> F75 -> F76 -> F112
// 0x141f30e6c  4889fa                         movq	%rdi, %rdx
// 0x141f30e6f  4883c718                       addq	$0x18, %rdi
// 0x141f30e73  4531f6                         xorl	%r14d, %r14d
// 0x141f30e76  eb9c                           jmp	0x141f30e14 <.text+0x1f2fe14>
// Frames: F127
// 0x141f30e78  896e30                         movl	%ebp, 0x30(%rsi)
// 0x141f30e7b  48b90200000000000080           movabsq	$-0x7ffffffffffffffe, %rcx # imm = 0x8000000000000002
// 0x141f30e85  488b442458                     movq	0x58(%rsp), %rax
// Frames: F128 -> F129
// 0x141f30e8a  488908                         movq	%rcx, (%rax)
// Frames: F130
// 0x141f30e8d  4883c478                       addq	$0x78, %rsp
// 0x141f30e91  5b                             popq	%rbx
// 0x141f30e92  5d                             popq	%rbp
// 0x141f30e93  5f                             popq	%rdi
// 0x141f30e94  5e                             popq	%rsi
// 0x141f30e95  415c                           popq	%r12
// 0x141f30e97  415d                           popq	%r13
// 0x141f30e99  415e                           popq	%r14
// 0x141f30e9b  415f                           popq	%r15
// 0x141f30e9d  c3                             retq
// Frames: F99 -> F109
// 0x141f30e9e  488d0d0b3f4e01                 leaq	0x14e3f0b(%rip), %rcx   # 0x143414db0
// 0x141f30ea5  4c8d05643f4e01                 leaq	0x14e3f64(%rip), %r8    # 0x143414e10
// 0x141f30eac  ba59000000                     movl	$0x59, %edx
// 0x141f30eb1  e82a07e800                     callq	0x142db15e0 <.text+0x2db05e0>
// Frames: F110
// 0x141f30eb6  4883c328                       addq	$0x28, %rbx
// 0x141f30eba  488d05d73f4e01                 leaq	0x14e3fd7(%rip), %rax   # 0x143414e98
// 0x141f30ec1  4889442428                     movq	%rax, 0x28(%rsp)
// 0x141f30ec6  48c7442420df000000             movq	$0xdf, 0x20(%rsp)
// 0x141f30ecf  4c8d0d523f4e01                 leaq	0x14e3f52(%rip), %r9    # 0x143414e28
// 0x141f30ed6  4c8d442470                     leaq	0x70(%rsp), %r8
// 0x141f30edb  31c9                           xorl	%ecx, %ecx
// 0x141f30edd  4889da                         movq	%rbx, %rdx
// 0x141f30ee0  e8a821e700                     callq	0x142da308d <.text+0x2da208d>
// Frames: F54 -> F111
// 0x141f30ee5  4883c334                       addq	$0x34, %rbx
// 0x141f30ee9  48895c2438                     movq	%rbx, 0x38(%rsp)
// 0x141f30eee  488d055b9bfffe                 leaq	-0x10064a5(%rip), %rax  # 0x140f2aa50 <.text+0xf29a50>
// 0x141f30ef5  4889442440                     movq	%rax, 0x40(%rsp)
// 0x141f30efa  488d4c2437                     leaq	0x37(%rsp), %rcx
// 0x141f30eff  48894c2448                     movq	%rcx, 0x48(%rsp)
// 0x141f30f04  4889442450                     movq	%rax, 0x50(%rsp)
// 0x141f30f09  488d0d103d4e01                 leaq	0x14e3d10(%rip), %rcx   # 0x143414c20
// 0x141f30f10  4c8d05b93d4e01                 leaq	0x14e3db9(%rip), %r8    # 0x143414cd0
// 0x141f30f17  488d542438                     leaq	0x38(%rsp), %rdx
// 0x141f30f1c  e87f09e800                     callq	0x142db18a0 <.text+0x2db08a0>
// Frames: F100 -> F101 -> F102 -> F103 -> F104 -> F61 -> F62 -> F54 -> F111
// 0x141f30f21  4883c334                       addq	$0x34, %rbx
// Frames: F102 -> F103 -> F104 -> F61 -> F62 -> F54 -> F111
// 0x141f30f25  48895c2438                     movq	%rbx, 0x38(%rsp)
// 0x141f30f2a  488d051f9bfffe                 leaq	-0x10064e1(%rip), %rax  # 0x140f2aa50 <.text+0xf29a50>
// 0x141f30f31  4889442440                     movq	%rax, 0x40(%rsp)
// 0x141f30f36  488d4c2437                     leaq	0x37(%rsp), %rcx
// 0x141f30f3b  48894c2448                     movq	%rcx, 0x48(%rsp)
// 0x141f30f40  4889442450                     movq	%rax, 0x50(%rsp)
// 0x141f30f45  488d0dd43c4e01                 leaq	0x14e3cd4(%rip), %rcx   # 0x143414c20
// 0x141f30f4c  4c8d05653d4e01                 leaq	0x14e3d65(%rip), %r8    # 0x143414cb8
// 0x141f30f53  488d542438                     leaq	0x38(%rsp), %rdx
// 0x141f30f58  e84309e800                     callq	0x142db18a0 <.text+0x2db08a0>
// 0x141f30f5d  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system.rs:127:0 bevy_ecs::system::system::System::run_without_applying_deferred::he82c709224fed643
// F2: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:767:0 core::option::Option::as_mut
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:744:0 bevy_ecs::system::function_system::impl$12::validate_param_unsafe
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system.rs:135:0 bevy_ecs::system::system::System::run_without_applying_deferred::he82c709224fed643
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs:184:0 bevy_ecs::world::World::id
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:251:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::id
// F7: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:745:0 bevy_ecs::system::function_system::impl$12::validate_param_unsafe
// F8: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/identifier.rs:12:0 bevy_ecs::world::identifier::impl$10::eq
// F9: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:2154:0 bevy_ecs::system::system_param::impl$108::validate_param
// F10: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:750:0 bevy_ecs::system::function_system::impl$12::validate_param_unsafe
// F11: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:406:0 bevy_ecs::storage::sparse_set::SparseSet::get
// F12: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs:333:0 bevy_ecs::storage::resource::Resources::get
// F13: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:870:0 bevy_ecs::system::system_param::impl$8::validate_param
// F14: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:219:0 core::slice::index::impl$2::get
// F15: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 core::slice::impl$0::get
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::SparseArray::get
// F17: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:745:0 core::option::Option::as_ref
// F18: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F19: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1546:0 core::slice::impl$0::get
// F20: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:612:0 alloc::raw_vec::RawVecInner::non_null
// F21: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:607:0 alloc::raw_vec::RawVecInner::ptr
// F22: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:295:0 alloc::raw_vec::RawVec::ptr
// F23: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1916:0 alloc::vec::Vec::as_ptr
// F24: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1815:0 alloc::vec::Vec::as_slice
// F25: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3729:0 alloc::vec::impl$9::deref
// F26: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:408:0 bevy_ecs::storage::sparse_set::impl$15::get::closure$0
// F27: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:0 core::option::Option::map
// F28: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 alloc::vec::impl$9::deref
// F29: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:253:0 core::slice::index::impl$2::get_unchecked
// F30: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 alloc::vec::impl$9::deref
// F31: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs:88:0 bevy_ecs::storage::blob_vec::BlobVec::is_empty
// F32: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs:89:0 bevy_ecs::storage::resource::ResourceData::is_present
// F33: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:664:0 core::option::Option::is_some_and
// F34: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:871:0 bevy_ecs::system::system_param::impl$8::validate_param
// F35: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:868:0 bevy_ecs::system::system_param::impl$8::validate_param
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:791:0 bevy_ecs::system::system_param::impl$7::validate_param
// F37: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs:16:0 bevy_ecs::message::message_reader::_::impl$0::validate_param
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F39: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 nonmax::NonMaxUsize::get
// F40: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:792:0 bevy_ecs::system::system_param::impl$7::validate_param
// F41: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:789:0 bevy_ecs::system::system_param::impl$7::validate_param
// F42: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3923:0 core::sync::atomic::atomic_add
// F43: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3136:0 core::sync::atomic::AtomicU32::fetch_add
// F44: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:341:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::increment_change_tick
// F45: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:688:0 bevy_ecs::system::function_system::impl$12::run_unsafe
// F46: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system.rs:139:0 bevy_ecs::system::system::System::run_without_applying_deferred::he82c709224fed643
// F47: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs:350:0 bevy_ecs::storage::blob_vec::BlobVec::get_ptr
// F48: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs:326:0 bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
// F49: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs:136:0 bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
// F50: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/bool.rs:66:0 core::bool::impl$0::then
// F51: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs:132:0 bevy_ecs::storage::resource::ResourceData::get_with_ticks
// F52: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:550:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F53: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:889:0 bevy_ecs::system::system_param::impl$8::get_param
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:2171:0 bevy_ecs::system::system_param::impl$108::get_param
// F55: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:697:0 bevy_ecs::system::function_system::impl$12::run_unsafe
// F56: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3891:0 core::sync::atomic::atomic_load
// F57: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:2844:0 core::sync::atomic::AtomicU32::load
// F58: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs:2869:0 bevy_ecs::world::World::read_change_tick
// F59: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:305:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
// F60: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:556:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F61: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:2171:0 bevy_ecs::system::system_param::impl$106::get_param
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs:16:0 bevy_ecs::message::message_reader::_::impl$0::get_param
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:655:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
// F64: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:811:0 bevy_ecs::system::system_param::impl$7::get_param
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:656:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
// F66: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/bool.rs:66:0 bevy_ecs::storage::resource::ResourceData::is_present
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs:549:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F68: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:768:0 core::option::Option::as_mut
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:690:0 bevy_ecs::system::function_system::impl$12::run_unsafe
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F72: crates/systems/roof/src/highlight_roof.rs:12:0 system_roof::highlight_roof::highlight_roof
// F73: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:166:0 core::ops::function::FnMut::call_mut
// F74: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:298:0 core::ops::function::impls::impl$3::call_mut
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:926:0 bevy_ecs::system::function_system::impl$38::run::call_inner
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:929:0 bevy_ecs::system::function_system::impl$38::run
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:711:0 bevy_ecs::system::function_system::impl$12::run_unsafe
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs:93:0 bevy_ecs::message::message_cursor::MessageCursor::read_with_id
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs:80:0 bevy_ecs::message::message_cursor::MessageCursor::read
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs:28:0 bevy_ecs::message::message_reader::MessageReader::read
// F81: crates/systems/roof/src/highlight_roof.rs:14:0 system_roof::highlight_roof::highlight_roof
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:95:0 bevy_ecs::message::iterators::impl$3::next
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:15:0 bevy_ecs::message::iterators::impl$0::next
// F84: crates/systems/roof/src/highlight_roof.rs:15:0 system_roof::highlight_roof::highlight_roof
// F85: crates/systems/roof/src/highlight_roof.rs:16:0 system_roof::highlight_roof::highlight_roof
// F86: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:332:0 core::iter::adapters::chain::and_then_or_clear
// F87: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:0 core::iter::adapters::chain::impl$1::next
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:89:0 bevy_ecs::message::iterators::impl$3::next
// F89: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:1720:0 core::ptr::non_null::impl$16::eq
// F90: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:180:0 core::slice::iter::impl$171::next
// F91: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:0 core::iter::adapters::chain::impl$1::next::closure$0
// F92: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1656:0 core::option::Option::or_else
// F93: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:659:0 core::ptr::non_null::NonNull::add
// F94: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:185:0 core::slice::iter::impl$171::next
// F95: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:713:0 bevy_ecs::system::function_system::impl$12::run_unsafe
// F96: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system.rs:452:0 bevy_ecs::system::system::impl$3::from
// F97: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:2189:0 core::result::impl$28::from_residual
// F98: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system.rs:140:0 bevy_ecs::system::system::System::run_without_applying_deferred::he82c709224fed643
// F99: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:971:0 core::option::Option::as_mut
// F100: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:284:0 core::ptr::non_null::NonNull::from_ref
// F101: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/rt.rs:48:0 core::fmt::rt::Argument::new_display
// F102: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:813:0 bevy_ecs::system::system_param::impl$7::get_param::closure$0
// F103: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1067:0 core::option::Option::unwrap_or_else
// F104: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:812:0 bevy_ecs::system::system_param::impl$7::get_param
// F105: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:891:0 bevy_ecs::system::system_param::impl$8::get_param::closure$0
// F106: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:890:0 bevy_ecs::system::system_param::impl$8::get_param
// F107: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:680:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hbfb556ccc94a8229 (.llvm.3036041080924543951)
// F108: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:688:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hbfb556ccc94a8229 (.llvm.3036041080924543951)
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:690:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hbfb556ccc94a8229 (.llvm.3036041080924543951)
// F110: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:691:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hbfb556ccc94a8229 (.llvm.3036041080924543951)
// F111: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:697:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hbfb556ccc94a8229 (.llvm.3036041080924543951)
// F112: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:711:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hbfb556ccc94a8229 (.llvm.3036041080924543951)
// F113: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:55:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F114: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:2345:0 core::num::impl$11::saturating_sub
// F115: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:57:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F116: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:60:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F117: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:923:0 core::num::impl$11::checked_sub
// F118: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:368:0 core::slice::index::impl$4::get
// F119: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:546:0 core::slice::index::impl$7::get
// F120: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:61:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F121: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:62:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F122: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:67:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F123: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 core::ptr::mut_ptr::impl$0::add
// F124: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F125: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1041:0 core::slice::impl$0::iter
// F126: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:69:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F127: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:713:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hbfb556ccc94a8229 (.llvm.3036041080924543951)
// F128: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:617:0 bevy_ecs::system::function_system::impl$6::into_result
// F129: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:714:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hbfb556ccc94a8229 (.llvm.3036041080924543951)
// F130: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:715:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$Out$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hbfb556ccc94a8229 (.llvm.3036041080924543951)
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/highlight_roof.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 2ddb324a12d4687faa94ce3d0f7b9963708233518defe630221588abbbc2097b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: highlight_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2426, 2430
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::highlight_roof::highlight_roof
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19A6F
// 首条位置: {"module":2426,"inlinee":105071}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2426,"record_offset":424480,"parent_record_offset":424440,"ancestor_record_offset":424440,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2426,"record_offset":424620,"parent_record_offset":424440,"ancestor_record_offset":424440,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":2426,"record_offset":424536,"parent_record_offset":424440,"ancestor_record_offset":424440,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2430,"record_offset":5276,"parent_record_offset":5252,"ancestor_record_offset":4856,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2430,"record_offset":5252,"parent_record_offset":5024,"ancestor_record_offset":4856,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2426,"record_offset":424504,"parent_record_offset":424480,"ancestor_record_offset":424440,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2430,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":4856,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/02430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2426,"record_offset":424648,"parent_record_offset":424620,"ancestor_record_offset":424440,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":2426,"record_offset":424560,"parent_record_offset":424536,"ancestor_record_offset":424440,"inlinee":9341,"source_line":79,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":2426,"record_offset":424584,"parent_record_offset":424560,"ancestor_record_offset":424440,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":2426,"record_offset":424704,"parent_record_offset":424680,"ancestor_record_offset":424440,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":2426,"record_offset":424680,"parent_record_offset":424648,"ancestor_record_offset":424440,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":2426,"record_offset":424868,"parent_record_offset":424844,"ancestor_record_offset":424440,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2430,"record_offset":5208,"parent_record_offset":5180,"ancestor_record_offset":4856,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/02430.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2430,"record_offset":5068,"parent_record_offset":5024,"ancestor_record_offset":4856,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/02430.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2426,"record_offset":424756,"parent_record_offset":424704,"ancestor_record_offset":424440,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2426,"record_offset":424728,"parent_record_offset":424704,"ancestor_record_offset":424440,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2426,"record_offset":424844,"parent_record_offset":424680,"ancestor_record_offset":424440,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2430,"record_offset":5540,"parent_record_offset":5516,"ancestor_record_offset":4856,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2426,"record_offset":424944,"parent_record_offset":424920,"ancestor_record_offset":424440,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2426,"record_offset":424804,"parent_record_offset":424780,"ancestor_record_offset":424440,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2430,"record_offset":5124,"parent_record_offset":5024,"ancestor_record_offset":4856,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2430,"record_offset":5492,"parent_record_offset":5024,"ancestor_record_offset":4856,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/02430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2430,"record_offset":5180,"parent_record_offset":5152,"ancestor_record_offset":4856,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/02430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2430,"record_offset":5152,"parent_record_offset":5124,"ancestor_record_offset":4856,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/02430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2430,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":4856,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2426,"record_offset":424780,"parent_record_offset":424756,"ancestor_record_offset":424440,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02426.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 27; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_roof::highlight_roof::highlight_roof
// 信息 ID: 56a94e658109b1793954fe29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 85732dd58efbaa0a26968afc: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)> >; module=2426, inline_record=424440; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)>; module=2430, inline_record=4856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
