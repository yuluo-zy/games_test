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
// Compiled procedure 267:528 -> rva_3f0980_len_683
//
// FUNCTION system_roof::impl$0::add_roof_events_and_resources
// Candidate interfaces: UNKNOWN
// Static incoming reference records: 1; complete records in enriched/assembly sidecars.
// Compiled procedure 267:828 -> rva_3f1010_len_985
//
// FUNCTION system_roof::gpu_flush_buffers
// Candidate interfaces: 5d58aab8b83fbd13d0042c71
// Static incoming reference records: 0; complete records in enriched/assembly sidecars.
// Inline site 766:3276 -> host 766:944 / rva_8ea200_len_215; exact_symbolizer_function_frame
//   Function-frame RVA span [0x8ea2ac, 0x8ea2d4), 10 instructions
//   Function-frame RVA span [0x8ea408, 0x8ea415), 3 instructions
// Inline site 1927:249384 -> host 1927:246500 / rva_12d5c50_len_2f5; exact_symbolizer_function_frame
//   Function-frame RVA span [0x12d5d52, 0x12d5d7a), 10 instructions
//   Function-frame RVA span [0x12d5f20, 0x12d5f2c), 2 instructions
//
// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.
//
// RANGE rva_3f0980_len_683: RVA 0x3f0980, 1667 PDB bytes; 317 instructions
// Evidence: _assembly/roof-v1/A/pack-000/rva_3f0980_len_683.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 267:528: system_roof::impl$0::add_roof_systems
// Frames: F1
// 0x1403f0980  55                             pushq	%rbp
// 0x1403f0981  56                             pushq	%rsi
// 0x1403f0982  4881ec28070000                 subq	$0x728, %rsp            # imm = 0x728
// 0x1403f0989  488dac2480000000               leaq	0x80(%rsp), %rbp
// 0x1403f0991  48c785a0060000feffffff         movq	$-0x2, 0x6a0(%rbp)
// 0x1403f099c  4889ce                         movq	%rcx, %rsi
// 0x1403f099f  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// Frames: F2
// 0x1403f09a6  e8952c4f00                     callq	0x1408e3640 <.text+0x8e2640>
// 0x1403f09ab  c6855005000012                 movb	$0x12, 0x550(%rbp)
// Frames: F3 -> F4 -> F2
// 0x1403f09b2  488b0de7188802                 movq	0x28818e7(%rip), %rcx   # 0x142c722a0
// 0x1403f09b9  4c8d0518a7ee01                 leaq	0x1eea718(%rip), %r8    # 0x1422db0d8
// 0x1403f09c0  488d9550050000                 leaq	0x550(%rbp), %rdx
// 0x1403f09c7  e8643a4f00                     callq	0x1408e4430 <.text+0x8e3430>
// Frames: F4 -> F2
// 0x1403f09cc  4989d0                         movq	%rdx, %r8
// 0x1403f09cf  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// 0x1403f09d6  4889c2                         movq	%rax, %rdx
// 0x1403f09d9  e8b273c4ff                     callq	0x140037d90 <.text+0x36d90>
// Frames: F5 -> F2
// 0x1403f09de  0f108530020000                 movups	0x230(%rbp), %xmm0
// 0x1403f09e5  0f298590060000                 movaps	%xmm0, 0x690(%rbp)
// 0x1403f09ec  0f108520020000                 movups	0x220(%rbp), %xmm0
// 0x1403f09f3  0f298580060000                 movaps	%xmm0, 0x680(%rbp)
// 0x1403f09fa  0f108510020000                 movups	0x210(%rbp), %xmm0
// 0x1403f0a01  0f298570060000                 movaps	%xmm0, 0x670(%rbp)
// 0x1403f0a08  0f1085d0010000                 movups	0x1d0(%rbp), %xmm0
// 0x1403f0a0f  0f108de0010000                 movups	0x1e0(%rbp), %xmm1
// 0x1403f0a16  0f1095f0010000                 movups	0x1f0(%rbp), %xmm2
// 0x1403f0a1d  0f109d00020000                 movups	0x200(%rbp), %xmm3
// 0x1403f0a24  0f299d60060000                 movaps	%xmm3, 0x660(%rbp)
// 0x1403f0a2b  0f299550060000                 movaps	%xmm2, 0x650(%rbp)
// 0x1403f0a32  0f298d40060000                 movaps	%xmm1, 0x640(%rbp)
// 0x1403f0a39  0f298530060000                 movaps	%xmm0, 0x630(%rbp)
// Frames: F6
// 0x1403f0a40  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// 0x1403f0a47  e8d4514f00                     callq	0x1408e5c20 <.text+0x8e4c20>
// Frames: F3 -> F4 -> F6
// 0x1403f0a4c  c6855005000014                 movb	$0x14, 0x550(%rbp)
// 0x1403f0a53  488b0d46188802                 movq	0x2881846(%rip), %rcx   # 0x142c722a0
// 0x1403f0a5a  4c8d0577a6ee01                 leaq	0x1eea677(%rip), %r8    # 0x1422db0d8
// 0x1403f0a61  488d9550050000                 leaq	0x550(%rbp), %rdx
// 0x1403f0a68  e8c3394f00                     callq	0x1408e4430 <.text+0x8e3430>
// Frames: F4 -> F6
// 0x1403f0a6d  4989d0                         movq	%rdx, %r8
// 0x1403f0a70  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// 0x1403f0a77  4889c2                         movq	%rax, %rdx
// 0x1403f0a7a  e81173c4ff                     callq	0x140037d90 <.text+0x36d90>
// Frames: F5 -> F6
// 0x1403f0a7f  0f108530020000                 movups	0x230(%rbp), %xmm0
// 0x1403f0a86  0f298520060000                 movaps	%xmm0, 0x620(%rbp)
// 0x1403f0a8d  0f108520020000                 movups	0x220(%rbp), %xmm0
// 0x1403f0a94  0f298510060000                 movaps	%xmm0, 0x610(%rbp)
// 0x1403f0a9b  0f108510020000                 movups	0x210(%rbp), %xmm0
// 0x1403f0aa2  0f298500060000                 movaps	%xmm0, 0x600(%rbp)
// 0x1403f0aa9  0f1085d0010000                 movups	0x1d0(%rbp), %xmm0
// 0x1403f0ab0  0f108de0010000                 movups	0x1e0(%rbp), %xmm1
// 0x1403f0ab7  0f1095f0010000                 movups	0x1f0(%rbp), %xmm2
// 0x1403f0abe  0f109d00020000                 movups	0x200(%rbp), %xmm3
// 0x1403f0ac5  0f299df0050000                 movaps	%xmm3, 0x5f0(%rbp)
// 0x1403f0acc  0f2995e0050000                 movaps	%xmm2, 0x5e0(%rbp)
// 0x1403f0ad3  0f298dd0050000                 movaps	%xmm1, 0x5d0(%rbp)
// 0x1403f0ada  0f2985c0050000                 movaps	%xmm0, 0x5c0(%rbp)
// Frames: F7
// 0x1403f0ae1  4c8d052875dd01                 leaq	0x1dd7528(%rip), %r8    # 0x1421c8010
// 0x1403f0ae8  488d8d60010000                 leaq	0x160(%rbp), %rcx
// 0x1403f0aef  b21e                           movb	$0x1e, %dl
// 0x1403f0af1  e88a674f00                     callq	0x1408e7280 <.text+0x8e6280>
// Frames: F8
// 0x1403f0af6  4c8d052b75dd01                 leaq	0x1dd752b(%rip), %r8    # 0x1421c8028
// 0x1403f0afd  488d8df0000000                 leaq	0xf0(%rbp), %rcx
// 0x1403f0b04  b215                           movb	$0x15, %dl
// 0x1403f0b06  e8855d4f00                     callq	0x1408e6890 <.text+0x8e5890>
// Frames: F9
// 0x1403f0b0b  4c8d052e75dd01                 leaq	0x1dd752e(%rip), %r8    # 0x1421c8040
// 0x1403f0b12  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x1403f0b19  b229                           movb	$0x29, %dl
// 0x1403f0b1b  e820734f00                     callq	0x1408e7e40 <.text+0x8e6e40>
// Frames: F10
// 0x1403f0b20  4c8d053175dd01                 leaq	0x1dd7531(%rip), %r8    # 0x1421c8058
// 0x1403f0b27  488d4d10                       leaq	0x10(%rbp), %rcx
// 0x1403f0b2b  b223                           movb	$0x23, %dl
// 0x1403f0b2d  e86e784f00                     callq	0x1408e83a0 <.text+0x8e73a0>
// Frames: F11
// 0x1403f0b32  4c8d053775dd01                 leaq	0x1dd7537(%rip), %r8    # 0x1421c8070
// 0x1403f0b39  488d4da0                       leaq	-0x60(%rbp), %rcx
// 0x1403f0b3d  b223                           movb	$0x23, %dl
// 0x1403f0b3f  e85c944f00                     callq	0x1408e9fa0 <.text+0x8e8fa0>
// Frames: F12
// 0x1403f0b44  4c8d053d75dd01                 leaq	0x1dd753d(%rip), %r8    # 0x1421c8088
// 0x1403f0b4b  488d8d50050000                 leaq	0x550(%rbp), %rcx
// 0x1403f0b52  b22c                           movb	$0x2c, %dl
// 0x1403f0b54  e847994f00                     callq	0x1408ea4a0 <.text+0x8e94a0>
// Frames: F13
// 0x1403f0b59  0f288590060000                 movaps	0x690(%rbp), %xmm0
// 0x1403f0b60  0f298530020000                 movaps	%xmm0, 0x230(%rbp)
// 0x1403f0b67  0f288580060000                 movaps	0x680(%rbp), %xmm0
// 0x1403f0b6e  0f298520020000                 movaps	%xmm0, 0x220(%rbp)
// 0x1403f0b75  0f288570060000                 movaps	0x670(%rbp), %xmm0
// 0x1403f0b7c  0f298510020000                 movaps	%xmm0, 0x210(%rbp)
// 0x1403f0b83  0f288530060000                 movaps	0x630(%rbp), %xmm0
// 0x1403f0b8a  0f288d40060000                 movaps	0x640(%rbp), %xmm1
// 0x1403f0b91  0f289550060000                 movaps	0x650(%rbp), %xmm2
// 0x1403f0b98  0f289d60060000                 movaps	0x660(%rbp), %xmm3
// 0x1403f0b9f  0f299d00020000                 movaps	%xmm3, 0x200(%rbp)
// 0x1403f0ba6  0f2995f0010000                 movaps	%xmm2, 0x1f0(%rbp)
// 0x1403f0bad  0f298de0010000                 movaps	%xmm1, 0x1e0(%rbp)
// 0x1403f0bb4  0f2985d0010000                 movaps	%xmm0, 0x1d0(%rbp)
// 0x1403f0bbb  0f2885c0050000                 movaps	0x5c0(%rbp), %xmm0
// 0x1403f0bc2  0f288dd0050000                 movaps	0x5d0(%rbp), %xmm1
// 0x1403f0bc9  0f2895e0050000                 movaps	0x5e0(%rbp), %xmm2
// 0x1403f0bd0  0f289df0050000                 movaps	0x5f0(%rbp), %xmm3
// 0x1403f0bd7  0f298540020000                 movaps	%xmm0, 0x240(%rbp)
// 0x1403f0bde  0f298d50020000                 movaps	%xmm1, 0x250(%rbp)
// 0x1403f0be5  0f299560020000                 movaps	%xmm2, 0x260(%rbp)
// 0x1403f0bec  0f299d70020000                 movaps	%xmm3, 0x270(%rbp)
// 0x1403f0bf3  0f288500060000                 movaps	0x600(%rbp), %xmm0
// 0x1403f0bfa  0f298580020000                 movaps	%xmm0, 0x280(%rbp)
// 0x1403f0c01  0f288510060000                 movaps	0x610(%rbp), %xmm0
// 0x1403f0c08  0f298590020000                 movaps	%xmm0, 0x290(%rbp)
// 0x1403f0c0f  0f288520060000                 movaps	0x620(%rbp), %xmm0
// 0x1403f0c16  0f2985a0020000                 movaps	%xmm0, 0x2a0(%rbp)
// 0x1403f0c1d  0f1085c0010000                 movups	0x1c0(%rbp), %xmm0
// 0x1403f0c24  0f298510030000                 movaps	%xmm0, 0x310(%rbp)
// 0x1403f0c2b  0f1085b0010000                 movups	0x1b0(%rbp), %xmm0
// 0x1403f0c32  0f298500030000                 movaps	%xmm0, 0x300(%rbp)
// 0x1403f0c39  0f1085a0010000                 movups	0x1a0(%rbp), %xmm0
// 0x1403f0c40  0f2985f0020000                 movaps	%xmm0, 0x2f0(%rbp)
// 0x1403f0c47  0f108560010000                 movups	0x160(%rbp), %xmm0
// 0x1403f0c4e  0f108d70010000                 movups	0x170(%rbp), %xmm1
// 0x1403f0c55  0f109580010000                 movups	0x180(%rbp), %xmm2
// 0x1403f0c5c  0f109d90010000                 movups	0x190(%rbp), %xmm3
// 0x1403f0c63  0f299de0020000                 movaps	%xmm3, 0x2e0(%rbp)
// 0x1403f0c6a  0f2995d0020000                 movaps	%xmm2, 0x2d0(%rbp)
// 0x1403f0c71  0f298dc0020000                 movaps	%xmm1, 0x2c0(%rbp)
// 0x1403f0c78  0f2985b0020000                 movaps	%xmm0, 0x2b0(%rbp)
// 0x1403f0c7f  0f108550010000                 movups	0x150(%rbp), %xmm0
// 0x1403f0c86  0f298580030000                 movaps	%xmm0, 0x380(%rbp)
// 0x1403f0c8d  0f108540010000                 movups	0x140(%rbp), %xmm0
// 0x1403f0c94  0f298570030000                 movaps	%xmm0, 0x370(%rbp)
// 0x1403f0c9b  0f108530010000                 movups	0x130(%rbp), %xmm0
// 0x1403f0ca2  0f298560030000                 movaps	%xmm0, 0x360(%rbp)
// 0x1403f0ca9  0f1085f0000000                 movups	0xf0(%rbp), %xmm0
// 0x1403f0cb0  0f108d00010000                 movups	0x100(%rbp), %xmm1
// 0x1403f0cb7  0f109510010000                 movups	0x110(%rbp), %xmm2
// 0x1403f0cbe  0f109d20010000                 movups	0x120(%rbp), %xmm3
// 0x1403f0cc5  0f299d50030000                 movaps	%xmm3, 0x350(%rbp)
// 0x1403f0ccc  0f299540030000                 movaps	%xmm2, 0x340(%rbp)
// 0x1403f0cd3  0f298d30030000                 movaps	%xmm1, 0x330(%rbp)
// 0x1403f0cda  0f298520030000                 movaps	%xmm0, 0x320(%rbp)
// 0x1403f0ce1  0f1085e0000000                 movups	0xe0(%rbp), %xmm0
// 0x1403f0ce8  0f2985f0030000                 movaps	%xmm0, 0x3f0(%rbp)
// 0x1403f0cef  0f1085d0000000                 movups	0xd0(%rbp), %xmm0
// 0x1403f0cf6  0f2985e0030000                 movaps	%xmm0, 0x3e0(%rbp)
// 0x1403f0cfd  0f1085c0000000                 movups	0xc0(%rbp), %xmm0
// 0x1403f0d04  0f2985d0030000                 movaps	%xmm0, 0x3d0(%rbp)
// 0x1403f0d0b  0f108580000000                 movups	0x80(%rbp), %xmm0
// 0x1403f0d12  0f108d90000000                 movups	0x90(%rbp), %xmm1
// 0x1403f0d19  0f1095a0000000                 movups	0xa0(%rbp), %xmm2
// 0x1403f0d20  0f109db0000000                 movups	0xb0(%rbp), %xmm3
// 0x1403f0d27  0f299dc0030000                 movaps	%xmm3, 0x3c0(%rbp)
// 0x1403f0d2e  0f2995b0030000                 movaps	%xmm2, 0x3b0(%rbp)
// 0x1403f0d35  0f298da0030000                 movaps	%xmm1, 0x3a0(%rbp)
// 0x1403f0d3c  0f298590030000                 movaps	%xmm0, 0x390(%rbp)
// 0x1403f0d43  0f104570                       movups	0x70(%rbp), %xmm0
// 0x1403f0d47  0f298560040000                 movaps	%xmm0, 0x460(%rbp)
// 0x1403f0d4e  0f104560                       movups	0x60(%rbp), %xmm0
// 0x1403f0d52  0f298550040000                 movaps	%xmm0, 0x450(%rbp)
// 0x1403f0d59  0f104550                       movups	0x50(%rbp), %xmm0
// 0x1403f0d5d  0f298540040000                 movaps	%xmm0, 0x440(%rbp)
// 0x1403f0d64  0f104510                       movups	0x10(%rbp), %xmm0
// 0x1403f0d68  0f104d20                       movups	0x20(%rbp), %xmm1
// 0x1403f0d6c  0f105530                       movups	0x30(%rbp), %xmm2
// 0x1403f0d70  0f105d40                       movups	0x40(%rbp), %xmm3
// 0x1403f0d74  0f299d30040000                 movaps	%xmm3, 0x430(%rbp)
// 0x1403f0d7b  0f299520040000                 movaps	%xmm2, 0x420(%rbp)
// 0x1403f0d82  0f298d10040000                 movaps	%xmm1, 0x410(%rbp)
// 0x1403f0d89  0f298500040000                 movaps	%xmm0, 0x400(%rbp)
// 0x1403f0d90  0f104500                       movups	(%rbp), %xmm0
// 0x1403f0d94  0f2985d0040000                 movaps	%xmm0, 0x4d0(%rbp)
// 0x1403f0d9b  0f1045f0                       movups	-0x10(%rbp), %xmm0
// 0x1403f0d9f  0f2985c0040000                 movaps	%xmm0, 0x4c0(%rbp)
// 0x1403f0da6  0f1045e0                       movups	-0x20(%rbp), %xmm0
// 0x1403f0daa  0f2985b0040000                 movaps	%xmm0, 0x4b0(%rbp)
// 0x1403f0db1  0f1045a0                       movups	-0x60(%rbp), %xmm0
// 0x1403f0db5  0f104db0                       movups	-0x50(%rbp), %xmm1
// 0x1403f0db9  0f1055c0                       movups	-0x40(%rbp), %xmm2
// 0x1403f0dbd  0f105dd0                       movups	-0x30(%rbp), %xmm3
// 0x1403f0dc1  0f299da0040000                 movaps	%xmm3, 0x4a0(%rbp)
// 0x1403f0dc8  0f299590040000                 movaps	%xmm2, 0x490(%rbp)
// 0x1403f0dcf  0f298d80040000                 movaps	%xmm1, 0x480(%rbp)
// 0x1403f0dd6  0f298570040000                 movaps	%xmm0, 0x470(%rbp)
// 0x1403f0ddd  0f1085b0050000                 movups	0x5b0(%rbp), %xmm0
// 0x1403f0de4  0f298540050000                 movaps	%xmm0, 0x540(%rbp)
// 0x1403f0deb  0f1085a0050000                 movups	0x5a0(%rbp), %xmm0
// 0x1403f0df2  0f298530050000                 movaps	%xmm0, 0x530(%rbp)
// 0x1403f0df9  0f108590050000                 movups	0x590(%rbp), %xmm0
// 0x1403f0e00  0f298520050000                 movaps	%xmm0, 0x520(%rbp)
// 0x1403f0e07  0f108550050000                 movups	0x550(%rbp), %xmm0
// 0x1403f0e0e  0f108d60050000                 movups	0x560(%rbp), %xmm1
// 0x1403f0e15  0f109570050000                 movups	0x570(%rbp), %xmm2
// 0x1403f0e1c  0f109d80050000                 movups	0x580(%rbp), %xmm3
// 0x1403f0e23  0f299d10050000                 movaps	%xmm3, 0x510(%rbp)
// 0x1403f0e2a  0f299500050000                 movaps	%xmm2, 0x500(%rbp)
// 0x1403f0e31  0f298df0040000                 movaps	%xmm1, 0x4f0(%rbp)
// 0x1403f0e38  0f2985e0040000                 movaps	%xmm0, 0x4e0(%rbp)
// 0x1403f0e3f  488d95d0010000                 leaq	0x1d0(%rbp), %rdx
// Frames: F14 -> F15
// 0x1403f0e46  4889f1                         movq	%rsi, %rcx
// 0x1403f0e49  e8b29c4f00                     callq	0x1408eab00 <.text+0x8e9b00>
// Frames: F16
// 0x1403f0e4e  4889f0                         movq	%rsi, %rax
// 0x1403f0e51  4881c428070000                 addq	$0x728, %rsp            # imm = 0x728
// 0x1403f0e58  5e                             popq	%rsi
// 0x1403f0e59  5d                             popq	%rbp
// 0x1403f0e5a  c3                             retq
// 0x1403f0e5b  0f1f440000                     nopl	(%rax,%rax)
// 0x1403f0e60  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403f0e65  55                             pushq	%rbp
// 0x1403f0e66  56                             pushq	%rsi
// 0x1403f0e67  4883ec28                       subq	$0x28, %rsp
// 0x1403f0e6b  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1403f0e72  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// Frames: F17 -> F2
// 0x1403f0e79  e8526bc4ff                     callq	0x1400379d0 <.text+0x369d0>
// Frames: F18 -> F2
// 0x1403f0e7e  90                             nop
// 0x1403f0e7f  4883c428                       addq	$0x28, %rsp
// 0x1403f0e83  5e                             popq	%rsi
// 0x1403f0e84  5d                             popq	%rbp
// 0x1403f0e85  c3                             retq
// 0x1403f0e86  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// 0x1403f0e90  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403f0e95  55                             pushq	%rbp
// 0x1403f0e96  56                             pushq	%rsi
// 0x1403f0e97  4883ec28                       subq	$0x28, %rsp
// 0x1403f0e9b  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1403f0ea2  488d8d30060000                 leaq	0x630(%rbp), %rcx
// Frames: F19
// 0x1403f0ea9  e8226bc4ff                     callq	0x1400379d0 <.text+0x369d0>
// Frames: F1
// 0x1403f0eae  90                             nop
// 0x1403f0eaf  4883c428                       addq	$0x28, %rsp
// 0x1403f0eb3  5e                             popq	%rsi
// 0x1403f0eb4  5d                             popq	%rbp
// 0x1403f0eb5  c3                             retq
// 0x1403f0eb6  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// 0x1403f0ec0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403f0ec5  55                             pushq	%rbp
// 0x1403f0ec6  56                             pushq	%rsi
// 0x1403f0ec7  4883ec28                       subq	$0x28, %rsp
// 0x1403f0ecb  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1403f0ed2  488d8dd0010000                 leaq	0x1d0(%rbp), %rcx
// Frames: F17 -> F6
// 0x1403f0ed9  e8f26ac4ff                     callq	0x1400379d0 <.text+0x369d0>
// 0x1403f0ede  90                             nop
// 0x1403f0edf  4883c428                       addq	$0x28, %rsp
// 0x1403f0ee3  5e                             popq	%rsi
// 0x1403f0ee4  5d                             popq	%rbp
// 0x1403f0ee5  c3                             retq
// 0x1403f0ee6  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// 0x1403f0ef0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403f0ef5  55                             pushq	%rbp
// 0x1403f0ef6  56                             pushq	%rsi
// 0x1403f0ef7  4883ec28                       subq	$0x28, %rsp
// 0x1403f0efb  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1403f0f02  488d8dc0050000                 leaq	0x5c0(%rbp), %rcx
// Frames: F19
// 0x1403f0f09  e8c26ac4ff                     callq	0x1400379d0 <.text+0x369d0>
// 0x1403f0f0e  90                             nop
// 0x1403f0f0f  4883c428                       addq	$0x28, %rsp
// 0x1403f0f13  5e                             popq	%rsi
// 0x1403f0f14  5d                             popq	%rbp
// 0x1403f0f15  c3                             retq
// 0x1403f0f16  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// 0x1403f0f20  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403f0f25  55                             pushq	%rbp
// 0x1403f0f26  56                             pushq	%rsi
// 0x1403f0f27  4883ec28                       subq	$0x28, %rsp
// 0x1403f0f2b  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1403f0f32  488d8d60010000                 leaq	0x160(%rbp), %rcx
// 0x1403f0f39  e8926ac4ff                     callq	0x1400379d0 <.text+0x369d0>
// 0x1403f0f3e  90                             nop
// 0x1403f0f3f  4883c428                       addq	$0x28, %rsp
// 0x1403f0f43  5e                             popq	%rsi
// 0x1403f0f44  5d                             popq	%rbp
// 0x1403f0f45  c3                             retq
// 0x1403f0f46  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// 0x1403f0f50  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403f0f55  55                             pushq	%rbp
// 0x1403f0f56  56                             pushq	%rsi
// 0x1403f0f57  4883ec28                       subq	$0x28, %rsp
// 0x1403f0f5b  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1403f0f62  488d8df0000000                 leaq	0xf0(%rbp), %rcx
// 0x1403f0f69  e8626ac4ff                     callq	0x1400379d0 <.text+0x369d0>
// 0x1403f0f6e  90                             nop
// 0x1403f0f6f  4883c428                       addq	$0x28, %rsp
// 0x1403f0f73  5e                             popq	%rsi
// 0x1403f0f74  5d                             popq	%rbp
// 0x1403f0f75  c3                             retq
// 0x1403f0f76  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// 0x1403f0f80  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403f0f85  55                             pushq	%rbp
// 0x1403f0f86  56                             pushq	%rsi
// 0x1403f0f87  4883ec28                       subq	$0x28, %rsp
// 0x1403f0f8b  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1403f0f92  488d8d80000000                 leaq	0x80(%rbp), %rcx
// 0x1403f0f99  e8326ac4ff                     callq	0x1400379d0 <.text+0x369d0>
// 0x1403f0f9e  90                             nop
// 0x1403f0f9f  4883c428                       addq	$0x28, %rsp
// 0x1403f0fa3  5e                             popq	%rsi
// 0x1403f0fa4  5d                             popq	%rbp
// 0x1403f0fa5  c3                             retq
// 0x1403f0fa6  662e0f1f840000000000           nopw	%cs:(%rax,%rax)
// 0x1403f0fb0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403f0fb5  55                             pushq	%rbp
// 0x1403f0fb6  56                             pushq	%rsi
// 0x1403f0fb7  4883ec28                       subq	$0x28, %rsp
// 0x1403f0fbb  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1403f0fc2  488d4d10                       leaq	0x10(%rbp), %rcx
// 0x1403f0fc6  e8056ac4ff                     callq	0x1400379d0 <.text+0x369d0>
// 0x1403f0fcb  90                             nop
// 0x1403f0fcc  4883c428                       addq	$0x28, %rsp
// 0x1403f0fd0  5e                             popq	%rsi
// 0x1403f0fd1  5d                             popq	%rbp
// 0x1403f0fd2  c3                             retq
// 0x1403f0fd3  666666662e0f1f840000000000     nopw	%cs:(%rax,%rax)
// 0x1403f0fe0  4889542410                     movq	%rdx, 0x10(%rsp)
// 0x1403f0fe5  55                             pushq	%rbp
// 0x1403f0fe6  56                             pushq	%rsi
// 0x1403f0fe7  4883ec28                       subq	$0x28, %rsp
// 0x1403f0feb  488daa80000000                 leaq	0x80(%rdx), %rbp
// 0x1403f0ff2  488d4da0                       leaq	-0x60(%rbp), %rcx
// 0x1403f0ff6  e8d569c4ff                     callq	0x1400379d0 <.text+0x369d0>
// 0x1403f0ffb  90                             nop
// 0x1403f0ffc  4883c428                       addq	$0x28, %rsp
// 0x1403f1000  5e                             popq	%rsi
// 0x1403f1001  5d                             popq	%rbp
// 0x1403f1002  c3                             retq
//
// RANGE rva_3f1010_len_985: RVA 0x3f1010, 2437 PDB bytes; 578 instructions
// Evidence: _assembly/roof-v1/A/pack-000/rva_3f1010_len_985.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 267:828: system_roof::impl$0::add_roof_events_and_resources
// Frames: F20
// 0x1403f1010  56                             pushq	%rsi
// 0x1403f1011  57                             pushq	%rdi
// 0x1403f1012  55                             pushq	%rbp
// 0x1403f1013  53                             pushq	%rbx
// 0x1403f1014  4883ec58                       subq	$0x58, %rsp
// Frames: F21
// 0x1403f1018  c744242002000000               movl	$0x2, 0x20(%rsp)
// 0x1403f1020  488d542420                     leaq	0x20(%rsp), %rdx
// 0x1403f1025  e8669f4f00                     callq	0x1408eaf90 <.text+0x8e9f90>
// 0x1403f102a  4889c1                         movq	%rax, %rcx
// 0x1403f102d  4531c0                         xorl	%r8d, %r8d
// 0x1403f1030  e8eb9c4f00                     callq	0x1408ead20 <.text+0x8e9d20>
// 0x1403f1035  4889c6                         movq	%rax, %rsi
// Frames: F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1038  4883b89000000000               cmpq	$0x0, 0x90(%rax)
// 0x1403f1040  0f84cb000000                   je	0x1403f1111 <.text+0x3f0111>
// Frames: F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1046  488b4678                       movq	0x78(%rsi), %rax
// 0x1403f104a  488b8e80000000                 movq	0x80(%rsi), %rcx
// 0x1403f1051  488d50e8                       leaq	-0x18(%rax), %rdx
// 0x1403f1055  49b88593f1ed445eb88a           movabsq	$-0x7547a1bb120e6c7b, %r8 # imm = 0x8AB85E44EDF19385
// 0x1403f105f  4531c9                         xorl	%r9d, %r9d
// 0x1403f1062  660f6f05267cd901               movdqa	0x1d97c26(%rip), %xmm0  # 0x142188c90
// 0x1403f106a  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1403f106e  49ba0c08df49762bad30           movabsq	$0x30ad2b7649df080c, %r10 # imm = 0x30AD2B7649DF080C
// 0x1403f1078  4d89c3                         movq	%r8, %r11
// Frames: F29 -> F30 -> F31 -> F32 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f107b  4921cb                         andq	%rcx, %r11
// 0x1403f107e  f3420f6f1418                   movdqu	(%rax,%r11), %xmm2
// Frames: F35 -> F36 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1084  660f6fda                       movdqa	%xmm2, %xmm3
// 0x1403f1088  660f74d8                       pcmpeqb	%xmm0, %xmm3
// 0x1403f108c  660fd7fb                       pmovmskb	%xmm3, %edi
// Frames: F38 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1090  6685ff                         testw	%di, %di
// 0x1403f1093  742b                           je	0x1403f10c0 <.text+0x3f00c0>
// Frames: F40 -> F41 -> F42 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1095  f30fbcdf                       tzcntl	%edi, %ebx
// Frames: F43 -> F44 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1099  8d6fff                         leal	-0x1(%rdi), %ebp
// 0x1403f109c  21fd                           andl	%edi, %ebp
// 0x1403f109e  89ef                           movl	%ebp, %edi
// Frames: F45 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f10a0  4c01db                         addq	%r11, %rbx
// 0x1403f10a3  4821cb                         andq	%rcx, %rbx
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f10a6  48f7db                         negq	%rbx
// Frames: F51 -> F46 -> F52 -> F53 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f10a9  488d1c5b                       leaq	(%rbx,%rbx,2), %rbx
// Frames: F54 -> F55 -> F56 -> F57 -> F58 -> F59 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f10ad  4c3914da                       cmpq	%r10, (%rdx,%rbx,8)
// Frames: F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f10b1  75dd                           jne	0x1403f1090 <.text+0x3f0090>
// 0x1403f10b3  4c3944da08                     cmpq	%r8, 0x8(%rdx,%rbx,8)
// 0x1403f10b8  75d6                           jne	0x1403f1090 <.text+0x3f0090>
// 0x1403f10ba  eb1d                           jmp	0x1403f10d9 <.text+0x3f00d9>
// 0x1403f10bc  0f1f4000                       nopl	(%rax)
// Frames: F35 -> F36 -> F60 -> F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f10c0  660f74d1                       pcmpeqb	%xmm1, %xmm2
// 0x1403f10c4  660fd7fa                       pmovmskb	%xmm2, %edi
// Frames: F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f10c8  85ff                           testl	%edi, %edi
// 0x1403f10ca  7545                           jne	0x1403f1111 <.text+0x3f0111>
// Frames: F62 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f10cc  4d01cb                         addq	%r9, %r11
// 0x1403f10cf  4983c310                       addq	$0x10, %r11
// Frames: F64 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f10d3  4983c110                       addq	$0x10, %r9
// 0x1403f10d7  eba2                           jmp	0x1403f107b <.text+0x3f007b>
// Frames: F65 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f10d9  488b44d8f8                     movq	-0x8(%rax,%rbx,8), %rax
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f10de  48398698010000                 cmpq	%rax, 0x198(%rsi)
// Frames: F74 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f10e5  762a                           jbe	0x1403f1111 <.text+0x3f0111>
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f10e7  488b8e90010000                 movq	0x190(%rsi), %rcx
// Frames: F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f10ee  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1403f10f2  4885c0                         testq	%rax, %rax
// Frames: F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f10f5  741a                           je	0x1403f1111 <.text+0x3f0111>
// Frames: F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f10f7  488b8e60010000                 movq	0x160(%rsi), %rcx
// Frames: F79 -> F80 -> F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f10fe  48f7d0                         notq	%rax
// Frames: F81 -> F82 -> F83 -> F78 -> F84 -> F26 -> F27 -> F21
// 0x1403f1101  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1403f1105  48c1e005                       shlq	$0x5, %rax
// 0x1403f1109  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F26 -> F27 -> F21
// 0x1403f110f  7516                           jne	0x1403f1127 <.text+0x3f0127>
// Frames: F85 -> F27 -> F21
// 0x1403f1111  4889f1                         movq	%rsi, %rcx
// 0x1403f1114  e807804d00                     callq	0x1408c9120 <.text+0x8c8120>
// Frames: F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1119  4883be9000000000               cmpq	$0x0, 0x90(%rsi)
// 0x1403f1121  0f84da000000                   je	0x1403f1201 <.text+0x3f0201>
// Frames: F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1127  488b4678                       movq	0x78(%rsi), %rax
// 0x1403f112b  488b8e80000000                 movq	0x80(%rsi), %rcx
// 0x1403f1132  488d50e8                       leaq	-0x18(%rax), %rdx
// 0x1403f1136  49b86f3d1b1ceada9351           movabsq	$0x5193daea1c1b3d6f, %r8 # imm = 0x5193DAEA1C1B3D6F
// 0x1403f1140  4531c9                         xorl	%r9d, %r9d
// 0x1403f1143  660f6f0575eed901               movdqa	0x1d9ee75(%rip), %xmm0  # 0x14218ffc0
// 0x1403f114b  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1403f114f  49ba671dc2ebe330549a           movabsq	$-0x65abcf1c143de299, %r10 # imm = 0x9A5430E3EBC21D67
// 0x1403f1159  4d89c3                         movq	%r8, %r11
// Frames: F29 -> F30 -> F31 -> F32 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f115c  4921cb                         andq	%rcx, %r11
// 0x1403f115f  f3420f6f1418                   movdqu	(%rax,%r11), %xmm2
// Frames: F35 -> F36 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1165  660f6fda                       movdqa	%xmm2, %xmm3
// 0x1403f1169  660f74d8                       pcmpeqb	%xmm0, %xmm3
// 0x1403f116d  660fd7fb                       pmovmskb	%xmm3, %edi
// 0x1403f1171  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F38 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1180  6685ff                         testw	%di, %di
// 0x1403f1183  742b                           je	0x1403f11b0 <.text+0x3f01b0>
// Frames: F40 -> F41 -> F42 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1185  f30fbcdf                       tzcntl	%edi, %ebx
// Frames: F43 -> F44 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1189  8d6fff                         leal	-0x1(%rdi), %ebp
// 0x1403f118c  21fd                           andl	%edi, %ebp
// 0x1403f118e  89ef                           movl	%ebp, %edi
// Frames: F45 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1190  4c01db                         addq	%r11, %rbx
// 0x1403f1193  4821cb                         andq	%rcx, %rbx
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1196  48f7db                         negq	%rbx
// Frames: F51 -> F46 -> F52 -> F53 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1199  488d1c5b                       leaq	(%rbx,%rbx,2), %rbx
// Frames: F54 -> F55 -> F56 -> F57 -> F58 -> F59 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f119d  4c3914da                       cmpq	%r10, (%rdx,%rbx,8)
// Frames: F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f11a1  75dd                           jne	0x1403f1180 <.text+0x3f0180>
// 0x1403f11a3  4c3944da08                     cmpq	%r8, 0x8(%rdx,%rbx,8)
// 0x1403f11a8  75d6                           jne	0x1403f1180 <.text+0x3f0180>
// 0x1403f11aa  eb1d                           jmp	0x1403f11c9 <.text+0x3f01c9>
// 0x1403f11ac  0f1f4000                       nopl	(%rax)
// Frames: F35 -> F36 -> F60 -> F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f11b0  660f74d1                       pcmpeqb	%xmm1, %xmm2
// 0x1403f11b4  660fd7fa                       pmovmskb	%xmm2, %edi
// Frames: F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f11b8  85ff                           testl	%edi, %edi
// 0x1403f11ba  7545                           jne	0x1403f1201 <.text+0x3f0201>
// Frames: F62 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f11bc  4d01cb                         addq	%r9, %r11
// 0x1403f11bf  4983c310                       addq	$0x10, %r11
// Frames: F64 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f11c3  4983c110                       addq	$0x10, %r9
// 0x1403f11c7  eb93                           jmp	0x1403f115c <.text+0x3f015c>
// Frames: F65 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f11c9  488b44d8f8                     movq	-0x8(%rax,%rbx,8), %rax
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f11ce  48398698010000                 cmpq	%rax, 0x198(%rsi)
// Frames: F74 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f11d5  762a                           jbe	0x1403f1201 <.text+0x3f0201>
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f11d7  488b8e90010000                 movq	0x190(%rsi), %rcx
// Frames: F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f11de  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1403f11e2  4885c0                         testq	%rax, %rax
// Frames: F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f11e5  741a                           je	0x1403f1201 <.text+0x3f0201>
// Frames: F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f11e7  488b8e60010000                 movq	0x160(%rsi), %rcx
// Frames: F79 -> F80 -> F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f11ee  48f7d0                         notq	%rax
// Frames: F81 -> F82 -> F83 -> F78 -> F84 -> F26 -> F27 -> F21
// 0x1403f11f1  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1403f11f5  48c1e005                       shlq	$0x5, %rax
// 0x1403f11f9  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F26 -> F27 -> F21
// 0x1403f11ff  7516                           jne	0x1403f1217 <.text+0x3f0217>
// Frames: F85 -> F27 -> F21
// 0x1403f1201  4889f1                         movq	%rsi, %rcx
// 0x1403f1204  e8d77b4d00                     callq	0x1408c8de0 <.text+0x8c7de0>
// Frames: F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1209  4883be9000000000               cmpq	$0x0, 0x90(%rsi)
// 0x1403f1211  0f84da000000                   je	0x1403f12f1 <.text+0x3f02f1>
// Frames: F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1217  488b4678                       movq	0x78(%rsi), %rax
// 0x1403f121b  488b8e80000000                 movq	0x80(%rsi), %rcx
// 0x1403f1222  488d50e8                       leaq	-0x18(%rax), %rdx
// 0x1403f1226  49b858f17d5d9be9b041           movabsq	$0x41b0e99b5d7df158, %r8 # imm = 0x41B0E99B5D7DF158
// 0x1403f1230  4531c9                         xorl	%r9d, %r9d
// 0x1403f1233  660f6f0545edd901               movdqa	0x1d9ed45(%rip), %xmm0  # 0x14218ff80
// 0x1403f123b  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1403f123f  49ba24fe0c050f81947e           movabsq	$0x7e94810f050cfe24, %r10 # imm = 0x7E94810F050CFE24
// 0x1403f1249  4d89c3                         movq	%r8, %r11
// Frames: F29 -> F30 -> F31 -> F32 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f124c  4921cb                         andq	%rcx, %r11
// 0x1403f124f  f3420f6f1418                   movdqu	(%rax,%r11), %xmm2
// Frames: F35 -> F36 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1255  660f6fda                       movdqa	%xmm2, %xmm3
// 0x1403f1259  660f74d8                       pcmpeqb	%xmm0, %xmm3
// 0x1403f125d  660fd7fb                       pmovmskb	%xmm3, %edi
// 0x1403f1261  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F38 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1270  6685ff                         testw	%di, %di
// 0x1403f1273  742b                           je	0x1403f12a0 <.text+0x3f02a0>
// Frames: F40 -> F41 -> F42 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1275  f30fbcdf                       tzcntl	%edi, %ebx
// Frames: F43 -> F44 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1279  8d6fff                         leal	-0x1(%rdi), %ebp
// 0x1403f127c  21fd                           andl	%edi, %ebp
// 0x1403f127e  89ef                           movl	%ebp, %edi
// Frames: F45 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1280  4c01db                         addq	%r11, %rbx
// 0x1403f1283  4821cb                         andq	%rcx, %rbx
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1286  48f7db                         negq	%rbx
// Frames: F51 -> F46 -> F52 -> F53 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1289  488d1c5b                       leaq	(%rbx,%rbx,2), %rbx
// Frames: F54 -> F55 -> F56 -> F57 -> F58 -> F59 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f128d  4c3914da                       cmpq	%r10, (%rdx,%rbx,8)
// Frames: F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1291  75dd                           jne	0x1403f1270 <.text+0x3f0270>
// 0x1403f1293  4c3944da08                     cmpq	%r8, 0x8(%rdx,%rbx,8)
// 0x1403f1298  75d6                           jne	0x1403f1270 <.text+0x3f0270>
// 0x1403f129a  eb1d                           jmp	0x1403f12b9 <.text+0x3f02b9>
// 0x1403f129c  0f1f4000                       nopl	(%rax)
// Frames: F35 -> F36 -> F60 -> F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f12a0  660f74d1                       pcmpeqb	%xmm1, %xmm2
// 0x1403f12a4  660fd7fa                       pmovmskb	%xmm2, %edi
// Frames: F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f12a8  85ff                           testl	%edi, %edi
// 0x1403f12aa  7545                           jne	0x1403f12f1 <.text+0x3f02f1>
// Frames: F62 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f12ac  4d01cb                         addq	%r9, %r11
// 0x1403f12af  4983c310                       addq	$0x10, %r11
// Frames: F64 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f12b3  4983c110                       addq	$0x10, %r9
// 0x1403f12b7  eb93                           jmp	0x1403f124c <.text+0x3f024c>
// Frames: F65 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f12b9  488b44d8f8                     movq	-0x8(%rax,%rbx,8), %rax
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f12be  48398698010000                 cmpq	%rax, 0x198(%rsi)
// Frames: F74 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f12c5  762a                           jbe	0x1403f12f1 <.text+0x3f02f1>
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f12c7  488b8e90010000                 movq	0x190(%rsi), %rcx
// Frames: F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f12ce  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1403f12d2  4885c0                         testq	%rax, %rax
// Frames: F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f12d5  741a                           je	0x1403f12f1 <.text+0x3f02f1>
// Frames: F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f12d7  488b8e60010000                 movq	0x160(%rsi), %rcx
// Frames: F79 -> F80 -> F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f12de  48f7d0                         notq	%rax
// Frames: F81 -> F82 -> F83 -> F78 -> F84 -> F26 -> F27 -> F21
// 0x1403f12e1  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1403f12e5  48c1e005                       shlq	$0x5, %rax
// 0x1403f12e9  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F26 -> F27 -> F21
// 0x1403f12ef  7516                           jne	0x1403f1307 <.text+0x3f0307>
// Frames: F85 -> F27 -> F21
// 0x1403f12f1  4889f1                         movq	%rsi, %rcx
// 0x1403f12f4  e8e7874d00                     callq	0x1408c9ae0 <.text+0x8c8ae0>
// Frames: F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f12f9  4883be9000000000               cmpq	$0x0, 0x90(%rsi)
// 0x1403f1301  0f84da000000                   je	0x1403f13e1 <.text+0x3f03e1>
// Frames: F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1307  488b4678                       movq	0x78(%rsi), %rax
// 0x1403f130b  488b8e80000000                 movq	0x80(%rsi), %rcx
// 0x1403f1312  488d50e8                       leaq	-0x18(%rax), %rdx
// 0x1403f1316  49b8638b405167d15311           movabsq	$0x1153d16751408b63, %r8 # imm = 0x1153D16751408B63
// 0x1403f1320  4531c9                         xorl	%r9d, %r9d
// 0x1403f1323  660f6f050521da01               movdqa	0x1da2105(%rip), %xmm0  # 0x142193430
// 0x1403f132b  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1403f132f  49ba60d69332d1d28477           movabsq	$0x7784d2d13293d660, %r10 # imm = 0x7784D2D13293D660
// 0x1403f1339  4d89c3                         movq	%r8, %r11
// Frames: F29 -> F30 -> F31 -> F32 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f133c  4921cb                         andq	%rcx, %r11
// 0x1403f133f  f3420f6f1418                   movdqu	(%rax,%r11), %xmm2
// Frames: F35 -> F36 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1345  660f6fda                       movdqa	%xmm2, %xmm3
// 0x1403f1349  660f74d8                       pcmpeqb	%xmm0, %xmm3
// 0x1403f134d  660fd7fb                       pmovmskb	%xmm3, %edi
// 0x1403f1351  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F38 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1360  6685ff                         testw	%di, %di
// 0x1403f1363  742b                           je	0x1403f1390 <.text+0x3f0390>
// Frames: F40 -> F41 -> F42 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1365  f30fbcdf                       tzcntl	%edi, %ebx
// Frames: F43 -> F44 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1369  8d6fff                         leal	-0x1(%rdi), %ebp
// 0x1403f136c  21fd                           andl	%edi, %ebp
// 0x1403f136e  89ef                           movl	%ebp, %edi
// Frames: F45 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1370  4c01db                         addq	%r11, %rbx
// 0x1403f1373  4821cb                         andq	%rcx, %rbx
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1376  48f7db                         negq	%rbx
// Frames: F51 -> F46 -> F52 -> F53 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1379  488d1c5b                       leaq	(%rbx,%rbx,2), %rbx
// Frames: F54 -> F55 -> F56 -> F57 -> F58 -> F59 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f137d  4c3914da                       cmpq	%r10, (%rdx,%rbx,8)
// Frames: F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1381  75dd                           jne	0x1403f1360 <.text+0x3f0360>
// 0x1403f1383  4c3944da08                     cmpq	%r8, 0x8(%rdx,%rbx,8)
// 0x1403f1388  75d6                           jne	0x1403f1360 <.text+0x3f0360>
// 0x1403f138a  eb1d                           jmp	0x1403f13a9 <.text+0x3f03a9>
// 0x1403f138c  0f1f4000                       nopl	(%rax)
// Frames: F35 -> F36 -> F60 -> F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1390  660f74d1                       pcmpeqb	%xmm1, %xmm2
// 0x1403f1394  660fd7fa                       pmovmskb	%xmm2, %edi
// Frames: F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1398  85ff                           testl	%edi, %edi
// 0x1403f139a  7545                           jne	0x1403f13e1 <.text+0x3f03e1>
// Frames: F62 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f139c  4d01cb                         addq	%r9, %r11
// 0x1403f139f  4983c310                       addq	$0x10, %r11
// Frames: F64 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f13a3  4983c110                       addq	$0x10, %r9
// 0x1403f13a7  eb93                           jmp	0x1403f133c <.text+0x3f033c>
// Frames: F65 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f13a9  488b44d8f8                     movq	-0x8(%rax,%rbx,8), %rax
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f13ae  48398698010000                 cmpq	%rax, 0x198(%rsi)
// Frames: F74 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f13b5  762a                           jbe	0x1403f13e1 <.text+0x3f03e1>
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f13b7  488b8e90010000                 movq	0x190(%rsi), %rcx
// Frames: F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f13be  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1403f13c2  4885c0                         testq	%rax, %rax
// Frames: F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f13c5  741a                           je	0x1403f13e1 <.text+0x3f03e1>
// Frames: F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f13c7  488b8e60010000                 movq	0x160(%rsi), %rcx
// Frames: F79 -> F80 -> F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f13ce  48f7d0                         notq	%rax
// Frames: F81 -> F82 -> F83 -> F78 -> F84 -> F26 -> F27 -> F21
// 0x1403f13d1  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1403f13d5  48c1e005                       shlq	$0x5, %rax
// 0x1403f13d9  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F26 -> F27 -> F21
// 0x1403f13df  7516                           jne	0x1403f13f7 <.text+0x3f03f7>
// Frames: F85 -> F27 -> F21
// 0x1403f13e1  4889f1                         movq	%rsi, %rcx
// 0x1403f13e4  e8778d4d00                     callq	0x1408ca160 <.text+0x8c9160>
// Frames: F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f13e9  4883be9000000000               cmpq	$0x0, 0x90(%rsi)
// 0x1403f13f1  0f84da000000                   je	0x1403f14d1 <.text+0x3f04d1>
// Frames: F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f13f7  488b4678                       movq	0x78(%rsi), %rax
// 0x1403f13fb  488b8e80000000                 movq	0x80(%rsi), %rcx
// 0x1403f1402  488d50e8                       leaq	-0x18(%rax), %rdx
// 0x1403f1406  49b897fe86c10e31891c           movabsq	$0x1c89310ec186fe97, %r8 # imm = 0x1C89310EC186FE97
// 0x1403f1410  4531c9                         xorl	%r9d, %r9d
// 0x1403f1413  660f6f05c520da01               movdqa	0x1da20c5(%rip), %xmm0  # 0x1421934e0
// 0x1403f141b  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1403f141f  49bad81ff8151d85ac86           movabsq	$-0x79537ae2ea07e028, %r10 # imm = 0x86AC851D15F81FD8
// 0x1403f1429  4d89c3                         movq	%r8, %r11
// Frames: F29 -> F30 -> F31 -> F32 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f142c  4921cb                         andq	%rcx, %r11
// 0x1403f142f  f3420f6f1418                   movdqu	(%rax,%r11), %xmm2
// Frames: F35 -> F36 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1435  660f6fda                       movdqa	%xmm2, %xmm3
// 0x1403f1439  660f74d8                       pcmpeqb	%xmm0, %xmm3
// 0x1403f143d  660fd7fb                       pmovmskb	%xmm3, %edi
// 0x1403f1441  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F38 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1450  6685ff                         testw	%di, %di
// 0x1403f1453  742b                           je	0x1403f1480 <.text+0x3f0480>
// Frames: F40 -> F41 -> F42 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1455  f30fbcdf                       tzcntl	%edi, %ebx
// Frames: F43 -> F44 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1459  8d6fff                         leal	-0x1(%rdi), %ebp
// 0x1403f145c  21fd                           andl	%edi, %ebp
// 0x1403f145e  89ef                           movl	%ebp, %edi
// Frames: F45 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1460  4c01db                         addq	%r11, %rbx
// 0x1403f1463  4821cb                         andq	%rcx, %rbx
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1466  48f7db                         negq	%rbx
// Frames: F51 -> F46 -> F52 -> F53 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1469  488d1c5b                       leaq	(%rbx,%rbx,2), %rbx
// Frames: F54 -> F55 -> F56 -> F57 -> F58 -> F59 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f146d  4c3914da                       cmpq	%r10, (%rdx,%rbx,8)
// Frames: F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1471  75dd                           jne	0x1403f1450 <.text+0x3f0450>
// 0x1403f1473  4c3944da08                     cmpq	%r8, 0x8(%rdx,%rbx,8)
// 0x1403f1478  75d6                           jne	0x1403f1450 <.text+0x3f0450>
// 0x1403f147a  eb1d                           jmp	0x1403f1499 <.text+0x3f0499>
// 0x1403f147c  0f1f4000                       nopl	(%rax)
// Frames: F35 -> F36 -> F60 -> F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1480  660f74d1                       pcmpeqb	%xmm1, %xmm2
// 0x1403f1484  660fd7fa                       pmovmskb	%xmm2, %edi
// Frames: F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1488  85ff                           testl	%edi, %edi
// 0x1403f148a  7545                           jne	0x1403f14d1 <.text+0x3f04d1>
// Frames: F62 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f148c  4d01cb                         addq	%r9, %r11
// 0x1403f148f  4983c310                       addq	$0x10, %r11
// Frames: F64 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1493  4983c110                       addq	$0x10, %r9
// 0x1403f1497  eb93                           jmp	0x1403f142c <.text+0x3f042c>
// Frames: F65 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1499  488b44d8f8                     movq	-0x8(%rax,%rbx,8), %rax
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f149e  48398698010000                 cmpq	%rax, 0x198(%rsi)
// Frames: F74 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f14a5  762a                           jbe	0x1403f14d1 <.text+0x3f04d1>
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f14a7  488b8e90010000                 movq	0x190(%rsi), %rcx
// Frames: F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f14ae  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1403f14b2  4885c0                         testq	%rax, %rax
// Frames: F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f14b5  741a                           je	0x1403f14d1 <.text+0x3f04d1>
// Frames: F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f14b7  488b8e60010000                 movq	0x160(%rsi), %rcx
// Frames: F79 -> F80 -> F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f14be  48f7d0                         notq	%rax
// Frames: F81 -> F82 -> F83 -> F78 -> F84 -> F26 -> F27 -> F21
// 0x1403f14c1  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1403f14c5  48c1e005                       shlq	$0x5, %rax
// 0x1403f14c9  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F26 -> F27 -> F21
// 0x1403f14cf  7516                           jne	0x1403f14e7 <.text+0x3f04e7>
// Frames: F85 -> F27 -> F21
// 0x1403f14d1  4889f1                         movq	%rsi, %rcx
// 0x1403f14d4  e8c7824d00                     callq	0x1408c97a0 <.text+0x8c87a0>
// Frames: F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f14d9  4883be9000000000               cmpq	$0x0, 0x90(%rsi)
// 0x1403f14e1  0f84da000000                   je	0x1403f15c1 <.text+0x3f05c1>
// Frames: F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f14e7  488b4678                       movq	0x78(%rsi), %rax
// 0x1403f14eb  488b8e80000000                 movq	0x80(%rsi), %rcx
// 0x1403f14f2  488d50e8                       leaq	-0x18(%rax), %rdx
// 0x1403f14f6  49b85a78767f85318e70           movabsq	$0x708e31857f76785a, %r8 # imm = 0x708E31857F76785A
// 0x1403f1500  4531c9                         xorl	%r9d, %r9d
// 0x1403f1503  660f6f0575ecd901               movdqa	0x1d9ec75(%rip), %xmm0  # 0x142190180
// 0x1403f150b  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1403f150f  49ba6f1b29eac188e15f           movabsq	$0x5fe188c1ea291b6f, %r10 # imm = 0x5FE188C1EA291B6F
// 0x1403f1519  4d89c3                         movq	%r8, %r11
// Frames: F29 -> F30 -> F31 -> F32 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f151c  4921cb                         andq	%rcx, %r11
// 0x1403f151f  f3420f6f1418                   movdqu	(%rax,%r11), %xmm2
// Frames: F35 -> F36 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1525  660f6fda                       movdqa	%xmm2, %xmm3
// 0x1403f1529  660f74d8                       pcmpeqb	%xmm0, %xmm3
// 0x1403f152d  660fd7fb                       pmovmskb	%xmm3, %edi
// 0x1403f1531  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F38 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1540  6685ff                         testw	%di, %di
// 0x1403f1543  742b                           je	0x1403f1570 <.text+0x3f0570>
// Frames: F40 -> F41 -> F42 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1545  f30fbcdf                       tzcntl	%edi, %ebx
// Frames: F43 -> F44 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1549  8d6fff                         leal	-0x1(%rdi), %ebp
// 0x1403f154c  21fd                           andl	%edi, %ebp
// 0x1403f154e  89ef                           movl	%ebp, %edi
// Frames: F45 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1550  4c01db                         addq	%r11, %rbx
// 0x1403f1553  4821cb                         andq	%rcx, %rbx
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1556  48f7db                         negq	%rbx
// Frames: F51 -> F46 -> F52 -> F53 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1559  488d1c5b                       leaq	(%rbx,%rbx,2), %rbx
// Frames: F54 -> F55 -> F56 -> F57 -> F58 -> F59 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f155d  4c3914da                       cmpq	%r10, (%rdx,%rbx,8)
// Frames: F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1561  75dd                           jne	0x1403f1540 <.text+0x3f0540>
// 0x1403f1563  4c3944da08                     cmpq	%r8, 0x8(%rdx,%rbx,8)
// 0x1403f1568  75d6                           jne	0x1403f1540 <.text+0x3f0540>
// 0x1403f156a  eb1d                           jmp	0x1403f1589 <.text+0x3f0589>
// 0x1403f156c  0f1f4000                       nopl	(%rax)
// Frames: F35 -> F36 -> F60 -> F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1570  660f74d1                       pcmpeqb	%xmm1, %xmm2
// 0x1403f1574  660fd7fa                       pmovmskb	%xmm2, %edi
// Frames: F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1578  85ff                           testl	%edi, %edi
// 0x1403f157a  7545                           jne	0x1403f15c1 <.text+0x3f05c1>
// Frames: F62 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f157c  4d01cb                         addq	%r9, %r11
// 0x1403f157f  4983c310                       addq	$0x10, %r11
// Frames: F64 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1583  4983c110                       addq	$0x10, %r9
// 0x1403f1587  eb93                           jmp	0x1403f151c <.text+0x3f051c>
// Frames: F65 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1589  488b44d8f8                     movq	-0x8(%rax,%rbx,8), %rax
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f158e  48398698010000                 cmpq	%rax, 0x198(%rsi)
// Frames: F74 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1595  762a                           jbe	0x1403f15c1 <.text+0x3f05c1>
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1597  488b8e90010000                 movq	0x190(%rsi), %rcx
// Frames: F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f159e  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1403f15a2  4885c0                         testq	%rax, %rax
// Frames: F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f15a5  741a                           je	0x1403f15c1 <.text+0x3f05c1>
// Frames: F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f15a7  488b8e60010000                 movq	0x160(%rsi), %rcx
// Frames: F79 -> F80 -> F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f15ae  48f7d0                         notq	%rax
// Frames: F81 -> F82 -> F83 -> F78 -> F84 -> F26 -> F27 -> F21
// 0x1403f15b1  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1403f15b5  48c1e005                       shlq	$0x5, %rax
// 0x1403f15b9  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F26 -> F27 -> F21
// 0x1403f15bf  7516                           jne	0x1403f15d7 <.text+0x3f05d7>
// Frames: F85 -> F27 -> F21
// 0x1403f15c1  4889f1                         movq	%rsi, %rcx
// 0x1403f15c4  e857884d00                     callq	0x1408c9e20 <.text+0x8c8e20>
// Frames: F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f15c9  4883be9000000000               cmpq	$0x0, 0x90(%rsi)
// 0x1403f15d1  0f84da000000                   je	0x1403f16b1 <.text+0x3f06b1>
// Frames: F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f15d7  488b4678                       movq	0x78(%rsi), %rax
// 0x1403f15db  488b8e80000000                 movq	0x80(%rsi), %rcx
// 0x1403f15e2  488d50e8                       leaq	-0x18(%rax), %rdx
// 0x1403f15e6  49b838624de9bee0fb7d           movabsq	$0x7dfbe0bee94d6238, %r8 # imm = 0x7DFBE0BEE94D6238
// 0x1403f15f0  4531c9                         xorl	%r9d, %r9d
// 0x1403f15f3  660f6f05a5ebd901               movdqa	0x1d9eba5(%rip), %xmm0  # 0x1421901a0
// 0x1403f15fb  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1403f15ff  49baab047db2a3ba55b6           movabsq	$-0x49aa455c4d82fb55, %r10 # imm = 0xB655BAA3B27D04AB
// 0x1403f1609  4d89c3                         movq	%r8, %r11
// Frames: F29 -> F30 -> F31 -> F32 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f160c  4921cb                         andq	%rcx, %r11
// 0x1403f160f  f3420f6f1418                   movdqu	(%rax,%r11), %xmm2
// Frames: F35 -> F36 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1615  660f6fda                       movdqa	%xmm2, %xmm3
// 0x1403f1619  660f74d8                       pcmpeqb	%xmm0, %xmm3
// 0x1403f161d  660fd7fb                       pmovmskb	%xmm3, %edi
// 0x1403f1621  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F38 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1630  6685ff                         testw	%di, %di
// 0x1403f1633  742b                           je	0x1403f1660 <.text+0x3f0660>
// Frames: F40 -> F41 -> F42 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1635  f30fbcdf                       tzcntl	%edi, %ebx
// Frames: F43 -> F44 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1639  8d6fff                         leal	-0x1(%rdi), %ebp
// 0x1403f163c  21fd                           andl	%edi, %ebp
// 0x1403f163e  89ef                           movl	%ebp, %edi
// Frames: F45 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1640  4c01db                         addq	%r11, %rbx
// 0x1403f1643  4821cb                         andq	%rcx, %rbx
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1646  48f7db                         negq	%rbx
// Frames: F51 -> F46 -> F52 -> F53 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1649  488d1c5b                       leaq	(%rbx,%rbx,2), %rbx
// Frames: F54 -> F55 -> F56 -> F57 -> F58 -> F59 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f164d  4c3914da                       cmpq	%r10, (%rdx,%rbx,8)
// Frames: F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1651  75dd                           jne	0x1403f1630 <.text+0x3f0630>
// 0x1403f1653  4c3944da08                     cmpq	%r8, 0x8(%rdx,%rbx,8)
// 0x1403f1658  75d6                           jne	0x1403f1630 <.text+0x3f0630>
// 0x1403f165a  eb1d                           jmp	0x1403f1679 <.text+0x3f0679>
// 0x1403f165c  0f1f4000                       nopl	(%rax)
// Frames: F35 -> F36 -> F60 -> F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1660  660f74d1                       pcmpeqb	%xmm1, %xmm2
// 0x1403f1664  660fd7fa                       pmovmskb	%xmm2, %edi
// Frames: F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1668  85ff                           testl	%edi, %edi
// 0x1403f166a  7545                           jne	0x1403f16b1 <.text+0x3f06b1>
// Frames: F62 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f166c  4d01cb                         addq	%r9, %r11
// 0x1403f166f  4983c310                       addq	$0x10, %r11
// Frames: F64 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1673  4983c110                       addq	$0x10, %r9
// 0x1403f1677  eb93                           jmp	0x1403f160c <.text+0x3f060c>
// Frames: F65 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1679  488b44d8f8                     movq	-0x8(%rax,%rbx,8), %rax
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f167e  48398698010000                 cmpq	%rax, 0x198(%rsi)
// Frames: F74 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1685  762a                           jbe	0x1403f16b1 <.text+0x3f06b1>
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1687  488b8e90010000                 movq	0x190(%rsi), %rcx
// Frames: F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f168e  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1403f1692  4885c0                         testq	%rax, %rax
// Frames: F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1695  741a                           je	0x1403f16b1 <.text+0x3f06b1>
// Frames: F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1697  488b8e60010000                 movq	0x160(%rsi), %rcx
// Frames: F79 -> F80 -> F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f169e  48f7d0                         notq	%rax
// Frames: F81 -> F82 -> F83 -> F78 -> F84 -> F26 -> F27 -> F21
// 0x1403f16a1  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1403f16a5  48c1e005                       shlq	$0x5, %rax
// 0x1403f16a9  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F26 -> F27 -> F21
// 0x1403f16af  7516                           jne	0x1403f16c7 <.text+0x3f06c7>
// Frames: F85 -> F27 -> F21
// 0x1403f16b1  4889f1                         movq	%rsi, %rcx
// 0x1403f16b4  e827914d00                     callq	0x1408ca7e0 <.text+0x8c97e0>
// Frames: F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f16b9  4883be9000000000               cmpq	$0x0, 0x90(%rsi)
// 0x1403f16c1  0f84da000000                   je	0x1403f17a1 <.text+0x3f07a1>
// Frames: F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f16c7  488b4678                       movq	0x78(%rsi), %rax
// 0x1403f16cb  488b8e80000000                 movq	0x80(%rsi), %rcx
// 0x1403f16d2  488d50e8                       leaq	-0x18(%rax), %rdx
// 0x1403f16d6  49b802f42066f678965f           movabsq	$0x5f9678f66620f402, %r8 # imm = 0x5F9678F66620F402
// 0x1403f16e0  4531c9                         xorl	%r9d, %r9d
// 0x1403f16e3  660f6f0515e9d901               movdqa	0x1d9e915(%rip), %xmm0  # 0x142190000
// 0x1403f16eb  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1403f16ef  49bacddbdc462679dbd0           movabsq	$-0x2f2486d9b9232433, %r10 # imm = 0xD0DB792646DCDBCD
// 0x1403f16f9  4d89c3                         movq	%r8, %r11
// Frames: F29 -> F30 -> F31 -> F32 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f16fc  4921cb                         andq	%rcx, %r11
// 0x1403f16ff  f3420f6f1418                   movdqu	(%rax,%r11), %xmm2
// Frames: F35 -> F36 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1705  660f6fda                       movdqa	%xmm2, %xmm3
// 0x1403f1709  660f74d8                       pcmpeqb	%xmm0, %xmm3
// 0x1403f170d  660fd7fb                       pmovmskb	%xmm3, %edi
// 0x1403f1711  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F38 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1720  6685ff                         testw	%di, %di
// 0x1403f1723  742b                           je	0x1403f1750 <.text+0x3f0750>
// Frames: F40 -> F41 -> F42 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1725  f30fbcdf                       tzcntl	%edi, %ebx
// Frames: F43 -> F44 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1729  8d6fff                         leal	-0x1(%rdi), %ebp
// 0x1403f172c  21fd                           andl	%edi, %ebp
// 0x1403f172e  89ef                           movl	%ebp, %edi
// Frames: F45 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1730  4c01db                         addq	%r11, %rbx
// 0x1403f1733  4821cb                         andq	%rcx, %rbx
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1736  48f7db                         negq	%rbx
// Frames: F51 -> F46 -> F52 -> F53 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1739  488d1c5b                       leaq	(%rbx,%rbx,2), %rbx
// Frames: F54 -> F55 -> F56 -> F57 -> F58 -> F59 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f173d  4c3914da                       cmpq	%r10, (%rdx,%rbx,8)
// Frames: F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1741  75dd                           jne	0x1403f1720 <.text+0x3f0720>
// 0x1403f1743  4c3944da08                     cmpq	%r8, 0x8(%rdx,%rbx,8)
// 0x1403f1748  75d6                           jne	0x1403f1720 <.text+0x3f0720>
// 0x1403f174a  eb1d                           jmp	0x1403f1769 <.text+0x3f0769>
// 0x1403f174c  0f1f4000                       nopl	(%rax)
// Frames: F35 -> F36 -> F60 -> F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1750  660f74d1                       pcmpeqb	%xmm1, %xmm2
// 0x1403f1754  660fd7fa                       pmovmskb	%xmm2, %edi
// Frames: F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1758  85ff                           testl	%edi, %edi
// 0x1403f175a  7545                           jne	0x1403f17a1 <.text+0x3f07a1>
// Frames: F62 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f175c  4d01cb                         addq	%r9, %r11
// 0x1403f175f  4983c310                       addq	$0x10, %r11
// Frames: F64 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1763  4983c110                       addq	$0x10, %r9
// 0x1403f1767  eb93                           jmp	0x1403f16fc <.text+0x3f06fc>
// Frames: F65 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1769  488b44d8f8                     movq	-0x8(%rax,%rbx,8), %rax
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f176e  48398698010000                 cmpq	%rax, 0x198(%rsi)
// Frames: F74 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1775  762a                           jbe	0x1403f17a1 <.text+0x3f07a1>
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1777  488b8e90010000                 movq	0x190(%rsi), %rcx
// Frames: F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f177e  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1403f1782  4885c0                         testq	%rax, %rax
// Frames: F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1785  741a                           je	0x1403f17a1 <.text+0x3f07a1>
// Frames: F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1787  488b8e60010000                 movq	0x160(%rsi), %rcx
// Frames: F79 -> F80 -> F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f178e  48f7d0                         notq	%rax
// Frames: F81 -> F82 -> F83 -> F78 -> F84 -> F26 -> F27 -> F21
// 0x1403f1791  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1403f1795  48c1e005                       shlq	$0x5, %rax
// 0x1403f1799  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F26 -> F27 -> F21
// 0x1403f179f  7516                           jne	0x1403f17b7 <.text+0x3f07b7>
// Frames: F85 -> F27 -> F21
// 0x1403f17a1  4889f1                         movq	%rsi, %rcx
// 0x1403f17a4  e8b77c4d00                     callq	0x1408c9460 <.text+0x8c8460>
// Frames: F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f17a9  4883be9000000000               cmpq	$0x0, 0x90(%rsi)
// 0x1403f17b1  0f84da000000                   je	0x1403f1891 <.text+0x3f0891>
// Frames: F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f17b7  488b4678                       movq	0x78(%rsi), %rax
// 0x1403f17bb  488b8e80000000                 movq	0x80(%rsi), %rcx
// 0x1403f17c2  488d50e8                       leaq	-0x18(%rax), %rdx
// 0x1403f17c6  49b881bd746ceb28ba69           movabsq	$0x69ba28eb6c74bd81, %r8 # imm = 0x69BA28EB6C74BD81
// 0x1403f17d0  4531c9                         xorl	%r9d, %r9d
// 0x1403f17d3  660f6f0565e8d901               movdqa	0x1d9e865(%rip), %xmm0  # 0x142190040
// 0x1403f17db  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1403f17df  49baca689e853cea632c           movabsq	$0x2c63ea3c859e68ca, %r10 # imm = 0x2C63EA3C859E68CA
// 0x1403f17e9  4d89c3                         movq	%r8, %r11
// Frames: F29 -> F30 -> F31 -> F32 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f17ec  4921cb                         andq	%rcx, %r11
// 0x1403f17ef  f3420f6f1418                   movdqu	(%rax,%r11), %xmm2
// Frames: F35 -> F36 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f17f5  660f6fda                       movdqa	%xmm2, %xmm3
// 0x1403f17f9  660f74d8                       pcmpeqb	%xmm0, %xmm3
// 0x1403f17fd  660fd7fb                       pmovmskb	%xmm3, %edi
// 0x1403f1801  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F38 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1810  6685ff                         testw	%di, %di
// 0x1403f1813  742b                           je	0x1403f1840 <.text+0x3f0840>
// Frames: F40 -> F41 -> F42 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1815  f30fbcdf                       tzcntl	%edi, %ebx
// Frames: F43 -> F44 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1819  8d6fff                         leal	-0x1(%rdi), %ebp
// 0x1403f181c  21fd                           andl	%edi, %ebp
// 0x1403f181e  89ef                           movl	%ebp, %edi
// Frames: F45 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1820  4c01db                         addq	%r11, %rbx
// 0x1403f1823  4821cb                         andq	%rcx, %rbx
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1826  48f7db                         negq	%rbx
// Frames: F51 -> F46 -> F52 -> F53 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1829  488d1c5b                       leaq	(%rbx,%rbx,2), %rbx
// Frames: F54 -> F55 -> F56 -> F57 -> F58 -> F59 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f182d  4c3914da                       cmpq	%r10, (%rdx,%rbx,8)
// Frames: F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1831  75dd                           jne	0x1403f1810 <.text+0x3f0810>
// 0x1403f1833  4c3944da08                     cmpq	%r8, 0x8(%rdx,%rbx,8)
// 0x1403f1838  75d6                           jne	0x1403f1810 <.text+0x3f0810>
// 0x1403f183a  eb1d                           jmp	0x1403f1859 <.text+0x3f0859>
// 0x1403f183c  0f1f4000                       nopl	(%rax)
// Frames: F35 -> F36 -> F60 -> F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1840  660f74d1                       pcmpeqb	%xmm1, %xmm2
// 0x1403f1844  660fd7fa                       pmovmskb	%xmm2, %edi
// Frames: F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1848  85ff                           testl	%edi, %edi
// 0x1403f184a  7545                           jne	0x1403f1891 <.text+0x3f0891>
// Frames: F62 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f184c  4d01cb                         addq	%r9, %r11
// 0x1403f184f  4983c310                       addq	$0x10, %r11
// Frames: F64 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1853  4983c110                       addq	$0x10, %r9
// 0x1403f1857  eb93                           jmp	0x1403f17ec <.text+0x3f07ec>
// Frames: F65 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1859  488b44d8f8                     movq	-0x8(%rax,%rbx,8), %rax
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f185e  48398698010000                 cmpq	%rax, 0x198(%rsi)
// Frames: F74 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1865  762a                           jbe	0x1403f1891 <.text+0x3f0891>
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1867  488b8e90010000                 movq	0x190(%rsi), %rcx
// Frames: F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f186e  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1403f1872  4885c0                         testq	%rax, %rax
// Frames: F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1875  741a                           je	0x1403f1891 <.text+0x3f0891>
// Frames: F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1877  488b8e60010000                 movq	0x160(%rsi), %rcx
// Frames: F79 -> F80 -> F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f187e  48f7d0                         notq	%rax
// Frames: F81 -> F82 -> F83 -> F78 -> F84 -> F26 -> F27 -> F21
// 0x1403f1881  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1403f1885  48c1e005                       shlq	$0x5, %rax
// 0x1403f1889  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F26 -> F27 -> F21
// 0x1403f188f  7516                           jne	0x1403f18a7 <.text+0x3f08a7>
// Frames: F85 -> F27 -> F21
// 0x1403f1891  4889f1                         movq	%rsi, %rcx
// 0x1403f1894  e8078c4d00                     callq	0x1408ca4a0 <.text+0x8c94a0>
// Frames: F22 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1899  4883be9000000000               cmpq	$0x0, 0x90(%rsi)
// 0x1403f18a1  0f84da000000                   je	0x1403f1981 <.text+0x3f0981>
// Frames: F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f18a7  488b4678                       movq	0x78(%rsi), %rax
// 0x1403f18ab  488b8e80000000                 movq	0x80(%rsi), %rcx
// 0x1403f18b2  488d50e8                       leaq	-0x18(%rax), %rdx
// 0x1403f18b6  49b8729cc83d22b2beee           movabsq	$-0x11414dddc237638e, %r8 # imm = 0xEEBEB2223DC89C72
// 0x1403f18c0  4531c9                         xorl	%r9d, %r9d
// 0x1403f18c3  660f6f05a5e7d901               movdqa	0x1d9e7a5(%rip), %xmm0  # 0x142190070
// 0x1403f18cb  660f76c9                       pcmpeqd	%xmm1, %xmm1
// 0x1403f18cf  49bae32a2b7b4d7d1229           movabsq	$0x29127d4d7b2b2ae3, %r10 # imm = 0x29127D4D7B2B2AE3
// 0x1403f18d9  4d89c3                         movq	%r8, %r11
// Frames: F29 -> F30 -> F31 -> F32 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f18dc  4921cb                         andq	%rcx, %r11
// 0x1403f18df  f3420f6f1418                   movdqu	(%rax,%r11), %xmm2
// Frames: F35 -> F36 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f18e5  660f6fda                       movdqa	%xmm2, %xmm3
// 0x1403f18e9  660f74d8                       pcmpeqb	%xmm0, %xmm3
// 0x1403f18ed  660fd7fb                       pmovmskb	%xmm3, %edi
// 0x1403f18f1  6666666666662e0f1f840000000000 nopw	%cs:(%rax,%rax)
// Frames: F38 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1900  6685ff                         testw	%di, %di
// 0x1403f1903  742b                           je	0x1403f1930 <.text+0x3f0930>
// Frames: F40 -> F41 -> F42 -> F39 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1905  f30fbcdf                       tzcntl	%edi, %ebx
// Frames: F43 -> F44 -> F37 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1909  8d6fff                         leal	-0x1(%rdi), %ebp
// 0x1403f190c  21fd                           andl	%edi, %ebp
// 0x1403f190e  89ef                           movl	%ebp, %edi
// Frames: F45 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1910  4c01db                         addq	%r11, %rbx
// 0x1403f1913  4821cb                         andq	%rcx, %rbx
// Frames: F46 -> F47 -> F48 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1916  48f7db                         negq	%rbx
// Frames: F51 -> F46 -> F52 -> F53 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1919  488d1c5b                       leaq	(%rbx,%rbx,2), %rbx
// Frames: F54 -> F55 -> F56 -> F57 -> F58 -> F59 -> F49 -> F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f191d  4c3914da                       cmpq	%r10, (%rdx,%rbx,8)
// Frames: F50 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1921  75dd                           jne	0x1403f1900 <.text+0x3f0900>
// 0x1403f1923  4c3944da08                     cmpq	%r8, 0x8(%rdx,%rbx,8)
// 0x1403f1928  75d6                           jne	0x1403f1900 <.text+0x3f0900>
// 0x1403f192a  eb1d                           jmp	0x1403f1949 <.text+0x3f0949>
// 0x1403f192c  0f1f4000                       nopl	(%rax)
// Frames: F35 -> F36 -> F60 -> F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1930  660f74d1                       pcmpeqb	%xmm1, %xmm2
// 0x1403f1934  660fd7fa                       pmovmskb	%xmm2, %edi
// Frames: F61 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1938  85ff                           testl	%edi, %edi
// 0x1403f193a  7545                           jne	0x1403f1981 <.text+0x3f0981>
// Frames: F62 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f193c  4d01cb                         addq	%r9, %r11
// 0x1403f193f  4983c310                       addq	$0x10, %r11
// Frames: F64 -> F63 -> F33 -> F34 -> F28 -> F23 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1943  4983c110                       addq	$0x10, %r9
// 0x1403f1947  eb93                           jmp	0x1403f18dc <.text+0x3f08dc>
// Frames: F65 -> F24 -> F25 -> F26 -> F27 -> F21
// 0x1403f1949  488b44d8f8                     movq	-0x8(%rax,%rbx,8), %rax
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f194e  48398698010000                 cmpq	%rax, 0x198(%rsi)
// Frames: F74 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1955  762a                           jbe	0x1403f1981 <.text+0x3f0981>
// Frames: F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1957  488b8e90010000                 movq	0x190(%rsi), %rcx
// Frames: F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f195e  488b04c1                       movq	(%rcx,%rax,8), %rax
// 0x1403f1962  4885c0                         testq	%rax, %rax
// Frames: F68 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1965  741a                           je	0x1403f1981 <.text+0x3f0981>
// Frames: F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f1967  488b8e60010000                 movq	0x160(%rsi), %rcx
// Frames: F79 -> F80 -> F78 -> F69 -> F70 -> F71 -> F72 -> F73 -> F26 -> F27 -> F21
// 0x1403f196e  48f7d0                         notq	%rax
// Frames: F81 -> F82 -> F83 -> F78 -> F84 -> F26 -> F27 -> F21
// 0x1403f1971  488d0440                       leaq	(%rax,%rax,2), %rax
// 0x1403f1975  48c1e005                       shlq	$0x5, %rax
// 0x1403f1979  48837c012000                   cmpq	$0x0, 0x20(%rcx,%rax)
// Frames: F26 -> F27 -> F21
// 0x1403f197f  7508                           jne	0x1403f1989 <.text+0x3f0989>
// Frames: F85 -> F27 -> F21
// 0x1403f1981  4889f1                         movq	%rsi, %rcx
// 0x1403f1984  e817714d00                     callq	0x1408c8aa0 <.text+0x8c7aa0>
// Frames: F86
// 0x1403f1989  4889f0                         movq	%rsi, %rax
// 0x1403f198c  4883c458                       addq	$0x58, %rsp
// 0x1403f1990  5b                             popq	%rbx
// 0x1403f1991  5d                             popq	%rbp
// 0x1403f1992  5f                             popq	%rdi
// 0x1403f1993  5e                             popq	%rsi
// 0x1403f1994  c3                             retq
//
// RANGE rva_8ea200_len_215: RVA 0x8ea200, 533 PDB bytes; 107 instructions
// Evidence: _assembly/roof-v1/A/pack-001/rva_8ea200_len_215.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 766:944: bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>
// Frames: F87
// 0x1408ea200  56                             pushq	%rsi
// 0x1408ea201  57                             pushq	%rdi
// 0x1408ea202  55                             pushq	%rbp
// 0x1408ea203  53                             pushq	%rbx
// 0x1408ea204  4881ec88000000                 subq	$0x88, %rsp
// 0x1408ea20b  bd01000000                     movl	$0x1, %ebp
// Frames: F88 -> F89 -> F90 -> F91 -> F92
// 0x1408ea210  f0                             lock
// 0x1408ea211  0fc1aa48040000                 xaddl	%ebp, 0x448(%rdx)
// Frames: F93 -> F94
// 0x1408ea218  48833900                       cmpq	$0x0, (%rcx)
// 0x1408ea21c  0f84ce010000                   je	0x1408ea3f0 <.text+0x8e93f0>
// Frames: F95
// 0x1408ea222  4889ce                         movq	%rcx, %rsi
// 0x1408ea225  488b4108                       movq	0x8(%rcx), %rax
// Frames: F69 -> F70 -> F96 -> F97 -> F98 -> F95
// 0x1408ea229  4c8b8298010000                 movq	0x198(%rdx), %r8
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F96 -> F97 -> F98 -> F95
// 0x1408ea230  4939c0                         cmpq	%rax, %r8
// Frames: F74 -> F68 -> F69 -> F70 -> F96 -> F97 -> F98 -> F95
// 0x1408ea233  0f86ad000000                   jbe	0x1408ea2e6 <.text+0x8e92e6>
// Frames: F69 -> F70 -> F96 -> F97 -> F98 -> F95
// 0x1408ea239  4c8b8a90010000                 movq	0x190(%rdx), %r9
// Frames: F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F96 -> F97 -> F98 -> F95
// 0x1408ea240  498b0cc1                       movq	(%r9,%rax,8), %rcx
// 0x1408ea244  4885c9                         testq	%rcx, %rcx
// Frames: F68 -> F69 -> F70 -> F96 -> F97 -> F98 -> F95
// 0x1408ea247  0f8499000000                   je	0x1408ea2e6 <.text+0x8e92e6>
// Frames: F78 -> F69 -> F70 -> F96 -> F97 -> F98 -> F95
// 0x1408ea24d  488b8260010000                 movq	0x160(%rdx), %rax
// Frames: F79 -> F80 -> F78 -> F69 -> F70 -> F96 -> F97 -> F98 -> F95
// 0x1408ea254  48f7d1                         notq	%rcx
// Frames: F99 -> F100 -> F101 -> F80 -> F78 -> F69 -> F70 -> F96 -> F97 -> F98 -> F95
// 0x1408ea257  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1408ea25b  48c1e105                       shlq	$0x5, %rcx
// Frames: F81 -> F82 -> F102 -> F103 -> F97 -> F98 -> F95
// 0x1408ea25f  48837c082000                   cmpq	$0x0, 0x20(%rax,%rcx)
// Frames: F104 -> F102 -> F103 -> F97 -> F98 -> F95
// 0x1408ea265  747f                           je	0x1408ea2e6 <.text+0x8e92e6>
// Frames: F95
// 0x1408ea267  4c8b5610                       movq	0x10(%rsi), %r10
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F105 -> F106 -> F98 -> F95
// 0x1408ea26b  4d39d0                         cmpq	%r10, %r8
// Frames: F74 -> F68 -> F69 -> F70 -> F105 -> F106 -> F98 -> F95
// 0x1408ea26e  0f86f7000000                   jbe	0x1408ea36b <.text+0x8e936b>
// Frames: F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F105 -> F106 -> F98 -> F95
// 0x1408ea274  4f8b04d1                       movq	(%r9,%r10,8), %r8
// 0x1408ea278  4d85c0                         testq	%r8, %r8
// Frames: F68 -> F69 -> F70 -> F105 -> F106 -> F98 -> F95
// 0x1408ea27b  0f84ea000000                   je	0x1408ea36b <.text+0x8e936b>
// Frames: F79 -> F80 -> F78 -> F69 -> F70 -> F105 -> F106 -> F98 -> F95
// 0x1408ea281  49f7d0                         notq	%r8
// Frames: F99 -> F100 -> F101 -> F80 -> F78 -> F69 -> F70 -> F105 -> F106 -> F98 -> F95
// 0x1408ea284  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x1408ea288  49c1e005                       shlq	$0x5, %r8
// Frames: F81 -> F82 -> F102 -> F107 -> F106 -> F98 -> F95
// 0x1408ea28c  4a837c002000                   cmpq	$0x0, 0x20(%rax,%r8)
// Frames: F104 -> F102 -> F107 -> F106 -> F98 -> F95
// 0x1408ea292  0f84d3000000                   je	0x1408ea36b <.text+0x8e936b>
// Frames: F98 -> F95
// 0x1408ea298  4801c1                         addq	%rax, %rcx
// 0x1408ea29b  488b7910                       movq	0x10(%rcx), %rdi
// 0x1408ea29f  4901c0                         addq	%rax, %r8
// Frames: F108 -> F109 -> F110 -> F104 -> F102 -> F107 -> F106 -> F98 -> F95
// 0x1408ea2a2  498b4810                       movq	0x10(%r8), %rcx
// Frames: F111 -> F112 -> F113 -> F114 -> F115 -> F106 -> F98 -> F95
// 0x1408ea2a6  8b8248040000                   movl	0x448(%rdx), %eax
// Frames: F116 -> F117 -> F118 -> F119 -> F120 -> F121 -> F122 -> F123
// 0x1408ea2ac  4189685c                       movl	%ebp, 0x5c(%r8)
// Frames: F118 -> F119 -> F120 -> F121 -> F122 -> F123
// 0x1408ea2b0  e8ebadfeff                     callq	0x1408d50a0 <.text+0x8d40a0>
// Frames: F124 -> F118 -> F119 -> F120 -> F121 -> F122 -> F123
// 0x1408ea2b5  4885c0                         testq	%rax, %rax
// 0x1408ea2b8  0f844a010000                   je	0x1408ea408 <.text+0x8e9408>
// Frames: F125 -> F119 -> F120 -> F121 -> F122 -> F123
// 0x1408ea2be  4889c3                         movq	%rax, %rbx
// 0x1408ea2c1  4889f9                         movq	%rdi, %rcx
// 0x1408ea2c4  e8570ad7ff                     callq	0x14065ad20 <.text+0x659d20>
// 0x1408ea2c9  4889d9                         movq	%rbx, %rcx
// 0x1408ea2cc  4889c2                         movq	%rax, %rdx
// 0x1408ea2cf  e8ec01ffff                     callq	0x1408da4c0 <.text+0x8d94c0>
// Frames: F126
// 0x1408ea2d4  89aef8000000                   movl	%ebp, 0xf8(%rsi)
// Frames: F127
// 0x1408ea2da  4881c488000000                 addq	$0x88, %rsp
// 0x1408ea2e1  5b                             popq	%rbx
// 0x1408ea2e2  5d                             popq	%rbp
// 0x1408ea2e3  5f                             popq	%rdi
// 0x1408ea2e4  5e                             popq	%rsi
// 0x1408ea2e5  c3                             retq
// Frames: F128 -> F129 -> F130 -> F98 -> F95
// 0x1408ea2e6  4881c690000000                 addq	$0x90, %rsi
// Frames: F131 -> F132 -> F129 -> F130 -> F98 -> F95
// 0x1408ea2ed  488d0504259f01                 leaq	0x19f2504(%rip), %rax   # 0x1422dc7f8
// 0x1408ea2f4  4889442428                     movq	%rax, 0x28(%rsp)
// 0x1408ea2f9  48c74424302c000000             movq	$0x2c, 0x30(%rsp)
// Frames: F128 -> F129 -> F130 -> F98 -> F95
// 0x1408ea302  4889742438                     movq	%rsi, 0x38(%rsp)
// 0x1408ea307  488d05e29c71ff                 leaq	-0x8e631e(%rip), %rax   # 0x140003ff0 <.text+0x2ff0>
// 0x1408ea30e  4889442440                     movq	%rax, 0x40(%rsp)
// 0x1408ea313  488d442428                     leaq	0x28(%rsp), %rax
// 0x1408ea318  4889442448                     movq	%rax, 0x48(%rsp)
// 0x1408ea31d  488d056c1c78ff                 leaq	-0x87e394(%rip), %rax   # 0x14006bf90 <.text+0x6af90>
// 0x1408ea324  4889442450                     movq	%rax, 0x50(%rsp)
// Frames: F133 -> F134 -> F129 -> F130 -> F98 -> F95
// 0x1408ea329  488d05a8249f01                 leaq	0x19f24a8(%rip), %rax   # 0x1422dc7d8
// 0x1408ea330  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1408ea335  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x1408ea33e  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x1408ea347  488d442438                     leaq	0x38(%rsp), %rax
// 0x1408ea34c  4889442468                     movq	%rax, 0x68(%rsp)
// 0x1408ea351  48c744247002000000             movq	$0x2, 0x70(%rsp)
// Frames: F128 -> F129 -> F130 -> F98 -> F95
// 0x1408ea35a  488d152f259f01                 leaq	0x19f252f(%rip), %rdx   # 0x1422dc890
// 0x1408ea361  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x1408ea366  e815477c01                     callq	0x1420aea80 <.text+0x20ada80>
// Frames: F135 -> F129 -> F136 -> F98 -> F95
// 0x1408ea36b  4881c690000000                 addq	$0x90, %rsi
// Frames: F131 -> F137 -> F129 -> F136 -> F98 -> F95
// 0x1408ea372  488d052f259f01                 leaq	0x19f252f(%rip), %rax   # 0x1422dc8a8
// 0x1408ea379  4889442428                     movq	%rax, 0x28(%rsp)
// 0x1408ea37e  48c74424303d000000             movq	$0x3d, 0x30(%rsp)
// Frames: F135 -> F129 -> F136 -> F98 -> F95
// 0x1408ea387  4889742438                     movq	%rsi, 0x38(%rsp)
// 0x1408ea38c  488d055d9c71ff                 leaq	-0x8e63a3(%rip), %rax   # 0x140003ff0 <.text+0x2ff0>
// 0x1408ea393  4889442440                     movq	%rax, 0x40(%rsp)
// 0x1408ea398  488d442428                     leaq	0x28(%rsp), %rax
// 0x1408ea39d  4889442448                     movq	%rax, 0x48(%rsp)
// 0x1408ea3a2  488d05e71b78ff                 leaq	-0x87e419(%rip), %rax   # 0x14006bf90 <.text+0x6af90>
// 0x1408ea3a9  4889442450                     movq	%rax, 0x50(%rsp)
// Frames: F133 -> F138 -> F129 -> F136 -> F98 -> F95
// 0x1408ea3ae  488d0523249f01                 leaq	0x19f2423(%rip), %rax   # 0x1422dc7d8
// 0x1408ea3b5  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1408ea3ba  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x1408ea3c3  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x1408ea3cc  488d442438                     leaq	0x38(%rsp), %rax
// 0x1408ea3d1  4889442468                     movq	%rax, 0x68(%rsp)
// 0x1408ea3d6  48c744247002000000             movq	$0x2, 0x70(%rsp)
// Frames: F135 -> F129 -> F136 -> F98 -> F95
// 0x1408ea3df  488d1502259f01                 leaq	0x19f2502(%rip), %rdx   # 0x1422dc8e8
// 0x1408ea3e6  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x1408ea3eb  e890467c01                     callq	0x1420aea80 <.text+0x20ada80>
// Frames: F139 -> F94
// 0x1408ea3f0  488d0de9259f01                 leaq	0x19f25e9(%rip), %rcx   # 0x1422dc9e0
// 0x1408ea3f7  4c8d0542269f01                 leaq	0x19f2642(%rip), %r8    # 0x1422dca40
// 0x1408ea3fe  ba5f000000                     movl	$0x5f, %edx
// 0x1408ea403  e858467c01                     callq	0x1420aea60 <.text+0x20ada60>
// Frames: F140 -> F118 -> F119 -> F120 -> F121 -> F122 -> F123
// 0x1408ea408  488d0dc902cd01                 leaq	0x1cd02c9(%rip), %rcx   # 0x1425ba6d8
// 0x1408ea40f  e82c467c01                     callq	0x1420aea40 <.text+0x20ada40>
// 0x1408ea414  cc                             int3
//
// RANGE rva_12d5c50_len_2f5: RVA 0x12d5c50, 757 PDB bytes; 148 instructions
// Evidence: _assembly/roof-v1/A/pack-001/rva_12d5c50_len_2f5.json
// Same-address PDB aliases: 1; retain all, no exclusive ownership claim.
// PDB 1927:246500: bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >
// Frames: F141
// 0x1412d5c50  56                             pushq	%rsi
// 0x1412d5c51  57                             pushq	%rdi
// 0x1412d5c52  55                             pushq	%rbp
// 0x1412d5c53  53                             pushq	%rbx
// 0x1412d5c54  4881ec88000000                 subq	$0x88, %rsp
// 0x1412d5c5b  4889ce                         movq	%rcx, %rsi
// Frames: F142 -> F143 -> F144 -> F145
// 0x1412d5c5e  488b8240040000                 movq	0x440(%rdx), %rax
// Frames: F144 -> F145
// 0x1412d5c65  4889442430                     movq	%rax, 0x30(%rsp)
// 0x1412d5c6a  48c744242801000000             movq	$0x1, 0x28(%rsp)
// Frames: F146 -> F147 -> F148 -> F145
// 0x1412d5c73  4883791800                     cmpq	$0x0, 0x18(%rcx)
// 0x1412d5c78  0f8422020000                   je	0x1412d5ea0 <.text+0x12d4ea0>
// 0x1412d5c7e  48394620                       cmpq	%rax, 0x20(%rsi)
// 0x1412d5c82  0f8518020000                   jne	0x1412d5ea0 <.text+0x12d4ea0>
// Frames: F149 -> F150 -> F151 -> F145
// 0x1412d5c88  488b82a8000000                 movq	0xa8(%rdx), %rax
// Frames: F152 -> F153 -> F151 -> F145
// 0x1412d5c8f  8b8e00010000                   movl	0x100(%rsi), %ecx
// Frames: F154 -> F155 -> F151 -> F145
// 0x1412d5c95  898600010000                   movl	%eax, 0x100(%rsi)
// Frames: F156 -> F157 -> F158 -> F159 -> F160 -> F145
// 0x1412d5c9b  4839c8                         cmpq	%rcx, %rax
// 0x1412d5c9e  0f8249020000                   jb	0x1412d5eed <.text+0x12d4eed>
// Frames: F93 -> F161 -> F145
// 0x1412d5ca4  740a                           je	0x1412d5cb0 <.text+0x12d4cb0>
// 0x1412d5ca6  48833e00                       cmpq	$0x0, (%rsi)
// 0x1412d5caa  0f844c020000                   je	0x1412d5efc <.text+0x12d4efc>
// Frames: F88 -> F89 -> F90 -> F91 -> F162 -> F163
// 0x1412d5cb0  bd01000000                     movl	$0x1, %ebp
// 0x1412d5cb5  f0                             lock
// 0x1412d5cb6  0fc1aa48040000                 xaddl	%ebp, 0x448(%rdx)
// Frames: F93 -> F164 -> F163
// 0x1412d5cbd  48833e00                       cmpq	$0x0, (%rsi)
// 0x1412d5cc1  0f8441020000                   je	0x1412d5f08 <.text+0x12d4f08>
// Frames: F165 -> F163
// 0x1412d5cc7  488b4608                       movq	0x8(%rsi), %rax
// Frames: F69 -> F70 -> F96 -> F97 -> F98 -> F165 -> F163
// 0x1412d5ccb  4c8b8298010000                 movq	0x198(%rdx), %r8
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F96 -> F97 -> F98 -> F165 -> F163
// 0x1412d5cd2  4939c0                         cmpq	%rax, %r8
// Frames: F74 -> F68 -> F69 -> F70 -> F96 -> F97 -> F98 -> F165 -> F163
// 0x1412d5cd5  0f86bb000000                   jbe	0x1412d5d96 <.text+0x12d4d96>
// Frames: F69 -> F70 -> F96 -> F97 -> F98 -> F165 -> F163
// 0x1412d5cdb  4c8b8a90010000                 movq	0x190(%rdx), %r9
// Frames: F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F96 -> F97 -> F98 -> F165 -> F163
// 0x1412d5ce2  498b0cc1                       movq	(%r9,%rax,8), %rcx
// 0x1412d5ce6  4885c9                         testq	%rcx, %rcx
// Frames: F68 -> F69 -> F70 -> F96 -> F97 -> F98 -> F165 -> F163
// 0x1412d5ce9  0f84a7000000                   je	0x1412d5d96 <.text+0x12d4d96>
// Frames: F78 -> F69 -> F70 -> F96 -> F97 -> F98 -> F165 -> F163
// 0x1412d5cef  488b8260010000                 movq	0x160(%rdx), %rax
// Frames: F79 -> F80 -> F78 -> F69 -> F70 -> F96 -> F97 -> F98 -> F165 -> F163
// 0x1412d5cf6  48f7d1                         notq	%rcx
// Frames: F99 -> F100 -> F101 -> F80 -> F78 -> F69 -> F70 -> F96 -> F97 -> F98 -> F165 -> F163
// 0x1412d5cf9  488d0c49                       leaq	(%rcx,%rcx,2), %rcx
// 0x1412d5cfd  48c1e105                       shlq	$0x5, %rcx
// Frames: F81 -> F82 -> F102 -> F103 -> F97 -> F98 -> F165 -> F163
// 0x1412d5d01  48837c082000                   cmpq	$0x0, 0x20(%rax,%rcx)
// Frames: F104 -> F102 -> F103 -> F97 -> F98 -> F165 -> F163
// 0x1412d5d07  0f8489000000                   je	0x1412d5d96 <.text+0x12d4d96>
// Frames: F165 -> F163
// 0x1412d5d0d  4c8b5610                       movq	0x10(%rsi), %r10
// Frames: F66 -> F67 -> F68 -> F69 -> F70 -> F105 -> F106 -> F98 -> F165 -> F163
// 0x1412d5d11  4d39d0                         cmpq	%r10, %r8
// Frames: F74 -> F68 -> F69 -> F70 -> F105 -> F106 -> F98 -> F165 -> F163
// 0x1412d5d14  0f8601010000                   jbe	0x1412d5e1b <.text+0x12d4e1b>
// Frames: F75 -> F76 -> F77 -> F68 -> F69 -> F70 -> F105 -> F106 -> F98 -> F165 -> F163
// 0x1412d5d1a  4f8b04d1                       movq	(%r9,%r10,8), %r8
// 0x1412d5d1e  4d85c0                         testq	%r8, %r8
// Frames: F68 -> F69 -> F70 -> F105 -> F106 -> F98 -> F165 -> F163
// 0x1412d5d21  0f84f4000000                   je	0x1412d5e1b <.text+0x12d4e1b>
// Frames: F79 -> F80 -> F78 -> F69 -> F70 -> F105 -> F106 -> F98 -> F165 -> F163
// 0x1412d5d27  49f7d0                         notq	%r8
// Frames: F99 -> F100 -> F101 -> F80 -> F78 -> F69 -> F70 -> F105 -> F106 -> F98 -> F165 -> F163
// 0x1412d5d2a  4f8d0440                       leaq	(%r8,%r8,2), %r8
// 0x1412d5d2e  49c1e005                       shlq	$0x5, %r8
// Frames: F81 -> F82 -> F102 -> F107 -> F106 -> F98 -> F165 -> F163
// 0x1412d5d32  4a837c002000                   cmpq	$0x0, 0x20(%rax,%r8)
// Frames: F104 -> F102 -> F107 -> F106 -> F98 -> F165 -> F163
// 0x1412d5d38  0f84dd000000                   je	0x1412d5e1b <.text+0x12d4e1b>
// Frames: F98 -> F165 -> F163
// 0x1412d5d3e  4801c1                         addq	%rax, %rcx
// 0x1412d5d41  488b7910                       movq	0x10(%rcx), %rdi
// 0x1412d5d45  4901c0                         addq	%rax, %r8
// Frames: F108 -> F109 -> F110 -> F104 -> F102 -> F107 -> F106 -> F98 -> F165 -> F163
// 0x1412d5d48  498b4810                       movq	0x10(%r8), %rcx
// Frames: F111 -> F112 -> F113 -> F114 -> F115 -> F106 -> F98 -> F165 -> F163
// 0x1412d5d4c  8b8248040000                   movl	0x448(%rdx), %eax
// Frames: F116 -> F117 -> F118 -> F119 -> F120 -> F121 -> F122 -> F166 -> F163
// 0x1412d5d52  4189685c                       movl	%ebp, 0x5c(%r8)
// Frames: F118 -> F119 -> F120 -> F121 -> F122 -> F166 -> F163
// 0x1412d5d56  e845f35fff                     callq	0x1408d50a0 <.text+0x8d40a0>
// Frames: F124 -> F118 -> F119 -> F120 -> F121 -> F122 -> F166 -> F163
// 0x1412d5d5b  4885c0                         testq	%rax, %rax
// 0x1412d5d5e  0f84bc010000                   je	0x1412d5f20 <.text+0x12d4f20>
// Frames: F125 -> F119 -> F120 -> F121 -> F122 -> F166 -> F163
// 0x1412d5d64  4889c3                         movq	%rax, %rbx
// 0x1412d5d67  4889f9                         movq	%rdi, %rcx
// 0x1412d5d6a  e8b14f38ff                     callq	0x14065ad20 <.text+0x659d20>
// 0x1412d5d6f  4889d9                         movq	%rbx, %rcx
// 0x1412d5d72  4889c2                         movq	%rax, %rdx
// 0x1412d5d75  e8464760ff                     callq	0x1408da4c0 <.text+0x8d94c0>
// Frames: F167 -> F163
// 0x1412d5d7a  89aef8000000                   movl	%ebp, 0xf8(%rsi)
// Frames: F93 -> F168 -> F169
// 0x1412d5d80  48833e00                       cmpq	$0x0, (%rsi)
// 0x1412d5d84  0f84a2010000                   je	0x1412d5f2c <.text+0x12d4f2c>
// Frames: F170
// 0x1412d5d8a  4881c488000000                 addq	$0x88, %rsp
// 0x1412d5d91  5b                             popq	%rbx
// 0x1412d5d92  5d                             popq	%rbp
// 0x1412d5d93  5f                             popq	%rdi
// 0x1412d5d94  5e                             popq	%rsi
// 0x1412d5d95  c3                             retq
// Frames: F128 -> F129 -> F130 -> F98 -> F165 -> F163
// 0x1412d5d96  4881c690000000                 addq	$0x90, %rsi
// Frames: F131 -> F132 -> F129 -> F130 -> F98 -> F165 -> F163
// 0x1412d5d9d  488d05b6ba2d01                 leaq	0x12dbab6(%rip), %rax   # 0x1425b185a
// 0x1412d5da4  4889442448                     movq	%rax, 0x48(%rsp)
// 0x1412d5da9  48c74424502c000000             movq	$0x2c, 0x50(%rsp)
// Frames: F128 -> F129 -> F130 -> F98 -> F165 -> F163
// 0x1412d5db2  4889742428                     movq	%rsi, 0x28(%rsp)
// 0x1412d5db7  488d0532e2d2fe                 leaq	-0x12d1dce(%rip), %rax  # 0x140003ff0 <.text+0x2ff0>
// 0x1412d5dbe  4889442430                     movq	%rax, 0x30(%rsp)
// 0x1412d5dc3  488d442448                     leaq	0x48(%rsp), %rax
// 0x1412d5dc8  4889442438                     movq	%rax, 0x38(%rsp)
// 0x1412d5dcd  488d05bc61d9fe                 leaq	-0x1269e44(%rip), %rax  # 0x14006bf90 <.text+0x6af90>
// 0x1412d5dd4  4889442440                     movq	%rax, 0x40(%rsp)
// Frames: F133 -> F134 -> F129 -> F130 -> F98 -> F165 -> F163
// 0x1412d5dd9  488d0580b82d01                 leaq	0x12db880(%rip), %rax   # 0x1425b1660
// 0x1412d5de0  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1412d5de5  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x1412d5dee  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x1412d5df7  488d442428                     leaq	0x28(%rsp), %rax
// 0x1412d5dfc  4889442468                     movq	%rax, 0x68(%rsp)
// 0x1412d5e01  48c744247002000000             movq	$0x2, 0x70(%rsp)
// Frames: F128 -> F129 -> F130 -> F98 -> F165 -> F163
// 0x1412d5e0a  488d1517b92d01                 leaq	0x12db917(%rip), %rdx   # 0x1425b1728
// 0x1412d5e11  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x1412d5e16  e8658cdd00                     callq	0x1420aea80 <.text+0x20ada80>
// Frames: F135 -> F129 -> F136 -> F98 -> F165 -> F163
// 0x1412d5e1b  4881c690000000                 addq	$0x90, %rsi
// Frames: F131 -> F137 -> F129 -> F136 -> F98 -> F165 -> F163
// 0x1412d5e22  488d0555bb2d01                 leaq	0x12dbb55(%rip), %rax   # 0x1425b197e
// 0x1412d5e29  4889442448                     movq	%rax, 0x48(%rsp)
// 0x1412d5e2e  48c74424503d000000             movq	$0x3d, 0x50(%rsp)
// Frames: F135 -> F129 -> F136 -> F98 -> F165 -> F163
// 0x1412d5e37  4889742428                     movq	%rsi, 0x28(%rsp)
// 0x1412d5e3c  488d05ade1d2fe                 leaq	-0x12d1e53(%rip), %rax  # 0x140003ff0 <.text+0x2ff0>
// 0x1412d5e43  4889442430                     movq	%rax, 0x30(%rsp)
// 0x1412d5e48  488d442448                     leaq	0x48(%rsp), %rax
// 0x1412d5e4d  4889442438                     movq	%rax, 0x38(%rsp)
// 0x1412d5e52  488d053761d9fe                 leaq	-0x1269ec9(%rip), %rax  # 0x14006bf90 <.text+0x6af90>
// 0x1412d5e59  4889442440                     movq	%rax, 0x40(%rsp)
// Frames: F133 -> F138 -> F129 -> F136 -> F98 -> F165 -> F163
// 0x1412d5e5e  488d05fbb72d01                 leaq	0x12db7fb(%rip), %rax   # 0x1425b1660
// 0x1412d5e65  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1412d5e6a  48c744246002000000             movq	$0x2, 0x60(%rsp)
// 0x1412d5e73  48c744247800000000             movq	$0x0, 0x78(%rsp)
// 0x1412d5e7c  488d442428                     leaq	0x28(%rsp), %rax
// 0x1412d5e81  4889442468                     movq	%rax, 0x68(%rsp)
// 0x1412d5e86  48c744247002000000             movq	$0x2, 0x70(%rsp)
// Frames: F135 -> F129 -> F136 -> F98 -> F165 -> F163
// 0x1412d5e8f  488d155abb2d01                 leaq	0x12dbb5a(%rip), %rdx   # 0x1425b19f0
// 0x1412d5e96  488d4c2458                     leaq	0x58(%rsp), %rcx
// 0x1412d5e9b  e8e08bdd00                     callq	0x1420aea80 <.text+0x20ada80>
// Frames: F144 -> F145
// 0x1412d5ea0  488d0519c52d01                 leaq	0x12dc519(%rip), %rax   # 0x1425b23c0
// 0x1412d5ea7  4889442458                     movq	%rax, 0x58(%rsp)
// 0x1412d5eac  48c744246001000000             movq	$0x1, 0x60(%rsp)
// 0x1412d5eb5  488d0584b82d01                 leaq	0x12db884(%rip), %rax   # 0x1425b1740
// 0x1412d5ebc  4889442468                     movq	%rax, 0x68(%rsp)
// 0x1412d5ec1  0f57c0                         xorps	%xmm0, %xmm0
// 0x1412d5ec4  0f11442470                     movups	%xmm0, 0x70(%rsp)
// 0x1412d5ec9  4883c618                       addq	$0x18, %rsi
// 0x1412d5ecd  488d05fcc42d01                 leaq	0x12dc4fc(%rip), %rax   # 0x1425b23d0
// 0x1412d5ed4  4889442420                     movq	%rax, 0x20(%rsp)
// 0x1412d5ed9  4c8d442428                     leaq	0x28(%rsp), %r8
// 0x1412d5ede  4c8d4c2458                     leaq	0x58(%rsp), %r9
// 0x1412d5ee3  31c9                           xorl	%ecx, %ecx
// 0x1412d5ee5  4889f2                         movq	%rsi, %rdx
// 0x1412d5ee8  e8b3b7dd00                     callq	0x1420b16a0 <.text+0x20b06a0>
// Frames: F171 -> F157 -> F158 -> F159 -> F160 -> F145
// 0x1412d5eed  4c8d05f4c42d01                 leaq	0x12dc4f4(%rip), %r8    # 0x1425b23e8
// 0x1412d5ef4  4889c2                         movq	%rax, %rdx
// 0x1412d5ef7  e8c490dd00                     callq	0x1420aefc0 <.text+0x20adfc0>
// Frames: F172 -> F161 -> F145
// 0x1412d5efc  488d0dfdc42d01                 leaq	0x12dc4fd(%rip), %rcx   # 0x1425b2400
// 0x1412d5f03  e8388bdd00                     callq	0x1420aea40 <.text+0x20ada40>
// Frames: F139 -> F164 -> F163
// 0x1412d5f08  488d0dd1bc2d01                 leaq	0x12dbcd1(%rip), %rcx   # 0x1425b1be0
// 0x1412d5f0f  4c8d0592bd2d01                 leaq	0x12dbd92(%rip), %r8    # 0x1425b1ca8
// 0x1412d5f16  ba5f000000                     movl	$0x5f, %edx
// 0x1412d5f1b  e8408bdd00                     callq	0x1420aea60 <.text+0x20ada60>
// Frames: F140 -> F118 -> F119 -> F120 -> F121 -> F122 -> F166 -> F163
// 0x1412d5f20  488d0db1472e01                 leaq	0x12e47b1(%rip), %rcx   # 0x1425ba6d8
// 0x1412d5f27  e8148bdd00                     callq	0x1420aea40 <.text+0x20ada40>
// Frames: F139 -> F168 -> F169
// 0x1412d5f2c  488d0dadbc2d01                 leaq	0x12dbcad(%rip), %rcx   # 0x1425b1be0
// 0x1412d5f33  4c8d0586bd2d01                 leaq	0x12dbd86(%rip), %r8    # 0x1425b1cc0
// 0x1412d5f3a  ba5f000000                     movl	$0x5f, %edx
// 0x1412d5f3f  e81c8bdd00                     callq	0x1420aea60 <.text+0x20ada60>
// 0x1412d5f44  cc                             int3
//
// SOURCE FRAME CATALOG
// F1: crates/systems/roof/src/lib.rs:61:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::ha3f1afa2c7452f82
// F2: crates/systems/roof/src/lib.rs:65:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::ha3f1afa2c7452f82
// F3: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/label.rs:131:0 bevy_ecs::schedule::set::SystemSet::intern
// F4: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs:461:0 bevy_ecs::schedule::config::impl$4::in_set
// F5: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs:463:0 bevy_ecs::schedule::config::impl$4::in_set
// F6: crates/systems/roof/src/lib.rs:78:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::ha3f1afa2c7452f82
// F7: crates/systems/roof/src/lib.rs:95:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::ha3f1afa2c7452f82
// F8: crates/systems/roof/src/lib.rs:106:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::ha3f1afa2c7452f82
// F9: crates/systems/roof/src/lib.rs:107:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::ha3f1afa2c7452f82
// F10: crates/systems/roof/src/lib.rs:108:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::ha3f1afa2c7452f82
// F11: crates/systems/roof/src/lib.rs:109:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::ha3f1afa2c7452f82
// F12: crates/systems/roof/src/lib.rs:110:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::ha3f1afa2c7452f82
// F13: crates/systems/roof/src/lib.rs:64:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::ha3f1afa2c7452f82
// F14: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs:289:0 bevy_app::app::App::add_systems
// F15: crates/systems/roof/src/lib.rs:62:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::ha3f1afa2c7452f82
// F16: crates/systems/roof/src/lib.rs:113:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::ha3f1afa2c7452f82
// F17: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs:464:0 bevy_ecs::schedule::config::impl$4::in_set
// F18: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs:455:0 bevy_ecs::schedule::config::impl$4::in_set
// F19: crates/systems/roof/src/lib.rs:111:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_systems::ha3f1afa2c7452f82
// F20: crates/systems/roof/src/lib.rs:115:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h5f5aa423779871e2
// F21: crates/systems/roof/src/lib.rs:116:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h5f5aa423779871e2
// F22: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs:1355:0 hashbrown::map::HashMap::get_inner
// F23: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs:1313:0 hashbrown::map::HashMap::get
// F24: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs:752:0 bevy_ecs::component::Components::get_resource_id
// F25: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs:1399:0 bevy_ecs::world::World::contains_resource
// F26: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs:308:0 bevy_app::sub_app::SubApp::add_event
// F27: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs:341:0 bevy_app::app::App::add_event
// F28: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs:1359:0 hashbrown::map::HashMap::get_inner
// F29: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/intrinsics.rs:2987:0 core::intrinsics::copy_nonoverlapping
// F30: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1240:0 core::core_arch::x86::sse2::_mm_loadu_si128
// F31: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs:53:0 hashbrown::raw::inner::sse2::Group::load
// F32: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:2185:0 hashbrown::raw::inner::RawTableInner::find_inner
// F33: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:1462:0 hashbrown::raw::inner::RawTable::find
// F34: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:1479:0 hashbrown::raw::inner::RawTable::get
// F35: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1445:0 core::core_arch::x86::sse2::_mm_movemask_epi8
// F36: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs:90:0 hashbrown::raw::inner::sse2::Group::match_byte
// F37: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:2187:0 hashbrown::raw::inner::RawTableInner::find_inner
// F38: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs:50:0 hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
// F39: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs:129:0 hashbrown::raw::inner::bitmask::impl$3::next
// F40: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/num/nonzero.rs:1565:0 core::num::nonzero::NonZero::trailing_zeros
// F41: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs:80:0 hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
// F42: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs:51:0 hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
// F43: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs:38:0 hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
// F44: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs:130:0 hashbrown::raw::inner::bitmask::impl$3::next
// F45: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:2190:0 hashbrown::raw::inner::RawTableInner::find_inner
// F46: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:1141:0 core::ptr::mut_ptr::impl$0::sub
// F47: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:382:0 hashbrown::raw::inner::Bucket::from_base_index
// F48: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:1022:0 hashbrown::raw::inner::RawTable::bucket
// F49: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:1464:0 hashbrown::raw::inner::impl$8::find::closure$0
// F50: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:2192:0 hashbrown::raw::inner::RawTableInner::find_inner
// F51: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mut_ptr.rs:488:0 core::ptr::mut_ptr::impl$0::offset
// F52: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:519:0 hashbrown::raw::inner::Bucket::as_ptr
// F53: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:683:0 hashbrown::raw::inner::RawTable::bucket
// F54: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1488:0 core::cmp::impls::impl$25::eq
// F55: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/tuple.rs:32:0 core::tuple::impl$11::eq
// F56: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/any.rs:617:0 core::any::impl$7::eq
// F57: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:1655:0 core::cmp::impls::impl$9::eq
// F58: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs:172:0 hashbrown::impl$0::equivalent
// F59: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs:229:0 hashbrown::map::equivalent_key::closure$0
// F60: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs:98:0 hashbrown::raw::inner::sse2::Group::match_empty
// F61: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:2197:0 hashbrown::raw::inner::RawTableInner::find_inner
// F62: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:183:0 hashbrown::raw::inner::ProbeSeq::move_next
// F63: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:2201:0 hashbrown::raw::inner::RawTableInner::find_inner
// F64: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs:182:0 hashbrown::raw::inner::ProbeSeq::move_next
// F65: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1870:0 core::option::Option::copied
// F66: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:222:0 core::slice::index::impl$2::get
// F67: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:621:0 core::slice::impl$0::get
// F68: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::SparseArray::get
// F69: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:370:0 bevy_ecs::storage::sparse_set::SparseSet::get
// F70: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs:286:0 bevy_ecs::storage::resource::Resources::get
// F71: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs:1400:0 bevy_ecs::world::impl$2::contains_resource::closure$0
// F72: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1413:0 core::option::Option::and_then
// F73: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs:1400:0 bevy_ecs::world::World::contains_resource
// F74: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1072:0 core::slice::impl$0::get
// F75: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:683:0 core::option::Option::as_ref
// F76: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:58:0 bevy_ecs::storage::sparse_set::impl$10::get::closure$0
// F77: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1073:0 core::slice::impl$0::get
// F78: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:1073:0 core::option::Option::map
// F79: /home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs:398:0 nonmax::NonMaxUsize::get
// F80: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs:372:0 bevy_ecs::storage::sparse_set::impl$15::get::closure$0
// F81: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs:93:0 bevy_ecs::storage::blob_vec::BlobVec::is_empty
// F82: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs:70:0 bevy_ecs::storage::resource::ResourceData::is_present
// F83: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs:1401:0 bevy_ecs::world::impl$2::contains_resource::closure$1
// F84: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs:1401:0 bevy_ecs::world::World::contains_resource
// F85: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs:309:0 bevy_app::sub_app::SubApp::add_event
// F86: crates/systems/roof/src/lib.rs:130:0 _$LT$bevy_app..app..App$u20$as$u20$system_roof..RoofSystems$GT$::add_roof_events_and_resources::h5f5aa423779871e2
// F87: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:515:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hc28e5bc42e18892f (.llvm.15264474953603526013)
// F88: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3343:0 core::sync::atomic::atomic_add
// F89: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:2667:0 core::sync::atomic::AtomicU32::fetch_add
// F90: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs:2065:0 bevy_ecs::world::World::increment_change_tick
// F91: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:279:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::increment_change_tick
// F92: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:519:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hc28e5bc42e18892f (.llvm.15264474953603526013)
// F93: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:705:0 core::option::Option::as_mut
// F94: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:528:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hc28e5bc42e18892f (.llvm.15264474953603526013)
// F95: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:527:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hc28e5bc42e18892f (.llvm.15264474953603526013)
// F96: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:543:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
// F97: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:531:0 bevy_ecs::system::system_param::impl$4::get_param
// F98: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:1474:0 bevy_ecs::system::system_param::impl$83::get_param
// F99: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/const_ptr.rs:964:0 core::ptr::const_ptr::impl$0::add
// F100: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:246:0 core::slice::index::impl$2::get_unchecked
// F101: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/mod.rs:686:0 nonmax::NonMaxUsize::get
// F102: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs:113:0 bevy_ecs::storage::resource::ResourceData::get_with_ticks
// F103: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:544:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
// F104: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/bool.rs:60:0 bevy_ecs::storage::resource::ResourceData::is_present
// F105: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:448:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F106: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:626:0 bevy_ecs::system::system_param::impl$7::get_param
// F107: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:449:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F108: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs:396:0 bevy_ecs::storage::blob_vec::BlobVec::get_ptr
// F109: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs:372:0 bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
// F110: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs:117:0 bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
// F111: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3308:0 core::sync::atomic::atomic_load
// F112: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3115:0 core::sync::atomic::AtomicU32::load
// F113: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs:2075:0 bevy_ecs::world::World::read_change_tick
// F114: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:255:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
// F115: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:455:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
// F116: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:310:0 bevy_ecs::change_detection::impl$29::set_changed
// F117: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs:327:0 bevy_ecs::change_detection::impl$30::deref_mut
// F118: crates/systems/roof/src/lib.rs:137:0 system_roof::gpu_flush_buffers
// F119: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/function.rs:166:0 core::ops::function::FnMut::call_mut
// F120: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/function.rs:294:0 core::ops::function::impls::impl$3::call_mut
// F121: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:710:0 bevy_ecs::system::function_system::impl$14::run::call_inner
// F122: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:713:0 bevy_ecs::system::function_system::impl$14::run
// F123: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:534:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hc28e5bc42e18892f (.llvm.15264474953603526013)
// F124: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:931:0 core::option::Option::unwrap
// F125: crates/systems/roof/src/lib.rs:138:0 system_roof::gpu_flush_buffers
// F126: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:535:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hc28e5bc42e18892f (.llvm.15264474953603526013)
// F127: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:537:0 _$LT$bevy_ecs..system..function_system..FunctionSystem$LT$Marker$C$F$GT$$u20$as$u20$bevy_ecs..system..system..System$GT$::run_unsafe::hc28e5bc42e18892f (.llvm.15264474953603526013)
// F128: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:534:0 bevy_ecs::system::system_param::impl$4::get_param::closure$0
// F129: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:978:0 core::option::Option::unwrap_or_else
// F130: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:533:0 bevy_ecs::system::system_param::impl$4::get_param
// F131: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/any.rs:700:0 core::any::type_name
// F132: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:537:0 bevy_ecs::system::system_param::impl$4::get_param::closure$0
// F133: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/fmt/mod.rs:343:0 core::fmt::Arguments::new_v1
// F134: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/panic.rs:106:0 bevy_ecs::system::system_param::impl$4::get_param::closure$0
// F135: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:629:0 bevy_ecs::system::system_param::impl$7::get_param::closure$0
// F136: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:628:0 bevy_ecs::system::system_param::impl$7::get_param
// F137: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs:632:0 bevy_ecs::system::system_param::impl$7::get_param::closure$0
// F138: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/panic.rs:106:0 bevy_ecs::system::system_param::impl$7::get_param::closure$0
// F139: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:896:0 core::option::Option::as_mut
// F140: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:933:0 core::option::Option::unwrap
// F141: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system.rs:77:0 bevy_ecs::system::system::System::run::h9bdfe70fb9a2ed7b
// F142: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs:184:0 bevy_ecs::world::World::id
// F143: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs:201:0 bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::id
// F144: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:567:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F145: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system.rs:79:0 bevy_ecs::system::system::System::run::h9bdfe70fb9a2ed7b
// F146: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:2175:0 core::option::impl$15::eq
// F147: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:2155:0 core::option::impl$14::eq
// F148: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/macros/mod.rs:53:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F149: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2203:0 alloc::vec::Vec::len
// F150: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs:734:0 bevy_ecs::archetype::Archetypes::generation
// F151: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:570:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F152: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1284:0 core::ptr::read
// F153: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/mem/mod.rs:921:0 bevy_ecs::archetype::Archetypes::generation
// F154: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1493:0 core::ptr::write
// F155: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/mem/mod.rs:922:0 bevy_ecs::archetype::Archetypes::generation
// F156: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:502:0 core::slice::index::impl$6::index
// F157: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:18:0 core::slice::index::impl$0::index
// F158: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:2829:0 alloc::vec::impl$12::index
// F159: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs:893:0 bevy_ecs::archetype::impl$10::index
// F160: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:572:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F161: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:573:0 bevy_ecs::system::function_system::impl$7::update_archetype_component_access
// F162: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:519:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F163: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system.rs:83:0 bevy_ecs::system::system::System::run::h9bdfe70fb9a2ed7b
// F164: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:528:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F165: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:527:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F166: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:534:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F167: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:535:0 bevy_ecs::system::function_system::impl$7::run_unsafe
// F168: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/function_system.rs:541:0 bevy_ecs::system::function_system::impl$7::apply_deferred
// F169: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system.rs:84:0 bevy_ecs::system::system::System::run::h9bdfe70fb9a2ed7b
// F170: /home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system.rs:86:0 bevy_ecs::system::system::System::run::h9bdfe70fb9a2ed7b
// F171: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/slice/index.rs:503:0 core::slice::index::impl$6::index
// F172: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:933:0 core::option::Option::as_mut
// === ORIGINAL ASSEMBLY V1 END ===
// Version: A
// Original source: crates/systems/roof/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 36bb7ca5d8d28d814e9822b0c37e7e3e4b4c683788a24c584a78f11c1eb8899a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 267, 766, 1927, 1946
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_app; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_roof::impl$0::add_roof_systems
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":267,"record_offset":528,"rva":4131200,"va":5372840320}
// 编译实例 RVAs: 0x3f0980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400379d0
//     owner=unresolved_namespace; count=9
//     target_aliases=84 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3904686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372840320,"raw_hex":"e8526bc4ff","size":5,"soft_fail":false,"target_va":5368936912,"va":5372841593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140037d90
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3902542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372840320,"raw_hex":"e8b273c4ff","size":5,"soft_fail":false,"target_va":5368937872,"va":5372840409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ea4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5216583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372840320,"raw_hex":"e847994f00","size":5,"soft_fail":false,"target_va":5378057376,"va":5372840788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5217458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372840320,"raw_hex":"e8b29c4f00","size":5,"soft_fail":false,"target_va":5378059008,"va":5372841545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5192292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372840320,"raw_hex":"e8643a4f00","size":5,"soft_fail":false,"target_va":5378032688,"va":5372840391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5188757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372840320,"raw_hex":"e8952c4f00","size":5,"soft_fail":false,"target_va":5378029120,"va":5372840358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::in_set
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t5203850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372840320,"raw_hex":"e88a674f00","size":5,"soft_fail":false,"target_va":5378044544,"va":5372840689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":267,"record_offset":796,"parent_record_offset":528,"ancestor_record_offset":528,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$4::in_set
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":267,"record_offset":656,"parent_record_offset":528,"ancestor_record_offset":528,"inlinee":6385,"source_line":455,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/label.rs
//     evidence={"module":267,"record_offset":696,"parent_record_offset":656,"ancestor_record_offset":528,"inlinee":6375,"source_line":129,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::impl$0::add_roof_events_and_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":267,"record_offset":828,"rva":4132880,"va":5372842000}
// 编译实例 RVAs: 0x3f1010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5218150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372842000,"raw_hex":"e8669f4f00","size":5,"soft_fail":false,"target_va":5378060176,"va":5372842021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventRegistry::register_event
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t5079047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372842000,"raw_hex":"e807804d00","size":5,"soft_fail":false,"target_va":5377921312,"va":5372842260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":267,"record_offset":964,"parent_record_offset":828,"ancestor_record_offset":828,"inlinee":10733,"source_line":337,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":267,"record_offset":988,"parent_record_offset":964,"ancestor_record_offset":828,"inlinee":10735,"source_line":304,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":267,"record_offset":1048,"parent_record_offset":1016,"ancestor_record_offset":828,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":267,"record_offset":2460,"parent_record_offset":2436,"ancestor_record_offset":828,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":267,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":828,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":267,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":828,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":267,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":828,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":267,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":828,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":267,"record_offset":2224,"parent_record_offset":2160,"ancestor_record_offset":828,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":267,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":828,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":267,"record_offset":1016,"parent_record_offset":988,"ancestor_record_offset":828,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":267,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":828,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":267,"record_offset":2412,"parent_record_offset":2388,"ancestor_record_offset":828,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":267,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":828,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":267,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":828,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":267,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":828,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":267,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":828,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":267,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":828,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":267,"record_offset":1260,"parent_record_offset":1236,"ancestor_record_offset":828,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":267,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":828,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":267,"record_offset":2036,"parent_record_offset":1016,"ancestor_record_offset":828,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":267,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":828,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":267,"record_offset":2004,"parent_record_offset":1048,"ancestor_record_offset":828,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":267,"record_offset":2288,"parent_record_offset":2108,"ancestor_record_offset":828,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":267,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":828,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":267,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":828,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":267,"record_offset":2160,"parent_record_offset":2132,"ancestor_record_offset":828,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":267,"record_offset":2196,"parent_record_offset":2160,"ancestor_record_offset":828,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":267,"record_offset":1792,"parent_record_offset":1768,"ancestor_record_offset":828,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":267,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":828,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":267,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":828,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":267,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":828,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":267,"record_offset":1696,"parent_record_offset":1500,"ancestor_record_offset":828,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":267,"record_offset":1608,"parent_record_offset":1524,"ancestor_record_offset":828,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":267,"record_offset":1552,"parent_record_offset":1524,"ancestor_record_offset":828,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":267,"record_offset":1952,"parent_record_offset":1172,"ancestor_record_offset":828,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":267,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":828,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":267,"record_offset":1148,"parent_record_offset":1124,"ancestor_record_offset":828,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":267,"record_offset":1124,"parent_record_offset":1096,"ancestor_record_offset":828,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":267,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":828,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":267,"record_offset":1380,"parent_record_offset":1352,"ancestor_record_offset":828,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":267,"record_offset":1408,"parent_record_offset":1380,"ancestor_record_offset":828,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":267,"record_offset":1468,"parent_record_offset":1352,"ancestor_record_offset":828,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":267,"record_offset":1352,"parent_record_offset":1172,"ancestor_record_offset":828,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":267,"record_offset":1500,"parent_record_offset":1172,"ancestor_record_offset":828,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":267,"record_offset":1212,"parent_record_offset":1172,"ancestor_record_offset":828,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":267,"record_offset":1296,"parent_record_offset":1172,"ancestor_record_offset":828,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":267,"record_offset":1868,"parent_record_offset":1172,"ancestor_record_offset":828,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":267,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":828,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00267.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_roof::gpu_flush_buffers
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB3B3
// 首条位置: {"module":766,"inlinee":46003}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":766,"record_offset":3312,"parent_record_offset":3276,"ancestor_record_offset":3276,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00766.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":766,"record_offset":3368,"parent_record_offset":3276,"ancestor_record_offset":3276,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":766,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":3276,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00766.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 46
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>", "constructor": "bevy_ecs::query::filter::With", "arguments": ["system_roof::utils::components::RoofSpire"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::iter::QueryIter", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "system_roof::visual::place_spires::place_roof_spires::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::roof_tile_ssbo::RoofSpireSsbo", "core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,system_roof::visual::place_spires::place_roof_spires::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::iter::impl$1::size_hint<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::iter::impl$1::size_hint", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >", "constructor": "tuple$", "arguments": ["country_core::resources::roofs::roof_shape::Roof", "country_core::resources::roofs::roof_tile_sim::RoofTileSimulation", "country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume", "country_core::resources::roofs::roof_animation::RoofAnimation", "country_core::resources::roofs::roof_tiles::RoofTiles", "system_roof::utils::components::RoofSpire", "system_roof::utils::components::RoofSupportBeams", "country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::system::commands::insert::closure_env$0", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::system::commands::WithEntity", "arguments": ["tuple$<>", "bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::state::QueryState", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::system::query::Query", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::new_with_access<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::state::QueryState::new_with_access", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::new_archetype_internal<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >", "constructor": "bevy_ecs::query::state::QueryState::new_archetype_internal", "arguments": ["tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >", "bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)", "void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::SystemConfigTupleMarker", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::in_set<tuple$<void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>,tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::in_set", "arguments": ["tuple$<void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::entity_ref::EntityWorldMut::insert<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::world::entity_ref::EntityWorldMut::insert", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::init_info<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::bundle::Bundles::init_info", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$24::component_ids<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>,bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impl$24::component_ids", "arguments": ["country_core::resources::roofs::roof_shape::Roof", "country_core::resources::roofs::roof_tile_sim::RoofTileSimulation", "country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume", "country_core::resources::roofs::roof_animation::RoofAnimation", "country_core::resources::roofs::roof_tiles::RoofTiles", "system_roof::utils::components::RoofSpire", "system_roof::utils::components::RoofSupportBeams", "country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>", "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::filter::impl$0::init_state<system_roof::utils::components::RoofSpire>", "constructor": "bevy_ecs::query::filter::impl$0::init_state", "arguments": ["system_roof::utils::components::RoofSpire"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<system_roof::utils::components::RoofSupportBeams,bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["system_roof::utils::components::RoofSupportBeams", "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<system_roof::utils::components::RoofSpire,bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["system_roof::utils::components::RoofSpire", "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_roof::utils::components::RoofSupportBeams
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >", "constructor": "tuple$", "arguments": ["country_core::resources::roofs::roof_shape::Roof", "country_core::resources::roofs::roof_tile_sim::RoofTileSimulation", "country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume", "country_core::resources::roofs::roof_animation::RoofAnimation", "country_core::resources::roofs::roof_tiles::RoofTiles", "system_roof::utils::components::RoofSpire", "system_roof::utils::components::RoofSupportBeams", "country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::system::commands::insert::closure_env$0", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::system::commands::WithEntity", "arguments": ["tuple$<>", "bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::entity_ref::EntityWorldMut::insert<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::world::entity_ref::EntityWorldMut::insert", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::init_info<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::bundle::Bundles::init_info", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "constructor": "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$24::component_ids<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>,bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impl$24::component_ids", "arguments": ["country_core::resources::roofs::roof_shape::Roof", "country_core::resources::roofs::roof_tile_sim::RoofTileSimulation", "country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume", "country_core::resources::roofs::roof_animation::RoofAnimation", "country_core::resources::roofs::roof_tiles::RoofTiles", "system_roof::utils::components::RoofSpire", "system_roof::utils::components::RoofSupportBeams", "country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>", "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<system_roof::utils::components::RoofSupportBeams,bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["system_roof::utils::components::RoofSupportBeams", "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<system_roof::utils::components::RoofSpire,bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > > >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["system_roof::utils::components::RoofSpire", "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 23; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof_ridge_length::RidgeGrabParams>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::roofs::roof_animation::RoofAnimation>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >,tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "tuple$<bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved> >", "tuple$<bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>,bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_roof::ui_edit_roof::RoofParamsCacheResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemRoofShape>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_roof::ui_edit_roof::RoofShapeEditAmount>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_roof::gpu_flush_buffers
// 信息 ID: 8692dfad823a98920d31dcb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5d58aab8b83fbd13d0042c71: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>; module=766, inline_record=3276; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >; module=1927, inline_record=249384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::impl$0::add_roof_events_and_resources
// 信息 ID: 8c85f0efd4a1e52debd96992
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3d005d -> 0x3f1010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_roof::impl$0::add_roof_systems
// 信息 ID: 416ec4d4fb1e606eda8db18a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3cf90e -> 0x3f0980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
