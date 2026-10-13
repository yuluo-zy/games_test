// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: create_roof.rs.assembly.json.gz

// FUNCTION system_roof::create_roof::create_roof::closure$0
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3778:1868 -> host 3778:268 / rva_2826b60_len_7cb; exact_symbolizer_function_frame
//   Function-frame RVA span [0x2826d8f, 0x2826dd1), 11 instructions
//   Function-frame RVA span [0x28271ef, 0x282721b), 8 instructions
//
// FUNCTION system_roof::create_roof::create_roof
// Candidate interfaces: dd8568a6b40cd41120ab11dd
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3778:268 -> rva_2826b60_len_7cb
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
// === ORIGINAL ASSEMBLY V1 END ===
// Version: C
// Original source: crates/systems/roof/src/create_roof.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5d49e05cc8065f095a20b4516f82f02736e11953db498fb5a8ac917e2d07bf2c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: create_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3778
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
// use system_roof; // namespace candidate
//   package identities: [{"name":"system-roof","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::create_roof::create_roof
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3778,"record_offset":268,"rva":42101600,"va":5410810720}
// 编译实例 RVAs: 0x2826b60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42089552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8b0c37dfd","size":5,"soft_fail":false,"target_va":5368722192,"va":5410811739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140104100
//     owner=unresolved_namespace; count=1
//     target_aliases=583 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-41037337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8e7d18dfd","size":5,"soft_fail":false,"target_va":5369774336,"va":5410811668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401f2bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-40060753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8afb89cfd","size":5,"soft_fail":false,"target_va":5370751920,"va":5410812668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f7b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32699435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8d50b0dfe","size":5,"soft_fail":false,"target_va":5378112288,"va":5410811718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b30
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32691612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8642a0dfe","size":5,"soft_fail":false,"target_va":5378120496,"va":5410812103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32684761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e827450dfe","size":5,"soft_fail":false,"target_va":5378126944,"va":5410811700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a04dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31596854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8cade1dfe","size":5,"soft_fail":false,"target_va":5379214800,"va":5410811649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27487111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e879945cfe","size":5,"soft_fail":false,"target_va":5383325056,"va":5410812162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbd5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25598641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e84f6579fe","size":5,"soft_fail":false,"target_va":5385213424,"va":5410812060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbf6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25590715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8458479fe","size":5,"soft_fail":false,"target_va":5385221840,"va":5410812550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5538040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8f8805400","size":5,"soft_fail":false,"target_va":5416350384,"va":5410812339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5538260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8d4815400","size":5,"soft_fail":false,"target_va":5416350496,"va":5410812231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: create_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5539467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e88b865400","size":5,"soft_fail":false,"target_va":5416351056,"va":5410811584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5809145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8f9a35800","size":5,"soft_fail":false,"target_va":5416621600,"va":5410812450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5809797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e885a65800","size":5,"soft_fail":false,"target_va":5416622240,"va":5410812438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_animation::RoofAnimation::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31741820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e884a81bfe","size":5,"soft_fail":false,"target_va":5379069840,"va":5410811655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29614058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e816203cfe","size":5,"soft_fail":false,"target_va":5381197472,"va":5410811525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29614488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8681e3cfe","size":5,"soft_fail":false,"target_va":5381197024,"va":5410811507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29614512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8501e3cfe","size":5,"soft_fail":false,"target_va":5381197008,"va":5410811515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_tile_sim::RoofTileSimulation::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31695802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8465c1cfe","size":5,"soft_fail":false,"target_va":5379115840,"va":5410811637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_wall_state::WallMaterial::gable_roof_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29169319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e859e942fe","size":5,"soft_fail":false,"target_va":5381642096,"va":5410811410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29576764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8c4b13cfe","size":5,"soft_fail":false,"target_va":5381234608,"va":5410811367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29907704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e808a537fe","size":5,"soft_fail":false,"target_va":5380903536,"va":5410811235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31214865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410810720,"raw_hex":"e8efb223fe","size":5,"soft_fail":false,"target_va":5379597264,"va":5410812124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3778,"record_offset":1516,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":3778,"record_offset":416,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":3778,"record_offset":1128,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3778,"record_offset":1836,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3778,"record_offset":1800,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3778,"record_offset":2748,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_direct] system_roof::visual::switch_roof_type::stone_floor_bundle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/visual/switch_roof_type.rs
//     evidence={"module":3778,"record_offset":2268,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":133038,"source_line":128,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3778,"record_offset":1352,"parent_record_offset":1320,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3778,"record_offset":1400,"parent_record_offset":1376,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3778,"record_offset":1376,"parent_record_offset":1352,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3778,"record_offset":1320,"parent_record_offset":1276,"ancestor_record_offset":268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3778,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3778,"record_offset":1244,"parent_record_offset":1200,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3778,"record_offset":1276,"parent_record_offset":1244,"ancestor_record_offset":268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3778,"record_offset":732,"parent_record_offset":504,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3778,"record_offset":1480,"parent_record_offset":1156,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3778,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3778,"record_offset":504,"parent_record_offset":480,"ancestor_record_offset":268,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3778,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":268,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":3778,"record_offset":480,"parent_record_offset":440,"ancestor_record_offset":268,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3778,"record_offset":1156,"parent_record_offset":1128,"ancestor_record_offset":268,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3778,"record_offset":1200,"parent_record_offset":1156,"ancestor_record_offset":268,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":3778,"record_offset":440,"parent_record_offset":416,"ancestor_record_offset":268,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3778,"record_offset":2304,"parent_record_offset":2268,"ancestor_record_offset":268,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3778,"record_offset":1592,"parent_record_offset":1568,"ancestor_record_offset":268,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3778,"record_offset":1568,"parent_record_offset":1540,"ancestor_record_offset":268,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3778,"record_offset":1672,"parent_record_offset":1648,"ancestor_record_offset":268,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3778,"record_offset":688,"parent_record_offset":660,"ancestor_record_offset":268,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3778,"record_offset":548,"parent_record_offset":504,"ancestor_record_offset":268,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3778,"record_offset":1616,"parent_record_offset":1592,"ancestor_record_offset":268,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3778,"record_offset":1648,"parent_record_offset":1568,"ancestor_record_offset":268,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3778,"record_offset":2772,"parent_record_offset":2748,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3778,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3778,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3778,"record_offset":1440,"parent_record_offset":1276,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3778,"record_offset":604,"parent_record_offset":504,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3778,"record_offset":972,"parent_record_offset":504,"ancestor_record_offset":268,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3778,"record_offset":660,"parent_record_offset":632,"ancestor_record_offset":268,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3778,"record_offset":632,"parent_record_offset":604,"ancestor_record_offset":268,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3778,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3778,"record_offset":1724,"parent_record_offset":1672,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] system_roof::create_roof::create_roof::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/create_roof.rs
//     evidence={"module":3778,"record_offset":1868,"parent_record_offset":1836,"ancestor_record_offset":268,"inlinee":133036,"source_line":34,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   [pdb_inline_transitive] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3778,"record_offset":2332,"parent_record_offset":2268,"ancestor_record_offset":268,"inlinee":27995,"source_line":15,"archive":"_pdb/C/modules/03778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::create_roof::create_roof::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x207AC
// 首条位置: {"module":3778,"inlinee":133036}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: create_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: create_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/roof/src/create_roof.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: create_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_roof::create_roof::create_roof
// 信息 ID: a417a91fc98f2d3067741d99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] dd8568a6b40cd41120ab11dd: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands)> > @ 0x1f2ce60 -> 0x2826b60 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::system::commands::Commands)> @ 0x1f2f2df -> 0x2826b60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::create_roof::create_roof::closure$0
// 信息 ID: bd5fd240c8ab9ffcc2eff8b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_roof::create_roof::create_roof", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::create_roof::create_roof; module=3778, inline_record=1868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
