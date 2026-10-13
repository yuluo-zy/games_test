// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: generate_roof_stone_floor_and_roof_bottom.rs.assembly.json.gz

// FUNCTION system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom
// Candidate interfaces: 86c0b8990b81fe22901ff782
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3774:268 -> rva_2821520_len_315
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_2821520_len_315: RVA 0x2821520, 789 PDB bytes; 194 instructions
// Evidence: _assembly/roof-v1/C/pack-002/rva_2821520_len_315.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3774:268: system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom
// Frames: F1
// 0x142821520  4157                           pushq	%r15
// 0x142821522  4156                           pushq	%r14
// 0x142821524  4155                           pushq	%r13
// 0x142821526  4154                           pushq	%r12
// 0x142821528  56                             pushq	%rsi
// 0x142821529  57                             pushq	%rdi
// 0x14282152a  55                             pushq	%rbp
// 0x14282152b  53                             pushq	%rbx
// 0x14282152c  4881eca8000000                 subq	$0xa8, %rsp
// 0x142821533  440f29942490000000             movaps	%xmm10, 0x90(%rsp)
// 0x14282153c  440f298c2480000000             movaps	%xmm9, 0x80(%rsp)
// 0x142821545  440f29442470                   movaps	%xmm8, 0x70(%rsp)
// 0x14282154b  0f297c2460                     movaps	%xmm7, 0x60(%rsp)
// 0x142821550  0f29742450                     movaps	%xmm6, 0x50(%rsp)
// Frames: F2 -> F3 -> F4
// 0x142821555  488b39                         movq	(%rcx), %rdi
// Frames: F5 -> F3 -> F4
// 0x142821558  4c8b5108                       movq	0x8(%rcx), %r10
// Frames: F6 -> F7 -> F8 -> F3 -> F4
// 0x14282155c  4c8b1f                         movq	(%rdi), %r11
// 0x14282155f  31c0                           xorl	%eax, %eax
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F4
// 0x142821561  4d89d9                         movq	%r11, %r9
// 0x142821564  4d2b4a18                       subq	0x18(%r10), %r9
// 0x142821568  4c0f42c8                       cmovbq	%rax, %r9
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F4
// 0x14282156c  4d2b5a38                       subq	0x38(%r10), %r11
// 0x142821570  4c0f42d8                       cmovbq	%rax, %r11
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x142821574  4c89c9                         movq	%r9, %rcx
// 0x142821577  48c1e105                       shlq	$0x5, %rcx
// 0x14282157b  49034a08                       addq	0x8(%r10), %rcx
// Frames: F17 -> F18 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x14282157f  498b5a10                       movq	0x10(%r10), %rbx
// Frames: F17 -> F18 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x142821583  4d8b7230                       movq	0x30(%r10), %r14
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x142821587  4c29cb                         subq	%r9, %rbx
// 0x14282158a  480f42d8                       cmovbq	%rax, %rbx
// 0x14282158e  be08000000                     movl	$0x8, %esi
// 0x142821593  480f42ce                       cmovbq	%rsi, %rcx
// Frames: F12 -> F13 -> F14 -> F15 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x142821597  4d89d9                         movq	%r11, %r9
// 0x14282159a  49c1e105                       shlq	$0x5, %r9
// 0x14282159e  4d034a28                       addq	0x28(%r10), %r9
// 0x1428215a2  4d29de                         subq	%r11, %r14
// 0x1428215a5  4c0f42ce                       cmovbq	%rsi, %r9
// 0x1428215a9  4c0f42f0                       cmovbq	%rax, %r14
// Frames: F20 -> F7 -> F8 -> F3 -> F4
// 0x1428215ad  4d8b5240                       movq	0x40(%r10), %r10
// 0x1428215b1  4d29f2                         subq	%r14, %r10
// 0x1428215b4  4929da                         subq	%rbx, %r10
// 0x1428215b7  48897c2448                     movq	%rdi, 0x48(%rsp)
// 0x1428215bc  4c8917                         movq	%r10, (%rdi)
// Frames: F21 -> F22 -> F23 -> F24 -> F7 -> F8 -> F3 -> F4
// 0x1428215bf  48c1e305                       shlq	$0x5, %rbx
// 0x1428215c3  48895c2430                     movq	%rbx, 0x30(%rsp)
// 0x1428215c8  49c1e605                       shlq	$0x5, %r14
// 0x1428215cc  4c89742428                     movq	%r14, 0x28(%rsp)
// 0x1428215d1  498b7008                       movq	0x8(%r8), %rsi
// 0x1428215d5  4885f6                         testq	%rsi, %rsi
// 0x1428215d8  7504                           jne	0x1428215de <.text+0x28205de>
// 0x1428215da  498b7010                       movq	0x10(%r8), %rsi
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F4
// 0x1428215de  48014c2430                     addq	%rcx, 0x30(%rsp)
// 0x1428215e3  4c014c2428                     addq	%r9, 0x28(%rsp)
// 0x1428215e8  4c8b2a                         movq	(%rdx), %r13
// 0x1428215eb  488b6a08                       movq	0x8(%rdx), %rbp
// 0x1428215ef  f30f1035c9e67100               movss	0x71e6c9(%rip), %xmm6   # 0x142f3fcc0
// 0x1428215f7  f30f103d25346e00               movss	0x6e3425(%rip), %xmm7   # 0x142f04a24
// 0x1428215ff  f3440f100518737000             movss	0x707318(%rip), %xmm8   # 0x142f28920
// 0x142821608  450f57c9                       xorps	%xmm9, %xmm9
// 0x14282160c  f3440f10153f346e00             movss	0x6e343f(%rip), %xmm10  # 0x142f04a54
// 0x142821615  4885c9                         testq	%rcx, %rcx
// 0x142821618  7416                           je	0x142821630 <.text+0x2820630>
// 0x14282161a  660f1f440000                   nopw	(%rax,%rax)
// 0x142821620  483b4c2430                     cmpq	0x30(%rsp), %rcx
// 0x142821625  7409                           je	0x142821630 <.text+0x2820630>
// 0x142821627  4c8d7920                       leaq	0x20(%rcx), %r15
// 0x14282162b  4d89cc                         movq	%r9, %r12
// 0x14282162e  eb1e                           jmp	0x14282164e <.text+0x282064e>
// Frames: F25 -> F30 -> F31 -> F27 -> F28 -> F29 -> F4
// 0x142821630  4d85c9                         testq	%r9, %r9
// 0x142821633  0f8489010000                   je	0x1428217c2 <.text+0x28207c2>
// 0x142821639  4c3b4c2428                     cmpq	0x28(%rsp), %r9
// 0x14282163e  0f847e010000                   je	0x1428217c2 <.text+0x28207c2>
// Frames: F32 -> F33 -> F30 -> F31 -> F27 -> F28 -> F29 -> F4
// 0x142821644  4d8d6120                       leaq	0x20(%r9), %r12
// 0x142821648  4531ff                         xorl	%r15d, %r15d
// 0x14282164b  4c89c9                         movq	%r9, %rcx
// Frames: F34 -> F29 -> F4
// 0x14282164e  488b542448                     movq	0x48(%rsp), %rdx
// 0x142821653  48ff02                         incq	(%rdx)
// Frames: F35
// 0x142821656  4c8b7108                       movq	0x8(%rcx), %r14
// 0x14282165a  4489f2                         movl	%r14d, %edx
// 0x14282165d  f7d2                           notl	%edx
// 0x14282165f  b901000000                     movl	$0x1, %ecx
// Frames: F36 -> F15 -> F37 -> F38 -> F39 -> F40 -> F35
// 0x142821664  483b5510                       cmpq	0x10(%rbp), %rdx
// Frames: F41 -> F42 -> F38 -> F39 -> F40 -> F35
// 0x142821668  0f838c010000                   jae	0x1428217fa <.text+0x28207fa>
// Frames: F43 -> F44 -> F45 -> F42 -> F38 -> F39 -> F40 -> F35
// 0x14282166e  488d1452                       leaq	(%rdx,%rdx,2), %rdx
// 0x142821672  48c1e203                       shlq	$0x3, %rdx
// 0x142821676  48035508                       addq	0x8(%rbp), %rdx
// 0x14282167a  4c89f7                         movq	%r14, %rdi
// 0x14282167d  48c1ef20                       shrq	$0x20, %rdi
// 0x142821681  397a10                         cmpl	%edi, 0x10(%rdx)
// Frames: F46 -> F42 -> F38 -> F39 -> F40 -> F35
// 0x142821684  0f8570010000                   jne	0x1428217fa <.text+0x28207fa>
// Frames: F47 -> F48 -> F40 -> F35
// 0x14282168a  833a00                         cmpl	$0x0, (%rdx)
// 0x14282168d  0f8467010000                   je	0x1428217fa <.text+0x28207fa>
// Frames: F49 -> F50 -> F40 -> F35
// 0x142821693  488b4208                       movq	0x8(%rdx), %rax
// 0x142821697  89c1                           movl	%eax, %ecx
// Frames: F51 -> F50 -> F40 -> F35
// 0x142821699  493b4d38                       cmpq	0x38(%r13), %rcx
// Frames: F52 -> F53 -> F50 -> F40 -> F35
// 0x14282169d  0f8355010000                   jae	0x1428217f8 <.text+0x28207f8>
// Frames: F54 -> F55 -> F56 -> F52 -> F53 -> F50 -> F40 -> F35
// 0x1428216a3  4d8b4528                       movq	0x28(%r13), %r8
// Frames: F21 -> F54 -> F55 -> F56 -> F52 -> F53 -> F50 -> F40 -> F35
// 0x1428216a7  c1e906                         shrl	$0x6, %ecx
// Frames: F57 -> F55 -> F56 -> F52 -> F53 -> F50 -> F40 -> F35
// 0x1428216aa  498b0cc8                       movq	(%r8,%rcx,8), %rcx
// Frames: F55 -> F56 -> F52 -> F53 -> F50 -> F40 -> F35
// 0x1428216ae  480fa3c1                       btq	%rax, %rcx
// Frames: F58 -> F40 -> F35
// 0x1428216b2  0f8340010000                   jae	0x1428217f8 <.text+0x28207f8>
// Frames: F59 -> F60 -> F61 -> F62 -> F17 -> F18 -> F63 -> F64 -> F40 -> F35
// 0x1428216b8  8b4a04                         movl	0x4(%rdx), %ecx
// 0x1428216bb  48c1e820                       shrq	$0x20, %rax
// 0x1428216bf  488b95a0010000                 movq	0x1a0(%rbp), %rdx
// Frames: F65 -> F66 -> F63 -> F64 -> F40 -> F35
// 0x1428216c6  488d04c0                       leaq	(%rax,%rax,8), %rax
// Frames: F67 -> F40 -> F35
// 0x1428216ca  4d8b8510010000                 movq	0x110(%r13), %r8
// Frames: F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F67 -> F40 -> F35
// 0x1428216d1  4c8b4cc218                     movq	0x18(%rdx,%rax,8), %r9
// 0x1428216d6  488b44c238                     movq	0x38(%rdx,%rax,8), %rax
// Frames: F74 -> F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F67 -> F40 -> F35
// 0x1428216db  4a8b04c0                       movq	(%rax,%r8,8), %rax
// Frames: F78 -> F79 -> F80 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F67 -> F40 -> F35
// 0x1428216df  48f7d0                         notq	%rax
// Frames: F81 -> F82 -> F79 -> F80 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F67 -> F40 -> F35
// 0x1428216e2  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1428216e6  48c1e004                       shlq	$0x4, %rax
// Frames: F83 -> F84 -> F85 -> F80 -> F86 -> F71 -> F72 -> F73 -> F67 -> F40 -> F35
// 0x1428216ea  498b440110                     movq	0x10(%r9,%rax), %rax
// Frames: F87 -> F88 -> F89 -> F90 -> F91 -> F92 -> F93 -> F40 -> F35
// 0x1428216ef  f7d1                           notl	%ecx
// Frames: F21 -> F94 -> F89 -> F90 -> F91 -> F92 -> F93 -> F40 -> F35
// 0x1428216f1  486bd158                       imulq	$0x58, %rcx, %rdx
// 0x1428216f5  31c9                           xorl	%ecx, %ecx
// Frames: F95
// 0x1428216f7  807c100800                     cmpb	$0x0, 0x8(%rax,%rdx)
// 0x1428216fc  753a                           jne	0x142821738 <.text+0x2820738>
// Frames: F96
// 0x1428216fe  4801d0                         addq	%rdx, %rax
// 0x142821701  f30f104014                     movss	0x14(%rax), %xmm0
// 0x142821706  f30f59c6                       mulss	%xmm6, %xmm0
// Frames: F97 -> F98
// 0x14282170a  e861a75f00                     callq	0x142e1be70 <.text+0x2e1ae70>
// Frames: F99 -> F100
// 0x14282170f  0f28cf                         movaps	%xmm7, %xmm1
// 0x142821712  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F101 -> F100
// 0x142821716  410f28c0                       movaps	%xmm8, %xmm0
// 0x14282171a  f30f5dc1                       minss	%xmm1, %xmm0
// 0x14282171e  31c0                           xorl	%eax, %eax
// Frames: F96
// 0x142821720  410f2ec1                       ucomiss	%xmm9, %xmm0
// 0x142821724  7205                           jb	0x14282172b <.text+0x282072b>
// 0x142821726  f3480f2cc0                     cvttss2si	%xmm0, %rax
// 0x14282172b  410f2ec2                       ucomiss	%xmm10, %xmm0
// 0x14282172f  b9ffffffff                     movl	$0xffffffff, %ecx       # imm = 0xFFFFFFFF
// 0x142821734  7702                           ja	0x142821738 <.text+0x2820738>
// 0x142821736  89c1                           movl	%eax, %ecx
// Frames: F102
// 0x142821738  e8239d0dfe                     callq	0x1408fb460 <.text+0x8fa460>
// Frames: F103 -> F102
// 0x14282173d  488d4c2438                     leaq	0x38(%rsp), %rcx
// 0x142821742  89c2                           movl	%eax, %edx
// 0x142821744  e8d7630dfe                     callq	0x1408f7b20 <.text+0x8f6b20>
// Frames: F104 -> F105 -> F106 -> F107 -> F108 -> F109 -> F110 -> F111 -> F112 -> F113
// 0x142821749  488b06                         movq	(%rsi), %rax
// 0x14282174c  488b5e10                       movq	0x10(%rsi), %rbx
// Frames: F114 -> F105 -> F106 -> F107 -> F108 -> F109 -> F110 -> F111 -> F112 -> F113
// 0x142821750  4829d8                         subq	%rbx, %rax
// Frames: F105 -> F106 -> F107 -> F108 -> F109 -> F110 -> F111 -> F112 -> F113
// 0x142821753  4883f82f                       cmpq	$0x2f, %rax
// Frames: F106 -> F107 -> F108 -> F109 -> F110 -> F111 -> F112 -> F113
// 0x142821757  7647                           jbe	0x1428217a0 <.text+0x28207a0>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F4
// 0x142821759  488b4608                       movq	0x8(%rsi), %rax
// 0x14282175d  488d0d1cb379fe                 leaq	-0x1864ce4(%rip), %rcx  # 0x140fbca80 <.text+0xfbba80>
// 0x142821764  48890c18                       movq	%rcx, (%rax,%rbx)
// 0x142821768  0f10442438                     movups	0x38(%rsp), %xmm0
// 0x14282176d  0f11441808                     movups	%xmm0, 0x8(%rax,%rbx)
// 0x142821772  c644181807                     movb	$0x7, 0x18(%rax,%rbx)
// 0x142821777  c644182000                     movb	$0x0, 0x20(%rax,%rbx)
// 0x14282177c  4c89741828                     movq	%r14, 0x28(%rax,%rbx)
// 0x142821781  4883c330                       addq	$0x30, %rbx
// 0x142821785  48895e10                       movq	%rbx, 0x10(%rsi)
// 0x142821789  4c89f9                         movq	%r15, %rcx
// 0x14282178c  4d89e1                         movq	%r12, %r9
// 0x14282178f  4889f8                         movq	%rdi, %rax
// 0x142821792  4885c9                         testq	%rcx, %rcx
// 0x142821795  0f8585feffff                   jne	0x142821620 <.text+0x2820620>
// 0x14282179b  e990feffff                     jmp	0x142821630 <.text+0x2820630>
// Frames: F115 -> F107 -> F108 -> F109 -> F110 -> F111 -> F112 -> F113
// 0x1428217a0  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x1428217a9  41b830000000                   movl	$0x30, %r8d
// 0x1428217af  41b901000000                   movl	$0x1, %r9d
// 0x1428217b5  4889f1                         movq	%rsi, %rcx
// 0x1428217b8  4889da                         movq	%rbx, %rdx
// 0x1428217bb  e850e55400                     callq	0x142d6fd10 <.text+0x2d6ed10>
// 0x1428217c0  eb97                           jmp	0x142821759 <.text+0x2820759>
// Frames: F116
// 0x1428217c2  0f28742450                     movaps	0x50(%rsp), %xmm6
// 0x1428217c7  0f287c2460                     movaps	0x60(%rsp), %xmm7
// 0x1428217cc  440f28442470                   movaps	0x70(%rsp), %xmm8
// 0x1428217d2  440f288c2480000000             movaps	0x80(%rsp), %xmm9
// 0x1428217db  440f28942490000000             movaps	0x90(%rsp), %xmm10
// 0x1428217e4  4881c4a8000000                 addq	$0xa8, %rsp
// 0x1428217eb  5b                             popq	%rbx
// 0x1428217ec  5d                             popq	%rbp
// 0x1428217ed  5f                             popq	%rdi
// 0x1428217ee  5e                             popq	%rsi
// 0x1428217ef  415c                           popq	%r12
// 0x1428217f1  415d                           popq	%r13
// 0x1428217f3  415e                           popq	%r14
// 0x1428217f5  415f                           popq	%r15
// 0x1428217f7  c3                             retq
// 0x1428217f8  31c9                           xorl	%ecx, %ecx
// Frames: F117 -> F35
// 0x1428217fa  48c1e020                       shlq	$0x20, %rax
// 0x1428217fe  4809c8                         orq	%rcx, %rax
// 0x142821801  4889442438                     movq	%rax, 0x38(%rsp)
// 0x142821806  4c89742440                     movq	%r14, 0x40(%rsp)
// 0x14282180b  488d05d66ad600                 leaq	0xd66ad6(%rip), %rax    # 0x1435882e8
// 0x142821812  4889442420                     movq	%rax, 0x20(%rsp)
// 0x142821817  488d0d026bd600                 leaq	0xd66b02(%rip), %rcx    # 0x143588320
// 0x14282181e  4c8d0ddb6ad600                 leaq	0xd66adb(%rip), %r9     # 0x143588300
// 0x142821825  4c8d442438                     leaq	0x38(%rsp), %r8
// 0x14282182a  ba2b000000                     movl	$0x2b, %edx
// 0x14282182f  e80cfe5800                     callq	0x142db1640 <.text+0x2db0640>
// 0x142821834  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:10:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h9f3e5a15322dec60
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:1065:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs:28:0 bevy_ecs::message::message_reader::MessageReader::read
// F4: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:15:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h9f3e5a15322dec60
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:377:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:55:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F7: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs:93:0 bevy_ecs::message::message_cursor::MessageCursor::read_with_id
// F8: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs:80:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F9: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:2345:0 core::num::impl$11::saturating_sub
// F10: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:57:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F11: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:60:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F12: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:923:0 core::num::impl$11::checked_sub
// F13: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:368:0 core::slice::index::impl$4::get
// F14: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:546:0 core::slice::index::impl$7::get
// F15: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 core::slice::impl$0::get
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:61:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F17: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1815:0 alloc::vec::Vec::as_slice
// F18: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3729:0 alloc::vec::impl$9::deref
// F19: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:62:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F20: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:67:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F21: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 core::ptr::mut_ptr::impl$0::add
// F22: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F23: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1041:0 core::slice::impl$0::iter
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:69:0 bevy_ecs::message::iterators::MessageIteratorWithId::new
// F25: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:767:0 core::option::Option::as_mut
// F26: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:332:0 core::iter::adapters::chain::and_then_or_clear
// F27: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:0 core::iter::adapters::chain::impl$1::next
// F28: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:89:0 bevy_ecs::message::iterators::impl$3::next
// F29: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:15:0 bevy_ecs::message::iterators::impl$0::next
// F30: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:0 core::iter::adapters::chain::impl$1::next::closure$0
// F31: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1656:0 core::option::Option::or_else
// F32: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:659:0 core::ptr::non_null::NonNull::add
// F33: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:185:0 core::slice::iter::impl$171::next
// F34: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:95:0 bevy_ecs::message::iterators::impl$3::next
// F35: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:16:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h9f3e5a15322dec60
// F36: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:219:0 core::slice::index::impl$2::get
// F37: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:957:0 bevy_ecs::entity::Entities::get_id_location
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:949:0 bevy_ecs::entity::Entities::get
// F39: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1546:0 bevy_ecs::system::query::Query::get_inner
// F40: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1389:0 bevy_ecs::system::query::Query::get
// F41: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1583:0 core::option::Option::filter
// F42: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:958:0 bevy_ecs::entity::Entities::get_id_location
// F43: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:214:0 bevy_ecs::entity::impl$37::eq
// F44: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:958:0 bevy_ecs::entity::impl$18::get_id_location::closure$0
// F45: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1584:0 core::option::Option::filter
// F46: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1589:0 core::option::Option::filter
// F47: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1338:0 core::option::Option::ok_or
// F48: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1547:0 bevy_ecs::system::query::Query::get_inner
// F49: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:123:0 bevy_ecs::archetype::ArchetypeId::index
// F50: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1551:0 bevy_ecs::system::query::Query::get_inner
// F51: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:393:0 bevy_ecs::archetype::ArchetypeId::index
// F52: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/bool.rs:66:0 core::bool::impl$0::then
// F53: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 bevy_ecs::archetype::ArchetypeId::index
// F54: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F55: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F56: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F57: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/internal_macros.rs:28:0 fixedbitset::FixedBitSet::get_unchecked
// F58: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1548:0 bevy_ecs::system::query::Query::get_inner
// F59: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:612:0 alloc::raw_vec::RawVecInner::non_null
// F60: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:607:0 alloc::raw_vec::RawVecInner::ptr
// F61: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:295:0 alloc::raw_vec::RawVec::ptr
// F62: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1916:0 alloc::vec::Vec::as_ptr
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:753:0 bevy_ecs::storage::table::Tables::get
// F64: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1580:0 bevy_ecs::system::query::Query::get_inner
// F65: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:221:0 core::slice::index::impl$2::get
// F66: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 alloc::vec::impl$9::deref
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1582:0 bevy_ecs::system::query::Query::get_inner
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:406:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:490:0 bevy_ecs::storage::table::Table::get_column
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:381:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1552:0 bevy_ecs::query::fetch::impl$36::set_table
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1539:0 bevy_ecs::query::fetch::impl$36::set_archetype
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$2::set_archetype
// F74: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:745:0 core::option::Option::as_ref
// F75: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F76: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1546:0 core::option::Option::and_then
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:408:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F80: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:0 core::option::Option::map
// F81: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:253:0 core::slice::index::impl$2::get_unchecked
// F82: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 nonmax::NonMaxUsize::get
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs:298:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:382:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:382:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxU32::get
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:116:0 bevy_ecs::storage::table::TableRow::index
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1609:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F90: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2966:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F91: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1604:0 bevy_ecs::query::fetch::impl$37::fetch
// F92: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2529:0 bevy_ecs::query::fetch::impl$89::fetch
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1591:0 bevy_ecs::system::query::Query::get_inner
// F94: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs:1079:0 bevy_ecs::storage::table::TableRow::index
// F95: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:18:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h9f3e5a15322dec60
// F96: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:20:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h9f3e5a15322dec60
// F97: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/num/f32.rs:542:0 std::f32::impl$0::log2
// F98: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:21:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h9f3e5a15322dec60
// F99: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1512:0 core::f32::impl$0::clamp
// F100: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:22:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h9f3e5a15322dec60
// F101: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1515:0 core::f32::impl$0::clamp
// F102: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:28:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h9f3e5a15322dec60
// F103: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/convert/mod.rs:778:0 core::convert::impl$3::into
// F104: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:617:0 alloc::raw_vec::RawVecInner::capacity
// F105: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:765:0 alloc::raw_vec::RawVecInner::needs_to_grow
// F106: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:671:0 alloc::raw_vec::RawVecInner::reserve
// F107: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:340:0 alloc::raw_vec::RawVec::reserve
// F108: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1446:0 alloc::vec::Vec::reserve
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:0:0 bevy_ecs::system::commands::Commands::queue_internal
// F110: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:589:0 bevy_ecs::system::commands::Commands::queue
// F111: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:1893:0 bevy_ecs::system::commands::EntityCommands::queue
// F112: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:1391:0 bevy_ecs::system::commands::EntityCommands::insert
// F113: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:27:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h9f3e5a15322dec60
// F114: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:2494:0 alloc::raw_vec::RawVecInner::capacity
// F115: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:673:0 alloc::raw_vec::RawVecInner::reserve
// F116: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs:100:0 system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom::h9f3e5a15322dec60
// F117: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1233:0 core::result::Result::unwrap
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 64e0e2a1faaf0b25f492b530a269352f05d5f75c09fee7357916af92be5f0344
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: generate_roof_stone_floor_and_roof_bottom.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3774
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3774,"record_offset":268,"rva":42079520,"va":5410788640}
// 编译实例 RVAs: 0x2821520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f7b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_roof_stone_floor_and_roof_bottom.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32676905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410788640,"raw_hex":"e8d7630dfe","size":5,"soft_fail":false,"target_va":5378112288,"va":5410789188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: generate_roof_stone_floor_and_roof_bottom.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32662237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410788640,"raw_hex":"e8239d0dfe","size":5,"soft_fail":false,"target_va":5378126944,"va":5410789176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_roof_stone_floor_and_roof_bottom.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5563728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410788640,"raw_hex":"e850e55400","size":5,"soft_fail":false,"target_va":5416353040,"va":5410789307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be70
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: generate_roof_stone_floor_and_roof_bottom.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6268769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410788640,"raw_hex":"e861a75f00","size":5,"soft_fail":false,"target_va":5417057904,"va":5410789130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5832204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410788640,"raw_hex":"e80cfe5800","size":5,"soft_fail":false,"target_va":5416621632,"va":5410789423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3774,"record_offset":1184,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":3774,"record_offset":488,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":3774,"record_offset":3036,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3774,"record_offset":1492,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3774,"record_offset":3008,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3774,"record_offset":2976,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3774,"record_offset":3328,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::log2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3774,"record_offset":2948,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":14908,"source_line":541,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3774,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3774,"record_offset":3176,"parent_record_offset":3148,"ancestor_record_offset":268,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3774,"record_offset":3260,"parent_record_offset":3236,"ancestor_record_offset":268,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3774,"record_offset":3236,"parent_record_offset":3204,"ancestor_record_offset":268,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3774,"record_offset":2140,"parent_record_offset":2116,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3774,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3774,"record_offset":3204,"parent_record_offset":3176,"ancestor_record_offset":268,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3774,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3774,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3774,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":268,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3774,"record_offset":804,"parent_record_offset":576,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3774,"record_offset":1800,"parent_record_offset":1516,"ancestor_record_offset":268,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3774,"record_offset":1564,"parent_record_offset":1516,"ancestor_record_offset":268,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3774,"record_offset":1588,"parent_record_offset":1564,"ancestor_record_offset":268,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3774,"record_offset":1704,"parent_record_offset":1672,"ancestor_record_offset":268,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3774,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":268,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3774,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":268,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3774,"record_offset":1212,"parent_record_offset":1184,"ancestor_record_offset":268,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":3774,"record_offset":552,"parent_record_offset":512,"ancestor_record_offset":268,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3774,"record_offset":2784,"parent_record_offset":2760,"ancestor_record_offset":268,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3774,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":268,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3774,"record_offset":2268,"parent_record_offset":2244,"ancestor_record_offset":268,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3774,"record_offset":2760,"parent_record_offset":2736,"ancestor_record_offset":268,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3774,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":268,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3774,"record_offset":2736,"parent_record_offset":1516,"ancestor_record_offset":268,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":3774,"record_offset":2220,"parent_record_offset":1516,"ancestor_record_offset":268,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":3774,"record_offset":2680,"parent_record_offset":2656,"ancestor_record_offset":268,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3774,"record_offset":2368,"parent_record_offset":2344,"ancestor_record_offset":268,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3774,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":268,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3774,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":268,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3774,"record_offset":2320,"parent_record_offset":2292,"ancestor_record_offset":268,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3774,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":268,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3774,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":268,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3774,"record_offset":1988,"parent_record_offset":1516,"ancestor_record_offset":268,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":3774,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":268,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3774,"record_offset":2632,"parent_record_offset":2608,"ancestor_record_offset":268,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":3774,"record_offset":3092,"parent_record_offset":3064,"ancestor_record_offset":268,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":3774,"record_offset":3120,"parent_record_offset":3092,"ancestor_record_offset":268,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":3774,"record_offset":3064,"parent_record_offset":3036,"ancestor_record_offset":268,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3774,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":268,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":3774,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":268,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":3774,"record_offset":1836,"parent_record_offset":1800,"ancestor_record_offset":268,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3774,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":268,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3774,"record_offset":1248,"parent_record_offset":1212,"ancestor_record_offset":268,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3774,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":268,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3774,"record_offset":760,"parent_record_offset":732,"ancestor_record_offset":268,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3774,"record_offset":620,"parent_record_offset":576,"ancestor_record_offset":268,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3774,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3774,"record_offset":2392,"parent_record_offset":2368,"ancestor_record_offset":268,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3774,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":268,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3774,"record_offset":2440,"parent_record_offset":2416,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3774,"record_offset":1672,"parent_record_offset":1588,"ancestor_record_offset":268,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3774,"record_offset":2480,"parent_record_offset":2344,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3774,"record_offset":1772,"parent_record_offset":1516,"ancestor_record_offset":268,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3774,"record_offset":1340,"parent_record_offset":1248,"ancestor_record_offset":268,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3774,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3774,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3774,"record_offset":676,"parent_record_offset":576,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3774,"record_offset":1028,"parent_record_offset":576,"ancestor_record_offset":268,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3774,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":268,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3774,"record_offset":2560,"parent_record_offset":2528,"ancestor_record_offset":268,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3774,"record_offset":732,"parent_record_offset":704,"ancestor_record_offset":268,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3774,"record_offset":704,"parent_record_offset":676,"ancestor_record_offset":268,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3774,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3774,"record_offset":1416,"parent_record_offset":1364,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3774,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":268,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3774,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":268,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3774,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":268,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3774,"record_offset":2864,"parent_record_offset":2832,"ancestor_record_offset":268,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/03774.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3774,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":268,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/03774.jsonl.gz"}
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
// 完整结构及原始类型记录: generate_roof_stone_floor_and_roof_bottom.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: generate_roof_stone_floor_and_roof_bottom.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: generate_roof_stone_floor_and_roof_bottom.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom
// 信息 ID: bf0dee0784eb60d6631c8480
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 86c0b8990b81fe22901ff782: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)> > @ 0x1f2dbe5 -> 0x2821520 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)> @ 0x1f3ed53 -> 0x2821520 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
