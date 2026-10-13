// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// === ORIGINAL ASSEMBLY V1 BEGIN ===
// Original PE bytes, AT&T x86-64 assembly; no target execution.
// PDB procedure bounds are retained; complete reachable CFG is unknown.
// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.
// Instruction frame chains run from innermost inline frame to outermost host.
// Machine-readable mapping: simulate_roof_tile_for_audio.rs.assembly.json.gz

// FUNCTION system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 3195:1224 -> host 3195:248 / rva_1988100_len_5a9; exact_symbolizer_function_frame
//   Function-frame RVA span [0x198834e, 0x198844e), 57 instructions
//   Function-frame RVA span [0x1988620, 0x19886a9), 18 instructions
//
// FUNCTION system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio
// Candidate interfaces: e512f3b422e4ecd930373d49
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3195:248 -> rva_1988100_len_5a9
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_1988100_len_5a9: RVA 0x1988100, 1449 PDB bytes; 298 instructions
// Evidence: _assembly/roof-v1/A/pack-002/rva_1988100_len_5a9.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3195:248: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio
// Frames: F1
// 0x141988100  4157                           pushq	%r15
// 0x141988102  4156                           pushq	%r14
// 0x141988104  4155                           pushq	%r13
// 0x141988106  4154                           pushq	%r12
// 0x141988108  56                             pushq	%rsi
// 0x141988109  57                             pushq	%rdi
// 0x14198810a  55                             pushq	%rbp
// 0x14198810b  53                             pushq	%rbx
// 0x14198810c  4881ec78010000                 subq	$0x178, %rsp            # imm = 0x178
// 0x141988113  440f29bc2460010000             movaps	%xmm15, 0x160(%rsp)
// 0x14198811c  440f29b42450010000             movaps	%xmm14, 0x150(%rsp)
// 0x141988125  440f29ac2440010000             movaps	%xmm13, 0x140(%rsp)
// 0x14198812e  440f29a42430010000             movaps	%xmm12, 0x130(%rsp)
// 0x141988137  440f299c2420010000             movaps	%xmm11, 0x120(%rsp)
// 0x141988140  440f29942410010000             movaps	%xmm10, 0x110(%rsp)
// 0x141988149  440f298c2400010000             movaps	%xmm9, 0x100(%rsp)
// 0x141988152  440f298424f0000000             movaps	%xmm8, 0xf0(%rsp)
// 0x14198815b  0f29bc24e0000000               movaps	%xmm7, 0xe0(%rsp)
// 0x141988163  0f29b424d0000000               movaps	%xmm6, 0xd0(%rsp)
// Frames: F2
// 0x14198816b  498b00                         movq	(%r8), %rax
// 0x14198816e  4889442460                     movq	%rax, 0x60(%rsp)
// 0x141988173  f30f104028                     movss	0x28(%rax), %xmm0
// 0x141988178  0f57f6                         xorps	%xmm6, %xmm6
// 0x14198817b  0f2ec6                         ucomiss	%xmm6, %xmm0
// 0x14198817e  756e                           jne	0x1419881ee <.text+0x19871ee>
// 0x141988180  7a6c                           jp	0x1419881ee <.text+0x19871ee>
// Frames: F3
// 0x141988182  0f28b424d0000000               movaps	0xd0(%rsp), %xmm6
// 0x14198818a  0f28bc24e0000000               movaps	0xe0(%rsp), %xmm7
// 0x141988192  440f288424f0000000             movaps	0xf0(%rsp), %xmm8
// 0x14198819b  440f288c2400010000             movaps	0x100(%rsp), %xmm9
// 0x1419881a4  440f28942410010000             movaps	0x110(%rsp), %xmm10
// 0x1419881ad  440f289c2420010000             movaps	0x120(%rsp), %xmm11
// 0x1419881b6  440f28a42430010000             movaps	0x130(%rsp), %xmm12
// 0x1419881bf  440f28ac2440010000             movaps	0x140(%rsp), %xmm13
// 0x1419881c8  440f28b42450010000             movaps	0x150(%rsp), %xmm14
// 0x1419881d1  440f28bc2460010000             movaps	0x160(%rsp), %xmm15
// 0x1419881da  4881c478010000                 addq	$0x178, %rsp            # imm = 0x178
// 0x1419881e1  5b                             popq	%rbx
// 0x1419881e2  5d                             popq	%rbp
// 0x1419881e3  5f                             popq	%rdi
// 0x1419881e4  5e                             popq	%rsi
// 0x1419881e5  415c                           popq	%r12
// 0x1419881e7  415d                           popq	%r13
// 0x1419881e9  415e                           popq	%r14
// 0x1419881eb  415f                           popq	%r15
// 0x1419881ed  c3                             retq
// Frames: F4 -> F5
// 0x1419881ee  488b01                         movq	(%rcx), %rax
// Frames: F6 -> F5
// 0x1419881f1  4c8b6108                       movq	0x8(%rcx), %r12
// 0x1419881f5  8b4914                         movl	0x14(%rcx), %ecx
// 0x1419881f8  894c2420                       movl	%ecx, 0x20(%rsp)
// Frames: F7 -> F8 -> F9 -> F6 -> F5
// 0x1419881fc  4c8ba8c8000000                 movq	0xc8(%rax), %r13
// 0x141988203  4889442470                     movq	%rax, 0x70(%rsp)
// 0x141988208  488b80d0000000                 movq	0xd0(%rax), %rax
// Frames: F10 -> F11 -> F12 -> F7 -> F8 -> F9 -> F6 -> F5
// 0x14198820f  498d7c8500                     leaq	(%r13,%rax,4), %rdi
// 0x141988214  8b421c                         movl	0x1c(%rdx), %eax
// 0x141988217  89442424                       movl	%eax, 0x24(%rsp)
// 0x14198821b  488b02                         movq	(%rdx), %rax
// 0x14198821e  488b4a10                       movq	0x10(%rdx), %rcx
// 0x141988222  48894c2458                     movq	%rcx, 0x58(%rsp)
// 0x141988227  4889442428                     movq	%rax, 0x28(%rsp)
// 0x14198822c  4883c020                       addq	$0x20, %rax
// 0x141988230  4889442450                     movq	%rax, 0x50(%rsp)
// 0x141988235  4531f6                         xorl	%r14d, %r14d
// 0x141988238  f30f103db8acd600               movss	0xd6acb8(%rip), %xmm7   # 0x1426f2ef8
// 0x141988240  f3440f100dbbacd600             movss	0xd6acbb(%rip), %xmm9   # 0x1426f2f04
// 0x141988249  f3440f10156e057c00             movss	0x7c056e(%rip), %xmm10  # 0x1421487c0
// 0x141988252  440f281dc6d47b00               movaps	0x7bd4c6(%rip), %xmm11  # 0x142145720
// 0x14198825a  f3440f1025a52b8300             movss	0x832ba5(%rip), %xmm12  # 0x1421bae08
// 0x141988263  f3440f103528f78a00             movss	0x8af728(%rip), %xmm14  # 0x142237994
// Frames: F5
// 0x14198826c  31c0                           xorl	%eax, %eax
// 0x14198826e  4889442468                     movq	%rax, 0x68(%rsp)
// 0x141988273  31ed                           xorl	%ebp, %ebp
// 0x141988275  31f6                           xorl	%esi, %esi
// Frames: F13 -> F14 -> F5
// 0x141988277  4839ee                         cmpq	%rbp, %rsi
// 0x14198827a  0f8580000000                   jne	0x141988300 <.text+0x1987300>
// Frames: F15 -> F16 -> F17 -> F14 -> F5
// 0x141988280  4939fd                         cmpq	%rdi, %r13
// Frames: F18 -> F17 -> F14 -> F5
// 0x141988283  0f84f9feffff                   je	0x141988182 <.text+0x1987182>
// Frames: F17 -> F14 -> F5
// 0x141988289  418b4d00                       movl	(%r13), %ecx
// Frames: F19 -> F20 -> F21 -> F17 -> F14 -> F5
// 0x14198828d  4983c504                       addq	$0x4, %r13
// Frames: F22 -> F23 -> F14 -> F5
// 0x141988291  498b842428010000               movq	0x128(%r12), %rax
// Frames: F24 -> F25 -> F22 -> F23 -> F14 -> F5
// 0x141988299  488d0cc9                       leaq	(%rcx,%rcx,8), %rcx
// Frames: F26 -> F27 -> F14 -> F5
// 0x14198829d  488b6cc810                     movq	0x10(%rax,%rcx,8), %rbp
// Frames: F13 -> F14 -> F5
// 0x1419882a2  4885ed                         testq	%rbp, %rbp
// 0x1419882a5  74d9                           je	0x141988280 <.text+0x1987280>
// Frames: F28 -> F29 -> F30 -> F31 -> F32 -> F14 -> F5
// 0x1419882a7  488d14c8                       leaq	(%rax,%rcx,8), %rdx
// 0x1419882ab  4c8b4c2470                     movq	0x70(%rsp), %r9
// 0x1419882b0  4d8b81d8000000                 movq	0xd8(%r9), %r8
// 0x1419882b7  4d8b89e0000000                 movq	0xe0(%r9), %r9
// 0x1419882be  488b44c838                     movq	0x38(%rax,%rcx,8), %rax
// 0x1419882c3  4a8b0cc0                       movq	(%rax,%r8,8), %rcx
// 0x1419882c7  48f7d1                         notq	%rcx
// 0x1419882ca  4a8b04c8                       movq	(%rax,%r9,8), %rax
// 0x1419882ce  48f7d0                         notq	%rax
// 0x1419882d1  488b5218                       movq	0x18(%rdx), %rdx
// 0x1419882d5  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1419882d9  48c1e105                       shlq	$0x5, %rcx
// 0x1419882dd  4c8b740a10                     movq	0x10(%rdx,%rcx), %r14
// 0x1419882e2  488b4c0a50                     movq	0x50(%rdx,%rcx), %rcx
// 0x1419882e7  48894c2430                     movq	%rcx, 0x30(%rsp)
// 0x1419882ec  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1419882f0  48c1e005                       shlq	$0x5, %rax
// 0x1419882f4  488b440210                     movq	0x10(%rdx,%rax), %rax
// 0x1419882f9  4889442468                     movq	%rax, 0x68(%rsp)
// 0x1419882fe  31f6                           xorl	%esi, %esi
// Frames: F33 -> F34 -> F35 -> F36 -> F14 -> F5
// 0x141988300  4189f7                         movl	%esi, %r15d
// Frames: F10 -> F37 -> F38 -> F35 -> F36 -> F14 -> F5
// 0x141988303  4b8d1cbf                       leaq	(%r15,%r15,4), %rbx
// 0x141988307  48c1e304                       shlq	$0x4, %rbx
// 0x14198830b  48035c2468                     addq	0x68(%rsp), %rbx
// Frames: F39 -> F14 -> F5
// 0x141988310  48ffc6                         incq	%rsi
// Frames: F40
// 0x141988313  4889d9                         movq	%rbx, %rcx
// 0x141988316  e84521d3fe                     callq	0x1406ba460 <.text+0x6b9460>
// 0x14198831b  84c0                           testb	%al, %al
// 0x14198831d  0f8554ffffff                   jne	0x141988277 <.text+0x1987277>
// Frames: F41
// 0x141988323  4b8d047f                       leaq	(%r15,%r15,2), %rax
// 0x141988327  4c89742478                     movq	%r14, 0x78(%rsp)
// 0x14198832c  4d8d34c6                       leaq	(%r14,%rax,8), %r14
// 0x141988330  488d4c243c                     leaq	0x3c(%rsp), %rcx
// 0x141988335  4889da                         movq	%rbx, %rdx
// 0x141988338  e8b320d3fe                     callq	0x1406ba3f0 <.text+0x6b93f0>
// Frames: F42
// 0x14198833d  f3440f106c2440                 movss	0x40(%rsp), %xmm13
// 0x141988344  488b442460                     movq	0x60(%rsp), %rax
// 0x141988349  f30f104028                     movss	0x28(%rax), %xmm0
// Frames: F43 -> F42
// 0x14198834e  f3410f1016                     movss	(%r14), %xmm2
// Frames: F44 -> F42
// 0x141988353  f3410f105e08                   movss	0x8(%r14), %xmm3
// 0x141988359  0f28ca                         movaps	%xmm2, %xmm1
// 0x14198835c  f30f5ec8                       divss	%xmm0, %xmm1
// 0x141988360  410f28e5                       movaps	%xmm13, %xmm4
// 0x141988364  f30f5ce2                       subss	%xmm2, %xmm4
// Frames: F45 -> F42
// 0x141988368  0f28ef                         movaps	%xmm7, %xmm5
// 0x14198836b  f30fc2e905                     cmpnltss	%xmm1, %xmm5
// 0x141988370  440f28c5                       movaps	%xmm5, %xmm8
// 0x141988374  440f54c3                       andps	%xmm3, %xmm8
// 0x141988378  410f55ed                       andnps	%xmm13, %xmm5
// 0x14198837c  410f56e8                       orps	%xmm8, %xmm5
// 0x141988380  f30fc2d605                     cmpnltss	%xmm6, %xmm2
// 0x141988385  440f28c2                       movaps	%xmm2, %xmm8
// 0x141988389  440f55c5                       andnps	%xmm5, %xmm8
// 0x14198838d  0f54d3                         andps	%xmm3, %xmm2
// 0x141988390  410f56d0                       orps	%xmm8, %xmm2
// 0x141988394  0f28eb                         movaps	%xmm3, %xmm5
// 0x141988397  f30fc2ec06                     cmpnless	%xmm4, %xmm5
// 0x14198839c  0f28e5                         movaps	%xmm5, %xmm4
// 0x14198839f  0f55e2                         andnps	%xmm2, %xmm4
// 0x1419883a2  0f54eb                         andps	%xmm3, %xmm5
// 0x1419883a5  0f56ec                         orps	%xmm4, %xmm5
// Frames: F46 -> F42
// 0x1419883a8  440f28f8                       movaps	%xmm0, %xmm15
// 0x1419883ac  f3440f593d47abd600             mulss	0xd6ab47(%rip), %xmm15  # 0x1426f2efc
// 0x1419883b5  f3450f587e0c                   addss	0xc(%r14), %xmm15
// Frames: F47 -> F42
// 0x1419883bb  440f28c0                       movaps	%xmm0, %xmm8
// 0x1419883bf  f3450f59c7                     mulss	%xmm15, %xmm8
// 0x1419883c4  f3440f58c5                     addss	%xmm5, %xmm8
// Frames: F48 -> F42
// 0x1419883c9  450f2ee8                       ucomiss	%xmm8, %xmm13
// 0x1419883cd  7252                           jb	0x141988421 <.text+0x1987421>
// Frames: F49 -> F50 -> F42
// 0x1419883cf  f3440f117c2448                 movss	%xmm15, 0x48(%rsp)
// 0x1419883d6  c744244c00007041               movl	$0x41700000, 0x4c(%rsp) # imm = 0x41700000
// 0x1419883de  450f2ef7                       ucomiss	%xmm15, %xmm14
// 0x1419883e2  0f8238020000                   jb	0x141988620 <.text+0x1987620>
// Frames: F50 -> F42
// 0x1419883e8  f30f590d10abd600               mulss	0xd6ab10(%rip), %xmm1   # 0x1426f2f00
// Frames: F51 -> F52 -> F42
// 0x1419883f0  410f28d5                       movaps	%xmm13, %xmm2
// 0x1419883f4  f3410f5fd0                     maxss	%xmm8, %xmm2
// 0x1419883f9  f3450fc2c003                   cmpunordss	%xmm8, %xmm8
// 0x1419883ff  410f28d8                       movaps	%xmm8, %xmm3
// 0x141988403  410f54dd                       andps	%xmm13, %xmm3
// 0x141988407  440f55c2                       andnps	%xmm2, %xmm8
// 0x14198840b  440f56c3                       orps	%xmm3, %xmm8
// Frames: F53 -> F50 -> F42
// 0x14198840f  f3440f5ff9                     maxss	%xmm1, %xmm15
// Frames: F54 -> F50 -> F42
// 0x141988414  410f28ce                       movaps	%xmm14, %xmm1
// 0x141988418  f3410f5dcf                     minss	%xmm15, %xmm1
// 0x14198841d  440f28f9                       movaps	%xmm1, %xmm15
// Frames: F55 -> F42
// 0x141988421  488b5c2430                     movq	0x30(%rsp), %rbx
// 0x141988426  f3410f59c1                     mulss	%xmm9, %xmm0
// Frames: F56 -> F55 -> F42
// 0x14198842b  e820c97b00                     callq	0x142144d50 <.text+0x2143d50>
// Frames: F55 -> F42
// 0x141988430  410f28ca                       movaps	%xmm10, %xmm1
// 0x141988434  f30f5cc8                       subss	%xmm0, %xmm1
// Frames: F57 -> F58 -> F59 -> F42
// 0x141988438  410f28c2                       movaps	%xmm10, %xmm0
// 0x14198843c  f30f5cc1                       subss	%xmm1, %xmm0
// Frames: F60 -> F58 -> F59 -> F42
// 0x141988440  f3410f59c0                     mulss	%xmm8, %xmm0
// 0x141988445  f3410f59cd                     mulss	%xmm13, %xmm1
// Frames: F61 -> F58 -> F59 -> F42
// 0x14198844a  f30f58c8                       addss	%xmm0, %xmm1
// Frames: F62
// 0x14198844e  f3410f114e08                   movss	%xmm1, 0x8(%r14)
// 0x141988454  8b442420                       movl	0x20(%rsp), %eax
// Frames: F63 -> F64 -> F65
// 0x141988458  428904bb                       movl	%eax, (%rbx,%r15,4)
// Frames: F65
// 0x14198845c  f3450f117e0c                   movss	%xmm15, 0xc(%r14)
// Frames: F66
// 0x141988462  41807e1400                     cmpb	$0x0, 0x14(%r14)
// 0x141988467  f3410f5ccd                     subss	%xmm13, %xmm1
// Frames: F67 -> F68
// 0x14198846c  410f54cb                       andps	%xmm11, %xmm1
// Frames: F66
// 0x141988470  7435                           je	0x1419884a7 <.text+0x19874a7>
// Frames: F69
// 0x141988472  410f2ecc                       ucomiss	%xmm12, %xmm1
// 0x141988476  7646                           jbe	0x1419884be <.text+0x19874be>
// Frames: F70
// 0x141988478  b001                           movb	$0x1, %al
// 0x14198847a  410f2ef7                       ucomiss	%xmm15, %xmm6
// 0x14198847e  0f8686010000                   jbe	0x14198860a <.text+0x198760a>
// Frames: F71
// 0x141988484  488b4c2460                     movq	0x60(%rsp), %rcx
// 0x141988489  f30f104128                     movss	0x28(%rcx), %xmm0
// 0x14198848e  8b4c2420                       movl	0x20(%rsp), %ecx
// Frames: F63 -> F64 -> F71
// 0x141988492  42890cbb                       movl	%ecx, (%rbx,%r15,4)
// Frames: F71
// 0x141988496  f3410f584610                   addss	0x10(%r14), %xmm0
// 0x14198849c  f3410f114610                   movss	%xmm0, 0x10(%r14)
// 0x1419884a2  e963010000                     jmp	0x14198860a <.text+0x198760a>
// Frames: F72
// 0x1419884a7  440f2ee1                       ucomiss	%xmm1, %xmm12
// 0x1419884ab  0f86d3000000                   jbe	0x141988584 <.text+0x1987584>
// Frames: F73
// 0x1419884b1  41c7461000000000               movl	$0x0, 0x10(%r14)
// 0x1419884b9  e9bf000000                     jmp	0x14198857d <.text+0x198757d>
// Frames: F74
// 0x1419884be  f3450f104610                   movss	0x10(%r14), %xmm8
// 0x1419884c4  440f2e05fcd58a00               ucomiss	0x8ad5fc(%rip), %xmm8   # 0x142235ac8
// 0x1419884cc  0f86ab000000                   jbe	0x14198857d <.text+0x198757d>
// Frames: F75
// 0x1419884d2  f3440f5805f1a28c00             addss	0x8ca2f1(%rip), %xmm8   # 0x1422527cc
// 0x1419884db  f3440f5e0524aad600             divss	0xd6aa24(%rip), %xmm8   # 0x1426f2f08
// Frames: F76
// 0x1419884e4  8b442444                       movl	0x44(%rsp), %eax
// 0x1419884e8  488d8c2488000000               leaq	0x88(%rsp), %rcx
// 0x1419884f0  894108                         movl	%eax, 0x8(%rcx)
// 0x1419884f3  488b44243c                     movq	0x3c(%rsp), %rax
// 0x1419884f8  488901                         movq	%rax, (%rcx)
// 0x1419884fb  8b442424                       movl	0x24(%rsp), %eax
// 0x1419884ff  488b4c2458                     movq	0x58(%rsp), %rcx
// Frames: F77 -> F78 -> F79 -> F76
// 0x141988504  8901                           movl	%eax, (%rcx)
// 0x141988506  488b442428                     movq	0x28(%rsp), %rax
// Frames: F80 -> F81 -> F79 -> F76
// 0x14198850b  488b5030                       movq	0x30(%rax), %rdx
// Frames: F82 -> F79 -> F76
// 0x14198850f  488b5840                       movq	0x40(%rax), %rbx
// Frames: F80 -> F81 -> F79 -> F76
// 0x141988513  483b5020                       cmpq	0x20(%rax), %rdx
// 0x141988517  7513                           jne	0x14198852c <.text+0x198752c>
// Frames: F83 -> F81 -> F79 -> F76
// 0x141988519  488b4c2450                     movq	0x50(%rsp), %rcx
// 0x14198851e  e8ed1f8ffe                     callq	0x14027a510 <.text+0x279510>
// 0x141988523  488b442428                     movq	0x28(%rsp), %rax
// Frames: F84 -> F81 -> F79 -> F76
// 0x141988528  488b5030                       movq	0x30(%rax), %rdx
// Frames: F85 -> F86 -> F84 -> F81 -> F79 -> F76
// 0x14198852c  0f57c0                         xorps	%xmm0, %xmm0
// 0x14198852f  f3410f5fc0                     maxss	%xmm8, %xmm0
// 0x141988534  410f28ca                       movaps	%xmm10, %xmm1
// 0x141988538  f30f5dc8                       minss	%xmm0, %xmm1
// 0x14198853c  4989c0                         movq	%rax, %r8
// 0x14198853f  488b4028                       movq	0x28(%rax), %rax
// Frames: F87 -> F84 -> F81 -> F79 -> F76
// 0x141988543  488d0c92                       leaq	(%rdx,%rdx,4), %rcx
// Frames: F88 -> F89 -> F81 -> F79 -> F76
// 0x141988547  48891cc8                       movq	%rbx, (%rax,%rcx,8)
// 0x14198854b  c644c80804                     movb	$0x4, 0x8(%rax,%rcx,8)
// 0x141988550  f30f114cc80c                   movss	%xmm1, 0xc(%rax,%rcx,8)
// 0x141988556  0f10842480000000               movups	0x80(%rsp), %xmm0
// 0x14198855e  0f1144c810                     movups	%xmm0, 0x10(%rax,%rcx,8)
// 0x141988563  8b942490000000                 movl	0x90(%rsp), %edx
// 0x14198856a  8954c820                       movl	%edx, 0x20(%rax,%rcx,8)
// Frames: F90 -> F81 -> F79 -> F76
// 0x14198856e  49ff4030                       incq	0x30(%r8)
// Frames: F91 -> F79 -> F76
// 0x141988572  49ff4040                       incq	0x40(%r8)
// 0x141988576  31c0                           xorl	%eax, %eax
// 0x141988578  e988000000                     jmp	0x141988605 <.text+0x1987605>
// 0x14198857d  31c0                           xorl	%eax, %eax
// 0x14198857f  e986000000                     jmp	0x14198860a <.text+0x198760a>
// Frames: F92
// 0x141988584  8b442444                       movl	0x44(%rsp), %eax
// 0x141988588  488d8c2488000000               leaq	0x88(%rsp), %rcx
// 0x141988590  89410f                         movl	%eax, 0xf(%rcx)
// 0x141988593  488b44243c                     movq	0x3c(%rsp), %rax
// 0x141988598  48894107                       movq	%rax, 0x7(%rcx)
// 0x14198859c  8b442424                       movl	0x24(%rsp), %eax
// 0x1419885a0  488b4c2458                     movq	0x58(%rsp), %rcx
// Frames: F77 -> F78 -> F79 -> F92
// 0x1419885a5  8901                           movl	%eax, (%rcx)
// 0x1419885a7  488b442428                     movq	0x28(%rsp), %rax
// Frames: F80 -> F81 -> F79 -> F92
// 0x1419885ac  488b5030                       movq	0x30(%rax), %rdx
// Frames: F82 -> F79 -> F92
// 0x1419885b0  488b5840                       movq	0x40(%rax), %rbx
// Frames: F80 -> F81 -> F79 -> F92
// 0x1419885b4  483b5020                       cmpq	0x20(%rax), %rdx
// 0x1419885b8  7513                           jne	0x1419885cd <.text+0x19875cd>
// Frames: F83 -> F81 -> F79 -> F92
// 0x1419885ba  488b4c2450                     movq	0x50(%rsp), %rcx
// 0x1419885bf  e84c1f8ffe                     callq	0x14027a510 <.text+0x279510>
// 0x1419885c4  488b442428                     movq	0x28(%rsp), %rax
// Frames: F84 -> F81 -> F79 -> F92
// 0x1419885c9  488b5030                       movq	0x30(%rax), %rdx
// Frames: F85 -> F86 -> F84 -> F81 -> F79 -> F92
// 0x1419885cd  4989c0                         movq	%rax, %r8
// 0x1419885d0  488b4028                       movq	0x28(%rax), %rax
// Frames: F87 -> F84 -> F81 -> F79 -> F92
// 0x1419885d4  488d0c92                       leaq	(%rdx,%rdx,4), %rcx
// Frames: F88 -> F89 -> F81 -> F79 -> F92
// 0x1419885d8  48891cc8                       movq	%rbx, (%rax,%rcx,8)
// 0x1419885dc  c644c80805                     movb	$0x5, 0x8(%rax,%rcx,8)
// 0x1419885e1  0f10842480000000               movups	0x80(%rsp), %xmm0
// 0x1419885e9  0f108c248b000000               movups	0x8b(%rsp), %xmm1
// 0x1419885f1  0f1144c809                     movups	%xmm0, 0x9(%rax,%rcx,8)
// 0x1419885f6  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// Frames: F90 -> F81 -> F79 -> F92
// 0x1419885fb  49ff4030                       incq	0x30(%r8)
// Frames: F91 -> F79 -> F92
// 0x1419885ff  49ff4040                       incq	0x40(%r8)
// 0x141988603  b001                           movb	$0x1, %al
// 0x141988605  488b5c2430                     movq	0x30(%rsp), %rbx
// Frames: F63 -> F64 -> F66
// 0x14198860a  8b4c2420                       movl	0x20(%rsp), %ecx
// 0x14198860e  42890cbb                       movl	%ecx, (%rbx,%r15,4)
// Frames: F66
// 0x141988612  41884614                       movb	%al, 0x14(%r14)
// 0x141988616  4c8b742478                     movq	0x78(%rsp), %r14
// 0x14198861b  e957fcffff                     jmp	0x141988277 <.text+0x1987277>
// Frames: F49 -> F50 -> F42
// 0x141988620  488d442448                     leaq	0x48(%rsp), %rax
// 0x141988625  48898424b0000000               movq	%rax, 0xb0(%rsp)
// 0x14198862d  488d05ac38effe                 leaq	-0x110c754(%rip), %rax  # 0x14087bee0 <.text+0x87aee0>
// 0x141988634  48898424b8000000               movq	%rax, 0xb8(%rsp)
// 0x14198863c  488d4c244c                     leaq	0x4c(%rsp), %rcx
// 0x141988641  48898c24c0000000               movq	%rcx, 0xc0(%rsp)
// 0x141988649  48898424c8000000               movq	%rax, 0xc8(%rsp)
// Frames: F93 -> F94 -> F50 -> F42
// 0x141988651  488d05e0a8d600                 leaq	0xd6a8e0(%rip), %rax    # 0x1426f2f38
// 0x141988658  4889842480000000               movq	%rax, 0x80(%rsp)
// 0x141988660  48c784248800000002000000       movq	$0x2, 0x88(%rsp)
// 0x14198866c  48c78424a000000000000000       movq	$0x0, 0xa0(%rsp)
// 0x141988678  488d8424b0000000               leaq	0xb0(%rsp), %rax
// 0x141988680  4889842490000000               movq	%rax, 0x90(%rsp)
// 0x141988688  48c784249800000002000000       movq	$0x2, 0x98(%rsp)
// Frames: F49 -> F50 -> F42
// 0x141988694  488d150da9d600                 leaq	0xd6a90d(%rip), %rdx    # 0x1426f2fa8
// 0x14198869b  488d8c2480000000               leaq	0x80(%rsp), %rcx
// 0x1419886a3  e8d8637200                     callq	0x1420aea80 <.text+0x20ada80>
// 0x1419886a8  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:14:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F2: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:21:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F3: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:85:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F4: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:476:0 bevy_ecs::system::query::Query::iter_mut
// F5: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:25:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F6: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs:477:0 bevy_ecs::system::query::Query::iter_mut
// F7: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1533:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F8: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:49:0 bevy_ecs::query::iter::QueryIter::new
// F9: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs:1246:0 bevy_ecs::query::state::QueryState::iter_unchecked_manual
// F10: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 core::ptr::mut_ptr::impl$0::add
// F11: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter.rs:95:0 core::slice::iter::Iter::new
// F12: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:1035:0 core::slice::impl$0::iter
// F13: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1596:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F14: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:886:0 bevy_ecs::query::iter::impl$1::next
// F15: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:1798:0 core::ptr::non_null::impl$12::eq
// F16: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$181::next
// F17: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1597:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F18: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:162:0 core::slice::iter::impl$181::next
// F19: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:626:0 core::ptr::non_null::NonNull::add
// F20: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:109:0 core::slice::iter::Iter::post_inc_start
// F21: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/iter/macros.rs:77:0 core::slice::iter::impl$181::next
// F22: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:835:0 bevy_ecs::storage::table::Tables::get
// F23: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1598:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F24: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:222:0 core::slice::index::impl$2::get
// F25: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:621:0 core::slice::impl$0::get
// F26: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:569:0 bevy_ecs::storage::table::Table::entities
// F27: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1605:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F28: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs:412:0 bevy_ecs::storage::blob_vec::BlobVec::get_slice
// F29: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:322:0 bevy_ecs::storage::table::Column::get_data_slice
// F30: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:957:0 bevy_ecs::query::fetch::impl$21::set_table
// F31: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs:196:0 bevy_ecs::query::world_query::impl$2::set_table
// F32: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1602:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F33: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs:127:0 bevy_ecs::storage::table::TableRow::as_usize
// F34: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:1317:0 bevy_ecs::query::fetch::impl$31::fetch
// F35: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs:208:0 bevy_ecs::query::world_query::impl$2::fetch
// F36: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1625:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F37: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs:506:0 bevy_ptr::ThinSlicePtr::get
// F38: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs:973:0 bevy_ecs::query::fetch::impl$21::fetch
// F39: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs:1627:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F40: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:26:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F41: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:30:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F42: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:33:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F43: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:89:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F44: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:90:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F45: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:96:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F46: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:109:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F47: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:110:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F48: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:113:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F49: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1520:0 core::f32::impl$0::clamp
// F50: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:117:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F51: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:925:0 core::f32::impl$0::max
// F52: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:115:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F53: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1521:0 core::f32::impl$0::clamp
// F54: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/f32.rs:1524:0 core::f32::impl$0::clamp
// F55: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:130:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F56: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/f32.rs:468:0 std::f32::impl$0::exp
// F57: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:214:0 core::ops::arith::impl$68::sub
// F58: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs:172:0 lerp::impl$0::lerp
// F59: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:132:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F60: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:344:0 core::ops::arith::impl$124::mul
// F61: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/arith.rs:105:0 core::ops::arith::impl$12::add
// F62: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:35:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F63: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:310:0 bevy_ecs::change_detection::impl$49::set_changed
// F64: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$50::deref_mut
// F65: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:36:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F66: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:38:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F67: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/f32.rs:199:0 std::f32::impl$0::abs
// F68: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:0:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F69: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:52:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F70: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:53:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F71: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:54:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F72: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:40:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F73: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:41:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F74: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:62:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F75: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:64:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F76: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:68:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F77: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:592:0 bevy_ecs::change_detection::impl$29::set_changed
// F78: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$30::deref_mut
// F79: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:625:0 bevy_ecs::event::EventWriter::send
// F80: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1970:0 alloc::vec::Vec::push
// F81: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:240:0 bevy_ecs::event::Events::send
// F82: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:233:0 bevy_ecs::event::Events::send
// F83: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1971:0 alloc::vec::Vec::push
// F84: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1974:0 alloc::vec::Vec::push
// F85: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:258:0 alloc::raw_vec::RawVec::ptr
// F86: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1365:0 alloc::vec::Vec::as_mut_ptr
// F87: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1049:0 alloc::vec::Vec::as_mut_ptr
// F88: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1493:0 core::ptr::write
// F89: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1975:0 alloc::vec::Vec::push
// F90: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1976:0 alloc::vec::Vec::push
// F91: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs:241:0 bevy_ecs::event::Events::send
// F92: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:44:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::ha9b3a28ad45ad43e
// F93: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/fmt/mod.rs:343:0 core::fmt::Arguments::new_v1
// F94: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/panic.rs:106:0 core::f32::impl$0::clamp
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/simulate_roof_tile_for_audio.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 6b0547e0ab685089fcf898f933c9e5b5a772284b3f06afd2fec0ce0b48f49d24
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: simulate_roof_tile_for_audio.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3195
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_roof; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3195,"record_offset":248,"rva":26771712,"va":5395480832}
// 编译实例 RVAs: 0x1988100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d50
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: simulate_roof_tile_for_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8112416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395480832,"raw_hex":"e820c97b00","size":5,"soft_fail":false,"target_va":5403594064,"va":5395481643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-24174611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395480832,"raw_hex":"e8ed1f8ffe","size":5,"soft_fail":false,"target_va":5371307280,"va":5395481886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7496664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395480832,"raw_hex":"e8d8637200","size":5,"soft_fail":false,"target_va":5402978944,"va":5395482275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19717819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395480832,"raw_hex":"e84521d3fe","size":5,"soft_fail":false,"target_va":5375763552,"va":5395481366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19717965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395480832,"raw_hex":"e8b320d3fe","size":5,"soft_fail":false,"target_va":5375763440,"va":5395481400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3195,"record_offset":1580,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3195,"record_offset":1720,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3195,"record_offset":624,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3195,"record_offset":424,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3195,"record_offset":1636,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_direct] system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/simulate_roof_tile_for_audio.rs
//     evidence={"module":3195,"record_offset":1224,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":101525,"source_line":88,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3195,"record_offset":1908,"parent_record_offset":1876,"ancestor_record_offset":248,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3195,"record_offset":1876,"parent_record_offset":1836,"ancestor_record_offset":248,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3195,"record_offset":1836,"parent_record_offset":1800,"ancestor_record_offset":248,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3195,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":248,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3195,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":248,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3195,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":248,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3195,"record_offset":1800,"parent_record_offset":1720,"ancestor_record_offset":248,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3195,"record_offset":1128,"parent_record_offset":1048,"ancestor_record_offset":248,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3195,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":248,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3195,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":248,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3195,"record_offset":476,"parent_record_offset":452,"ancestor_record_offset":248,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3195,"record_offset":500,"parent_record_offset":476,"ancestor_record_offset":248,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3195,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":248,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3195,"record_offset":452,"parent_record_offset":424,"ancestor_record_offset":248,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3195,"record_offset":1048,"parent_record_offset":648,"ancestor_record_offset":248,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3195,"record_offset":936,"parent_record_offset":648,"ancestor_record_offset":248,"inlinee":14619,"source_line":192,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3195,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":248,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3195,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":248,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3195,"record_offset":908,"parent_record_offset":648,"ancestor_record_offset":248,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3195,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":248,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3195,"record_offset":824,"parent_record_offset":648,"ancestor_record_offset":248,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3195,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":248,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3195,"record_offset":1292,"parent_record_offset":1224,"ancestor_record_offset":248,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3195,"record_offset":1380,"parent_record_offset":1224,"ancestor_record_offset":248,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3195,"record_offset":1348,"parent_record_offset":1292,"ancestor_record_offset":248,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3195,"record_offset":1488,"parent_record_offset":1436,"ancestor_record_offset":248,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3195,"record_offset":1544,"parent_record_offset":1436,"ancestor_record_offset":248,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3195,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":248,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3195,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3195,"record_offset":788,"parent_record_offset":764,"ancestor_record_offset":248,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3195,"record_offset":736,"parent_record_offset":700,"ancestor_record_offset":248,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3195,"record_offset":1940,"parent_record_offset":1836,"ancestor_record_offset":248,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3195,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":248,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3195,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":248,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3195,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":248,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3195,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":248,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3195,"record_offset":764,"parent_record_offset":700,"ancestor_record_offset":248,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3195,"record_offset":700,"parent_record_offset":648,"ancestor_record_offset":248,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3195,"record_offset":1436,"parent_record_offset":1224,"ancestor_record_offset":248,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3195,"record_offset":1408,"parent_record_offset":1224,"ancestor_record_offset":248,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18C95
// 首条位置: {"module":3195,"inlinee":101525}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3195,"record_offset":1292,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3195,"record_offset":1380,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3195,"record_offset":1436,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3195,"record_offset":1408,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3195,"record_offset":1348,"parent_record_offset":1292,"ancestor_record_offset":1224,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3195,"record_offset":1488,"parent_record_offset":1436,"ancestor_record_offset":1224,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3195,"record_offset":1544,"parent_record_offset":1436,"ancestor_record_offset":1224,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03195.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3195,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":1224,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03195.jsonl.gz"}
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
// 完整结构及原始类型记录: simulate_roof_tile_for_audio.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: simulate_roof_tile_for_audio.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: simulate_roof_tile_for_audio.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >

// ------------------------------------------------------------------
// 函数: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// 信息 ID: 39e75949ab0ab449effbb793
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio; module=3195, inline_record=1224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio
// 信息 ID: f28f1900c235cceec72d7530
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e512f3b422e4ecd930373d49: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >)> > @ 0x12d611a -> 0x1988100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >)> @ 0x12e0096 -> 0x1988100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
