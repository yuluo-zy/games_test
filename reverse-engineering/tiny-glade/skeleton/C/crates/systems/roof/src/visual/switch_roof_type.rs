// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: switch_roof_type.rs.assembly.json.gz

// FUNCTION system_roof::visual::switch_roof_type::stone_floor_bundle
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3778:2268 -> host 3778:268 / rva_2826b60_len_7cb; exact_symbolizer_function_frame
//   Function-frame RVA span [0x2826f32, 0x2826f84), 18 instructions
// Inline site 3781:4236 -> host 3781:268 / rva_2828820_len_989; exact_symbolizer_function_frame
// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.
//   Function-frame RVA span [0x2828c7e, 0x2828cc6), 13 instructions
//   Function-frame RVA span [0x2828da0, 0x2828deb), 14 instructions
// Inline site 3781:4984 -> host 3781:268 / rva_2828820_len_989; exact_symbolizer_function_frame
// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.
//   Function-frame RVA span [0x2828c7e, 0x2828cc6), 13 instructions
//   Function-frame RVA span [0x2828da0, 0x2828deb), 14 instructions
//
// FUNCTION system_roof::visual::switch_roof_type::switch_roof_type
// Candidate interfaces: aac53fb1b214cbb91e9f2332
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3781:268 -> rva_2828820_len_989
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_2826b60_len_7cb: RVA 0x2826b60, 1995 PDB bytes; 423 instructions
// Evidence: _assembly/roof-v1/C/pack-002/rva_2826b60_len_7cb.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3778:268: system_roof::create_roof::create_roof
// Frames: F1
// 0x142826b60  55                             pushq	%rbp
// 0x142826b61  4157                           pushq	%r15
// 0x142826b63  4156                           pushq	%r14
// 0x142826b65  4155                           pushq	%r13
// 0x142826b67  4154                           pushq	%r12
// 0x142826b69  56                             pushq	%rsi
// 0x142826b6a  57                             pushq	%rdi
// 0x142826b6b  53                             pushq	%rbx
// 0x142826b6c  4881ec78030000                 subq	$0x378, %rsp            # imm = 0x378
// 0x142826b73  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x142826b7b  440f298de0020000               movaps	%xmm9, 0x2e0(%rbp)
// 0x142826b83  440f2985d0020000               movaps	%xmm8, 0x2d0(%rbp)
// 0x142826b8b  0f29bdc0020000                 movaps	%xmm7, 0x2c0(%rbp)
// 0x142826b92  0f29b5b0020000                 movaps	%xmm6, 0x2b0(%rbp)
// Frames: F2 -> F3 -> F4
// 0x142826b99  48c785a8020000feffffff         movq	$-0x2, 0x2a8(%rbp)
// 0x142826ba4  488b19                         movq	(%rcx), %rbx
// Frames: F5 -> F3 -> F4
// 0x142826ba7  488b4908                       movq	0x8(%rcx), %rcx
// Frames: F6 -> F7 -> F8 -> F3 -> F4
// 0x142826bab  4c8b1b                         movq	(%rbx), %r11
// 0x142826bae  4531d2                         xorl	%r10d, %r10d
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F4
// 0x142826bb1  4c89d8                         movq	%r11, %rax
// 0x142826bb4  482b4118                       subq	0x18(%rcx), %rax
// 0x142826bb8  490f42c2                       cmovbq	%r10, %rax
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F4
// 0x142826bbc  4c2b5938                       subq	0x38(%rcx), %r11
// 0x142826bc0  4d0f42da                       cmovbq	%r10, %r11
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x142826bc4  4889c7                         movq	%rax, %rdi
// 0x142826bc7  48c1e706                       shlq	$0x6, %rdi
// 0x142826bcb  48037908                       addq	0x8(%rcx), %rdi
// Frames: F17 -> F18 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x142826bcf  4c8b7110                       movq	0x10(%rcx), %r14
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F4
// 0x142826bd3  4929c6                         subq	%rax, %r14
// 0x142826bd6  4d0f42f2                       cmovbq	%r10, %r14
// 0x142826bda  be08000000                     movl	$0x8, %esi
// 0x142826bdf  480f42fe                       cmovbq	%rsi, %rdi
// Frames: F12 -> F13 -> F14 -> F15 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x142826be3  4c89d8                         movq	%r11, %rax
// 0x142826be6  48c1e006                       shlq	$0x6, %rax
// 0x142826bea  48034128                       addq	0x28(%rcx), %rax
// Frames: F17 -> F18 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x142826bee  4c8b7930                       movq	0x30(%rcx), %r15
// Frames: F12 -> F13 -> F14 -> F15 -> F19 -> F7 -> F8 -> F3 -> F4
// 0x142826bf2  4d29df                         subq	%r11, %r15
// 0x142826bf5  480f42c6                       cmovbq	%rsi, %rax
// 0x142826bf9  4c8b9d60030000                 movq	0x360(%rbp), %r11
// 0x142826c00  4d0f42fa                       cmovbq	%r10, %r15
// Frames: F20 -> F7 -> F8 -> F3 -> F4
// 0x142826c04  488b4940                       movq	0x40(%rcx), %rcx
// 0x142826c08  4c29f9                         subq	%r15, %rcx
// 0x142826c0b  4c29f1                         subq	%r14, %rcx
// 0x142826c0e  48899d30020000                 movq	%rbx, 0x230(%rbp)
// 0x142826c15  48890b                         movq	%rcx, (%rbx)
// Frames: F21 -> F22 -> F23 -> F24 -> F7 -> F8 -> F3 -> F4
// 0x142826c18  49c1e606                       shlq	$0x6, %r14
// 0x142826c1c  4901fe                         addq	%rdi, %r14
// 0x142826c1f  4c89b550020000                 movq	%r14, 0x250(%rbp)
// 0x142826c26  49c1e706                       shlq	$0x6, %r15
// 0x142826c2a  4901c7                         addq	%rax, %r15
// 0x142826c2d  4c89bd58020000                 movq	%r15, 0x258(%rbp)
// 0x142826c34  488b1a                         movq	(%rdx), %rbx
// 0x142826c37  418b491c                       movl	0x1c(%r9), %ecx
// 0x142826c3b  898da0020000                   movl	%ecx, 0x2a0(%rbp)
// 0x142826c41  498b09                         movq	(%r9), %rcx
// 0x142826c44  498b5110                       movq	0x10(%r9), %rdx
// 0x142826c48  48899560020000                 movq	%rdx, 0x260(%rbp)
// 0x142826c4f  418b531c                       movl	0x1c(%r11), %edx
// 0x142826c53  899598020000                   movl	%edx, 0x298(%rbp)
// 0x142826c59  498b13                         movq	(%r11), %rdx
// 0x142826c5c  4d8b4b10                       movq	0x10(%r11), %r9
// 0x142826c60  4c898d40020000                 movq	%r9, 0x240(%rbp)
// 0x142826c67  458b481c                       movl	0x1c(%r8), %r9d
// 0x142826c6b  44898d9c020000                 movl	%r9d, 0x29c(%rbp)
// 0x142826c72  498b30                         movq	(%r8), %rsi
// 0x142826c75  4d8b4010                       movq	0x10(%r8), %r8
// 0x142826c79  4c898548020000                 movq	%r8, 0x248(%rbp)
// 0x142826c80  440f280da89f6d00               movaps	0x6d9fa8(%rip), %xmm9   # 0x142f00c30
// 0x142826c88  4c8d7508                       leaq	0x8(%rbp), %r14
// 0x142826c8c  48898d90020000                 movq	%rcx, 0x290(%rbp)
// 0x142826c93  4883c120                       addq	$0x20, %rcx
// 0x142826c97  48898d70020000                 movq	%rcx, 0x270(%rbp)
// 0x142826c9e  48899578020000                 movq	%rdx, 0x278(%rbp)
// 0x142826ca5  488d4a20                       leaq	0x20(%rdx), %rcx
// 0x142826ca9  48898d68020000                 movq	%rcx, 0x268(%rbp)
// 0x142826cb0  48899d38020000                 movq	%rbx, 0x238(%rbp)
// 0x142826cb7  eb45                           jmp	0x142826cfe <.text+0x2825cfe>
// 0x142826cb9  0f1f8000000000                 nopl	(%rax)
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F32 -> F33 -> F34
// 0x142826cc0  488b4628                       movq	0x28(%rsi), %rax
// Frames: F35 -> F29 -> F30 -> F31 -> F32 -> F33 -> F34
// 0x142826cc4  4c89f9                         movq	%r15, %rcx
// 0x142826cc7  48c1e105                       shlq	$0x5, %rcx
// Frames: F36 -> F37 -> F30 -> F31 -> F32 -> F33 -> F34
// 0x142826ccb  4c892408                       movq	%r12, (%rax,%rcx)
// 0x142826ccf  48897c0808                     movq	%rdi, 0x8(%rax,%rcx)
// 0x142826cd4  4c896c0810                     movq	%r13, 0x10(%rax,%rcx)
// 0x142826cd9  c644081800                     movb	$0x0, 0x18(%rax,%rcx)
// Frames: F38 -> F30 -> F31 -> F32 -> F33 -> F34
// 0x142826cde  49ffc7                         incq	%r15
// 0x142826ce1  4c897e30                       movq	%r15, 0x30(%rsi)
// Frames: F39 -> F32 -> F33 -> F34
// 0x142826ce5  48ff4640                       incq	0x40(%rsi)
// 0x142826ce9  488bbd80020000                 movq	0x280(%rbp), %rdi
// 0x142826cf0  488b8528020000                 movq	0x228(%rbp), %rax
// 0x142826cf7  488b9d38020000                 movq	0x238(%rbp), %rbx
// Frames: F40 -> F41 -> F42 -> F43 -> F44 -> F4
// 0x142826cfe  4885ff                         testq	%rdi, %rdi
// 0x142826d01  741d                           je	0x142826d20 <.text+0x2825d20>
// 0x142826d03  483bbd50020000                 cmpq	0x250(%rbp), %rdi
// 0x142826d0a  7414                           je	0x142826d20 <.text+0x2825d20>
// 0x142826d0c  488d4f40                       leaq	0x40(%rdi), %rcx
// 0x142826d10  48898d80020000                 movq	%rcx, 0x280(%rbp)
// 0x142826d17  4989c4                         movq	%rax, %r12
// 0x142826d1a  eb2c                           jmp	0x142826d48 <.text+0x2825d48>
// 0x142826d1c  0f1f4000                       nopl	(%rax)
// Frames: F40 -> F45 -> F46 -> F42 -> F43 -> F44 -> F4
// 0x142826d20  4885c0                         testq	%rax, %rax
// 0x142826d23  0f8494040000                   je	0x1428271bd <.text+0x28261bd>
// 0x142826d29  483b8558020000                 cmpq	0x258(%rbp), %rax
// 0x142826d30  0f8487040000                   je	0x1428271bd <.text+0x28261bd>
// Frames: F47 -> F48 -> F45 -> F46 -> F42 -> F43 -> F44 -> F4
// 0x142826d36  4c8d6040                       leaq	0x40(%rax), %r12
// 0x142826d3a  48c7858002000000000000         movq	$0x0, 0x280(%rbp)
// 0x142826d45  4889c7                         movq	%rax, %rdi
// Frames: F49 -> F44 -> F4
// 0x142826d48  488b8530020000                 movq	0x230(%rbp), %rax
// 0x142826d4f  48ff00                         incq	(%rax)
// Frames: F50
// 0x142826d52  4c8b6f08                       movq	0x8(%rdi), %r13
// 0x142826d56  4889d9                         movq	%rbx, %rcx
// 0x142826d59  4c89ea                         movq	%r13, %rdx
// 0x142826d5c  4c8d05151ad600                 leaq	0xd61a15(%rip), %r8     # 0x143588778
// 0x142826d63  e808a537fe                     callq	0x140ba1270 <.text+0xba0270>
// Frames: F51 -> F50
// 0x142826d68  4885c0                         testq	%rax, %rax
// 0x142826d6b  0f84aa040000                   je	0x14282721b <.text+0x282621b>
// Frames: F52
// 0x142826d71  4989c7                         movq	%rax, %r15
// 0x142826d74  4889d9                         movq	%rbx, %rcx
// 0x142826d77  4c89ea                         movq	%r13, %rdx
// 0x142826d7a  4c8d05271ad600                 leaq	0xd61a27(%rip), %r8     # 0x1435887a8
// 0x142826d81  e8eaa437fe                     callq	0x140ba1270 <.text+0xba0270>
// Frames: F53 -> F54
// 0x142826d86  4885c0                         testq	%rax, %rax
// 0x142826d89  0f8498040000                   je	0x142827227 <.text+0x2826227>
// Frames: F55 -> F56 -> F54
// 0x142826d8f  8b8858020000                   movl	0x258(%rax), %ecx
// 0x142826d95  83f902                         cmpl	$0x2, %ecx
// 0x142826d98  0f8351040000                   jae	0x1428271ef <.text+0x28261ef>
// 0x142826d9e  4c89a528020000                 movq	%r12, 0x228(%rbp)
// 0x142826da5  488b906c020000                 movq	0x26c(%rax), %rdx
// 0x142826dac  4c8d85a0010000                 leaq	0x1a0(%rbp), %r8
// 0x142826db3  49895010                       movq	%rdx, 0x10(%r8)
// 0x142826db7  0f10805c020000                 movups	0x25c(%rax), %xmm0
// 0x142826dbe  410f1100                       movups	%xmm0, (%r8)
// Frames: F57 -> F56 -> F54
// 0x142826dc2  0fb6909f020000                 movzbl	0x29f(%rax), %edx
// 0x142826dc9  440fb6a0a3020000               movzbl	0x2a3(%rax), %r12d
// Frames: F58
// 0x142826dd1  898d9c010000                   movl	%ecx, 0x19c(%rbp)
// 0x142826dd7  4488a5a6020000                 movb	%r12b, 0x2a6(%rbp)
// 0x142826dde  8895a5020000                   movb	%dl, 0x2a5(%rbp)
// Frames: F59
// 0x142826de4  4c89f9                         movq	%r15, %rcx
// 0x142826de7  e8c4b13cfe                     callq	0x140bf1fb0 <.text+0xbf0fb0>
// 0x142826dec  0f28f0                         movaps	%xmm0, %xmm6
// Frames: F60
// 0x142826def  0f104720                       movups	0x20(%rdi), %xmm0
// 0x142826df3  0f104f2c                       movups	0x2c(%rdi), %xmm1
// 0x142826df7  0f118d4c010000                 movups	%xmm1, 0x14c(%rbp)
// 0x142826dfe  0f298540010000                 movaps	%xmm0, 0x140(%rbp)
// Frames: F61
// 0x142826e05  8b5f10                         movl	0x10(%rdi), %ebx
// Frames: F62
// 0x142826e08  488d8da5020000                 leaq	0x2a5(%rbp), %rcx
// 0x142826e0f  4489e2                         movl	%r12d, %edx
// 0x142826e12  e859e942fe                     callq	0x140c55770 <.text+0xc54770>
// Frames: F63
// 0x142826e17  8b4f1c                         movl	0x1c(%rdi), %ecx
// 0x142826e1a  898dc0010000                   movl	%ecx, 0x1c0(%rbp)
// 0x142826e20  488b4f14                       movq	0x14(%rdi), %rcx
// 0x142826e24  48898db8010000                 movq	%rcx, 0x1b8(%rbp)
// Frames: F64
// 0x142826e2b  0f10859c010000                 movups	0x19c(%rbp), %xmm0
// 0x142826e32  0f108da8010000                 movups	0x1a8(%rbp), %xmm1
// 0x142826e39  0f114dcc                       movups	%xmm1, -0x34(%rbp)
// 0x142826e3d  0f2945c0                       movaps	%xmm0, -0x40(%rbp)
// 0x142826e41  488d8db8010000                 leaq	0x1b8(%rbp), %rcx
// 0x142826e48  48894c2438                     movq	%rcx, 0x38(%rsp)
// 0x142826e4d  88442430                       movb	%al, 0x30(%rsp)
// 0x142826e51  895c2428                       movl	%ebx, 0x28(%rsp)
// 0x142826e55  f30f11742420                   movss	%xmm6, 0x20(%rsp)
// 0x142826e5b  488dbde8000000                 leaq	0xe8(%rbp), %rdi
// 0x142826e62  4889f9                         movq	%rdi, %rcx
// 0x142826e65  4c89ea                         movq	%r13, %rdx
// 0x142826e68  4c8d8540010000                 leaq	0x140(%rbp), %r8
// 0x142826e6f  4c8d4dc0                       leaq	-0x40(%rbp), %r9
// 0x142826e73  e8681e3cfe                     callq	0x140be8ce0 <.text+0xbe7ce0>
// Frames: F65
// 0x142826e78  4889f9                         movq	%rdi, %rcx
// 0x142826e7b  e8501e3cfe                     callq	0x140be8cd0 <.text+0xbe7cd0>
// 0x142826e80  89c3                           movl	%eax, %ebx
// Frames: F66
// 0x142826e82  4889f9                         movq	%rdi, %rcx
// 0x142826e85  e816203cfe                     callq	0x140be8ea0 <.text+0xbe7ea0>
// 0x142826e8a  84c0                           testb	%al, %al
// 0x142826e8c  745d                           je	0x142826eeb <.text+0x2825eeb>
// Frames: F67
// 0x142826e8e  4c8bbde8000000                 movq	0xe8(%rbp), %r15
// 0x142826e95  8b85a0020000                   movl	0x2a0(%rbp), %eax
// 0x142826e9b  488b8d60020000                 movq	0x260(%rbp), %rcx
// Frames: F68 -> F69 -> F33 -> F70
// 0x142826ea2  8901                           movl	%eax, (%rcx)
// 0x142826ea4  488b8590020000                 movq	0x290(%rbp), %rax
// Frames: F71 -> F30 -> F31 -> F72 -> F33 -> F70
// 0x142826eab  488b7830                       movq	0x30(%rax), %rdi
// Frames: F73 -> F72 -> F33 -> F70
// 0x142826eaf  4c8b6040                       movq	0x40(%rax), %r12
// Frames: F74 -> F30 -> F31 -> F72 -> F33 -> F70
// 0x142826eb3  483b7820                       cmpq	0x20(%rax), %rdi
// 0x142826eb7  750c                           jne	0x142826ec5 <.text+0x2825ec5>
// Frames: F75 -> F30 -> F31 -> F72 -> F33 -> F70
// 0x142826eb9  488b8d70020000                 movq	0x270(%rbp), %rcx
// 0x142826ec0  e88b865400                     callq	0x142d6f550 <.text+0x2d6e550>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F72 -> F33 -> F70
// 0x142826ec5  488b9590020000                 movq	0x290(%rbp), %rdx
// 0x142826ecc  488b4228                       movq	0x28(%rdx), %rax
// Frames: F35 -> F29 -> F30 -> F31 -> F72 -> F33 -> F70
// 0x142826ed0  4889f9                         movq	%rdi, %rcx
// 0x142826ed3  48c1e104                       shlq	$0x4, %rcx
// Frames: F36 -> F37 -> F30 -> F31 -> F72 -> F33 -> F70
// 0x142826ed7  4c892408                       movq	%r12, (%rax,%rcx)
// 0x142826edb  4c897c0808                     movq	%r15, 0x8(%rax,%rcx)
// Frames: F38 -> F30 -> F31 -> F72 -> F33 -> F70
// 0x142826ee0  48ffc7                         incq	%rdi
// 0x142826ee3  48897a30                       movq	%rdi, 0x30(%rdx)
// Frames: F39 -> F72 -> F33 -> F70
// 0x142826ee7  48ff4240                       incq	0x40(%rdx)
// Frames: F76
// 0x142826eeb  488d8d10020000                 leaq	0x210(%rbp), %rcx
// 0x142826ef2  0f28ce                         movaps	%xmm6, %xmm1
// 0x142826ef5  e8465c1cfe                     callq	0x1409ecb40 <.text+0x9ebb40>
// Frames: F77
// 0x142826efa  488d8db8010000                 leaq	0x1b8(%rbp), %rcx
// 0x142826f01  e8cade1dfe                     callq	0x140a04dd0 <.text+0xa03dd0>
// 0x142826f06  90                             nop
// Frames: F78
// 0x142826f07  e884a81bfe                     callq	0x1409e1790 <.text+0x9e0790>
// 0x142826f0c  90                             nop
// Frames: F79
// 0x142826f0d  0f28f8                         movaps	%xmm0, %xmm7
// 0x142826f10  440f28c1                       movaps	%xmm1, %xmm8
// 0x142826f14  e8e7d18dfd                     callq	0x140104100 <.text+0x103100>
// 0x142826f19  90                             nop
// 0x142826f1a  4989d7                         movq	%rdx, %r15
// 0x142826f1d  48898588020000                 movq	%rax, 0x288(%rbp)
// 0x142826f24  48898500020000                 movq	%rax, 0x200(%rbp)
// 0x142826f2b  48899508020000                 movq	%rdx, 0x208(%rbp)
// Frames: F80 -> F81
// 0x142826f32  31c9                           xorl	%ecx, %ecx
// 0x142826f34  e827450dfe                     callq	0x1408fb460 <.text+0x8fa460>
// 0x142826f39  90                             nop
// 0x142826f3a  488dbd70010000                 leaq	0x170(%rbp), %rdi
// Frames: F82 -> F80 -> F81
// 0x142826f41  4889f9                         movq	%rdi, %rcx
// 0x142826f44  89c2                           movl	%eax, %edx
// 0x142826f46  e8d50b0dfe                     callq	0x1408f7b20 <.text+0x8f6b20>
// 0x142826f4b  90                             nop
// Frames: F83 -> F84 -> F81
// 0x142826f4c  c747e800000000                 movl	$0x0, -0x18(%rdi)
// 0x142826f53  48c747e000000000               movq	$0x0, -0x20(%rdi)
// Frames: F85 -> F81
// 0x142826f5b  e8b0c37dfd                     callq	0x140003310 <.text+0x2310>
// 0x142826f60  90                             nop
// Frames: F86 -> F81
// 0x142826f61  c6858001000007                 movb	$0x7, 0x180(%rbp)
// 0x142826f68  440f298d40010000               movaps	%xmm9, 0x140(%rbp)
// 0x142826f70  8b05a61cd600                   movl	0xd61ca6(%rip), %eax    # 0x143588c1c
// 0x142826f76  8947f4                         movl	%eax, -0xc(%rdi)
// 0x142826f79  488b05941cd600                 movq	0xd61c94(%rip), %rax    # 0x143588c14
// 0x142826f80  488947ec                       movq	%rax, -0x14(%rdi)
// Frames: F87
// 0x142826f84  488b8538010000                 movq	0x138(%rbp), %rax
// 0x142826f8b  49894650                       movq	%rax, 0x50(%r14)
// 0x142826f8f  0f108528010000                 movups	0x128(%rbp), %xmm0
// 0x142826f96  410f114640                     movups	%xmm0, 0x40(%r14)
// 0x142826f9b  0f1085e8000000                 movups	0xe8(%rbp), %xmm0
// 0x142826fa2  0f108df8000000                 movups	0xf8(%rbp), %xmm1
// 0x142826fa9  0f109508010000                 movups	0x108(%rbp), %xmm2
// 0x142826fb0  0f109d18010000                 movups	0x118(%rbp), %xmm3
// 0x142826fb7  410f115e30                     movups	%xmm3, 0x30(%r14)
// 0x142826fbc  410f115620                     movups	%xmm2, 0x20(%r14)
// 0x142826fc1  410f114e10                     movups	%xmm1, 0x10(%r14)
// 0x142826fc6  410f1106                       movups	%xmm0, (%r14)
// 0x142826fca  0f108510020000                 movups	0x210(%rbp), %xmm0
// 0x142826fd1  410f114658                     movups	%xmm0, 0x58(%r14)
// 0x142826fd6  488b8520020000                 movq	0x220(%rbp), %rax
// 0x142826fdd  49894668                       movq	%rax, 0x68(%r14)
// 0x142826fe1  0f1085b8010000                 movups	0x1b8(%rbp), %xmm0
// 0x142826fe8  0f108dc8010000                 movups	0x1c8(%rbp), %xmm1
// 0x142826fef  0f1095d8010000                 movups	0x1d8(%rbp), %xmm2
// 0x142826ff6  0f109de8010000                 movups	0x1e8(%rbp), %xmm3
// 0x142826ffd  0f2945c0                       movaps	%xmm0, -0x40(%rbp)
// 0x142827001  0f294dd0                       movaps	%xmm1, -0x30(%rbp)
// 0x142827005  0f2955e0                       movaps	%xmm2, -0x20(%rbp)
// 0x142827009  0f295df0                       movaps	%xmm3, -0x10(%rbp)
// 0x14282700d  488b85f8010000                 movq	0x1f8(%rbp), %rax
// 0x142827014  48894500                       movq	%rax, (%rbp)
// 0x142827018  f30f117d78                     movss	%xmm7, 0x78(%rbp)
// 0x14282701d  f3440f11457c                   movss	%xmm8, 0x7c(%rbp)
// 0x142827023  488b8588020000                 movq	0x288(%rbp), %rax
// 0x14282702a  48898580000000                 movq	%rax, 0x80(%rbp)
// 0x142827031  4c89bd88000000                 movq	%r15, 0x88(%rbp)
// 0x142827038  0f288580010000                 movaps	0x180(%rbp), %xmm0
// 0x14282703f  410f2986c8000000               movaps	%xmm0, 0xc8(%r14)
// 0x142827047  0f288540010000                 movaps	0x140(%rbp), %xmm0
// 0x14282704e  0f288d50010000                 movaps	0x150(%rbp), %xmm1
// 0x142827055  0f289560010000                 movaps	0x160(%rbp), %xmm2
// 0x14282705c  0f289d70010000                 movaps	0x170(%rbp), %xmm3
// 0x142827063  410f299eb8000000               movaps	%xmm3, 0xb8(%r14)
// 0x14282706b  410f2996a8000000               movaps	%xmm2, 0xa8(%r14)
// 0x142827073  410f298e98000000               movaps	%xmm1, 0x98(%r14)
// 0x14282707b  410f298688000000               movaps	%xmm0, 0x88(%r14)
// 0x142827083  488d8d40010000                 leaq	0x140(%rbp), %rcx
// 0x14282708a  488b9568030000                 movq	0x368(%rbp), %rdx
// 0x142827091  4c8d45c0                       leaq	-0x40(%rbp), %r8
// 0x142827095  4c8d0d3c17d600                 leaq	0xd6173c(%rip), %r9     # 0x1435887d8
// 0x14282709c  e84f6579fe                     callq	0x140fbd5f0 <.text+0xfbc5f0>
// Frames: F88
// 0x1428270a1  488bbd40010000                 movq	0x140(%rbp), %rdi
// Frames: F89
// 0x1428270a8  84db                           testb	%bl, %bl
// 0x1428270aa  7414                           je	0x1428270c0 <.text+0x28260c0>
// 0x1428270ac  4531ff                         xorl	%r15d, %r15d
// 0x1428270af  eb24                           jmp	0x1428270d5 <.text+0x28260d5>
// 0x1428270b1  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F90
// 0x1428270c0  488d8da6020000                 leaq	0x2a6(%rbp), %rcx
// 0x1428270c7  e8642a0dfe                     callq	0x1408f9b30 <.text+0x8f8b30>
// 0x1428270cc  8885a7020000                   movb	%al, 0x2a7(%rbp)
// 0x1428270d2  41b706                         movb	$0x6, %r15b
// Frames: F91
// 0x1428270d5  488d8d9c010000                 leaq	0x19c(%rbp), %rcx
// 0x1428270dc  e8efb223fe                     callq	0x140a623d0 <.text+0xa613d0>
// 0x1428270e1  f30f118510020000               movss	%xmm0, 0x210(%rbp)
// 0x1428270e9  f30f118d14020000               movss	%xmm1, 0x214(%rbp)
// 0x1428270f1  488d8db8010000                 leaq	0x1b8(%rbp), %rcx
// 0x1428270f8  488d9510020000                 leaq	0x210(%rbp), %rdx
// 0x1428270ff  0f28d6                         movaps	%xmm6, %xmm2
// 0x142827102  e879945cfe                     callq	0x140df0580 <.text+0xdef580>
// Frames: F92
// 0x142827107  8b85c0010000                   movl	0x1c0(%rbp), %eax
// 0x14282710d  418946ce                       movl	%eax, -0x32(%r14)
// 0x142827111  488b85b8010000                 movq	0x1b8(%rbp), %rax
// 0x142827118  498946c6                       movq	%rax, -0x3a(%r14)
// 0x14282711c  8b8598020000                   movl	0x298(%rbp), %eax
// 0x142827122  488b8d40020000                 movq	0x240(%rbp), %rcx
// Frames: F68 -> F69 -> F33 -> F92
// 0x142827129  8901                           movl	%eax, (%rcx)
// 0x14282712b  488b8578020000                 movq	0x278(%rbp), %rax
// Frames: F71 -> F30 -> F31 -> F72 -> F33 -> F92
// 0x142827132  4c8b6030                       movq	0x30(%rax), %r12
// Frames: F73 -> F72 -> F33 -> F92
// 0x142827136  488b5840                       movq	0x40(%rax), %rbx
// Frames: F74 -> F30 -> F31 -> F72 -> F33 -> F92
// 0x14282713a  4c3b6020                       cmpq	0x20(%rax), %r12
// 0x14282713e  750c                           jne	0x14282714c <.text+0x282614c>
// Frames: F75 -> F30 -> F31 -> F72 -> F33 -> F92
// 0x142827140  488b8d68020000                 movq	0x268(%rbp), %rcx
// 0x142827147  e8d4815400                     callq	0x142d6f320 <.text+0x2d6e320>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F72 -> F33 -> F92
// 0x14282714c  488b9578020000                 movq	0x278(%rbp), %rdx
// 0x142827153  488b4228                       movq	0x28(%rdx), %rax
// Frames: F35 -> F29 -> F30 -> F31 -> F72 -> F33 -> F92
// 0x142827157  4b8d0ca4                       leaq	(%r12,%r12,4), %rcx
// Frames: F36 -> F37 -> F30 -> F31 -> F72 -> F33 -> F92
// 0x14282715b  48891cc8                       movq	%rbx, (%rax,%rcx,8)
// 0x14282715f  44887cc808                     movb	%r15b, 0x8(%rax,%rcx,8)
// 0x142827164  440fb685a7020000               movzbl	0x2a7(%rbp), %r8d
// 0x14282716c  448844c809                     movb	%r8b, 0x9(%rax,%rcx,8)
// 0x142827171  0f1045c0                       movups	-0x40(%rbp), %xmm0
// 0x142827175  0f104dca                       movups	-0x36(%rbp), %xmm1
// 0x142827179  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// 0x14282717e  0f1144c80a                     movups	%xmm0, 0xa(%rax,%rcx,8)
// Frames: F38 -> F30 -> F31 -> F72 -> F33 -> F92
// 0x142827183  49ffc4                         incq	%r12
// 0x142827186  4c896230                       movq	%r12, 0x30(%rdx)
// Frames: F39 -> F72 -> F33 -> F92
// 0x14282718a  48ff4240                       incq	0x40(%rdx)
// 0x14282718e  8b859c020000                   movl	0x29c(%rbp), %eax
// 0x142827194  488b8d48020000                 movq	0x248(%rbp), %rcx
// Frames: F68 -> F93 -> F33 -> F34
// 0x14282719b  8901                           movl	%eax, (%rcx)
// Frames: F71 -> F30 -> F31 -> F32 -> F33 -> F34
// 0x14282719d  4c8b7e30                       movq	0x30(%rsi), %r15
// Frames: F73 -> F32 -> F33 -> F34
// 0x1428271a1  4c8b6640                       movq	0x40(%rsi), %r12
// Frames: F74 -> F30 -> F31 -> F32 -> F33 -> F34
// 0x1428271a5  4c3b7e20                       cmpq	0x20(%rsi), %r15
// 0x1428271a9  0f8511fbffff                   jne	0x142826cc0 <.text+0x2825cc0>
// Frames: F75 -> F30 -> F31 -> F32 -> F33 -> F34
// 0x1428271af  488d4e20                       leaq	0x20(%rsi), %rcx
// 0x1428271b3  e8f8805400                     callq	0x142d6f2b0 <.text+0x2d6e2b0>
// 0x1428271b8  e903fbffff                     jmp	0x142826cc0 <.text+0x2825cc0>
// Frames: F94
// 0x1428271bd  0f28b5b0020000                 movaps	0x2b0(%rbp), %xmm6
// 0x1428271c4  0f28bdc0020000                 movaps	0x2c0(%rbp), %xmm7
// 0x1428271cb  440f2885d0020000               movaps	0x2d0(%rbp), %xmm8
// 0x1428271d3  440f288de0020000               movaps	0x2e0(%rbp), %xmm9
// 0x1428271db  4881c478030000                 addq	$0x378, %rsp            # imm = 0x378
// 0x1428271e2  5b                             popq	%rbx
// 0x1428271e3  5f                             popq	%rdi
// 0x1428271e4  5e                             popq	%rsi
// 0x1428271e5  415c                           popq	%r12
// 0x1428271e7  415d                           popq	%r13
// 0x1428271e9  415e                           popq	%r14
// 0x1428271eb  415f                           popq	%r15
// 0x1428271ed  5d                             popq	%rbp
// 0x1428271ee  c3                             retq
// Frames: F95 -> F56 -> F54
// 0x1428271ef  480558020000                   addq	$0x258, %rax            # imm = 0x258
// 0x1428271f5  488945c0                       movq	%rax, -0x40(%rbp)
// 0x1428271f9  488d0530010000                 leaq	0x130(%rip), %rax       # 0x142827330 <.text+0x2826330>
// 0x142827200  488945c8                       movq	%rax, -0x38(%rbp)
// 0x142827204  488d0de515d600                 leaq	0xd615e5(%rip), %rcx    # 0x1435887f0
// 0x14282720b  4c8d052e16d600                 leaq	0xd6162e(%rip), %r8     # 0x143588840
// 0x142827212  488d55c0                       leaq	-0x40(%rbp), %rdx
// 0x142827216  e885a65800                     callq	0x142db18a0 <.text+0x2db08a0>
// Frames: F96 -> F50
// 0x14282721b  488d0d6e15d600                 leaq	0xd6156e(%rip), %rcx    # 0x143588790
// 0x142827222  e8f9a35800                     callq	0x142db1620 <.text+0x2db0620>
// Frames: F96 -> F97
// 0x142827227  488d0d9215d600                 leaq	0xd61592(%rip), %rcx    # 0x1435887c0
// 0x14282722e  e8eda35800                     callq	0x142db1620 <.text+0x2db0620>
// 0x142827233  cc                             int3
// 0x142827234  6666662e0f1f840000000000       nopw	%cs:(%rax,%rax)
// 0x142827240  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x142827245  55                             pushq	%rbp
// 0x142827246  4157                           pushq	%r15
// 0x142827248  4156                           pushq	%r14
// 0x14282724a  4155                           pushq	%r13
// 0x14282724c  4154                           pushq	%r12
// 0x14282724e  56                             pushq	%rsi
// 0x14282724f  57                             pushq	%rdi
// 0x142827250  53                             pushq	%rbx
// 0x142827251  4881ec88000000                 subq	$0x88, %rsp
// 0x142827258  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14282725f  440f294c2440                   movaps	%xmm9, 0x40(%rsp)
// 0x142827265  440f29442450                   movaps	%xmm8, 0x50(%rsp)
// 0x14282726b  0f297c2460                     movaps	%xmm7, 0x60(%rsp)
// 0x142827270  0f29742470                     movaps	%xmm6, 0x70(%rsp)
// Frames: F98 -> F98 -> F99
// 0x142827275  4883bd8802000000               cmpq	$0x0, 0x288(%rbp)
// 0x14282727d  740c                           je	0x14282728b <.text+0x282628b>
// 0x14282727f  488d8d00020000                 leaq	0x200(%rbp), %rcx
// 0x142827286  e8458479fe                     callq	0x140fbf6d0 <.text+0xfbe6d0>
// Frames: F99
// 0x14282728b  0f28742470                     movaps	0x70(%rsp), %xmm6
// 0x142827290  0f287c2460                     movaps	0x60(%rsp), %xmm7
// 0x142827295  440f28442450                   movaps	0x50(%rsp), %xmm8
// 0x14282729b  440f284c2440                   movaps	0x40(%rsp), %xmm9
// 0x1428272a1  4881c488000000                 addq	$0x88, %rsp
// 0x1428272a8  5b                             popq	%rbx
// 0x1428272a9  5f                             popq	%rdi
// 0x1428272aa  5e                             popq	%rsi
// 0x1428272ab  415c                           popq	%r12
// 0x1428272ad  415d                           popq	%r13
// 0x1428272af  415e                           popq	%r14
// 0x1428272b1  415f                           popq	%r15
// 0x1428272b3  5d                             popq	%rbp
// 0x1428272b4  c3                             retq
// 0x1428272b5  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// 0x1428272c0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1428272c5  55                             pushq	%rbp
// 0x1428272c6  4157                           pushq	%r15
// 0x1428272c8  4156                           pushq	%r14
// 0x1428272ca  4155                           pushq	%r13
// 0x1428272cc  4154                           pushq	%r12
// 0x1428272ce  56                             pushq	%rsi
// 0x1428272cf  57                             pushq	%rdi
// 0x1428272d0  53                             pushq	%rbx
// 0x1428272d1  4881ec88000000                 subq	$0x88, %rsp
// 0x1428272d8  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1428272df  440f294c2440                   movaps	%xmm9, 0x40(%rsp)
// 0x1428272e5  440f29442450                   movaps	%xmm8, 0x50(%rsp)
// 0x1428272eb  0f297c2460                     movaps	%xmm7, 0x60(%rsp)
// 0x1428272f0  0f29742470                     movaps	%xmm6, 0x70(%rsp)
// 0x1428272f5  488d8db8010000                 leaq	0x1b8(%rbp), %rcx
// 0x1428272fc  e8afb89cfd                     callq	0x1401f2bb0 <.text+0x1f1bb0>
// 0x142827301  0f28742470                     movaps	0x70(%rsp), %xmm6
// 0x142827306  0f287c2460                     movaps	0x60(%rsp), %xmm7
// 0x14282730b  440f28442450                   movaps	0x50(%rsp), %xmm8
// 0x142827311  440f284c2440                   movaps	0x40(%rsp), %xmm9
// Frames: F1
// 0x142827317  4881c488000000                 addq	$0x88, %rsp
// 0x14282731e  5b                             popq	%rbx
// 0x14282731f  5f                             popq	%rdi
// 0x142827320  5e                             popq	%rsi
// 0x142827321  415c                           popq	%r12
// 0x142827323  415d                           popq	%r13
// 0x142827325  415e                           popq	%r14
// 0x142827327  415f                           popq	%r15
// 0x142827329  5d                             popq	%rbp
// 0x14282732a  c3                             retq
//
// RANGE rva_2828820_len_989: RVA 0x2828820, 2441 PDB bytes; 558 instructions
// Evidence: _assembly/roof-v1/C/pack-003/rva_2828820_len_989.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3781:268: system_roof::visual::switch_roof_type::switch_roof_type
// Frames: F100
// 0x142828820  55                             pushq	%rbp
// 0x142828821  4157                           pushq	%r15
// 0x142828823  4156                           pushq	%r14
// 0x142828825  4155                           pushq	%r13
// 0x142828827  4154                           pushq	%r12
// 0x142828829  56                             pushq	%rsi
// 0x14282882a  57                             pushq	%rdi
// 0x14282882b  53                             pushq	%rbx
// 0x14282882c  4881eca8010000                 subq	$0x1a8, %rsp            # imm = 0x1A8
// 0x142828833  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x14282883b  440f298d10010000               movaps	%xmm9, 0x110(%rbp)
// 0x142828843  440f298500010000               movaps	%xmm8, 0x100(%rbp)
// 0x14282884b  0f29bdf0000000                 movaps	%xmm7, 0xf0(%rbp)
// 0x142828852  0f29b5e0000000                 movaps	%xmm6, 0xe0(%rbp)
// 0x142828859  48c785d8000000feffffff         movq	$-0x2, 0xd8(%rbp)
// 0x142828864  4c8b9590010000                 movq	0x190(%rbp), %r10
// Frames: F2 -> F3 -> F101
// 0x14282886b  4c8b31                         movq	(%rcx), %r14
// Frames: F5 -> F3 -> F101
// 0x14282886e  488b7108                       movq	0x8(%rcx), %rsi
// Frames: F6 -> F7 -> F8 -> F3 -> F101
// 0x142828872  4d8b1e                         movq	(%r14), %r11
// 0x142828875  31ff                           xorl	%edi, %edi
// Frames: F9 -> F10 -> F7 -> F8 -> F3 -> F101
// 0x142828877  4c89d9                         movq	%r11, %rcx
// 0x14282887a  482b4e18                       subq	0x18(%rsi), %rcx
// 0x14282887e  480f42cf                       cmovbq	%rdi, %rcx
// Frames: F9 -> F11 -> F7 -> F8 -> F3 -> F101
// 0x142828882  4c2b5e38                       subq	0x38(%rsi), %r11
// 0x142828886  4c0f42df                       cmovbq	%rdi, %r11
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F101
// 0x14282888a  4889c8                         movq	%rcx, %rax
// 0x14282888d  48c1e005                       shlq	$0x5, %rax
// 0x142828891  48034608                       addq	0x8(%rsi), %rax
// Frames: F17 -> F18 -> F16 -> F7 -> F8 -> F3 -> F101
// 0x142828895  4c8b7e10                       movq	0x10(%rsi), %r15
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F101
// 0x142828899  4929cf                         subq	%rcx, %r15
// 0x14282889c  4c0f42ff                       cmovbq	%rdi, %r15
// Frames: F17 -> F18 -> F19 -> F7 -> F8 -> F3 -> F101
// 0x1428288a0  4c8b6630                       movq	0x30(%rsi), %r12
// 0x1428288a4  bb08000000                     movl	$0x8, %ebx
// Frames: F12 -> F13 -> F14 -> F15 -> F16 -> F7 -> F8 -> F3 -> F101
// 0x1428288a9  480f42c3                       cmovbq	%rbx, %rax
// Frames: F12 -> F13 -> F14 -> F15 -> F19 -> F7 -> F8 -> F3 -> F101
// 0x1428288ad  4c89d9                         movq	%r11, %rcx
// 0x1428288b0  48c1e105                       shlq	$0x5, %rcx
// 0x1428288b4  48034e28                       addq	0x28(%rsi), %rcx
// 0x1428288b8  4d29dc                         subq	%r11, %r12
// 0x1428288bb  4c8b9da0010000                 movq	0x1a0(%rbp), %r11
// 0x1428288c2  480f42cb                       cmovbq	%rbx, %rcx
// 0x1428288c6  488b9da8010000                 movq	0x1a8(%rbp), %rbx
// 0x1428288cd  4c0f42e7                       cmovbq	%rdi, %r12
// Frames: F20 -> F7 -> F8 -> F3 -> F101
// 0x1428288d1  488b7640                       movq	0x40(%rsi), %rsi
// 0x1428288d5  4c29e6                         subq	%r12, %rsi
// 0x1428288d8  4c29fe                         subq	%r15, %rsi
// 0x1428288db  488bbd98010000                 movq	0x198(%rbp), %rdi
// 0x1428288e2  4c8975c0                       movq	%r14, -0x40(%rbp)
// 0x1428288e6  498936                         movq	%rsi, (%r14)
// Frames: F21 -> F22 -> F23 -> F24 -> F7 -> F8 -> F3 -> F101
// 0x1428288e9  49c1e705                       shlq	$0x5, %r15
// 0x1428288ed  4c897d08                       movq	%r15, 0x8(%rbp)
// 0x1428288f1  49c1e405                       shlq	$0x5, %r12
// 0x1428288f5  4c896520                       movq	%r12, 0x20(%rbp)
// 0x1428288f9  488b7208                       movq	0x8(%rdx), %rsi
// 0x1428288fd  4885f6                         testq	%rsi, %rsi
// 0x142828900  7504                           jne	0x142828906 <.text+0x2827906>
// 0x142828902  488b7210                       movq	0x10(%rdx), %rsi
// 0x142828906  48014508                       addq	%rax, 0x8(%rbp)
// 0x14282890a  48014d20                       addq	%rcx, 0x20(%rbp)
// 0x14282890e  4c8b37                         movq	(%rdi), %r14
// 0x142828911  4c8b6f08                       movq	0x8(%rdi), %r13
// 0x142828915  488b13                         movq	(%rbx), %rdx
// 0x142828918  48895530                       movq	%rdx, 0x30(%rbp)
// 0x14282891c  488b5310                       movq	0x10(%rbx), %rdx
// 0x142828920  488955e8                       movq	%rdx, -0x18(%rbp)
// 0x142828924  8b531c                         movl	0x1c(%rbx), %edx
// 0x142828927  899594000000                   movl	%edx, 0x94(%rbp)
// 0x14282892d  498b10                         movq	(%r8), %rdx
// 0x142828930  48895510                       movq	%rdx, 0x10(%rbp)
// 0x142828934  418b531c                       movl	0x1c(%r11), %edx
// 0x142828938  8995cc000000                   movl	%edx, 0xcc(%rbp)
// 0x14282893e  498b13                         movq	(%r11), %rdx
// 0x142828941  4d8b4310                       movq	0x10(%r11), %r8
// 0x142828945  4c8945f8                       movq	%r8, -0x8(%rbp)
// 0x142828949  458b421c                       movl	0x1c(%r10), %r8d
// 0x14282894d  448985d0000000                 movl	%r8d, 0xd0(%rbp)
// 0x142828954  4d8b02                         movq	(%r10), %r8
// 0x142828957  4d8b5210                       movq	0x10(%r10), %r10
// 0x14282895b  4c895518                       movq	%r10, 0x18(%rbp)
// 0x14282895f  458b511c                       movl	0x1c(%r9), %r10d
// 0x142828963  44899590000000                 movl	%r10d, 0x90(%rbp)
// 0x14282896a  4d8b39                         movq	(%r9), %r15
// 0x14282896d  4d8b4910                       movq	0x10(%r9), %r9
// 0x142828971  4c894dd8                       movq	%r9, -0x28(%rbp)
// 0x142828975  0f2835b4826d00                 movaps	0x6d82b4(%rip), %xmm6   # 0x142f00c30
// 0x14282897c  f30f103da0c06d00               movss	0x6dc0a0(%rip), %xmm7   # 0x142f04a24
// 0x142828984  f3440f10057fa57c00             movss	0x7ca57f(%rip), %xmm8   # 0x142ff2f0c
// 0x14282898d  f3440f100d6a897c00             movss	0x7c896a(%rip), %xmm9   # 0x142ff1300
// 0x142828996  488995b8000000                 movq	%rdx, 0xb8(%rbp)
// 0x14282899d  488d5220                       leaq	0x20(%rdx), %rdx
// 0x1428289a1  48895528                       movq	%rdx, 0x28(%rbp)
// 0x1428289a5  4c8985c0000000                 movq	%r8, 0xc0(%rbp)
// 0x1428289ac  498d5020                       leaq	0x20(%r8), %rdx
// 0x1428289b0  48895538                       movq	%rdx, 0x38(%rbp)
// 0x1428289b4  498d5720                       leaq	0x20(%r15), %rdx
// 0x1428289b8  488955e0                       movq	%rdx, -0x20(%rbp)
// 0x1428289bc  4c8975c8                       movq	%r14, -0x38(%rbp)
// 0x1428289c0  4c896df0                       movq	%r13, -0x10(%rbp)
// 0x1428289c4  4c897dd0                       movq	%r15, -0x30(%rbp)
// 0x1428289c8  eb31                           jmp	0x1428289fb <.text+0x28279fb>
// 0x1428289ca  660f1f440000                   nopw	(%rax,%rax)
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F32 -> F33 -> F102
// 0x1428289d0  498b4728                       movq	0x28(%r15), %rax
// Frames: F35 -> F29 -> F30 -> F31 -> F32 -> F33 -> F102
// 0x1428289d4  4889f9                         movq	%rdi, %rcx
// 0x1428289d7  48c1e104                       shlq	$0x4, %rcx
// Frames: F36 -> F37 -> F30 -> F31 -> F32 -> F33 -> F102
// 0x1428289db  4c893408                       movq	%r14, (%rax,%rcx)
// 0x1428289df  48895c0808                     movq	%rbx, 0x8(%rax,%rcx)
// Frames: F38 -> F30 -> F31 -> F32 -> F33 -> F102
// 0x1428289e4  48ffc7                         incq	%rdi
// 0x1428289e7  49897f30                       movq	%rdi, 0x30(%r15)
// Frames: F39 -> F32 -> F33 -> F102
// 0x1428289eb  49ff4740                       incq	0x40(%r15)
// 0x1428289ef  488b45b8                       movq	-0x48(%rbp), %rax
// 0x1428289f3  488b4db0                       movq	-0x50(%rbp), %rcx
// 0x1428289f7  4c8b75c8                       movq	-0x38(%rbp), %r14
// Frames: F40 -> F41 -> F42 -> F43 -> F44 -> F101
// 0x1428289fb  4885c0                         testq	%rax, %rax
// 0x1428289fe  7410                           je	0x142828a10 <.text+0x2827a10>
// 0x142828a00  483b4508                       cmpq	0x8(%rbp), %rax
// 0x142828a04  740a                           je	0x142828a10 <.text+0x2827a10>
// 0x142828a06  4c8d5820                       leaq	0x20(%rax), %r11
// 0x142828a0a  4989ca                         movq	%rcx, %r10
// 0x142828a0d  eb1e                           jmp	0x142828a2d <.text+0x2827a2d>
// 0x142828a0f  90                             nop
// Frames: F40 -> F45 -> F46 -> F42 -> F43 -> F44 -> F101
// 0x142828a10  4885c9                         testq	%rcx, %rcx
// 0x142828a13  0f848a060000                   je	0x1428290a3 <.text+0x28280a3>
// 0x142828a19  483b4d20                       cmpq	0x20(%rbp), %rcx
// 0x142828a1d  0f8480060000                   je	0x1428290a3 <.text+0x28280a3>
// Frames: F47 -> F48 -> F45 -> F46 -> F42 -> F43 -> F44 -> F101
// 0x142828a23  4c8d5120                       leaq	0x20(%rcx), %r10
// 0x142828a27  4531db                         xorl	%r11d, %r11d
// 0x142828a2a  4889c8                         movq	%rcx, %rax
// Frames: F49 -> F44 -> F101
// 0x142828a2d  488b4dc0                       movq	-0x40(%rbp), %rcx
// 0x142828a31  48ff01                         incq	(%rcx)
// Frames: F103
// 0x142828a34  488b4810                       movq	0x10(%rax), %rcx
// 0x142828a38  89ca                           movl	%ecx, %edx
// 0x142828a3a  f7d2                           notl	%edx
// 0x142828a3c  48898db0000000                 movq	%rcx, 0xb0(%rbp)
// Frames: F104
// 0x142828a43  0fb67819                       movzbl	0x19(%rax), %edi
// 0x142828a47  4088bdd7000000                 movb	%dil, 0xd7(%rbp)
// 0x142828a4e  41b901000000                   movl	$0x1, %r9d
// Frames: F105 -> F15 -> F106 -> F107 -> F108 -> F109 -> F110
// 0x142828a54  493b5510                       cmpq	0x10(%r13), %rdx
// Frames: F111 -> F112 -> F107 -> F108 -> F109 -> F110
// 0x142828a58  0f8398060000                   jae	0x1428290f6 <.text+0x28280f6>
// Frames: F113 -> F114 -> F115 -> F112 -> F107 -> F108 -> F109 -> F110
// 0x142828a5e  4c8d0452                       leaq	(%rdx,%rdx,2), %r8
// 0x142828a62  49c1e003                       shlq	$0x3, %r8
// 0x142828a66  4d034508                       addq	0x8(%r13), %r8
// 0x142828a6a  4889ca                         movq	%rcx, %rdx
// 0x142828a6d  48c1ea20                       shrq	$0x20, %rdx
// 0x142828a71  41395010                       cmpl	%edx, 0x10(%r8)
// Frames: F116 -> F112 -> F107 -> F108 -> F109 -> F110
// 0x142828a75  0f857b060000                   jne	0x1428290f6 <.text+0x28280f6>
// Frames: F117 -> F118 -> F109 -> F110
// 0x142828a7b  41833800                       cmpl	$0x0, (%r8)
// 0x142828a7f  0f8476060000                   je	0x1428290fb <.text+0x28280fb>
// Frames: F119 -> F120 -> F109 -> F110
// 0x142828a85  4c895db8                       movq	%r11, -0x48(%rbp)
// 0x142828a89  4c8955b0                       movq	%r10, -0x50(%rbp)
// 0x142828a8d  498b5008                       movq	0x8(%r8), %rdx
// 0x142828a91  4189d1                         movl	%edx, %r9d
// Frames: F121 -> F120 -> F109 -> F110
// 0x142828a94  4d3b4e38                       cmpq	0x38(%r14), %r9
// Frames: F122 -> F123 -> F120 -> F109 -> F110
// 0x142828a98  0f835a060000                   jae	0x1428290f8 <.text+0x28280f8>
// Frames: F124 -> F125 -> F126 -> F122 -> F123 -> F120 -> F109 -> F110
// 0x142828a9e  4d8b5628                       movq	0x28(%r14), %r10
// Frames: F21 -> F124 -> F125 -> F126 -> F122 -> F123 -> F120 -> F109 -> F110
// 0x142828aa2  41c1e906                       shrl	$0x6, %r9d
// Frames: F127 -> F125 -> F126 -> F122 -> F123 -> F120 -> F109 -> F110
// 0x142828aa6  4f8b0cca                       movq	(%r10,%r9,8), %r9
// Frames: F125 -> F126 -> F122 -> F123 -> F120 -> F109 -> F110
// 0x142828aaa  490fa3d1                       btq	%rdx, %r9
// Frames: F128 -> F109 -> F110
// 0x142828aae  0f8344060000                   jae	0x1428290f8 <.text+0x28280f8>
// Frames: F25 -> F26 -> F27 -> F129 -> F17 -> F18 -> F130 -> F131 -> F109 -> F110
// 0x142828ab4  4c8b6008                       movq	0x8(%rax), %r12
// 0x142828ab8  0fb65818                       movzbl	0x18(%rax), %ebx
// 0x142828abc  418b4004                       movl	0x4(%r8), %eax
// 0x142828ac0  48c1ea20                       shrq	$0x20, %rdx
// 0x142828ac4  498b8da0010000                 movq	0x1a0(%r13), %rcx
// Frames: F132 -> F133 -> F130 -> F131 -> F109 -> F110
// 0x142828acb  488d14d2                       leaq	(%rdx,%rdx,8), %rdx
// Frames: F134 -> F109 -> F110
// 0x142828acf  4d8b8610010000                 movq	0x110(%r14), %r8
// Frames: F135 -> F136 -> F137 -> F138 -> F139 -> F134 -> F109 -> F110
// 0x142828ad6  4c8b4cd118                     movq	0x18(%rcx,%rdx,8), %r9
// 0x142828adb  488b4cd138                     movq	0x38(%rcx,%rdx,8), %rcx
// Frames: F140 -> F141 -> F142 -> F143 -> F135 -> F136 -> F137 -> F138 -> F139 -> F134 -> F109 -> F110
// 0x142828ae0  4a8b0cc1                       movq	(%rcx,%r8,8), %rcx
// Frames: F144 -> F145 -> F56 -> F135 -> F136 -> F137 -> F138 -> F139 -> F134 -> F109 -> F110
// 0x142828ae4  48f7d1                         notq	%rcx
// Frames: F146 -> F147 -> F145 -> F56 -> F135 -> F136 -> F137 -> F138 -> F139 -> F134 -> F109 -> F110
// 0x142828ae7  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x142828aeb  48c1e104                       shlq	$0x4, %rcx
// Frames: F148 -> F149 -> F150 -> F151 -> F152 -> F153 -> F109 -> F110
// 0x142828aef  f7d0                           notl	%eax
// Frames: F21 -> F154 -> F150 -> F151 -> F152 -> F153 -> F109 -> F110
// 0x142828af1  4c6bf858                       imulq	$0x58, %rax, %r15
// 0x142828af5  4d037c0910                     addq	0x10(%r9,%rcx), %r15
// Frames: F155
// 0x142828afa  4c89f9                         movq	%r15, %rcx
// 0x142828afd  e8ce013cfe                     callq	0x140be8cd0 <.text+0xbe7cd0>
// 0x142828b02  884540                         movb	%al, 0x40(%rbp)
// Frames: F156 -> F155
// 0x142828b05  4038c7                         cmpb	%al, %dil
// Frames: F155
// 0x142828b08  0f85c7050000                   jne	0x1428290d5 <.text+0x28280d5>
// Frames: F157
// 0x142828b0e  4c8badb0000000                 movq	0xb0(%rbp), %r13
// 0x142828b15  4084ff                         testb	%dil, %dil
// 0x142828b18  0f8482020000                   je	0x142828da0 <.text+0x2827da0>
// Frames: F158 -> F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F165 -> F166 -> F167
// 0x142828b1e  488b06                         movq	(%rsi), %rax
// 0x142828b21  488b7e10                       movq	0x10(%rsi), %rdi
// Frames: F168 -> F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F165 -> F166 -> F167
// 0x142828b25  4829f8                         subq	%rdi, %rax
// Frames: F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F165 -> F166 -> F167
// 0x142828b28  4883f817                       cmpq	$0x17, %rax
// 0x142828b2c  4c8d7570                       leaq	0x70(%rbp), %r14
// Frames: F160 -> F161 -> F162 -> F163 -> F164 -> F165 -> F166 -> F167
// 0x142828b30  0f86fe040000                   jbe	0x142829034 <.text+0x2828034>
// Frames: F169
// 0x142828b36  488b4608                       movq	0x8(%rsi), %rax
// 0x142828b3a  488d0d7f3e79fe                 leaq	-0x186c181(%rip), %rcx  # 0x140fbc9c0 <.text+0xfbb9c0>
// 0x142828b41  48890c38                       movq	%rcx, (%rax,%rdi)
// 0x142828b45  4c896c3808                     movq	%r13, 0x8(%rax,%rdi)
// 0x142828b4a  488d0d2f4179fe                 leaq	-0x186bed1(%rip), %rcx  # 0x140fbcc80 <.text+0xfbbc80>
// 0x142828b51  48894c3810                     movq	%rcx, 0x10(%rax,%rdi)
// 0x142828b56  4883c718                       addq	$0x18, %rdi
// 0x142828b5a  48897e10                       movq	%rdi, 0x10(%rsi)
// 0x142828b5e  488d85b0000000                 leaq	0xb0(%rbp), %rax
// 0x142828b65  488985a0000000                 movq	%rax, 0xa0(%rbp)
// 0x142828b6c  488d052d2c6afe                 leaq	-0x195d3d3(%rip), %rax  # 0x140ecb7a0 <.text+0xeca7a0>
// 0x142828b73  488985a8000000                 movq	%rax, 0xa8(%rbp)
// Frames: F170 -> F171 -> F172 -> F169
// 0x142828b7a  488d4d40                       leaq	0x40(%rbp), %rcx
// 0x142828b7e  488d153b00d600                 leaq	0xd6003b(%rip), %rdx    # 0x143588bc0
// 0x142828b85  4c8d85a0000000                 leaq	0xa0(%rbp), %r8
// 0x142828b8c  e85fe772fe                     callq	0x140f572f0 <.text+0xf562f0>
// Frames: F173 -> F169
// 0x142828b91  488b4540                       movq	0x40(%rbp), %rax
// 0x142828b95  48898598000000                 movq	%rax, 0x98(%rbp)
// 0x142828b9c  488b5548                       movq	0x48(%rbp), %rdx
// 0x142828ba0  4c8b4550                       movq	0x50(%rbp), %r8
// Frames: F169
// 0x142828ba4  488b4d10                       movq	0x10(%rbp), %rcx
// 0x142828ba8  48895500                       movq	%rdx, (%rbp)
// 0x142828bac  e86f9f1efe                     callq	0x140a12b20 <.text+0xa11b20>
// 0x142828bb1  90                             nop
// Frames: F174
// 0x142828bb2  4883f801                       cmpq	$0x1, %rax
// 0x142828bb6  751d                           jne	0x142828bd5 <.text+0x2827bd5>
// Frames: F175
// 0x142828bb8  4889d7                         movq	%rdx, %rdi
// 0x142828bbb  488b4d10                       movq	0x10(%rbp), %rcx
// 0x142828bbf  e89c130dfe                     callq	0x1408f9f60 <.text+0x8f8f60>
// 0x142828bc4  90                             nop
// 0x142828bc5  488b08                         movq	(%rax), %rcx
// Frames: F176 -> F175
// 0x142828bc8  4883c110                       addq	$0x10, %rcx
// Frames: F175
// 0x142828bcc  4889fa                         movq	%rdi, %rdx
// 0x142828bcf  e88ce713fe                     callq	0x140967360 <.text+0x966360>
// 0x142828bd4  90                             nop
// Frames: F177 -> F178 -> F179 -> F98 -> F98 -> F98 -> F180
// 0x142828bd5  488b9598000000                 movq	0x98(%rbp), %rdx
// 0x142828bdc  4885d2                         testq	%rdx, %rdx
// 0x142828bdf  4c8b6df0                       movq	-0x10(%rbp), %r13
// 0x142828be3  740f                           je	0x142828bf4 <.text+0x2827bf4>
// Frames: F181 -> F182 -> F183 -> F184 -> F185 -> F179 -> F98 -> F98 -> F98 -> F180
// 0x142828be5  41b801000000                   movl	$0x1, %r8d
// 0x142828beb  488b4d00                       movq	(%rbp), %rcx
// 0x142828bef  e8fccce7fd                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F186
// 0x142828bf4  488d8da0000000                 leaq	0xa0(%rbp), %rcx
// 0x142828bfb  4c89fa                         movq	%r15, %rdx
// 0x142828bfe  e8cd023cfe                     callq	0x140be8ed0 <.text+0xbe7ed0>
// Frames: F187
// 0x142828c03  8b85a8000000                   movl	0xa8(%rbp), %eax
// 0x142828c09  418946e7                       movl	%eax, -0x19(%r14)
// 0x142828c0d  488b85a0000000                 movq	0xa0(%rbp), %rax
// 0x142828c14  498946df                       movq	%rax, -0x21(%r14)
// 0x142828c18  8b85cc000000                   movl	0xcc(%rbp), %eax
// 0x142828c1e  488b4df8                       movq	-0x8(%rbp), %rcx
// Frames: F68 -> F69 -> F33 -> F187
// 0x142828c22  8901                           movl	%eax, (%rcx)
// 0x142828c24  488b85b8000000                 movq	0xb8(%rbp), %rax
// Frames: F71 -> F30 -> F31 -> F72 -> F33 -> F187
// 0x142828c2b  488b7830                       movq	0x30(%rax), %rdi
// Frames: F73 -> F72 -> F33 -> F187
// 0x142828c2f  488b5840                       movq	0x40(%rax), %rbx
// Frames: F74 -> F30 -> F31 -> F72 -> F33 -> F187
// 0x142828c33  483b7820                       cmpq	0x20(%rax), %rdi
// 0x142828c37  7509                           jne	0x142828c42 <.text+0x2827c42>
// Frames: F75 -> F30 -> F31 -> F72 -> F33 -> F187
// 0x142828c39  488b4d28                       movq	0x28(%rbp), %rcx
// 0x142828c3d  e8de665400                     callq	0x142d6f320 <.text+0x2d6e320>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F72 -> F33 -> F187
// 0x142828c42  488b95b8000000                 movq	0xb8(%rbp), %rdx
// 0x142828c49  488b4228                       movq	0x28(%rdx), %rax
// Frames: F35 -> F29 -> F30 -> F31 -> F72 -> F33 -> F187
// 0x142828c4d  488d0cbf                       leaq	(%rdi,%rdi,4), %rcx
// Frames: F36 -> F37 -> F30 -> F31 -> F72 -> F33 -> F187
// 0x142828c51  48891cc8                       movq	%rbx, (%rax,%rcx,8)
// 0x142828c55  c644c80800                     movb	$0x0, 0x8(%rax,%rcx,8)
// 0x142828c5a  0f104540                       movups	0x40(%rbp), %xmm0
// 0x142828c5e  0f104d4b                       movups	0x4b(%rbp), %xmm1
// 0x142828c62  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// 0x142828c67  0f1144c809                     movups	%xmm0, 0x9(%rax,%rcx,8)
// Frames: F38 -> F30 -> F31 -> F72 -> F33 -> F187
// 0x142828c6c  48ffc7                         incq	%rdi
// 0x142828c6f  48897a30                       movq	%rdi, 0x30(%rdx)
// Frames: F39 -> F72 -> F33 -> F187
// 0x142828c73  48ff4240                       incq	0x40(%rdx)
// Frames: F188
// 0x142828c77  488b9db0000000                 movq	0xb0(%rbp), %rbx
// Frames: F80 -> F188
// 0x142828c7e  31c9                           xorl	%ecx, %ecx
// 0x142828c80  e8db270dfe                     callq	0x1408fb460 <.text+0x8fa460>
// Frames: F82 -> F80 -> F188
// 0x142828c85  488d4d70                       leaq	0x70(%rbp), %rcx
// 0x142828c89  89c2                           movl	%eax, %edx
// 0x142828c8b  e890ee0cfe                     callq	0x1408f7b20 <.text+0x8f6b20>
// Frames: F83 -> F84 -> F188
// 0x142828c90  41c746e800000000               movl	$0x0, -0x18(%r14)
// 0x142828c98  49c746e000000000               movq	$0x0, -0x20(%r14)
// Frames: F85 -> F188
// 0x142828ca0  e86ba67dfd                     callq	0x140003310 <.text+0x2310>
// Frames: F86 -> F188
// 0x142828ca5  c6858000000007                 movb	$0x7, 0x80(%rbp)
// 0x142828cac  0f297540                       movaps	%xmm6, 0x40(%rbp)
// 0x142828cb0  48b80000803f0000803f           movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
// 0x142828cba  498946ec                       movq	%rax, -0x14(%r14)
// 0x142828cbe  41c746f40000803f               movl	$0x3f800000, -0xc(%r14) # imm = 0x3F800000
// Frames: F158 -> F159 -> F160 -> F161 -> F162 -> F163 -> F189 -> F190 -> F191 -> F188
// 0x142828cc6  488b06                         movq	(%rsi), %rax
// 0x142828cc9  488b7e10                       movq	0x10(%rsi), %rdi
// Frames: F168 -> F159 -> F160 -> F161 -> F162 -> F163 -> F189 -> F190 -> F191 -> F188
// 0x142828ccd  4829f8                         subq	%rdi, %rax
// Frames: F159 -> F160 -> F161 -> F162 -> F163 -> F189 -> F190 -> F191 -> F188
// 0x142828cd0  4883f877                       cmpq	$0x77, %rax
// Frames: F160 -> F161 -> F162 -> F163 -> F189 -> F190 -> F191 -> F188
// 0x142828cd4  0f86a4030000                   jbe	0x14282907e <.text+0x282807e>
// Frames: F192
// 0x142828cda  488b4608                       movq	0x8(%rsi), %rax
// 0x142828cde  488d0dfb3a79fe                 leaq	-0x186c505(%rip), %rcx  # 0x140fbc7e0 <.text+0xfbb7e0>
// 0x142828ce5  48890c38                       movq	%rcx, (%rax,%rdi)
// 0x142828ce9  0f284540                       movaps	0x40(%rbp), %xmm0
// 0x142828ced  0f284d50                       movaps	0x50(%rbp), %xmm1
// 0x142828cf1  0f285560                       movaps	0x60(%rbp), %xmm2
// 0x142828cf5  0f285d70                       movaps	0x70(%rbp), %xmm3
// 0x142828cf9  0f11443808                     movups	%xmm0, 0x8(%rax,%rdi)
// 0x142828cfe  0f114c3818                     movups	%xmm1, 0x18(%rax,%rdi)
// 0x142828d03  0f11543828                     movups	%xmm2, 0x28(%rax,%rdi)
// 0x142828d08  0f115c3838                     movups	%xmm3, 0x38(%rax,%rdi)
// 0x142828d0d  0f288580000000                 movaps	0x80(%rbp), %xmm0
// 0x142828d14  0f11443848                     movups	%xmm0, 0x48(%rax,%rdi)
// 0x142828d19  c644385800                     movb	$0x0, 0x58(%rax,%rdi)
// 0x142828d1e  48895c3868                     movq	%rbx, 0x68(%rax,%rdi)
// 0x142828d23  4883c778                       addq	$0x78, %rdi
// 0x142828d27  48897e10                       movq	%rdi, 0x10(%rsi)
// 0x142828d2b  f3410f104734                   movss	0x34(%r15), %xmm0
// 0x142828d31  0f2ec7                         ucomiss	%xmm7, %xmm0
// 0x142828d34  0f8566020000                   jne	0x142828fa0 <.text+0x2827fa0>
// 0x142828d3a  0f8a60020000                   jp	0x142828fa0 <.text+0x2827fa0>
// Frames: F193
// 0x142828d40  498b1f                         movq	(%r15), %rbx
// 0x142828d43  8b85d0000000                   movl	0xd0(%rbp), %eax
// 0x142828d49  488b4d18                       movq	0x18(%rbp), %rcx
// Frames: F68 -> F69 -> F33 -> F194
// 0x142828d4d  8901                           movl	%eax, (%rcx)
// 0x142828d4f  488b85c0000000                 movq	0xc0(%rbp), %rax
// Frames: F71 -> F30 -> F31 -> F72 -> F33 -> F194
// 0x142828d56  488b7830                       movq	0x30(%rax), %rdi
// Frames: F73 -> F72 -> F33 -> F194
// 0x142828d5a  4c8b7040                       movq	0x40(%rax), %r14
// Frames: F74 -> F30 -> F31 -> F72 -> F33 -> F194
// 0x142828d5e  483b7820                       cmpq	0x20(%rax), %rdi
// 0x142828d62  7509                           jne	0x142828d6d <.text+0x2827d6d>
// Frames: F75 -> F30 -> F31 -> F72 -> F33 -> F194
// 0x142828d64  488b4d38                       movq	0x38(%rbp), %rcx
// 0x142828d68  e8e3675400                     callq	0x142d6f550 <.text+0x2d6e550>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F72 -> F33 -> F194
// 0x142828d6d  488b95c0000000                 movq	0xc0(%rbp), %rdx
// 0x142828d74  488b4228                       movq	0x28(%rdx), %rax
// Frames: F35 -> F29 -> F30 -> F31 -> F72 -> F33 -> F194
// 0x142828d78  4889f9                         movq	%rdi, %rcx
// 0x142828d7b  48c1e104                       shlq	$0x4, %rcx
// Frames: F36 -> F37 -> F30 -> F31 -> F72 -> F33 -> F194
// 0x142828d7f  4c893408                       movq	%r14, (%rax,%rcx)
// 0x142828d83  48895c0808                     movq	%rbx, 0x8(%rax,%rcx)
// Frames: F38 -> F30 -> F31 -> F72 -> F33 -> F194
// 0x142828d88  48ffc7                         incq	%rdi
// 0x142828d8b  48897a30                       movq	%rdi, 0x30(%rdx)
// Frames: F39 -> F72 -> F33 -> F194
// 0x142828d8f  48ff4240                       incq	0x40(%rdx)
// Frames: F192
// 0x142828d93  e908020000                     jmp	0x142828fa0 <.text+0x2827fa0>
// 0x142828d98  0f1f840000000000               nopl	(%rax,%rax)
// Frames: F80 -> F195
// 0x142828da0  31c9                           xorl	%ecx, %ecx
// 0x142828da2  e8b9260dfe                     callq	0x1408fb460 <.text+0x8fa460>
// 0x142828da7  4c8d7570                       leaq	0x70(%rbp), %r14
// Frames: F82 -> F80 -> F195
// 0x142828dab  4c89f1                         movq	%r14, %rcx
// 0x142828dae  89c2                           movl	%eax, %edx
// 0x142828db0  e86bed0cfe                     callq	0x1408f7b20 <.text+0x8f6b20>
// Frames: F83 -> F84 -> F195
// 0x142828db5  41c746e800000000               movl	$0x0, -0x18(%r14)
// 0x142828dbd  49c746e000000000               movq	$0x0, -0x20(%r14)
// Frames: F85 -> F195
// 0x142828dc5  e846a57dfd                     callq	0x140003310 <.text+0x2310>
// Frames: F86 -> F195
// 0x142828dca  c6858000000007                 movb	$0x7, 0x80(%rbp)
// 0x142828dd1  0f297540                       movaps	%xmm6, 0x40(%rbp)
// 0x142828dd5  48b80000803f0000803f           movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
// 0x142828ddf  498946ec                       movq	%rax, -0x14(%r14)
// 0x142828de3  41c746f40000803f               movl	$0x3f800000, -0xc(%r14) # imm = 0x3F800000
// Frames: F158 -> F159 -> F160 -> F161 -> F162 -> F163 -> F189 -> F190 -> F191 -> F195
// 0x142828deb  488b06                         movq	(%rsi), %rax
// 0x142828dee  488b7e10                       movq	0x10(%rsi), %rdi
// Frames: F168 -> F159 -> F160 -> F161 -> F162 -> F163 -> F189 -> F190 -> F191 -> F195
// 0x142828df2  4829f8                         subq	%rdi, %rax
// Frames: F159 -> F160 -> F161 -> F162 -> F163 -> F189 -> F190 -> F191 -> F195
// 0x142828df5  4883f877                       cmpq	$0x77, %rax
// Frames: F160 -> F161 -> F162 -> F163 -> F189 -> F190 -> F191 -> F195
// 0x142828df9  0f865a020000                   jbe	0x142829059 <.text+0x2828059>
// Frames: F196
// 0x142828dff  488b4608                       movq	0x8(%rsi), %rax
// 0x142828e03  488d0dd63979fe                 leaq	-0x186c62a(%rip), %rcx  # 0x140fbc7e0 <.text+0xfbb7e0>
// 0x142828e0a  48890c38                       movq	%rcx, (%rax,%rdi)
// 0x142828e0e  0f284540                       movaps	0x40(%rbp), %xmm0
// 0x142828e12  0f284d50                       movaps	0x50(%rbp), %xmm1
// 0x142828e16  0f285560                       movaps	0x60(%rbp), %xmm2
// 0x142828e1a  0f285d70                       movaps	0x70(%rbp), %xmm3
// 0x142828e1e  0f11443808                     movups	%xmm0, 0x8(%rax,%rdi)
// 0x142828e23  0f114c3818                     movups	%xmm1, 0x18(%rax,%rdi)
// 0x142828e28  0f11543828                     movups	%xmm2, 0x28(%rax,%rdi)
// 0x142828e2d  0f115c3838                     movups	%xmm3, 0x38(%rax,%rdi)
// 0x142828e32  0f288580000000                 movaps	0x80(%rbp), %xmm0
// 0x142828e39  0f11443848                     movups	%xmm0, 0x48(%rax,%rdi)
// 0x142828e3e  c644385800                     movb	$0x0, 0x58(%rax,%rdi)
// 0x142828e43  4c896c3868                     movq	%r13, 0x68(%rax,%rdi)
// 0x142828e48  4883c778                       addq	$0x78, %rdi
// 0x142828e4c  48897e10                       movq	%rdi, 0x10(%rsi)
// 0x142828e50  410fb67f54                     movzbl	0x54(%r15), %edi
// Frames: F197
// 0x142828e55  488d8da0000000                 leaq	0xa0(%rbp), %rcx
// 0x142828e5c  4c89fa                         movq	%r15, %rdx
// 0x142828e5f  e86c003cfe                     callq	0x140be8ed0 <.text+0xbe7ed0>
// Frames: F198
// 0x142828e64  8b85a8000000                   movl	0xa8(%rbp), %eax
// 0x142828e6a  418946e6                       movl	%eax, -0x1a(%r14)
// 0x142828e6e  488b85a0000000                 movq	0xa0(%rbp), %rax
// 0x142828e75  498946de                       movq	%rax, -0x22(%r14)
// 0x142828e79  8b85cc000000                   movl	0xcc(%rbp), %eax
// 0x142828e7f  488b4df8                       movq	-0x8(%rbp), %rcx
// Frames: F68 -> F69 -> F33 -> F198
// 0x142828e83  8901                           movl	%eax, (%rcx)
// 0x142828e85  488b85b8000000                 movq	0xb8(%rbp), %rax
// Frames: F71 -> F30 -> F31 -> F72 -> F33 -> F198
// 0x142828e8c  4c8b6830                       movq	0x30(%rax), %r13
// Frames: F73 -> F72 -> F33 -> F198
// 0x142828e90  4c8b7040                       movq	0x40(%rax), %r14
// Frames: F74 -> F30 -> F31 -> F72 -> F33 -> F198
// 0x142828e94  4c3b6820                       cmpq	0x20(%rax), %r13
// 0x142828e98  7509                           jne	0x142828ea3 <.text+0x2827ea3>
// Frames: F75 -> F30 -> F31 -> F72 -> F33 -> F198
// 0x142828e9a  488b4d28                       movq	0x28(%rbp), %rcx
// 0x142828e9e  e87d645400                     callq	0x142d6f320 <.text+0x2d6e320>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F72 -> F33 -> F198
// 0x142828ea3  488b95b8000000                 movq	0xb8(%rbp), %rdx
// 0x142828eaa  488b4228                       movq	0x28(%rdx), %rax
// Frames: F35 -> F29 -> F30 -> F31 -> F72 -> F33 -> F198
// 0x142828eae  4a8d0cad00000000               leaq	(,%r13,4), %rcx
// 0x142828eb6  4c01e9                         addq	%r13, %rcx
// Frames: F36 -> F37 -> F30 -> F31 -> F72 -> F33 -> F198
// 0x142828eb9  4c8934c8                       movq	%r14, (%rax,%rcx,8)
// 0x142828ebd  c644c80806                     movb	$0x6, 0x8(%rax,%rcx,8)
// 0x142828ec2  40887cc809                     movb	%dil, 0x9(%rax,%rcx,8)
// 0x142828ec7  0f104540                       movups	0x40(%rbp), %xmm0
// 0x142828ecb  0f104d4a                       movups	0x4a(%rbp), %xmm1
// 0x142828ecf  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// 0x142828ed4  0f1144c80a                     movups	%xmm0, 0xa(%rax,%rcx,8)
// Frames: F38 -> F30 -> F31 -> F72 -> F33 -> F198
// 0x142828ed9  49ffc5                         incq	%r13
// 0x142828edc  4c896a30                       movq	%r13, 0x30(%rdx)
// Frames: F39 -> F72 -> F33 -> F198
// 0x142828ee0  48ff4240                       incq	0x40(%rdx)
// Frames: F199
// 0x142828ee4  f3410f104734                   movss	0x34(%r15), %xmm0
// 0x142828eea  0f2ec7                         ucomiss	%xmm7, %xmm0
// 0x142828eed  7555                           jne	0x142828f44 <.text+0x2827f44>
// 0x142828eef  7a53                           jp	0x142828f44 <.text+0x2827f44>
// Frames: F200
// 0x142828ef1  4d8b2f                         movq	(%r15), %r13
// 0x142828ef4  8b85d0000000                   movl	0xd0(%rbp), %eax
// 0x142828efa  488b4d18                       movq	0x18(%rbp), %rcx
// Frames: F68 -> F69 -> F33 -> F201
// 0x142828efe  8901                           movl	%eax, (%rcx)
// 0x142828f00  488b85c0000000                 movq	0xc0(%rbp), %rax
// Frames: F71 -> F30 -> F31 -> F72 -> F33 -> F201
// 0x142828f07  488b7830                       movq	0x30(%rax), %rdi
// Frames: F73 -> F72 -> F33 -> F201
// 0x142828f0b  4c8b7040                       movq	0x40(%rax), %r14
// Frames: F74 -> F30 -> F31 -> F72 -> F33 -> F201
// 0x142828f0f  483b7820                       cmpq	0x20(%rax), %rdi
// 0x142828f13  7509                           jne	0x142828f1e <.text+0x2827f1e>
// Frames: F75 -> F30 -> F31 -> F72 -> F33 -> F201
// 0x142828f15  488b4d38                       movq	0x38(%rbp), %rcx
// 0x142828f19  e832665400                     callq	0x142d6f550 <.text+0x2d6e550>
// Frames: F25 -> F26 -> F27 -> F28 -> F29 -> F30 -> F31 -> F72 -> F33 -> F201
// 0x142828f1e  488b95c0000000                 movq	0xc0(%rbp), %rdx
// 0x142828f25  488b4228                       movq	0x28(%rdx), %rax
// Frames: F35 -> F29 -> F30 -> F31 -> F72 -> F33 -> F201
// 0x142828f29  4889f9                         movq	%rdi, %rcx
// 0x142828f2c  48c1e104                       shlq	$0x4, %rcx
// Frames: F36 -> F37 -> F30 -> F31 -> F72 -> F33 -> F201
// 0x142828f30  4c893408                       movq	%r14, (%rax,%rcx)
// 0x142828f34  4c896c0808                     movq	%r13, 0x8(%rax,%rcx)
// Frames: F38 -> F30 -> F31 -> F72 -> F33 -> F201
// 0x142828f39  48ffc7                         incq	%rdi
// 0x142828f3c  48897a30                       movq	%rdi, 0x30(%rdx)
// Frames: F39 -> F72 -> F33 -> F201
// 0x142828f40  48ff4240                       incq	0x40(%rdx)
// Frames: F202
// 0x142828f44  80fb01                         cmpb	$0x1, %bl
// 0x142828f47  4c8b6df0                       movq	-0x10(%rbp), %r13
// 0x142828f4b  7553                           jne	0x142828fa0 <.text+0x2827fa0>
// Frames: F203
// 0x142828f4d  488b4d30                       movq	0x30(%rbp), %rcx
// 0x142828f51  4c89e2                         movq	%r12, %rdx
// 0x142828f54  4c8d054dfcd500                 leaq	0xd5fc4d(%rip), %r8     # 0x143588ba8
// 0x142828f5b  e8a0983dfe                     callq	0x140c02800 <.text+0xc01800>
// 0x142828f60  f30f104054                     movss	0x54(%rax), %xmm0
// Frames: F204
// 0x142828f65  410f2ec0                       ucomiss	%xmm8, %xmm0
// 0x142828f69  7502                           jne	0x142828f6d <.text+0x2827f6d>
// 0x142828f6b  7b33                           jnp	0x142828fa0 <.text+0x2827fa0>
// Frames: F205 -> F206
// 0x142828f6d  410f28c8                       movaps	%xmm8, %xmm1
// 0x142828f71  f30f5fc8                       maxss	%xmm0, %xmm1
// Frames: F207 -> F206
// 0x142828f75  410f28d1                       movaps	%xmm9, %xmm2
// 0x142828f79  f30f5dd1                       minss	%xmm1, %xmm2
// 0x142828f7d  488b45e8                       movq	-0x18(%rbp), %rax
// 0x142828f81  8b8d94000000                   movl	0x94(%rbp), %ecx
// Frames: F68 -> F69 -> F208
// 0x142828f87  8908                           movl	%ecx, (%rax)
// 0x142828f89  488b4d30                       movq	0x30(%rbp), %rcx
// Frames: F208
// 0x142828f8d  4c89e2                         movq	%r12, %rdx
// 0x142828f90  e83bae3dfe                     callq	0x140c03dd0 <.text+0xc02dd0>
// 0x142828f95  66662e0f1f840000000000         nopw	%cs:(%rax,%rax)
// Frames: F209
// 0x142828fa0  488b9db0000000                 movq	0xb0(%rbp), %rbx
// Frames: F158 -> F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F165 -> F166 -> F209
// 0x142828fa7  488b06                         movq	(%rsi), %rax
// 0x142828faa  488b7e10                       movq	0x10(%rsi), %rdi
// Frames: F168 -> F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F165 -> F166 -> F209
// 0x142828fae  4829f8                         subq	%rdi, %rax
// Frames: F159 -> F160 -> F161 -> F162 -> F163 -> F164 -> F165 -> F166 -> F209
// 0x142828fb1  4883f817                       cmpq	$0x17, %rax
// Frames: F160 -> F161 -> F162 -> F163 -> F164 -> F165 -> F166 -> F209
// 0x142828fb5  765b                           jbe	0x142829012 <.text+0x2828012>
// Frames: F210
// 0x142828fb7  488b4608                       movq	0x8(%rsi), %rax
// 0x142828fbb  488d0d6e3b79fe                 leaq	-0x186c492(%rip), %rcx  # 0x140fbcb30 <.text+0xfbbb30>
// 0x142828fc2  48890c38                       movq	%rcx, (%rax,%rdi)
// 0x142828fc6  48895c3808                     movq	%rbx, 0x8(%rax,%rdi)
// 0x142828fcb  488d0dae3c79fe                 leaq	-0x186c352(%rip), %rcx  # 0x140fbcc80 <.text+0xfbbc80>
// 0x142828fd2  48894c3810                     movq	%rcx, 0x10(%rax,%rdi)
// 0x142828fd7  4883c718                       addq	$0x18, %rdi
// 0x142828fdb  48897e10                       movq	%rdi, 0x10(%rsi)
// 0x142828fdf  498b1f                         movq	(%r15), %rbx
// 0x142828fe2  8b8590000000                   movl	0x90(%rbp), %eax
// 0x142828fe8  488b4dd8                       movq	-0x28(%rbp), %rcx
// Frames: F68 -> F93 -> F33 -> F102
// 0x142828fec  8901                           movl	%eax, (%rcx)
// 0x142828fee  4c8b7dd0                       movq	-0x30(%rbp), %r15
// Frames: F71 -> F30 -> F31 -> F32 -> F33 -> F102
// 0x142828ff2  498b7f30                       movq	0x30(%r15), %rdi
// Frames: F73 -> F32 -> F33 -> F102
// 0x142828ff6  4d8b7740                       movq	0x40(%r15), %r14
// Frames: F74 -> F30 -> F31 -> F32 -> F33 -> F102
// 0x142828ffa  493b7f20                       cmpq	0x20(%r15), %rdi
// 0x142828ffe  0f85ccf9ffff                   jne	0x1428289d0 <.text+0x28279d0>
// Frames: F75 -> F30 -> F31 -> F32 -> F33 -> F102
// 0x142829004  488b4de0                       movq	-0x20(%rbp), %rcx
// 0x142829008  e843655400                     callq	0x142d6f550 <.text+0x2d6e550>
// 0x14282900d  e9bef9ffff                     jmp	0x1428289d0 <.text+0x28279d0>
// Frames: F211 -> F161 -> F162 -> F163 -> F164 -> F165 -> F166 -> F209
// 0x142829012  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x14282901b  41b818000000                   movl	$0x18, %r8d
// 0x142829021  41b901000000                   movl	$0x1, %r9d
// 0x142829027  4889f1                         movq	%rsi, %rcx
// 0x14282902a  4889fa                         movq	%rdi, %rdx
// 0x14282902d  e8de6c5400                     callq	0x142d6fd10 <.text+0x2d6ed10>
// 0x142829032  eb83                           jmp	0x142828fb7 <.text+0x2827fb7>
// Frames: F211 -> F161 -> F162 -> F163 -> F164 -> F165 -> F166 -> F167
// 0x142829034  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x14282903d  41b818000000                   movl	$0x18, %r8d
// 0x142829043  41b901000000                   movl	$0x1, %r9d
// 0x142829049  4889f1                         movq	%rsi, %rcx
// 0x14282904c  4889fa                         movq	%rdi, %rdx
// 0x14282904f  e8bc6c5400                     callq	0x142d6fd10 <.text+0x2d6ed10>
// 0x142829054  e9ddfaffff                     jmp	0x142828b36 <.text+0x2827b36>
// Frames: F211 -> F161 -> F162 -> F163 -> F189 -> F190 -> F191 -> F195
// 0x142829059  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x142829062  41b878000000                   movl	$0x78, %r8d
// 0x142829068  41b901000000                   movl	$0x1, %r9d
// 0x14282906e  4889f1                         movq	%rsi, %rcx
// 0x142829071  4889fa                         movq	%rdi, %rdx
// 0x142829074  e8976c5400                     callq	0x142d6fd10 <.text+0x2d6ed10>
// 0x142829079  e981fdffff                     jmp	0x142828dff <.text+0x2827dff>
// Frames: F211 -> F161 -> F162 -> F163 -> F189 -> F190 -> F191 -> F188
// 0x14282907e  48c744242001000000             movq	$0x1, 0x20(%rsp)
// 0x142829087  41b878000000                   movl	$0x78, %r8d
// 0x14282908d  41b901000000                   movl	$0x1, %r9d
// 0x142829093  4889f1                         movq	%rsi, %rcx
// 0x142829096  4889fa                         movq	%rdi, %rdx
// 0x142829099  e8726c5400                     callq	0x142d6fd10 <.text+0x2d6ed10>
// 0x14282909e  e937fcffff                     jmp	0x142828cda <.text+0x2827cda>
// Frames: F212
// 0x1428290a3  0f28b5e0000000                 movaps	0xe0(%rbp), %xmm6
// 0x1428290aa  0f28bdf0000000                 movaps	0xf0(%rbp), %xmm7
// 0x1428290b1  440f288500010000               movaps	0x100(%rbp), %xmm8
// 0x1428290b9  440f288d10010000               movaps	0x110(%rbp), %xmm9
// 0x1428290c1  4881c4a8010000                 addq	$0x1a8, %rsp            # imm = 0x1A8
// 0x1428290c8  5b                             popq	%rbx
// 0x1428290c9  5f                             popq	%rdi
// 0x1428290ca  5e                             popq	%rsi
// 0x1428290cb  415c                           popq	%r12
// 0x1428290cd  415d                           popq	%r13
// 0x1428290cf  415e                           popq	%r14
// 0x1428290d1  415f                           popq	%r15
// 0x1428290d3  5d                             popq	%rbp
// 0x1428290d4  c3                             retq
// Frames: F155
// 0x1428290d5  488d05b4fad500                 leaq	0xd5fab4(%rip), %rax    # 0x143588b90
// 0x1428290dc  4889442428                     movq	%rax, 0x28(%rsp)
// 0x1428290e1  488d5540                       leaq	0x40(%rbp), %rdx
// 0x1428290e5  4c8d85d7000000                 leaq	0xd7(%rbp), %r8
// 0x1428290ec  31c9                           xorl	%ecx, %ecx
// 0x1428290ee  4531c9                         xorl	%r9d, %r9d
// 0x1428290f1  e811135f00                     callq	0x142e1a407 <.text+0x2e19407>
// 0x1428290f6  eb03                           jmp	0x1428290fb <.text+0x28280fb>
// 0x1428290f8  4531c9                         xorl	%r9d, %r9d
// Frames: F213 -> F110
// 0x1428290fb  44894d40                       movl	%r9d, 0x40(%rbp)
// 0x1428290ff  895544                         movl	%edx, 0x44(%rbp)
// 0x142829102  48894d48                       movq	%rcx, 0x48(%rbp)
// 0x142829106  488d056bfad500                 leaq	0xd5fa6b(%rip), %rax    # 0x143588b78
// 0x14282910d  4889442420                     movq	%rax, 0x20(%rsp)
// 0x142829112  488d0dcffad500                 leaq	0xd5facf(%rip), %rcx    # 0x143588be8
// 0x142829119  4c8d0da8fad500                 leaq	0xd5faa8(%rip), %r9     # 0x143588bc8
// 0x142829120  4c8d4540                       leaq	0x40(%rbp), %r8
// 0x142829124  ba2b000000                     movl	$0x2b, %edx
// 0x142829129  e812855800                     callq	0x142db1640 <.text+0x2db0640>
// 0x14282912e  cc                             int3
// 0x14282912f  90                             nop
// 0x142829130  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x142829135  55                             pushq	%rbp
// 0x142829136  4157                           pushq	%r15
// 0x142829138  4156                           pushq	%r14
// 0x14282913a  4155                           pushq	%r13
// 0x14282913c  4154                           pushq	%r12
// 0x14282913e  56                             pushq	%rsi
// 0x14282913f  57                             pushq	%rdi
// 0x142829140  53                             pushq	%rbx
// 0x142829141  4883ec78                       subq	$0x78, %rsp
// 0x142829145  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x14282914c  440f294c2430                   movaps	%xmm9, 0x30(%rsp)
// 0x142829152  440f29442440                   movaps	%xmm8, 0x40(%rsp)
// 0x142829158  0f297c2450                     movaps	%xmm7, 0x50(%rsp)
// 0x14282915d  0f29742460                     movaps	%xmm6, 0x60(%rsp)
// Frames: F177 -> F178 -> F179 -> F98 -> F98 -> F98 -> F180
// 0x142829162  4883bd9800000000               cmpq	$0x0, 0x98(%rbp)
// 0x14282916a  7416                           je	0x142829182 <.text+0x2828182>
// Frames: F181 -> F182 -> F183 -> F184 -> F185 -> F179 -> F98 -> F98 -> F98 -> F180
// 0x14282916c  41b801000000                   movl	$0x1, %r8d
// 0x142829172  488b4d00                       movq	(%rbp), %rcx
// 0x142829176  488b9598000000                 movq	0x98(%rbp), %rdx
// 0x14282917d  e86ec7e7fd                     callq	0x1406a58f0 <.text+0x6a48f0>
// Frames: F100
// 0x142829182  0f28742460                     movaps	0x60(%rsp), %xmm6
// 0x142829187  0f287c2450                     movaps	0x50(%rsp), %xmm7
// 0x14282918c  440f28442440                   movaps	0x40(%rsp), %xmm8
// 0x142829192  440f284c2430                   movaps	0x30(%rsp), %xmm9
// 0x142829198  4883c478                       addq	$0x78, %rsp
// 0x14282919c  5b                             popq	%rbx
// 0x14282919d  5f                             popq	%rdi
// 0x14282919e  5e                             popq	%rsi
// 0x14282919f  415c                           popq	%r12
// 0x1428291a1  415d                           popq	%r13
// 0x1428291a3  415e                           popq	%r14
// 0x1428291a5  415f                           popq	%r15
// 0x1428291a7  5d                             popq	%rbp
// 0x1428291a8  c3                             retq
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/create_roof.rs:17:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F2: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:1065:0 bevy_ecs::system::system_param::impl$14::deref_mut
// F3: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs:28:0 bevy_ecs::message::message_reader::MessageReader::read
// F4: crates/systems/roof/src/create_roof.rs:29:0 system_roof::create_roof::create_roof::hc621d06ab743637e
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
// F25: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:612:0 alloc::raw_vec::RawVecInner::non_null
// F26: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:607:0 alloc::raw_vec::RawVecInner::ptr
// F27: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:295:0 alloc::raw_vec::RawVec::ptr
// F28: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2000:0 alloc::vec::Vec::as_mut_ptr
// F29: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1032:0 alloc::vec::Vec::push_mut
// F30: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:992:0 alloc::vec::Vec::push
// F31: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:143:0 bevy_ecs::message::messages::Messages::write_with_caller
// F32: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:126:0 bevy_ecs::message::messages::Messages::write
// F33: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs:70:0 bevy_ecs::message::message_writer::MessageWriter::write
// F34: crates/systems/roof/src/create_roof.rs:86:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F35: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:961:0 alloc::vec::Vec::as_mut_ptr
// F36: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1917:0 core::ptr::write
// F37: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1033:0 alloc::vec::Vec::push_mut
// F38: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1034:0 alloc::vec::Vec::push_mut
// F39: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:144:0 bevy_ecs::message::messages::Messages::write_with_caller
// F40: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:767:0 core::option::Option::as_mut
// F41: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:332:0 core::iter::adapters::chain::and_then_or_clear
// F42: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:0 core::iter::adapters::chain::impl$1::next
// F43: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:89:0 bevy_ecs::message::iterators::impl$3::next
// F44: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:15:0 bevy_ecs::message::iterators::impl$0::next
// F45: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:0 core::iter::adapters::chain::impl$1::next::closure$0
// F46: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1656:0 core::option::Option::or_else
// F47: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:659:0 core::ptr::non_null::NonNull::add
// F48: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:185:0 core::slice::iter::impl$171::next
// F49: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs:95:0 bevy_ecs::message::iterators::impl$3::next
// F50: crates/systems/roof/src/create_roof.rs:30:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F51: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:0 core::option::Option::unwrap
// F52: crates/systems/roof/src/create_roof.rs:33:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F53: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1164:0 core::option::Option::map
// F54: crates/systems/roof/src/create_roof.rs:34:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F55: crates/systems/roof/src/create_roof.rs:35:0 system_roof::create_roof::create_roof::closure$0
// F56: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:0 core::option::Option::map
// F57: crates/systems/roof/src/create_roof.rs:42:0 system_roof::create_roof::create_roof::closure$0
// F58: crates/systems/roof/src/create_roof.rs:32:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F59: crates/systems/roof/src/create_roof.rs:45:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F60: crates/systems/roof/src/create_roof.rs:48:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F61: crates/systems/roof/src/create_roof.rs:51:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F62: crates/systems/roof/src/create_roof.rs:52:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F63: crates/systems/roof/src/create_roof.rs:53:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F64: crates/systems/roof/src/create_roof.rs:46:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F65: crates/systems/roof/src/create_roof.rs:55:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F66: crates/systems/roof/src/create_roof.rs:57:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F67: crates/systems/roof/src/create_roof.rs:59:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F70: crates/systems/roof/src/create_roof.rs:58:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F71: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1025:0 alloc::vec::Vec::push_mut
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:126:0 bevy_ecs::change_detection::impl$38::deref_mut
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs:131:0 bevy_ecs::message::messages::Messages::write_with_caller
// F74: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1028:0 alloc::vec::Vec::push_mut
// F75: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1029:0 alloc::vec::Vec::push_mut
// F76: crates/systems/roof/src/create_roof.rs:65:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F77: crates/systems/roof/src/create_roof.rs:66:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F78: crates/systems/roof/src/create_roof.rs:67:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F79: crates/systems/roof/src/create_roof.rs:69:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F80: crates/systems/roof/src/visual/switch_roof_type.rs:132:0 system_roof::visual::switch_roof_type::stone_floor_bundle
// F81: crates/systems/roof/src/create_roof.rs:72:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F82: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/convert/mod.rs:778:0 core::convert::impl$3::into
// F83: crates/utils/src/components/transform.rs:16:0 utils::components::transform::Transform::identity
// F84: crates/systems/roof/src/visual/switch_roof_type.rs:135:0 system_roof::visual::switch_roof_type::stone_floor_bundle
// F85: crates/systems/roof/src/visual/switch_roof_type.rs:136:0 system_roof::visual::switch_roof_type::stone_floor_bundle
// F86: crates/systems/roof/src/visual/switch_roof_type.rs:129:0 system_roof::visual::switch_roof_type::stone_floor_bundle
// F87: crates/systems/roof/src/create_roof.rs:63:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F88: crates/systems/roof/src/create_roof.rs:75:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F89: crates/systems/roof/src/create_roof.rs:78:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F90: crates/systems/roof/src/create_roof.rs:80:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F91: crates/systems/roof/src/create_roof.rs:84:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F92: crates/systems/roof/src/create_roof.rs:77:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F93: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs:435:0 bevy_ecs::message::messages::Messages::write
// F94: crates/systems/roof/src/create_roof.rs:92:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F95: crates/systems/roof/src/create_roof.rs:37:0 system_roof::create_roof::create_roof::closure$0
// F96: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1016:0 core::option::Option::unwrap
// F97: crates/systems/roof/src/create_roof.rs:43:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F98: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:805:0 core::ptr::drop_in_place
// F99: crates/systems/roof/src/create_roof.rs:73:0 system_roof::create_roof::create_roof::hc621d06ab743637e
// F100: crates/systems/roof/src/visual/switch_roof_type.rs:24:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F101: crates/systems/roof/src/visual/switch_roof_type.rs:41:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F102: crates/systems/roof/src/visual/switch_roof_type.rs:121:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F103: crates/systems/roof/src/visual/switch_roof_type.rs:37:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F104: crates/systems/roof/src/visual/switch_roof_type.rs:38:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F105: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:219:0 core::slice::index::impl$2::get
// F106: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:957:0 bevy_ecs::entity::Entities::get_id_location
// F107: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:949:0 bevy_ecs::entity::Entities::get
// F108: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1546:0 bevy_ecs::system::query::Query::get_inner
// F109: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1389:0 bevy_ecs::system::query::Query::get
// F110: crates/systems/roof/src/visual/switch_roof_type.rs:43:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F111: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1583:0 core::option::Option::filter
// F112: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:958:0 bevy_ecs::entity::Entities::get_id_location
// F113: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:214:0 bevy_ecs::entity::impl$37::eq
// F114: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs:958:0 bevy_ecs::entity::impl$18::get_id_location::closure$0
// F115: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1584:0 core::option::Option::filter
// F116: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1589:0 core::option::Option::filter
// F117: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1338:0 core::option::Option::ok_or
// F118: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1547:0 bevy_ecs::system::query::Query::get_inner
// F119: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs:123:0 bevy_ecs::archetype::ArchetypeId::index
// F120: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1551:0 bevy_ecs::system::query::Query::get_inner
// F121: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:393:0 bevy_ecs::archetype::ArchetypeId::index
// F122: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/bool.rs:66:0 core::bool::impl$0::then
// F123: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 bevy_ecs::archetype::ArchetypeId::index
// F124: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:172:0 fixedbitset::FixedBitSet::get_unchecked
// F125: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:410:0 fixedbitset::FixedBitSet::contains_unchecked
// F126: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs:395:0 fixedbitset::impl$2::contains::closure$0
// F127: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/internal_macros.rs:28:0 fixedbitset::FixedBitSet::get_unchecked
// F128: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1548:0 bevy_ecs::system::query::Query::get_inner
// F129: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1916:0 alloc::vec::Vec::as_ptr
// F130: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:753:0 bevy_ecs::storage::table::Tables::get
// F131: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1580:0 bevy_ecs::system::query::Query::get_inner
// F132: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:221:0 core::slice::index::impl$2::get
// F133: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:576:0 alloc::vec::impl$9::deref
// F134: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1582:0 bevy_ecs::system::query::Query::get_inner
// F135: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:406:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F136: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:490:0 bevy_ecs::storage::table::Table::get_column
// F137: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:381:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F138: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1552:0 bevy_ecs::query::fetch::impl$36::set_table
// F139: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1539:0 bevy_ecs::query::fetch::impl$36::set_archetype
// F140: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:745:0 core::option::Option::as_ref
// F141: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F142: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1546:0 core::option::Option::and_then
// F143: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F144: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F145: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs:408:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F146: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:253:0 core::slice::index::impl$2::get_unchecked
// F147: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:646:0 nonmax::NonMaxUsize::get
// F148: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxU32::get
// F149: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs:116:0 bevy_ecs::storage::table::TableRow::index
// F150: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1609:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F151: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:2966:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F152: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs:1604:0 bevy_ecs::query::fetch::impl$37::fetch
// F153: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs:1591:0 bevy_ecs::system::query::Query::get_inner
// F154: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs:1079:0 bevy_ecs::storage::table::TableRow::index
// F155: crates/systems/roof/src/visual/switch_roof_type.rs:46:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F156: crates/country-core/src/resources/roofs/roof_shape.rs:31:0 country_core::resources::roofs::roof_shape::impl$12::eq
// F157: crates/systems/roof/src/visual/switch_roof_type.rs:48:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F158: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:617:0 alloc::raw_vec::RawVecInner::capacity
// F159: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:765:0 alloc::raw_vec::RawVecInner::needs_to_grow
// F160: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:671:0 alloc::raw_vec::RawVecInner::reserve
// F161: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:340:0 alloc::raw_vec::RawVec::reserve
// F162: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1446:0 alloc::vec::Vec::reserve
// F163: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:0:0 bevy_ecs::system::commands::Commands::queue_internal
// F164: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:644:0 bevy_ecs::system::commands::Commands::queue_handled
// F165: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:1940:0 bevy_ecs::system::commands::EntityCommands::queue_handled
// F166: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:1660:0 bevy_ecs::system::commands::EntityCommands::remove
// F167: crates/systems/roof/src/visual/switch_roof_type.rs:50:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F168: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:2494:0 alloc::raw_vec::RawVecInner::capacity
// F169: crates/systems/roof/src/visual/switch_roof_type.rs:54:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F170: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/fmt.rs:659:0 alloc::fmt::format::closure$0
// F171: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1278:0 core::option::Option::map_or_else
// F172: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/fmt.rs:659:0 alloc::fmt::format
// F173: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/hint.rs:614:0 core::hint::must_use
// F174: crates/systems/roof/src/visual/switch_roof_type.rs:53:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F175: crates/systems/roof/src/visual/switch_roof_type.rs:56:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F176: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2422:0 alloc::sync::impl$34::deref
// F177: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:632:0 alloc::raw_vec::RawVecInner::current_memory
// F178: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:872:0 alloc::raw_vec::RawVecInner::deallocate
// F179: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:423:0 alloc::raw_vec::impl$3::drop
// F180: crates/systems/roof/src/visual/switch_roof_type.rs:57:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F181: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:115:0 alloc::alloc::dealloc
// F182: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:209:0 alloc::alloc::Global::deallocate_impl_runtime
// F183: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:324:0 alloc::alloc::Global::deallocate_impl
// F184: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:442:0 alloc::alloc::impl$1::deallocate
// F185: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:874:0 alloc::raw_vec::RawVecInner::deallocate
// F186: crates/systems/roof/src/visual/switch_roof_type.rs:61:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F187: crates/systems/roof/src/visual/switch_roof_type.rs:59:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F188: crates/systems/roof/src/visual/switch_roof_type.rs:64:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F189: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:589:0 bevy_ecs::system::commands::Commands::queue
// F190: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:1893:0 bevy_ecs::system::commands::EntityCommands::queue
// F191: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:1391:0 bevy_ecs::system::commands::EntityCommands::insert
// F192: crates/systems/roof/src/visual/switch_roof_type.rs:67:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F193: crates/systems/roof/src/visual/switch_roof_type.rs:69:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F194: crates/systems/roof/src/visual/switch_roof_type.rs:68:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F195: crates/systems/roof/src/visual/switch_roof_type.rs:78:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F196: crates/systems/roof/src/visual/switch_roof_type.rs:83:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F197: crates/systems/roof/src/visual/switch_roof_type.rs:87:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F198: crates/systems/roof/src/visual/switch_roof_type.rs:80:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F199: crates/systems/roof/src/visual/switch_roof_type.rs:91:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F200: crates/systems/roof/src/visual/switch_roof_type.rs:93:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F201: crates/systems/roof/src/visual/switch_roof_type.rs:92:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F202: crates/systems/roof/src/visual/switch_roof_type.rs:104:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F203: crates/systems/roof/src/visual/switch_roof_type.rs:105:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F204: crates/systems/roof/src/visual/switch_roof_type.rs:110:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F205: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1512:0 core::f32::impl$0::clamp
// F206: crates/systems/roof/src/visual/switch_roof_type.rs:108:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F207: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1515:0 core::f32::impl$0::clamp
// F208: crates/systems/roof/src/visual/switch_roof_type.rs:111:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F209: crates/systems/roof/src/visual/switch_roof_type.rs:118:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F210: crates/systems/roof/src/visual/switch_roof_type.rs:122:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F211: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:673:0 alloc::raw_vec::RawVecInner::reserve
// F212: crates/systems/roof/src/visual/switch_roof_type.rs:125:0 system_roof::visual::switch_roof_type::switch_roof_type::hed516f6d52cc6218
// F213: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1233:0 core::result::Result::unwrap
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/visual/switch_roof_type.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 63b2dc2380ee07ff9567406c789875b23c4e4c37050aa5704e0e2b051cd941fd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: switch_roof_type.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3778, 3781
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use system_roof; // namespace candidate
//   package identities: [{"name":"system-roof","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::visual::switch_roof_type::switch_roof_type
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3781,"record_offset":268,"rva":42108960,"va":5410818080}
// 编译实例 RVAs: 0x2828820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-35140356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e8fccce7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5410819055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42097045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e86ba67dfd","size":5,"soft_fail":false,"target_va":5368722192,"va":5410819232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f7b20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32706928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e890ee0cfe","size":5,"soft_fail":false,"target_va":5378112288,"va":5410819211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9f60
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32697444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e89c130dfe","size":5,"soft_fail":false,"target_va":5378121568,"va":5410819007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=2
//     target_aliases=20 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32692261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e8db270dfe","size":5,"soft_fail":false,"target_va":5378126944,"va":5410819200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5531358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e8de665400","size":5,"soft_fail":false,"target_va":5416350496,"va":5410819133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5531619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e8e3675400","size":5,"soft_fail":false,"target_va":5416351056,"va":5410819432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: switch_roof_type.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5532894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e8de6c5400","size":5,"soft_fail":false,"target_va":5416353040,"va":5410820141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-26024097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e85fe772fe","size":5,"soft_fail":false,"target_va":5384794864,"va":5410818956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6230801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e811135f00","size":5,"soft_fail":false,"target_va":5417051143,"va":5410820337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5801234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e812855800","size":5,"soft_fail":false,"target_va":5416621632,"va":5410820393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::try_get_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31547537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e86f9f1efe","size":5,"soft_fail":false,"target_va":5379271456,"va":5410818988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::components::transient_mesh::TransientMeshRegistry::deferred_delete_mesh
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32249972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e88ce713fe","size":5,"soft_fail":false,"target_va":5378569056,"va":5410819023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29621555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e8cd023cfe","size":5,"soft_fail":false,"target_va":5381197520,"va":5410819070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29621810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e8ce013cfe","size":5,"soft_fail":false,"target_va":5381197008,"va":5410818813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::get_unwrapped
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29517664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e8a0983dfe","size":5,"soft_fail":false,"target_va":5381302272,"va":5410819931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29512133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410818080,"raw_hex":"e83bae3dfe","size":5,"soft_fail":false,"target_va":5381307856,"va":5410819984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3781,"record_offset":3456,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3781,"record_offset":3568,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3781,"record_offset":6120,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3781,"record_offset":1564,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":3781,"record_offset":464,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":3781,"record_offset":1180,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":3781,"record_offset":4332,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::remove
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":3781,"record_offset":3160,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11389,"source_line":1659,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3781,"record_offset":1848,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3781,"record_offset":6088,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":3781,"record_offset":3540,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3781,"record_offset":3596,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3781,"record_offset":6468,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3781,"record_offset":3132,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":133059,"source_line":31,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_direct] system_roof::visual::switch_roof_type::stone_floor_bundle
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/visual/switch_roof_type.rs
//     evidence={"module":3781,"record_offset":4236,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":133038,"source_line":128,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3781,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3781,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3781,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3781,"record_offset":3748,"parent_record_offset":3692,"ancestor_record_offset":268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3781,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":268,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3781,"record_offset":1400,"parent_record_offset":1368,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3781,"record_offset":3304,"parent_record_offset":3276,"ancestor_record_offset":268,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3781,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":268,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3781,"record_offset":3720,"parent_record_offset":3692,"ancestor_record_offset":268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3781,"record_offset":3692,"parent_record_offset":3668,"ancestor_record_offset":268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3781,"record_offset":3364,"parent_record_offset":3332,"ancestor_record_offset":268,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3781,"record_offset":1448,"parent_record_offset":1424,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3781,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3781,"record_offset":3332,"parent_record_offset":3304,"ancestor_record_offset":268,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3781,"record_offset":3668,"parent_record_offset":3644,"ancestor_record_offset":268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3781,"record_offset":1368,"parent_record_offset":1328,"ancestor_record_offset":268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3781,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3781,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3781,"record_offset":1296,"parent_record_offset":1252,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3781,"record_offset":1328,"parent_record_offset":1296,"ancestor_record_offset":268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3781,"record_offset":3276,"parent_record_offset":3244,"ancestor_record_offset":268,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3781,"record_offset":800,"parent_record_offset":552,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3781,"record_offset":2156,"parent_record_offset":1872,"ancestor_record_offset":268,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3781,"record_offset":1528,"parent_record_offset":1208,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3781,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3781,"record_offset":1920,"parent_record_offset":1872,"ancestor_record_offset":268,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3781,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":268,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3781,"record_offset":2060,"parent_record_offset":2028,"ancestor_record_offset":268,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3781,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":268,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3781,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":268,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3781,"record_offset":1588,"parent_record_offset":1564,"ancestor_record_offset":268,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":3781,"record_offset":528,"parent_record_offset":488,"ancestor_record_offset":268,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3781,"record_offset":1208,"parent_record_offset":1180,"ancestor_record_offset":268,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3781,"record_offset":1252,"parent_record_offset":1208,"ancestor_record_offset":268,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3781,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":268,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3781,"record_offset":2576,"parent_record_offset":1872,"ancestor_record_offset":268,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3781,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":268,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3781,"record_offset":2948,"parent_record_offset":1872,"ancestor_record_offset":268,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3781,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":268,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3781,"record_offset":2696,"parent_record_offset":2672,"ancestor_record_offset":268,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3781,"record_offset":2672,"parent_record_offset":2648,"ancestor_record_offset":268,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3781,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":268,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3781,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":268,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3781,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":268,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3781,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":268,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3781,"record_offset":2344,"parent_record_offset":1872,"ancestor_record_offset":268,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":3781,"record_offset":4388,"parent_record_offset":4360,"ancestor_record_offset":268,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_handled
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":3781,"record_offset":3216,"parent_record_offset":3188,"ancestor_record_offset":268,"inlinee":11370,"source_line":639,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":3781,"record_offset":3244,"parent_record_offset":3216,"ancestor_record_offset":268,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":3781,"record_offset":4360,"parent_record_offset":4332,"ancestor_record_offset":268,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue_handled
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":3781,"record_offset":3188,"parent_record_offset":3160,"ancestor_record_offset":268,"inlinee":11369,"source_line":1934,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3781,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":268,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":3781,"record_offset":488,"parent_record_offset":464,"ancestor_record_offset":268,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":3781,"record_offset":2192,"parent_record_offset":2156,"ancestor_record_offset":268,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3781,"record_offset":4272,"parent_record_offset":4236,"ancestor_record_offset":268,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3781,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":268,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3781,"record_offset":1616,"parent_record_offset":1588,"ancestor_record_offset":268,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3781,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":268,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3781,"record_offset":752,"parent_record_offset":720,"ancestor_record_offset":268,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3781,"record_offset":600,"parent_record_offset":552,"ancestor_record_offset":268,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3781,"record_offset":2744,"parent_record_offset":2720,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3781,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":268,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3781,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":268,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3781,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3781,"record_offset":2028,"parent_record_offset":1944,"ancestor_record_offset":268,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3781,"record_offset":2808,"parent_record_offset":2672,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3781,"record_offset":3480,"parent_record_offset":3456,"ancestor_record_offset":268,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3781,"record_offset":2128,"parent_record_offset":1872,"ancestor_record_offset":268,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3781,"record_offset":1696,"parent_record_offset":1616,"ancestor_record_offset":268,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3781,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3781,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3781,"record_offset":1796,"parent_record_offset":1772,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3781,"record_offset":1488,"parent_record_offset":1328,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3781,"record_offset":656,"parent_record_offset":552,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3781,"record_offset":1024,"parent_record_offset":552,"ancestor_record_offset":268,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3781,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":268,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3781,"record_offset":2888,"parent_record_offset":2856,"ancestor_record_offset":268,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3781,"record_offset":720,"parent_record_offset":688,"ancestor_record_offset":268,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3781,"record_offset":688,"parent_record_offset":656,"ancestor_record_offset":268,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3781,"record_offset":1048,"parent_record_offset":1024,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3781,"record_offset":1772,"parent_record_offset":1720,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3781,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":268,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3781,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":268,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3781,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":268,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3781,"record_offset":3052,"parent_record_offset":3020,"ancestor_record_offset":268,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3781,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":268,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   [pdb_inline_transitive] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3781,"record_offset":4300,"parent_record_offset":4236,"ancestor_record_offset":268,"inlinee":27995,"source_line":15,"archive":"_pdb/C/modules/03781.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::visual::switch_roof_type::stone_floor_bundle
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x207AE
// 首条位置: {"module":3778,"inlinee":133038}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3778,"record_offset":2304,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3778,"record_offset":2332,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":27995,"source_line":15,"archive":"_pdb/C/modules/03778.jsonl.gz"}
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
// 完整结构及原始类型记录: switch_roof_type.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: switch_roof_type.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: switch_roof_type.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_roof::visual::switch_roof_type::stone_floor_bundle
// 信息 ID: 300f5a124d70ccdfd182f0bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::create_roof::create_roof; module=3778, inline_record=2268; 不是独立调用边。
// [PDB 内联宿主] system_roof::visual::switch_roof_type::switch_roof_type; module=3781, inline_record=4236; 不是独立调用边。
// [PDB 内联宿主] system_roof::visual::switch_roof_type::switch_roof_type; module=3781, inline_record=4984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::visual::switch_roof_type::switch_roof_type
// 信息 ID: 4bcac63b40db3cb406d99159
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] aac53fb1b214cbb91e9f2332: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>)> > @ 0x1f27fba -> 0x2828820 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>)> @ 0x1f31966 -> 0x2828820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
