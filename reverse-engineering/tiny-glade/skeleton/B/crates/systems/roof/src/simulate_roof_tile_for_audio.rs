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
// Inline site 3464:3856 -> host 3464:260 / rva_21a7090_len_676; exact_symbolizer_function_frame
//   Function-frame RVA span [0x21a743f, 0x21a751e), 54 instructions
//   Function-frame RVA span [0x21a76ee, 0x21a7706), 5 instructions
//
// FUNCTION system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio
// Candidate interfaces: e19dab4ee16ec2a583424ab7
// Static incoming reference records: 2; complete records in enriched/assembly sidecars.
// Compiled procedure 3464:260 -> rva_21a7090_len_676
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_21a7090_len_676: RVA 0x21a7090, 1654 PDB bytes; 357 instructions
// Evidence: _assembly/roof-v1/B/pack-001/rva_21a7090_len_676.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 3464:260: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio
// Frames: F1
// 0x1421a7090  4157                           pushq	%r15
// 0x1421a7092  4156                           pushq	%r14
// 0x1421a7094  4155                           pushq	%r13
// 0x1421a7096  4154                           pushq	%r12
// 0x1421a7098  56                             pushq	%rsi
// 0x1421a7099  57                             pushq	%rdi
// 0x1421a709a  55                             pushq	%rbp
// 0x1421a709b  53                             pushq	%rbx
// 0x1421a709c  4881ec58010000                 subq	$0x158, %rsp            # imm = 0x158
// 0x1421a70a3  440f29bc2440010000             movaps	%xmm15, 0x140(%rsp)
// 0x1421a70ac  440f29b42430010000             movaps	%xmm14, 0x130(%rsp)
// 0x1421a70b5  440f29ac2420010000             movaps	%xmm13, 0x120(%rsp)
// 0x1421a70be  440f29a42410010000             movaps	%xmm12, 0x110(%rsp)
// 0x1421a70c7  440f299c2400010000             movaps	%xmm11, 0x100(%rsp)
// 0x1421a70d0  440f299424f0000000             movaps	%xmm10, 0xf0(%rsp)
// 0x1421a70d9  440f298c24e0000000             movaps	%xmm9, 0xe0(%rsp)
// 0x1421a70e2  440f298424d0000000             movaps	%xmm8, 0xd0(%rsp)
// 0x1421a70eb  0f29bc24c0000000               movaps	%xmm7, 0xc0(%rsp)
// 0x1421a70f3  0f29b424b0000000               movaps	%xmm6, 0xb0(%rsp)
// Frames: F2
// 0x1421a70fb  4d8b38                         movq	(%r8), %r15
// 0x1421a70fe  f3410f104728                   movss	0x28(%r15), %xmm0
// 0x1421a7104  0f57ff                         xorps	%xmm7, %xmm7
// 0x1421a7107  0f2ec7                         ucomiss	%xmm7, %xmm0
// 0x1421a710a  756e                           jne	0x1421a717a <.text+0x21a617a>
// 0x1421a710c  7a6c                           jp	0x1421a717a <.text+0x21a617a>
// Frames: F3
// 0x1421a710e  0f28b424b0000000               movaps	0xb0(%rsp), %xmm6
// 0x1421a7116  0f28bc24c0000000               movaps	0xc0(%rsp), %xmm7
// 0x1421a711e  440f288424d0000000             movaps	0xd0(%rsp), %xmm8
// 0x1421a7127  440f288c24e0000000             movaps	0xe0(%rsp), %xmm9
// 0x1421a7130  440f289424f0000000             movaps	0xf0(%rsp), %xmm10
// 0x1421a7139  440f289c2400010000             movaps	0x100(%rsp), %xmm11
// 0x1421a7142  440f28a42410010000             movaps	0x110(%rsp), %xmm12
// 0x1421a714b  440f28ac2420010000             movaps	0x120(%rsp), %xmm13
// 0x1421a7154  440f28b42430010000             movaps	0x130(%rsp), %xmm14
// 0x1421a715d  440f28bc2440010000             movaps	0x140(%rsp), %xmm15
// 0x1421a7166  4881c458010000                 addq	$0x158, %rsp            # imm = 0x158
// 0x1421a716d  5b                             popq	%rbx
// 0x1421a716e  5d                             popq	%rbp
// 0x1421a716f  5f                             popq	%rdi
// 0x1421a7170  5e                             popq	%rsi
// 0x1421a7171  415c                           popq	%r12
// 0x1421a7173  415d                           popq	%r13
// 0x1421a7175  415e                           popq	%r14
// 0x1421a7177  415f                           popq	%r15
// 0x1421a7179  c3                             retq
// Frames: F4 -> F5 -> F6 -> F7 -> F8
// 0x1421a717a  4c8b01                         movq	(%rcx), %r8
// 0x1421a717d  488b4108                       movq	0x8(%rcx), %rax
// 0x1421a7181  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1421a7186  8b4114                         movl	0x14(%rcx), %eax
// 0x1421a7189  89442430                       movl	%eax, 0x30(%rsp)
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F15 -> F16 -> F17 -> F7 -> F8
// 0x1421a718d  498b98f8000000                 movq	0xf8(%r8), %rbx
// Frames: F13 -> F14 -> F15 -> F16 -> F17 -> F7 -> F8
// 0x1421a7194  498b8000010000                 movq	0x100(%r8), %rax
// Frames: F18 -> F19 -> F20 -> F15 -> F16 -> F17 -> F7 -> F8
// 0x1421a719b  488d3c83                       leaq	(%rbx,%rax,4), %rdi
// 0x1421a719f  4c89442450                     movq	%r8, 0x50(%rsp)
// Frames: F21 -> F16 -> F17 -> F7 -> F8
// 0x1421a71a4  410fb68008010000               movzbl	0x108(%r8), %eax
// 0x1421a71ac  88442427                       movb	%al, 0x27(%rsp)
// 0x1421a71b0  8b421c                         movl	0x1c(%rdx), %eax
// 0x1421a71b3  89442434                       movl	%eax, 0x34(%rsp)
// 0x1421a71b7  488b02                         movq	(%rdx), %rax
// 0x1421a71ba  488b4a10                       movq	0x10(%rdx), %rcx
// 0x1421a71be  48898c2480000000               movq	%rcx, 0x80(%rsp)
// 0x1421a71c6  4889442438                     movq	%rax, 0x38(%rsp)
// 0x1421a71cb  488d4020                       leaq	0x20(%rax), %rax
// 0x1421a71cf  4889442478                     movq	%rax, 0x78(%rsp)
// 0x1421a71d4  b808000000                     movl	$0x8, %eax
// 0x1421a71d9  4889842488000000               movq	%rax, 0x88(%rsp)
// 0x1421a71e1  48c744242800000000             movq	$0x0, 0x28(%rsp)
// 0x1421a71ea  f3440f100595fae400             movss	0xe4fa95(%rip), %xmm8   # 0x142ff6c88
// 0x1421a71f3  f3440f101598fae400             movss	0xe4fa98(%rip), %xmm10  # 0x142ff6c94
// 0x1421a71fc  f3440f101dffe67700             movss	0x77e6ff(%rip), %xmm11  # 0x142925904
// 0x1421a7205  440f282573777800               movaps	0x787773(%rip), %xmm12  # 0x14292e980
// 0x1421a720d  f3440f102d6ee77700             movss	0x77e76e(%rip), %xmm13  # 0x142925984
// 0x1421a7216  f3440f103d4d5e8200             movss	0x825e4d(%rip), %xmm15  # 0x1429cd06c
// Frames: F8
// 0x1421a721f  48c744244000000000             movq	$0x0, 0x40(%rsp)
// 0x1421a7228  31ed                           xorl	%ebp, %ebp
// 0x1421a722a  4531e4                         xorl	%r12d, %r12d
// Frames: F22 -> F23 -> F8
// 0x1421a722d  807c242700                     cmpb	$0x0, 0x27(%rsp)
// 0x1421a7232  744c                           je	0x1421a7280 <.text+0x21a6280>
// Frames: F24 -> F23 -> F8
// 0x1421a7234  4939ec                         cmpq	%rbp, %r12
// 0x1421a7237  0f8553010000                   jne	0x1421a7390 <.text+0x21a6390>
// Frames: F25 -> F26 -> F27 -> F23 -> F8
// 0x1421a723d  4839fb                         cmpq	%rdi, %rbx
// Frames: F28 -> F27 -> F23 -> F8
// 0x1421a7240  0f84c8feffff                   je	0x1421a710e <.text+0x21a610e>
// 0x1421a7246  488b442458                     movq	0x58(%rsp), %rax
// 0x1421a724b  488b80a8010000                 movq	0x1a8(%rax), %rax
// 0x1421a7252  66666666662e0f1f840000000000   nopw	%cs:(%rax,%rax)
// Frames: F27 -> F23 -> F8
// 0x1421a7260  8b0b                           movl	(%rbx), %ecx
// Frames: F29 -> F30 -> F31 -> F32 -> F23 -> F8
// 0x1421a7262  488d0cc9                       leaq	(%rcx,%rcx,8), %rcx
// Frames: F33 -> F34 -> F35 -> F36 -> F23 -> F8
// 0x1421a7266  488b6cc810                     movq	0x10(%rax,%rcx,8), %rbp
// Frames: F36 -> F23 -> F8
// 0x1421a726b  4885ed                         testq	%rbp, %rbp
// 0x1421a726e  7552                           jne	0x1421a72c2 <.text+0x21a62c2>
// Frames: F25 -> F26 -> F27 -> F23 -> F8
// 0x1421a7270  4883c304                       addq	$0x4, %rbx
// 0x1421a7274  4839fb                         cmpq	%rdi, %rbx
// 0x1421a7277  75e7                           jne	0x1421a7260 <.text+0x21a6260>
// 0x1421a7279  e990feffff                     jmp	0x1421a710e <.text+0x21a610e>
// 0x1421a727e  6690                           nop
// Frames: F37 -> F23 -> F8
// 0x1421a7280  4939ec                         cmpq	%rbp, %r12
// 0x1421a7283  0f8557010000                   jne	0x1421a73e0 <.text+0x21a63e0>
// Frames: F25 -> F26 -> F38 -> F23 -> F8
// 0x1421a7289  4839fb                         cmpq	%rdi, %rbx
// Frames: F28 -> F38 -> F23 -> F8
// 0x1421a728c  0f847cfeffff                   je	0x1421a710e <.text+0x21a610e>
// 0x1421a7292  488b442458                     movq	0x58(%rsp), %rax
// 0x1421a7297  488b8000010000                 movq	0x100(%rax), %rax
// 0x1421a729e  6690                           nop
// Frames: F38 -> F23 -> F8
// 0x1421a72a0  8b0b                           movl	(%rbx), %ecx
// Frames: F29 -> F30 -> F39 -> F40 -> F23 -> F8
// 0x1421a72a2  488d0c89                       leaq	(%rcx,%rcx,4), %rcx
// 0x1421a72a6  48c1e105                       shlq	$0x5, %rcx
// Frames: F33 -> F34 -> F41 -> F42 -> F23 -> F8
// 0x1421a72aa  488b6c0858                     movq	0x58(%rax,%rcx), %rbp
// Frames: F42 -> F23 -> F8
// 0x1421a72af  4885ed                         testq	%rbp, %rbp
// 0x1421a72b2  757a                           jne	0x1421a732e <.text+0x21a632e>
// Frames: F25 -> F26 -> F38 -> F23 -> F8
// 0x1421a72b4  4883c304                       addq	$0x4, %rbx
// 0x1421a72b8  4839fb                         cmpq	%rdi, %rbx
// Frames: F28 -> F38 -> F23 -> F8
// 0x1421a72bb  75e3                           jne	0x1421a72a0 <.text+0x21a62a0>
// 0x1421a72bd  e94cfeffff                     jmp	0x1421a710e <.text+0x21a610e>
// Frames: F43 -> F23 -> F8
// 0x1421a72c2  4c8d04c8                       leaq	(%rax,%rcx,8), %r8
// 0x1421a72c6  488b442450                     movq	0x50(%rsp), %rax
// 0x1421a72cb  4c8b8810010000                 movq	0x110(%rax), %r9
// 0x1421a72d2  488b8818010000                 movq	0x118(%rax), %rcx
// Frames: F44 -> F45 -> F46 -> F47 -> F43 -> F23 -> F8
// 0x1421a72d9  498b4018                       movq	0x18(%r8), %rax
// 0x1421a72dd  498b5038                       movq	0x38(%r8), %rdx
// Frames: F48 -> F49 -> F50 -> F51 -> F44 -> F45 -> F46 -> F47 -> F43 -> F23 -> F8
// 0x1421a72e1  4e8b0cca                       movq	(%rdx,%r9,8), %r9
// Frames: F52 -> F53 -> F54 -> F44 -> F45 -> F46 -> F47 -> F43 -> F23 -> F8
// 0x1421a72e5  49f7d1                         notq	%r9
// Frames: F55 -> F56 -> F57 -> F53 -> F54 -> F44 -> F45 -> F46 -> F47 -> F43 -> F23 -> F8
// 0x1421a72e8  4f8d0c49                       leaq	(%r9,%r9,2), %r9
// 0x1421a72ec  49c1e104                       shlq	$0x4, %r9
// Frames: F58 -> F59 -> F60 -> F47 -> F43 -> F23 -> F8
// 0x1421a72f0  4e8b540810                     movq	0x10(%rax,%r9), %r10
// 0x1421a72f5  4c89542440                     movq	%r10, 0x40(%rsp)
// Frames: F61 -> F62 -> F63 -> F47 -> F43 -> F23 -> F8
// 0x1421a72fa  4e8b4c0828                     movq	0x28(%rax,%r9), %r9
// 0x1421a72ff  4c894c2460                     movq	%r9, 0x60(%rsp)
// Frames: F29 -> F30 -> F51 -> F44 -> F45 -> F64 -> F65 -> F47 -> F43 -> F23 -> F8
// 0x1421a7304  4883c304                       addq	$0x4, %rbx
// 0x1421a7308  49394840                       cmpq	%rcx, 0x40(%r8)
// 0x1421a730c  7676                           jbe	0x1421a7384 <.text+0x21a6384>
// Frames: F48 -> F49 -> F66 -> F51 -> F44 -> F45 -> F64 -> F65 -> F47 -> F43 -> F23 -> F8
// 0x1421a730e  488b0cca                       movq	(%rdx,%rcx,8), %rcx
// 0x1421a7312  4885c9                         testq	%rcx, %rcx
// Frames: F66 -> F51 -> F44 -> F45 -> F64 -> F65 -> F47 -> F43 -> F23 -> F8
// 0x1421a7315  746d                           je	0x1421a7384 <.text+0x21a6384>
// Frames: F52 -> F53 -> F54 -> F44 -> F45 -> F64 -> F65 -> F47 -> F43 -> F23 -> F8
// 0x1421a7317  48f7d1                         notq	%rcx
// Frames: F58 -> F59 -> F67 -> F54 -> F68 -> F65 -> F47 -> F43 -> F23 -> F8
// 0x1421a731a  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1421a731e  48c1e104                       shlq	$0x4, %rcx
// 0x1421a7322  488b440810                     movq	0x10(%rax,%rcx), %rax
// 0x1421a7327  4889442428                     movq	%rax, 0x28(%rsp)
// 0x1421a732c  eb5f                           jmp	0x1421a738d <.text+0x21a638d>
// Frames: F69 -> F70 -> F23 -> F8
// 0x1421a732e  4801c8                         addq	%rcx, %rax
// 0x1421a7331  8b8894000000                   movl	0x94(%rax), %ecx
// 0x1421a7337  488b542458                     movq	0x58(%rsp), %rdx
// Frames: F29 -> F30 -> F71 -> F70 -> F23 -> F8
// 0x1421a733c  4c8b8aa8010000                 movq	0x1a8(%rdx), %r9
// 0x1421a7343  4c8d14c9                       leaq	(%rcx,%rcx,8), %r10
// 0x1421a7347  488b4c2450                     movq	0x50(%rsp), %rcx
// Frames: F72 -> F23 -> F8
// 0x1421a734c  4c8b8118010000                 movq	0x118(%rcx), %r8
// Frames: F44 -> F45 -> F46 -> F73 -> F74 -> F72 -> F23 -> F8
// 0x1421a7353  4b8b4cd118                     movq	0x18(%r9,%r10,8), %rcx
// 0x1421a7358  4b8b54d138                     movq	0x38(%r9,%r10,8), %rdx
// Frames: F29 -> F30 -> F51 -> F44 -> F45 -> F64 -> F65 -> F75 -> F74 -> F72 -> F23 -> F8
// 0x1421a735d  4f3944d140                     cmpq	%r8, 0x40(%r9,%r10,8)
// 0x1421a7362  7631                           jbe	0x1421a7395 <.text+0x21a6395>
// Frames: F48 -> F49 -> F66 -> F51 -> F44 -> F45 -> F64 -> F65 -> F75 -> F74 -> F72 -> F23 -> F8
// 0x1421a7364  4e8b04c2                       movq	(%rdx,%r8,8), %r8
// 0x1421a7368  4d85c0                         testq	%r8, %r8
// Frames: F66 -> F51 -> F44 -> F45 -> F64 -> F65 -> F75 -> F74 -> F72 -> F23 -> F8
// 0x1421a736b  7428                           je	0x1421a7395 <.text+0x21a6395>
// Frames: F52 -> F53 -> F54 -> F44 -> F45 -> F64 -> F65 -> F75 -> F74 -> F72 -> F23 -> F8
// 0x1421a736d  49f7d0                         notq	%r8
// Frames: F58 -> F59 -> F67 -> F54 -> F68 -> F65 -> F75 -> F74 -> F72 -> F23 -> F8
// 0x1421a7370  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x1421a7374  49c1e004                       shlq	$0x4, %r8
// 0x1421a7378  4e8b440110                     movq	0x10(%rcx,%r8), %r8
// 0x1421a737d  4c89442428                     movq	%r8, 0x28(%rsp)
// 0x1421a7382  eb1a                           jmp	0x1421a739e <.text+0x21a639e>
// 0x1421a7384  48c744242800000000             movq	$0x0, 0x28(%rsp)
// 0x1421a738d  4531e4                         xorl	%r12d, %r12d
// Frames: F76 -> F77 -> F78 -> F79 -> F80 -> F81 -> F23 -> F8
// 0x1421a7390  4489e0                         movl	%r12d, %eax
// 0x1421a7393  eb5e                           jmp	0x1421a73f3 <.text+0x21a63f3>
// 0x1421a7395  48c744242800000000             movq	$0x0, 0x28(%rsp)
// Frames: F9 -> F10 -> F11 -> F12 -> F13 -> F14 -> F82 -> F83 -> F23 -> F8
// 0x1421a739e  4c8b442450                     movq	0x50(%rsp), %r8
// 0x1421a73a3  4d8b8010010000                 movq	0x110(%r8), %r8
// 0x1421a73aa  4a8b14c2                       movq	(%rdx,%r8,8), %rdx
// 0x1421a73ae  48f7d2                         notq	%rdx
// 0x1421a73b1  488d1452                       leaq	(%rdx,%rdx,2), %rdx
// 0x1421a73b5  48c1e204                       shlq	$0x4, %rdx
// 0x1421a73b9  4c8b441110                     movq	0x10(%rcx,%rdx), %r8
// 0x1421a73be  4c89442440                     movq	%r8, 0x40(%rsp)
// 0x1421a73c3  488b4c1128                     movq	0x28(%rcx,%rdx), %rcx
// 0x1421a73c8  48894c2460                     movq	%rcx, 0x60(%rsp)
// 0x1421a73cd  4883c304                       addq	$0x4, %rbx
// 0x1421a73d1  488b4050                       movq	0x50(%rax), %rax
// 0x1421a73d5  4889842488000000               movq	%rax, 0x88(%rsp)
// 0x1421a73dd  4531e4                         xorl	%r12d, %r12d
// Frames: F84 -> F85 -> F23 -> F8
// 0x1421a73e0  4c89e0                         movq	%r12, %rax
// 0x1421a73e3  48c1e004                       shlq	$0x4, %rax
// 0x1421a73e7  488b8c2488000000               movq	0x88(%rsp), %rcx
// Frames: F84 -> F86 -> F23 -> F8
// 0x1421a73ef  8b440108                       movl	0x8(%rcx,%rax), %eax
// Frames: F18 -> F87 -> F77 -> F78 -> F79 -> F80 -> F88 -> F23 -> F8
// 0x1421a73f3  488d0c40                       leaq	(%rax,%rax,2), %rcx
// 0x1421a73f7  488b542440                     movq	0x40(%rsp), %rdx
// 0x1421a73fc  488d34ca                       leaq	(%rdx,%rcx,8), %rsi
// 0x1421a7400  488b4c2460                     movq	0x60(%rsp), %rcx
// Frames: F18 -> F87 -> F89 -> F78 -> F79 -> F80 -> F88 -> F23 -> F8
// 0x1421a7405  4c8d2c81                       leaq	(%rcx,%rax,4), %r13
// Frames: F18 -> F87 -> F90 -> F78 -> F91 -> F80 -> F88 -> F23 -> F8
// 0x1421a7409  4c6bf058                       imulq	$0x58, %rax, %r14
// 0x1421a740d  4c03742428                     addq	0x28(%rsp), %r14
// 0x1421a7412  49ffc4                         incq	%r12
// Frames: F92
// 0x1421a7415  4c89f1                         movq	%r14, %rcx
// 0x1421a7418  e863bf73fe                     callq	0x1408e3380 <.text+0x8e2380>
// 0x1421a741d  84c0                           testb	%al, %al
// 0x1421a741f  0f8508feffff                   jne	0x1421a722d <.text+0x21a622d>
// Frames: F93
// 0x1421a7425  488d4c246c                     leaq	0x6c(%rsp), %rcx
// 0x1421a742a  4c89f2                         movq	%r14, %rdx
// 0x1421a742d  e8eebd73fe                     callq	0x1408e3220 <.text+0x8e2220>
// Frames: F94
// 0x1421a7432  f3440f10742470                 movss	0x70(%rsp), %xmm14
// 0x1421a7439  f3410f104728                   movss	0x28(%r15), %xmm0
// Frames: F95 -> F94
// 0x1421a743f  f30f1016                       movss	(%rsi), %xmm2
// Frames: F96 -> F94
// 0x1421a7443  f30f105e08                     movss	0x8(%rsi), %xmm3
// 0x1421a7448  0f28ca                         movaps	%xmm2, %xmm1
// 0x1421a744b  f30f5ec8                       divss	%xmm0, %xmm1
// 0x1421a744f  410f28e6                       movaps	%xmm14, %xmm4
// 0x1421a7453  f30f5ce2                       subss	%xmm2, %xmm4
// Frames: F97 -> F94
// 0x1421a7457  410f28e8                       movaps	%xmm8, %xmm5
// 0x1421a745b  f30fc2e905                     cmpnltss	%xmm1, %xmm5
// 0x1421a7460  0f28f5                         movaps	%xmm5, %xmm6
// 0x1421a7463  0f54f3                         andps	%xmm3, %xmm6
// 0x1421a7466  410f55ee                       andnps	%xmm14, %xmm5
// 0x1421a746a  0f56ee                         orps	%xmm6, %xmm5
// 0x1421a746d  f30fc2d705                     cmpnltss	%xmm7, %xmm2
// 0x1421a7472  0f28f2                         movaps	%xmm2, %xmm6
// 0x1421a7475  0f55f5                         andnps	%xmm5, %xmm6
// 0x1421a7478  0f54d3                         andps	%xmm3, %xmm2
// 0x1421a747b  0f56d6                         orps	%xmm6, %xmm2
// 0x1421a747e  0f28eb                         movaps	%xmm3, %xmm5
// 0x1421a7481  f30fc2ec06                     cmpnless	%xmm4, %xmm5
// 0x1421a7486  0f28e5                         movaps	%xmm5, %xmm4
// 0x1421a7489  0f55e2                         andnps	%xmm2, %xmm4
// 0x1421a748c  0f54eb                         andps	%xmm3, %xmm5
// 0x1421a748f  0f56ec                         orps	%xmm4, %xmm5
// Frames: F98 -> F94
// 0x1421a7492  0f28f0                         movaps	%xmm0, %xmm6
// 0x1421a7495  f30f5935eff7e400               mulss	0xe4f7ef(%rip), %xmm6   # 0x142ff6c8c
// 0x1421a749d  f30f58760c                     addss	0xc(%rsi), %xmm6
// Frames: F99 -> F94
// 0x1421a74a2  440f28c8                       movaps	%xmm0, %xmm9
// 0x1421a74a6  f3440f59ce                     mulss	%xmm6, %xmm9
// 0x1421a74ab  f3440f58cd                     addss	%xmm5, %xmm9
// Frames: F100 -> F94
// 0x1421a74b0  450f2ef1                       ucomiss	%xmm9, %xmm14
// 0x1421a74b4  7240                           jb	0x1421a74f6 <.text+0x21a64f6>
// Frames: F101 -> F102 -> F94
// 0x1421a74b6  440f2efe                       ucomiss	%xmm6, %xmm15
// Frames: F103 -> F104 -> F102 -> F94
// 0x1421a74ba  0f822e020000                   jb	0x1421a76ee <.text+0x21a66ee>
// Frames: F102 -> F94
// 0x1421a74c0  f30f590dc8f7e400               mulss	0xe4f7c8(%rip), %xmm1   # 0x142ff6c90
// Frames: F105 -> F106 -> F94
// 0x1421a74c8  410f28d6                       movaps	%xmm14, %xmm2
// 0x1421a74cc  f3410f5fd1                     maxss	%xmm9, %xmm2
// 0x1421a74d1  f3450fc2c903                   cmpunordss	%xmm9, %xmm9
// 0x1421a74d7  410f28d9                       movaps	%xmm9, %xmm3
// 0x1421a74db  410f54de                       andps	%xmm14, %xmm3
// 0x1421a74df  440f55ca                       andnps	%xmm2, %xmm9
// 0x1421a74e3  440f56cb                       orps	%xmm3, %xmm9
// Frames: F107 -> F102 -> F94
// 0x1421a74e7  f30f5ff1                       maxss	%xmm1, %xmm6
// Frames: F108 -> F102 -> F94
// 0x1421a74eb  410f28cf                       movaps	%xmm15, %xmm1
// 0x1421a74ef  f30f5dce                       minss	%xmm6, %xmm1
// 0x1421a74f3  0f28f1                         movaps	%xmm1, %xmm6
// Frames: F109 -> F94
// 0x1421a74f6  f3410f59c2                     mulss	%xmm10, %xmm0
// Frames: F110 -> F109 -> F94
// 0x1421a74fb  e8c0c97700                     callq	0x142923ec0 <.text+0x2922ec0>
// Frames: F109 -> F94
// 0x1421a7500  410f28cb                       movaps	%xmm11, %xmm1
// 0x1421a7504  f30f5cc8                       subss	%xmm0, %xmm1
// Frames: F111 -> F112 -> F113 -> F94
// 0x1421a7508  410f28c3                       movaps	%xmm11, %xmm0
// 0x1421a750c  f30f5cc1                       subss	%xmm1, %xmm0
// Frames: F114 -> F112 -> F113 -> F94
// 0x1421a7510  f3410f59c1                     mulss	%xmm9, %xmm0
// 0x1421a7515  f3410f59ce                     mulss	%xmm14, %xmm1
// Frames: F115 -> F112 -> F113 -> F94
// 0x1421a751a  f30f58c8                       addss	%xmm0, %xmm1
// Frames: F116
// 0x1421a751e  f30f114e08                     movss	%xmm1, 0x8(%rsi)
// 0x1421a7523  8b442430                       movl	0x30(%rsp), %eax
// Frames: F117 -> F118 -> F119
// 0x1421a7527  41894500                       movl	%eax, (%r13)
// Frames: F119
// 0x1421a752b  f30f11760c                     movss	%xmm6, 0xc(%rsi)
// 0x1421a7530  f3410f5cce                     subss	%xmm14, %xmm1
// Frames: F120 -> F121
// 0x1421a7535  410f54cc                       andps	%xmm12, %xmm1
// Frames: F122
// 0x1421a7539  807e1400                       cmpb	$0x0, 0x14(%rsi)
// 0x1421a753d  742e                           je	0x1421a756d <.text+0x21a656d>
// Frames: F123
// 0x1421a753f  410f2ecd                       ucomiss	%xmm13, %xmm1
// 0x1421a7543  763e                           jbe	0x1421a7583 <.text+0x21a6583>
// Frames: F124
// 0x1421a7545  b001                           movb	$0x1, %al
// 0x1421a7547  0f2efe                         ucomiss	%xmm6, %xmm7
// 0x1421a754a  0f868e010000                   jbe	0x1421a76de <.text+0x21a66de>
// Frames: F125
// 0x1421a7550  f3410f104728                   movss	0x28(%r15), %xmm0
// 0x1421a7556  8b4c2430                       movl	0x30(%rsp), %ecx
// Frames: F117 -> F118 -> F125
// 0x1421a755a  41894d00                       movl	%ecx, (%r13)
// Frames: F125
// 0x1421a755e  f30f584610                     addss	0x10(%rsi), %xmm0
// 0x1421a7563  f30f114610                     movss	%xmm0, 0x10(%rsi)
// Frames: F124
// 0x1421a7568  e971010000                     jmp	0x1421a76de <.text+0x21a66de>
// Frames: F126
// 0x1421a756d  440f2ee9                       ucomiss	%xmm1, %xmm13
// 0x1421a7571  0f86d7000000                   jbe	0x1421a764e <.text+0x21a664e>
// Frames: F127
// 0x1421a7577  c7461000000000                 movl	$0x0, 0x10(%rsi)
// 0x1421a757e  e9c4000000                     jmp	0x1421a7647 <.text+0x21a6647>
// Frames: F128
// 0x1421a7583  f30f107610                     movss	0x10(%rsi), %xmm6
// 0x1421a7588  0f2e35e5907b00                 ucomiss	0x7b90e5(%rip), %xmm6   # 0x142960674
// 0x1421a758f  0f86b2000000                   jbe	0x1421a7647 <.text+0x21a6647>
// Frames: F129
// 0x1421a7595  f30f583573ae8d00               addss	0x8dae73(%rip), %xmm6   # 0x142a82410
// 0x1421a759d  f30f5e35ab508e00               divss	0x8e50ab(%rip), %xmm6   # 0x142a8c650
// Frames: F130
// 0x1421a75a5  8b442474                       movl	0x74(%rsp), %eax
// 0x1421a75a9  488d8c24a3000000               leaq	0xa3(%rsp), %rcx
// 0x1421a75b1  894101                         movl	%eax, 0x1(%rcx)
// 0x1421a75b4  488b44246c                     movq	0x6c(%rsp), %rax
// 0x1421a75b9  488941f9                       movq	%rax, -0x7(%rcx)
// 0x1421a75bd  8b442434                       movl	0x34(%rsp), %eax
// 0x1421a75c1  488b8c2480000000               movq	0x80(%rsp), %rcx
// Frames: F131 -> F132 -> F133 -> F130
// 0x1421a75c9  8901                           movl	%eax, (%rcx)
// 0x1421a75cb  488b442438                     movq	0x38(%rsp), %rax
// Frames: F134 -> F135 -> F136 -> F133 -> F130
// 0x1421a75d0  4c8b7030                       movq	0x30(%rax), %r14
// Frames: F137 -> F136 -> F133 -> F130
// 0x1421a75d4  488b5040                       movq	0x40(%rax), %rdx
// Frames: F138 -> F135 -> F136 -> F133 -> F130
// 0x1421a75d8  4c3b7020                       cmpq	0x20(%rax), %r14
// 0x1421a75dc  751b                           jne	0x1421a75f9 <.text+0x21a65f9>
// Frames: F139 -> F135 -> F136 -> F133 -> F130
// 0x1421a75de  488b4c2478                     movq	0x78(%rsp), %rcx
// 0x1421a75e3  4889542448                     movq	%rdx, 0x48(%rsp)
// 0x1421a75e8  488d1581b79c00                 leaq	0x9cb781(%rip), %rdx    # 0x142b72d70
// 0x1421a75ef  e8bcb43bfe                     callq	0x140562ab0 <.text+0x561ab0>
// 0x1421a75f4  488b542448                     movq	0x48(%rsp), %rdx
// Frames: F9 -> F10 -> F11 -> F140 -> F141 -> F135 -> F136 -> F133 -> F130
// 0x1421a75f9  0f57c0                         xorps	%xmm0, %xmm0
// 0x1421a75fc  f30f5fc6                       maxss	%xmm6, %xmm0
// 0x1421a7600  410f28cb                       movaps	%xmm11, %xmm1
// 0x1421a7604  f30f5dc8                       minss	%xmm0, %xmm1
// 0x1421a7608  4c8b442438                     movq	0x38(%rsp), %r8
// 0x1421a760d  498b4028                       movq	0x28(%r8), %rax
// Frames: F142 -> F141 -> F135 -> F136 -> F133 -> F130
// 0x1421a7611  4b8d0cb6                       leaq	(%r14,%r14,4), %rcx
// Frames: F143 -> F144 -> F135 -> F136 -> F133 -> F130
// 0x1421a7615  488914c8                       movq	%rdx, (%rax,%rcx,8)
// 0x1421a7619  c644c80804                     movb	$0x4, 0x8(%rax,%rcx,8)
// 0x1421a761e  f30f114cc80c                   movss	%xmm1, 0xc(%rax,%rcx,8)
// 0x1421a7624  8b9424a4000000                 movl	0xa4(%rsp), %edx
// 0x1421a762b  8954c820                       movl	%edx, 0x20(%rax,%rcx,8)
// 0x1421a762f  0f10842494000000               movups	0x94(%rsp), %xmm0
// 0x1421a7637  0f1144c810                     movups	%xmm0, 0x10(%rax,%rcx,8)
// Frames: F145 -> F135 -> F136 -> F133 -> F130
// 0x1421a763c  49ffc6                         incq	%r14
// 0x1421a763f  4d897030                       movq	%r14, 0x30(%r8)
// Frames: F146 -> F136 -> F133 -> F130
// 0x1421a7643  49ff4040                       incq	0x40(%r8)
// 0x1421a7647  31c0                           xorl	%eax, %eax
// 0x1421a7649  e990000000                     jmp	0x1421a76de <.text+0x21a66de>
// Frames: F147
// 0x1421a764e  4c897c2448                     movq	%r15, 0x48(%rsp)
// 0x1421a7653  8b442474                       movl	0x74(%rsp), %eax
// 0x1421a7657  488d8c24a3000000               leaq	0xa3(%rsp), %rcx
// 0x1421a765f  894108                         movl	%eax, 0x8(%rcx)
// 0x1421a7662  488b44246c                     movq	0x6c(%rsp), %rax
// 0x1421a7667  488901                         movq	%rax, (%rcx)
// 0x1421a766a  8b442434                       movl	0x34(%rsp), %eax
// 0x1421a766e  488b8c2480000000               movq	0x80(%rsp), %rcx
// Frames: F131 -> F132 -> F133 -> F147
// 0x1421a7676  8901                           movl	%eax, (%rcx)
// 0x1421a7678  488b442438                     movq	0x38(%rsp), %rax
// Frames: F134 -> F135 -> F136 -> F133 -> F147
// 0x1421a767d  4c8b7030                       movq	0x30(%rax), %r14
// Frames: F137 -> F136 -> F133 -> F147
// 0x1421a7681  4c8b7840                       movq	0x40(%rax), %r15
// Frames: F138 -> F135 -> F136 -> F133 -> F147
// 0x1421a7685  4c3b7020                       cmpq	0x20(%rax), %r14
// 0x1421a7689  7511                           jne	0x1421a769c <.text+0x21a669c>
// Frames: F139 -> F135 -> F136 -> F133 -> F147
// 0x1421a768b  488b4c2478                     movq	0x78(%rsp), %rcx
// 0x1421a7690  488d15d9b69c00                 leaq	0x9cb6d9(%rip), %rdx    # 0x142b72d70
// 0x1421a7697  e814b43bfe                     callq	0x140562ab0 <.text+0x561ab0>
// Frames: F9 -> F10 -> F11 -> F140 -> F141 -> F135 -> F136 -> F133 -> F147
// 0x1421a769c  488b542438                     movq	0x38(%rsp), %rdx
// 0x1421a76a1  488b4228                       movq	0x28(%rdx), %rax
// Frames: F142 -> F141 -> F135 -> F136 -> F133 -> F147
// 0x1421a76a5  4b8d0cb6                       leaq	(%r14,%r14,4), %rcx
// Frames: F143 -> F144 -> F135 -> F136 -> F133 -> F147
// 0x1421a76a9  4c893cc8                       movq	%r15, (%rax,%rcx,8)
// 0x1421a76ad  c644c80805                     movb	$0x5, 0x8(%rax,%rcx,8)
// 0x1421a76b2  0f10842494000000               movups	0x94(%rsp), %xmm0
// 0x1421a76ba  0f108c249f000000               movups	0x9f(%rsp), %xmm1
// 0x1421a76c2  0f114cc814                     movups	%xmm1, 0x14(%rax,%rcx,8)
// 0x1421a76c7  0f1144c809                     movups	%xmm0, 0x9(%rax,%rcx,8)
// Frames: F145 -> F135 -> F136 -> F133 -> F147
// 0x1421a76cc  49ffc6                         incq	%r14
// 0x1421a76cf  4c897230                       movq	%r14, 0x30(%rdx)
// Frames: F146 -> F136 -> F133 -> F147
// 0x1421a76d3  48ff4240                       incq	0x40(%rdx)
// 0x1421a76d7  b001                           movb	$0x1, %al
// 0x1421a76d9  4c8b7c2448                     movq	0x48(%rsp), %r15
// Frames: F117 -> F118 -> F122
// 0x1421a76de  8b4c2430                       movl	0x30(%rsp), %ecx
// 0x1421a76e2  41894d00                       movl	%ecx, (%r13)
// Frames: F122
// 0x1421a76e6  884614                         movb	%al, 0x14(%rsi)
// Frames: F8
// 0x1421a76e9  e93ffbffff                     jmp	0x1421a722d <.text+0x21a622d>
// Frames: F148 -> F149 -> F102 -> F94
// 0x1421a76ee  4c8d05f3f5e400                 leaq	0xe4f5f3(%rip), %r8     # 0x142ff6ce8
// 0x1421a76f5  f30f100d6f598200               movss	0x82596f(%rip), %xmm1   # 0x1429cd06c
// 0x1421a76fd  0f28c6                         movaps	%xmm6, %xmm0
// 0x1421a7700  e87b3dbefe                     callq	0x140d8b480 <.text+0xd8a480>
// 0x1421a7705  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:14:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F2: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:21:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F3: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:85:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F4: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:640:0 bevy_ecs::system::query::Query::copy_unsafe
// F5: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:622:0 bevy_ecs::system::query::Query::reborrow_unsafe
// F6: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:604:0 bevy_ecs::system::query::Query::reborrow
// F7: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:702:0 bevy_ecs::system::query::Query::iter_mut
// F8: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:25:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F9: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:508:0 alloc::raw_vec::RawVecInner::non_null
// F10: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:503:0 alloc::raw_vec::RawVecInner::ptr
// F11: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/raw_vec.rs:282:0 alloc::raw_vec::RawVec::ptr
// F12: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1671:0 alloc::vec::Vec::as_ptr
// F13: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1575:0 alloc::vec::Vec::as_slice
// F14: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:3266:0 alloc::vec::impl$8::deref
// F15: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2372:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F16: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:52:0 bevy_ecs::query::iter::QueryIter::new
// F17: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs:2579:0 bevy_ecs::system::query::Query::reborrow
// F18: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 core::ptr::mut_ptr::impl$0::add
// F19: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter.rs:102:0 core::slice::iter::Iter::new
// F20: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:1051:0 alloc::vec::impl$8::deref
// F21: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2373:0 bevy_ecs::query::iter::QueryIterationCursor::init
// F22: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2464:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F23: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:889:0 bevy_ecs::query::iter::impl$1::next
// F24: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2467:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F25: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/non_null.rs:1623:0 core::ptr::non_null::impl$14::eq
// F26: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:44:0 core::slice::iter::impl$182::next
// F27: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2468:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F28: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/iter/macros.rs:25:0 core::slice::iter::impl$182::next
// F29: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:228:0 core::slice::index::impl$2::get
// F30: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:598:0 core::slice::impl$0::get
// F31: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:722:0 bevy_ecs::storage::table::Tables::get
// F32: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2469:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F33: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2681:0 alloc::vec::Vec::len
// F34: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2706:0 alloc::vec::Vec::is_empty
// F35: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:616:0 bevy_ecs::storage::table::Table::is_empty
// F36: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2470:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F37: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2505:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F38: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2506:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F39: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:896:0 bevy_ecs::archetype::Archetypes::get
// F40: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2507:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F41: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:616:0 bevy_ecs::archetype::Archetype::is_empty
// F42: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2508:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F43: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2476:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F44: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:396:0 bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
// F45: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:476:0 bevy_ecs::storage::table::Table::get_column
// F46: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1544:0 bevy_ecs::query::fetch::impl$36::set_table
// F47: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs:199:0 bevy_ecs::query::world_query::impl$2::set_table
// F48: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:707:0 core::option::Option::as_ref
// F49: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:0 core::ops::function::FnOnce::call_once
// F50: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::option::Option::and_then
// F51: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:60:0 bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
// F52: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs:154:0 nonmax::NonMaxUsize::get
// F53: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs:398:0 bevy_ecs::storage::sparse_set::impl$16::get::closure$0
// F54: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1119:0 core::option::Option::map
// F55: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:90:0 core::slice::index::get_noubcheck
// F56: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/index.rs:256:0 core::slice::index::impl$2::get_unchecked
// F57: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/slice/mod.rs:663:0 nonmax::NonMaxUsize::get
// F58: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs:148:0 bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
// F59: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs:304:0 bevy_ecs::storage::table::column::ThinColumn::get_data_slice
// F60: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1546:0 bevy_ecs::query::fetch::impl$36::set_table
// F61: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs:305:0 bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
// F62: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs:320:0 bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
// F63: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1548:0 bevy_ecs::query::fetch::impl$36::set_table
// F64: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:367:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F65: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1181:0 bevy_ecs::query::fetch::impl$26::set_table
// F66: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/option.rs:1452:0 core::slice::impl$0::get
// F67: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
// F68: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:368:0 bevy_ecs::storage::table::Table::get_data_slice_for
// F69: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:461:0 bevy_ecs::archetype::Archetype::table_id
// F70: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2511:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F71: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:722:0 bevy_ecs::archetype::Archetype::table_id
// F72: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2515:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F73: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1533:0 bevy_ecs::query::fetch::impl$36::set_archetype
// F74: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs:191:0 bevy_ecs::query::world_query::impl$2::set_archetype
// F75: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1169:0 bevy_ecs::query::fetch::impl$26::set_archetype
// F76: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs:129:0 bevy_ecs::storage::table::TableRow::as_usize
// F77: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1608:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F78: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2474:0 bevy_ecs::query::fetch::StorageSwitch::extract
// F79: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1601:0 bevy_ecs::query::fetch::impl$37::fetch
// F80: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:2090:0 bevy_ecs::query::fetch::impl$61::fetch
// F81: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2498:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F82: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:467:0 bevy_ecs::archetype::Archetype::entities
// F83: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2528:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F84: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs:326:0 bevy_ecs::archetype::ArchetypeEntity::table_row
// F85: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2540:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F86: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:2555:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F87: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs:532:0 bevy_ptr::ThinSlicePtr::get
// F88: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs:0:0 bevy_ecs::query::iter::QueryIterationCursor::next
// F89: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1612:0 bevy_ecs::query::fetch::impl$37::fetch::closure$0
// F90: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1239:0 bevy_ecs::query::fetch::impl$27::fetch::closure$0
// F91: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs:1234:0 bevy_ecs::query::fetch::impl$37::fetch
// F92: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:26:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F93: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:30:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F94: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:33:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F95: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:89:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F96: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:90:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F97: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:96:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F98: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:109:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F99: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:110:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F100: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:113:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F101: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1411:0 core::f32::impl$0::clamp
// F102: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:117:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F103: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/intrinsics/mod.rs:1498:0 core::intrinsics::likely
// F104: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/panic.rs:243:0 core::f32::impl$0::clamp
// F105: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:891:0 core::f32::impl$0::max
// F106: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:115:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F107: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1418:0 core::f32::impl$0::clamp
// F108: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1421:0 core::f32::impl$0::clamp
// F109: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:130:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F110: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/f32.rs:397:0 std::f32::impl$0::exp
// F111: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:214:0 core::ops::arith::impl$77::sub
// F112: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs:172:0 lerp::impl$0::lerp
// F113: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:132:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// F114: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:344:0 core::ops::arith::impl$141::mul
// F115: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/arith.rs:105:0 core::ops::arith::impl$13::add
// F116: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:35:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F117: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$57::set_changed
// F118: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$58::deref_mut
// F119: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:36:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F120: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/num/f32.rs:1448:0 core::f32::impl$0::abs
// F121: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:0:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F122: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:38:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F123: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:52:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F124: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:53:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F125: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:54:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F126: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:40:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F127: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:41:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F128: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:62:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F129: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:64:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F130: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:68:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F131: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:398:0 bevy_ecs::change_detection::impl$37::set_changed
// F132: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs:435:0 bevy_ecs::change_detection::impl$38::deref_mut
// F133: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs:75:0 bevy_ecs::event::writer::EventWriter::write
// F134: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2419:0 alloc::vec::Vec::push
// F135: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:139:0 bevy_ecs::event::collections::Events::send_with_caller
// F136: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:125:0 bevy_ecs::change_detection::impl$38::deref_mut
// F137: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:130:0 bevy_ecs::event::collections::Events::send_with_caller
// F138: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2422:0 alloc::vec::Vec::push
// F139: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2423:0 alloc::vec::Vec::push
// F140: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:1734:0 alloc::vec::Vec::as_mut_ptr
// F141: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2426:0 alloc::vec::Vec::push
// F142: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mut_ptr.rs:1057:0 alloc::vec::Vec::as_mut_ptr
// F143: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ptr/mod.rs:1582:0 core::ptr::write
// F144: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2427:0 alloc::vec::Vec::push
// F145: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/alloc/src/vec/mod.rs:2428:0 alloc::vec::Vec::push
// F146: /home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs:140:0 bevy_ecs::event::collections::Events::send_with_caller
// F147: crates/systems/roof/src/simulate_roof_tile_for_audio.rs:44:0 system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio::h24eafaa9a54ca9d2
// F148: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/intrinsics/mod.rs:3886:0 core::f32::impl$0::clamp::do_panic
// F149: /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/panic.rs:223:0 core::f32::impl$0::clamp
// === ORIGINAL ASSEMBLY V1 END ===
// Version: B
// Original source: crates/systems/roof/src/simulate_roof_tile_for_audio.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b1027b945944db068e6b812619c7c585db4b778cabebf5c849ee6c8199f552f3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: simulate_roof_tile_for_audio.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3464
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_roof; // namespace candidate
//   package identities: [{"name":"system-roof","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3464,"record_offset":260,"rva":35287184,"va":5403996304}
// 编译实例 RVAs: 0x21a7090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: simulate_roof_tile_for_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7850432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403996304,"raw_hex":"e8c0c97700","size":5,"soft_fail":false,"target_va":5411847872,"va":5403997435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-29641540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403996304,"raw_hex":"e8bcb43bfe","size":5,"soft_fail":false,"target_va":5374356144,"va":5403997679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::f32::_$LT$impl$u20$f32$GT$::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21086853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403996304,"raw_hex":"e87b3dbefe","size":5,"soft_fail":false,"target_va":5382911104,"va":5403997952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25968797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403996304,"raw_hex":"e863bf73fe","size":5,"soft_fail":false,"target_va":5378028416,"va":5403997208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25969170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403996304,"raw_hex":"e8eebd73fe","size":5,"soft_fail":false,"target_va":5378028064,"va":5403997229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3464,"record_offset":4244,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3464,"record_offset":4384,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3464,"record_offset":848,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3464,"record_offset":436,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3464,"record_offset":4300,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_direct] system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-roof","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/roof/src/simulate_roof_tile_for_audio.rs
//     evidence={"module":3464,"record_offset":3856,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":116674,"source_line":88,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3464,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":260,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3464,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":260,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3464,"record_offset":704,"parent_record_offset":680,"ancestor_record_offset":260,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3464,"record_offset":4548,"parent_record_offset":4504,"ancestor_record_offset":260,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3464,"record_offset":656,"parent_record_offset":632,"ancestor_record_offset":260,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3464,"record_offset":632,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3464,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":260,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3464,"record_offset":1160,"parent_record_offset":1136,"ancestor_record_offset":260,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3464,"record_offset":4504,"parent_record_offset":4468,"ancestor_record_offset":260,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3464,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":260,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3464,"record_offset":3252,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3464,"record_offset":1356,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3464,"record_offset":2392,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3464,"record_offset":3448,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3464,"record_offset":1272,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3464,"record_offset":4440,"parent_record_offset":4408,"ancestor_record_offset":260,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3464,"record_offset":4408,"parent_record_offset":4384,"ancestor_record_offset":260,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3464,"record_offset":4268,"parent_record_offset":4244,"ancestor_record_offset":260,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3464,"record_offset":4468,"parent_record_offset":4408,"ancestor_record_offset":260,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3464,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":260,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3464,"record_offset":2620,"parent_record_offset":2484,"ancestor_record_offset":260,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3464,"record_offset":1928,"parent_record_offset":1440,"ancestor_record_offset":260,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3464,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":260,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3464,"record_offset":2508,"parent_record_offset":2484,"ancestor_record_offset":260,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3464,"record_offset":1464,"parent_record_offset":1440,"ancestor_record_offset":260,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3464,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":260,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3464,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":260,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3464,"record_offset":3112,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3464,"record_offset":548,"parent_record_offset":460,"ancestor_record_offset":260,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3464,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":260,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3464,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":260,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3464,"record_offset":2484,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3464,"record_offset":1440,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3464,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":260,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3464,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":260,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3464,"record_offset":1520,"parent_record_offset":1496,"ancestor_record_offset":260,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3464,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":260,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3464,"record_offset":1496,"parent_record_offset":1464,"ancestor_record_offset":260,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3464,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":260,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3464,"record_offset":1112,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3464,"record_offset":3208,"parent_record_offset":3184,"ancestor_record_offset":260,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3464,"record_offset":1028,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3464,"record_offset":1868,"parent_record_offset":1464,"ancestor_record_offset":260,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3464,"record_offset":1812,"parent_record_offset":1464,"ancestor_record_offset":260,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3464,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":260,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3464,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":260,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3464,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":260,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3464,"record_offset":460,"parent_record_offset":436,"ancestor_record_offset":260,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3464,"record_offset":492,"parent_record_offset":460,"ancestor_record_offset":260,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3464,"record_offset":3608,"parent_record_offset":3580,"ancestor_record_offset":260,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3464,"record_offset":3924,"parent_record_offset":3856,"ancestor_record_offset":260,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic.rs
//     evidence={"module":3464,"record_offset":4008,"parent_record_offset":3924,"ancestor_record_offset":260,"inlinee":6395,"source_line":211,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3464,"record_offset":4044,"parent_record_offset":3856,"ancestor_record_offset":260,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3464,"record_offset":3980,"parent_record_offset":3924,"ancestor_record_offset":260,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3464,"record_offset":4208,"parent_record_offset":4100,"ancestor_record_offset":260,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3464,"record_offset":4152,"parent_record_offset":4100,"ancestor_record_offset":260,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3464,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":260,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3464,"record_offset":1592,"parent_record_offset":1568,"ancestor_record_offset":260,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3464,"record_offset":1568,"parent_record_offset":1544,"ancestor_record_offset":260,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3464,"record_offset":1616,"parent_record_offset":1592,"ancestor_record_offset":260,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3464,"record_offset":1656,"parent_record_offset":1520,"ancestor_record_offset":260,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3464,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":260,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3464,"record_offset":992,"parent_record_offset":956,"ancestor_record_offset":260,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3464,"record_offset":4668,"parent_record_offset":4504,"ancestor_record_offset":260,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3464,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":260,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3464,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":260,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3464,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":260,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3464,"record_offset":1736,"parent_record_offset":1704,"ancestor_record_offset":260,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3464,"record_offset":772,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3464,"record_offset":956,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3464,"record_offset":4100,"parent_record_offset":3856,"ancestor_record_offset":260,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3464,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":260,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3464,"record_offset":4072,"parent_record_offset":3856,"ancestor_record_offset":260,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C7C2
// 首条位置: {"module":3464,"inlinee":116674}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3464,"record_offset":3924,"parent_record_offset":3856,"ancestor_record_offset":3856,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3464,"record_offset":4044,"parent_record_offset":3856,"ancestor_record_offset":3856,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3464,"record_offset":4100,"parent_record_offset":3856,"ancestor_record_offset":3856,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3464,"record_offset":4072,"parent_record_offset":3856,"ancestor_record_offset":3856,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic.rs
//     evidence={"module":3464,"record_offset":4008,"parent_record_offset":3924,"ancestor_record_offset":3856,"inlinee":6395,"source_line":211,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3464,"record_offset":3980,"parent_record_offset":3924,"ancestor_record_offset":3856,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3464,"record_offset":4208,"parent_record_offset":4100,"ancestor_record_offset":3856,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3464,"record_offset":4152,"parent_record_offset":4100,"ancestor_record_offset":3856,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3464,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":3856,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03464.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >

// ------------------------------------------------------------------
// 函数: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile
// 信息 ID: e42fe9620e35a06f837ad150
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio; module=3464, inline_record=3856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio
// 信息 ID: 122842de978b88b249592305
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e19dab4ee16ec2a583424ab7: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >)> > > @ 0x1a299f6 -> 0x21a7090 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >)> > @ 0x1a30cb1 -> 0x21a7090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
