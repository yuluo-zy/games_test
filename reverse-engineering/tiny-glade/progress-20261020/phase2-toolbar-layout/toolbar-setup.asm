
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142345890: 55                          	pushq	%rbp
142345891: 41 57                       	pushq	%r15
142345893: 41 56                       	pushq	%r14
142345895: 41 55                       	pushq	%r13
142345897: 41 54                       	pushq	%r12
142345899: 56                          	pushq	%rsi
14234589a: 57                          	pushq	%rdi
14234589b: 53                          	pushq	%rbx
14234589c: 48 81 ec 58 03 00 00        	subq	$0x358, %rsp            # imm = 0x358
1423458a3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1423458ab: 0f 29 bd c0 02 00 00        	movaps	%xmm7, 0x2c0(%rbp)
1423458b2: 0f 29 b5 b0 02 00 00        	movaps	%xmm6, 0x2b0(%rbp)
1423458b9: 48 c7 85 a8 02 00 00 fe ff ff ff    	movq	$-0x2, 0x2a8(%rbp)
1423458c4: 4d 89 c4                    	movq	%r8, %r12
1423458c7: 48 89 d7                    	movq	%rdx, %rdi
1423458ca: 48 89 cb                    	movq	%rcx, %rbx
1423458cd: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1423458d4: e8 37 1b ca fe              	callq	0x140fe7410 <.text+0xfe6410>
1423458d9: 48 c7 85 28 02 00 00 00 00 00 00    	movq	$0x0, 0x228(%rbp)
1423458e4: 48 c7 85 30 02 00 00 08 00 00 00    	movq	$0x8, 0x230(%rbp)
1423458ef: 48 c7 85 38 02 00 00 00 00 00 00    	movq	$0x0, 0x238(%rbp)
1423458fa: 44 8b 73 1c                 	movl	0x1c(%rbx), %r14d
1423458fe: 4c 8b 7b 10                 	movq	0x10(%rbx), %r15
142345902: 45 89 37                    	movl	%r14d, (%r15)
142345905: 48 8b 0b                    	movq	(%rbx), %rcx
142345908: 48 89 8d 40 02 00 00        	movq	%rcx, 0x240(%rbp)
14234590f: e8 6c ea c5 fe              	callq	0x140fa4380 <.text+0xfa3380>
142345914: 4c 8b ad 40 02 00 00        	movq	0x240(%rbp), %r13
14234591b: 49 8b 85 98 00 00 00        	movq	0x98(%r13), %rax
142345922: 48 85 c0                    	testq	%rax, %rax
142345925: 74 1a                       	je	0x142345941 <.text+0x2344941>
142345927: 49 8b 8d a0 00 00 00        	movq	0xa0(%r13), %rcx
14234592e: 48 c1 e0 03                 	shlq	$0x3, %rax
142345932: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
142345936: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14234593c: e8 df e2 2c fe              	callq	0x140613c20 <.text+0x612c20>
142345941: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
142345948: 41 b8 b0 00 00 00           	movl	$0xb0, %r8d
14234594e: 4c 89 e9                    	movq	%r13, %rcx
142345951: e8 aa e2 5d 00              	callq	0x142923c00 <.text+0x2922c00>
142345956: 45 89 37                    	movl	%r14d, (%r15)
142345959: 48 8b 37                    	movq	(%rdi), %rsi
14234595c: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
142345960: 4c 89 ea                    	movq	%r13, %rdx
142345963: e8 48 73 b0 ff              	callq	0x141e4ccb0 <.text+0x1e4bcb0>
142345968: 0f b6 05 9a f2 2e 01        	movzbl	0x12ef29a(%rip), %eax   # 0x143634c09
14234596f: b9 10 02 00 00              	movl	$0x210, %ecx            # imm = 0x210
142345974: ba 08 00 00 00              	movl	$0x8, %edx
142345979: e8 92 e2 2c fe              	callq	0x140613c10 <.text+0x612c10>
14234597e: 48 85 c0                    	testq	%rax, %rax
142345981: 0f 84 ca 17 00 00           	je	0x142347151 <.text+0x2346151>
142345987: 4c 89 a5 98 02 00 00        	movq	%r12, 0x298(%rbp)
14234598e: 4c 89 bd 38 01 00 00        	movq	%r15, 0x138(%rbp)
142345995: 44 89 b5 8c 01 00 00        	movl	%r14d, 0x18c(%rbp)
14234599c: 48 89 85 48 02 00 00        	movq	%rax, 0x248(%rbp)
1423459a3: 0f b6 05 5f f2 2e 01        	movzbl	0x12ef25f(%rip), %eax   # 0x143634c09
1423459aa: b9 c0 00 00 00              	movl	$0xc0, %ecx
1423459af: ba 08 00 00 00              	movl	$0x8, %edx
1423459b4: e8 57 e2 2c fe              	callq	0x140613c10 <.text+0x612c10>
1423459b9: 48 85 c0                    	testq	%rax, %rax
1423459bc: 0f 84 a3 17 00 00           	je	0x142347165 <.text+0x2346165>
1423459c2: 48 8b 1d cf 66 2e 01        	movq	0x12e66cf(%rip), %rbx   # 0x14362c098
1423459c9: 80 3b 00                    	cmpb	$0x0, (%rbx)
1423459cc: 41 be 07 00 00 00           	movl	$0x7, %r14d
1423459d2: 41 bf 01 00 00 00           	movl	$0x1, %r15d
1423459d8: b9 01 00 00 00              	movl	$0x1, %ecx
1423459dd: 41 0f 45 ce                 	cmovnel	%r14d, %ecx
1423459e1: 48 8d 15 cd 8d d0 00        	leaq	0xd08dcd(%rip), %rdx    # 0x14304e7b5
1423459e8: 4c 8d 05 b8 8d d0 00        	leaq	0xd08db8(%rip), %r8     # 0x14304e7a7
1423459ef: 4c 0f 45 c2                 	cmovneq	%rdx, %r8
1423459f3: 41 bc 23 00 00 00           	movl	$0x23, %r12d
1423459f9: ba 0e 00 00 00              	movl	$0xe, %edx
1423459fe: 49 0f 45 d4                 	cmovneq	%r12, %rdx
142345a02: c6 00 01                    	movb	$0x1, (%rax)
142345a05: c7 40 08 06 00 00 00        	movl	$0x6, 0x8(%rax)
142345a0c: c6 40 0c 01                 	movb	$0x1, 0xc(%rax)
142345a10: 4c 8d 0d 71 8d d0 00        	leaq	0xd08d71(%rip), %r9     # 0x14304e788
142345a17: 4c 89 48 20                 	movq	%r9, 0x20(%rax)
142345a1b: 48 c7 40 28 0d 00 00 00     	movq	$0xd, 0x28(%rax)
142345a23: f2 0f 10 35 85 8c 5e 00     	movsd	0x5e8c85(%rip), %xmm6   # 0x14292e6b0
142345a2b: f2 0f 11 70 30              	movsd	%xmm6, 0x30(%rax)
142345a30: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
142345a34: c6 40 40 01                 	movb	$0x1, 0x40(%rax)
142345a38: c7 40 48 06 00 00 00        	movl	$0x6, 0x48(%rax)
142345a3f: c6 40 4c 02                 	movb	$0x2, 0x4c(%rax)
142345a43: 4c 8d 0d 4b 8d d0 00        	leaq	0xd08d4b(%rip), %r9     # 0x14304e795
142345a4a: 4c 89 48 60                 	movq	%r9, 0x60(%rax)
142345a4e: 48 c7 40 68 12 00 00 00     	movq	$0x12, 0x68(%rax)
142345a56: f2 0f 11 70 70              	movsd	%xmm6, 0x70(%rax)
142345a5b: c6 40 78 00                 	movb	$0x0, 0x78(%rax)
142345a5f: 88 88 80 00 00 00           	movb	%cl, 0x80(%rax)
142345a65: c7 80 88 00 00 00 0b 00 00 00       	movl	$0xb, 0x88(%rax)
142345a6f: c6 80 8c 00 00 00 3f        	movb	$0x3f, 0x8c(%rax)
142345a76: 4c 89 80 a0 00 00 00        	movq	%r8, 0xa0(%rax)
142345a7d: 48 89 90 a8 00 00 00        	movq	%rdx, 0xa8(%rax)
142345a84: f2 0f 11 b0 b0 00 00 00     	movsd	%xmm6, 0xb0(%rax)
142345a8c: c6 80 b8 00 00 00 00        	movb	$0x0, 0xb8(%rax)
142345a93: 48 c7 85 70 02 00 00 01 00 00 00    	movq	$0x1, 0x270(%rbp)
142345a9e: 48 c7 85 78 02 00 00 00 00 00 00    	movq	$0x0, 0x278(%rbp)
142345aa9: f2 0f 11 b5 80 02 00 00     	movsd	%xmm6, 0x280(%rbp)
142345ab1: c6 85 50 02 00 00 04        	movb	$0x4, 0x250(%rbp)
142345ab8: 48 c7 85 58 02 00 00 03 00 00 00    	movq	$0x3, 0x258(%rbp)
142345ac3: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
142345aca: 48 c7 85 68 02 00 00 03 00 00 00    	movq	$0x3, 0x268(%rbp)
142345ad5: c6 85 88 02 00 00 00        	movb	$0x0, 0x288(%rbp)
142345adc: 0f b6 05 26 f1 2e 01        	movzbl	0x12ef126(%rip), %eax   # 0x143634c09
142345ae3: b9 80 00 00 00              	movl	$0x80, %ecx
142345ae8: ba 08 00 00 00              	movl	$0x8, %edx
142345aed: e8 1e e1 2c fe              	callq	0x140613c10 <.text+0x612c10>
142345af2: 48 85 c0                    	testq	%rax, %rax
142345af5: 0f 84 7e 16 00 00           	je	0x142347179 <.text+0x2346179>
142345afb: 80 3b 00                    	cmpb	$0x0, (%rbx)
142345afe: 45 0f 45 fe                 	cmovnel	%r14d, %r15d
142345b02: 48 8d 0d e8 8c d0 00        	leaq	0xd08ce8(%rip), %rcx    # 0x14304e7f1
142345b09: 48 8d 15 d4 8c d0 00        	leaq	0xd08cd4(%rip), %rdx    # 0x14304e7e4
142345b10: 48 0f 45 d1                 	cmovneq	%rcx, %rdx
142345b14: b9 0d 00 00 00              	movl	$0xd, %ecx
142345b19: 49 0f 45 cc                 	cmovneq	%r12, %rcx
142345b1d: c6 00 01                    	movb	$0x1, (%rax)
142345b20: c7 40 08 06 00 00 00        	movl	$0x6, 0x8(%rax)
142345b27: c6 40 0c 00                 	movb	$0x0, 0xc(%rax)
142345b2b: 4c 8d 35 a6 8c d0 00        	leaq	0xd08ca6(%rip), %r14    # 0x14304e7d8
142345b32: 4c 89 70 20                 	movq	%r14, 0x20(%rax)
142345b36: 48 c7 40 28 0c 00 00 00     	movq	$0xc, 0x28(%rax)
142345b3e: f2 0f 11 70 30              	movsd	%xmm6, 0x30(%rax)
142345b43: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
142345b47: 44 88 78 40                 	movb	%r15b, 0x40(%rax)
142345b4b: c7 40 48 12 00 00 00        	movl	$0x12, 0x48(%rax)
142345b52: 48 89 50 60                 	movq	%rdx, 0x60(%rax)
142345b56: 48 89 48 68                 	movq	%rcx, 0x68(%rax)
142345b5a: f2 0f 11 70 70              	movsd	%xmm6, 0x70(%rax)
142345b5f: c6 40 78 00                 	movb	$0x0, 0x78(%rax)
142345b63: 48 c7 85 b0 01 00 00 01 00 00 00    	movq	$0x1, 0x1b0(%rbp)
142345b6e: 48 c7 85 b8 01 00 00 00 00 00 00    	movq	$0x0, 0x1b8(%rbp)
142345b79: f2 0f 11 b5 c0 01 00 00     	movsd	%xmm6, 0x1c0(%rbp)
142345b81: c6 85 90 01 00 00 04        	movb	$0x4, 0x190(%rbp)
142345b88: 48 c7 85 98 01 00 00 02 00 00 00    	movq	$0x2, 0x198(%rbp)
142345b93: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
142345b9a: 48 c7 85 a8 01 00 00 02 00 00 00    	movq	$0x2, 0x1a8(%rbp)
142345ba5: c6 85 c8 01 00 00 00        	movb	$0x0, 0x1c8(%rbp)
142345bac: 0f b6 05 56 f0 2e 01        	movzbl	0x12ef056(%rip), %eax   # 0x143634c09
142345bb3: b9 40 00 00 00              	movl	$0x40, %ecx
142345bb8: ba 08 00 00 00              	movl	$0x8, %edx
142345bbd: e8 4e e0 2c fe              	callq	0x140613c10 <.text+0x612c10>
142345bc2: 48 85 c0                    	testq	%rax, %rax
142345bc5: 0f 84 c2 15 00 00           	je	0x14234718d <.text+0x234618d>
142345bcb: 48 89 c7                    	movq	%rax, %rdi
142345bce: 0f b6 05 34 f0 2e 01        	movzbl	0x12ef034(%rip), %eax   # 0x143634c09
142345bd5: b9 80 00 00 00              	movl	$0x80, %ecx
142345bda: ba 08 00 00 00              	movl	$0x8, %edx
142345bdf: e8 2c e0 2c fe              	callq	0x140613c10 <.text+0x612c10>
142345be4: 48 85 c0                    	testq	%rax, %rax
142345be7: 4c 8b a5 48 02 00 00        	movq	0x248(%rbp), %r12
142345bee: 0f 84 ad 15 00 00           	je	0x1423471a1 <.text+0x23461a1>
142345bf4: 0f 10 85 90 01 00 00        	movups	0x190(%rbp), %xmm0
142345bfb: 0f 10 8d a0 01 00 00        	movups	0x1a0(%rbp), %xmm1
142345c02: 0f 10 95 b0 01 00 00        	movups	0x1b0(%rbp), %xmm2
142345c09: 0f 10 9d c0 01 00 00        	movups	0x1c0(%rbp), %xmm3
142345c10: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
142345c14: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
142345c18: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
142345c1c: 0f 11 00                    	movups	%xmm0, (%rax)
142345c1f: 0f 10 85 50 02 00 00        	movups	0x250(%rbp), %xmm0
142345c26: 0f 10 8d 60 02 00 00        	movups	0x260(%rbp), %xmm1
142345c2d: 0f 10 95 70 02 00 00        	movups	0x270(%rbp), %xmm2
142345c34: 0f 10 9d 80 02 00 00        	movups	0x280(%rbp), %xmm3
142345c3b: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
142345c3f: 0f 11 48 50                 	movups	%xmm1, 0x50(%rax)
142345c43: 0f 11 50 60                 	movups	%xmm2, 0x60(%rax)
142345c47: 0f 11 58 70                 	movups	%xmm3, 0x70(%rax)
142345c4b: c6 07 05                    	movb	$0x5, (%rdi)
142345c4e: 48 c7 47 08 02 00 00 00     	movq	$0x2, 0x8(%rdi)
142345c56: 48 89 47 10                 	movq	%rax, 0x10(%rdi)
142345c5a: 48 c7 47 18 02 00 00 00     	movq	$0x2, 0x18(%rdi)
142345c62: 48 c7 47 20 01 00 00 00     	movq	$0x1, 0x20(%rdi)
142345c6a: 48 c7 47 28 00 00 00 00     	movq	$0x0, 0x28(%rdi)
142345c72: f2 0f 11 77 30              	movsd	%xmm6, 0x30(%rdi)
142345c77: c6 47 38 00                 	movb	$0x0, 0x38(%rdi)
142345c7b: 48 8d 05 92 8b d0 00        	leaq	0xd08b92(%rip), %rax    # 0x14304e814
142345c82: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
142345c89: 48 c7 85 e0 00 00 00 10 00 00 00    	movq	$0x10, 0xe0(%rbp)
142345c94: c6 85 e8 00 00 00 00        	movb	$0x0, 0xe8(%rbp)
142345c9b: 48 c7 85 f0 00 00 00 01 00 00 00    	movq	$0x1, 0xf0(%rbp)
142345ca6: 48 89 bd f8 00 00 00        	movq	%rdi, 0xf8(%rbp)
142345cad: 48 c7 85 00 01 00 00 01 00 00 00    	movq	$0x1, 0x100(%rbp)
142345cb8: 0f b6 05 4a ef 2e 01        	movzbl	0x12eef4a(%rip), %eax   # 0x143634c09
142345cbf: b9 80 00 00 00              	movl	$0x80, %ecx
142345cc4: ba 08 00 00 00              	movl	$0x8, %edx
142345cc9: e8 42 df 2c fe              	callq	0x140613c10 <.text+0x612c10>
142345cce: 48 85 c0                    	testq	%rax, %rax
142345cd1: 0f 84 e5 14 00 00           	je	0x1423471bc <.text+0x23461bc>
142345cd7: 66 c7 00 02 0a              	movw	$0xa02, (%rax)          # imm = 0xA02
142345cdc: 48 8d 0d 41 8b d0 00        	leaq	0xd08b41(%rip), %rcx    # 0x14304e824
142345ce3: 48 89 48 20                 	movq	%rcx, 0x20(%rax)
142345ce7: 48 c7 40 28 16 00 00 00     	movq	$0x16, 0x28(%rax)
142345cef: f2 0f 10 3d e9 98 c4 00     	movsd	0xc498e9(%rip), %xmm7   # 0x142f8f5e0
142345cf7: f2 0f 11 78 30              	movsd	%xmm7, 0x30(%rax)
142345cfc: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
142345d00: 66 c7 40 40 03 0a           	movw	$0xa03, 0x40(%rax)      # imm = 0xA03
142345d06: 48 8d 0d 2d 8b d0 00        	leaq	0xd08b2d(%rip), %rcx    # 0x14304e83a
142345d0d: 48 89 48 60                 	movq	%rcx, 0x60(%rax)
142345d11: 48 c7 40 68 12 00 00 00     	movq	$0x12, 0x68(%rax)
142345d19: f2 0f 11 70 70              	movsd	%xmm6, 0x70(%rax)
142345d1e: c6 40 78 00                 	movb	$0x0, 0x78(%rax)
142345d22: 48 c7 85 70 02 00 00 01 00 00 00    	movq	$0x1, 0x270(%rbp)
142345d2d: 48 c7 85 78 02 00 00 00 00 00 00    	movq	$0x0, 0x278(%rbp)
142345d38: f2 0f 11 b5 80 02 00 00     	movsd	%xmm6, 0x280(%rbp)
142345d40: c6 85 50 02 00 00 04        	movb	$0x4, 0x250(%rbp)
142345d47: 48 c7 85 58 02 00 00 02 00 00 00    	movq	$0x2, 0x258(%rbp)
142345d52: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
142345d59: 48 c7 85 68 02 00 00 02 00 00 00    	movq	$0x2, 0x268(%rbp)
142345d64: c6 85 88 02 00 00 00        	movb	$0x0, 0x288(%rbp)
142345d6b: 80 3b 00                    	cmpb	$0x0, (%rbx)
142345d6e: 0f 84 24 01 00 00           	je	0x142345e98 <.text+0x2344e98>
142345d74: 0f b6 05 8e ee 2e 01        	movzbl	0x12eee8e(%rip), %eax   # 0x143634c09
142345d7b: b9 c0 00 00 00              	movl	$0xc0, %ecx
142345d80: ba 08 00 00 00              	movl	$0x8, %edx
142345d85: e8 86 de 2c fe              	callq	0x140613c10 <.text+0x612c10>
142345d8a: 48 85 c0                    	testq	%rax, %rax
142345d8d: 0f 84 9b 14 00 00           	je	0x14234722e <.text+0x234622e>
142345d93: c6 00 07                    	movb	$0x7, (%rax)
142345d96: 48 8d 0d 02 8b d0 00        	leaq	0xd08b02(%rip), %rcx    # 0x14304e89f
142345d9d: 48 89 48 20                 	movq	%rcx, 0x20(%rax)
142345da1: 48 c7 40 28 2a 00 00 00     	movq	$0x2a, 0x28(%rax)
142345da9: f2 0f 11 70 30              	movsd	%xmm6, 0x30(%rax)
142345dae: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
142345db2: c6 40 40 07                 	movb	$0x7, 0x40(%rax)
142345db6: 48 89 48 60                 	movq	%rcx, 0x60(%rax)
142345dba: 48 c7 40 68 2a 00 00 00     	movq	$0x2a, 0x68(%rax)
142345dc2: f2 0f 11 70 70              	movsd	%xmm6, 0x70(%rax)
142345dc7: c6 40 78 00                 	movb	$0x0, 0x78(%rax)
142345dcb: c6 80 80 00 00 00 07        	movb	$0x7, 0x80(%rax)
142345dd2: 48 89 88 a0 00 00 00        	movq	%rcx, 0xa0(%rax)
142345dd9: 48 c7 80 a8 00 00 00 2a 00 00 00    	movq	$0x2a, 0xa8(%rax)
142345de4: f2 0f 11 b0 b0 00 00 00     	movsd	%xmm6, 0xb0(%rax)
142345dec: c6 80 b8 00 00 00 00        	movb	$0x0, 0xb8(%rax)
142345df3: 48 c7 85 b0 01 00 00 01 00 00 00    	movq	$0x1, 0x1b0(%rbp)
142345dfe: 48 c7 85 b8 01 00 00 00 00 00 00    	movq	$0x0, 0x1b8(%rbp)
142345e09: f2 0f 11 b5 c0 01 00 00     	movsd	%xmm6, 0x1c0(%rbp)
142345e11: c6 85 90 01 00 00 04        	movb	$0x4, 0x190(%rbp)
142345e18: 48 c7 85 98 01 00 00 03 00 00 00    	movq	$0x3, 0x198(%rbp)
142345e23: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
142345e2a: 48 c7 85 a8 01 00 00 03 00 00 00    	movq	$0x3, 0x1a8(%rbp)
142345e35: c6 85 c8 01 00 00 00        	movb	$0x0, 0x1c8(%rbp)
142345e3c: 0f b6 05 c6 ed 2e 01        	movzbl	0x12eedc6(%rip), %eax   # 0x143634c09
142345e43: b9 40 00 00 00              	movl	$0x40, %ecx
142345e48: ba 08 00 00 00              	movl	$0x8, %edx
142345e4d: e8 be dd 2c fe              	callq	0x140613c10 <.text+0x612c10>
142345e52: 48 85 c0                    	testq	%rax, %rax
142345e55: 0f 84 fb 13 00 00           	je	0x142347256 <.text+0x2346256>
142345e5b: 48 89 c7                    	movq	%rax, %rdi
142345e5e: 0f b6 05 a4 ed 2e 01        	movzbl	0x12eeda4(%rip), %eax   # 0x143634c09
142345e65: b9 80 00 00 00              	movl	$0x80, %ecx
142345e6a: ba 08 00 00 00              	movl	$0x8, %edx
142345e6f: e8 9c dd 2c fe              	callq	0x140613c10 <.text+0x612c10>
142345e74: 48 85 c0                    	testq	%rax, %rax
142345e77: 0f 85 a1 01 00 00           	jne	0x14234601e <.text+0x234501e>
142345e7d: 48 89 bd 98 02 00 00        	movq	%rdi, 0x298(%rbp)
142345e84: b9 08 00 00 00              	movl	$0x8, %ecx
142345e89: ba 80 00 00 00              	movl	$0x80, %edx
142345e8e: e8 6d 31 59 00              	callq	0x1428d9000 <.text+0x28d8000>
142345e93: e9 e8 14 00 00              	jmp	0x142347380 <.text+0x2346380>
142345e98: 0f b6 05 6a ed 2e 01        	movzbl	0x12eed6a(%rip), %eax   # 0x143634c09
142345e9f: b9 00 01 00 00              	movl	$0x100, %ecx            # imm = 0x100
142345ea4: ba 08 00 00 00              	movl	$0x8, %edx
142345ea9: e8 62 dd 2c fe              	callq	0x140613c10 <.text+0x612c10>
142345eae: 48 85 c0                    	testq	%rax, %rax
142345eb1: 0f 84 8b 13 00 00           	je	0x142347242 <.text+0x2346242>
142345eb7: c6 00 01                    	movb	$0x1, (%rax)
142345eba: c7 40 08 07 00 00 00        	movl	$0x7, 0x8(%rax)
142345ec1: 48 8d 0d 84 89 d0 00        	leaq	0xd08984(%rip), %rcx    # 0x14304e84c
142345ec8: 48 89 48 20                 	movq	%rcx, 0x20(%rax)
142345ecc: 48 c7 40 28 14 00 00 00     	movq	$0x14, 0x28(%rax)
142345ed4: f2 0f 11 70 30              	movsd	%xmm6, 0x30(%rax)
142345ed9: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
142345edd: c6 40 40 01                 	movb	$0x1, 0x40(%rax)
142345ee1: c7 40 48 08 00 00 00        	movl	$0x8, 0x48(%rax)
142345ee8: 48 c7 40 50 01 00 00 00     	movq	$0x1, 0x50(%rax)
142345ef0: 48 8d 0d 69 89 d0 00        	leaq	0xd08969(%rip), %rcx    # 0x14304e860
142345ef7: 48 89 48 60                 	movq	%rcx, 0x60(%rax)
142345efb: 48 c7 40 68 15 00 00 00     	movq	$0x15, 0x68(%rax)
142345f03: f2 0f 11 70 70              	movsd	%xmm6, 0x70(%rax)
142345f08: c6 40 78 00                 	movb	$0x0, 0x78(%rax)
142345f0c: c6 80 80 00 00 00 01        	movb	$0x1, 0x80(%rax)
142345f13: c7 80 88 00 00 00 08 00 00 00       	movl	$0x8, 0x88(%rax)
142345f1d: 48 c7 80 90 00 00 00 02 00 00 00    	movq	$0x2, 0x90(%rax)
142345f28: 48 8d 0d 46 89 d0 00        	leaq	0xd08946(%rip), %rcx    # 0x14304e875
142345f2f: 48 89 88 a0 00 00 00        	movq	%rcx, 0xa0(%rax)
142345f36: 48 c7 80 a8 00 00 00 15 00 00 00    	movq	$0x15, 0xa8(%rax)
142345f41: f2 0f 11 b0 b0 00 00 00     	movsd	%xmm6, 0xb0(%rax)
142345f49: c6 80 b8 00 00 00 00        	movb	$0x0, 0xb8(%rax)
142345f50: c6 80 c0 00 00 00 01        	movb	$0x1, 0xc0(%rax)
142345f57: c7 80 c8 00 00 00 08 00 00 00       	movl	$0x8, 0xc8(%rax)
142345f61: 48 c7 80 d0 00 00 00 03 00 00 00    	movq	$0x3, 0xd0(%rax)
142345f6c: 48 8d 0d 17 89 d0 00        	leaq	0xd08917(%rip), %rcx    # 0x14304e88a
142345f73: 48 89 88 e0 00 00 00        	movq	%rcx, 0xe0(%rax)
142345f7a: 48 c7 80 e8 00 00 00 15 00 00 00    	movq	$0x15, 0xe8(%rax)
142345f85: f2 0f 11 b0 f0 00 00 00     	movsd	%xmm6, 0xf0(%rax)
142345f8d: c6 80 f8 00 00 00 00        	movb	$0x0, 0xf8(%rax)
142345f94: 48 c7 85 b0 01 00 00 01 00 00 00    	movq	$0x1, 0x1b0(%rbp)
142345f9f: 48 c7 85 b8 01 00 00 00 00 00 00    	movq	$0x0, 0x1b8(%rbp)
142345faa: f2 0f 11 b5 c0 01 00 00     	movsd	%xmm6, 0x1c0(%rbp)
142345fb2: c6 85 90 01 00 00 04        	movb	$0x4, 0x190(%rbp)
142345fb9: 48 c7 85 98 01 00 00 04 00 00 00    	movq	$0x4, 0x198(%rbp)
142345fc4: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
142345fcb: 48 c7 85 a8 01 00 00 04 00 00 00    	movq	$0x4, 0x1a8(%rbp)
142345fd6: c6 85 c8 01 00 00 00        	movb	$0x0, 0x1c8(%rbp)
142345fdd: 0f b6 05 25 ec 2e 01        	movzbl	0x12eec25(%rip), %eax   # 0x143634c09
142345fe4: b9 40 00 00 00              	movl	$0x40, %ecx
142345fe9: ba 08 00 00 00              	movl	$0x8, %edx
142345fee: e8 1d dc 2c fe              	callq	0x140613c10 <.text+0x612c10>
142345ff3: 48 85 c0                    	testq	%rax, %rax
142345ff6: 0f 84 6e 12 00 00           	je	0x14234726a <.text+0x234626a>
142345ffc: 48 89 c7                    	movq	%rax, %rdi
142345fff: 0f b6 05 03 ec 2e 01        	movzbl	0x12eec03(%rip), %eax   # 0x143634c09
142346006: b9 80 00 00 00              	movl	$0x80, %ecx
14234600b: ba 08 00 00 00              	movl	$0x8, %edx
142346010: e8 fb db 2c fe              	callq	0x140613c10 <.text+0x612c10>
142346015: 48 85 c0                    	testq	%rax, %rax
142346018: 0f 84 60 12 00 00           	je	0x14234727e <.text+0x234627e>
14234601e: 0f 10 85 50 02 00 00        	movups	0x250(%rbp), %xmm0
142346025: 0f 10 8d 60 02 00 00        	movups	0x260(%rbp), %xmm1
14234602c: 0f 10 95 70 02 00 00        	movups	0x270(%rbp), %xmm2
142346033: 0f 10 9d 80 02 00 00        	movups	0x280(%rbp), %xmm3
14234603a: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
14234603e: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
142346042: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
142346046: 0f 11 00                    	movups	%xmm0, (%rax)
142346049: 0f 10 85 90 01 00 00        	movups	0x190(%rbp), %xmm0
142346050: 0f 10 8d a0 01 00 00        	movups	0x1a0(%rbp), %xmm1
142346057: 0f 10 95 b0 01 00 00        	movups	0x1b0(%rbp), %xmm2
14234605e: 0f 10 9d c0 01 00 00        	movups	0x1c0(%rbp), %xmm3
142346065: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
142346069: 0f 11 48 50                 	movups	%xmm1, 0x50(%rax)
14234606d: 0f 11 50 60                 	movups	%xmm2, 0x60(%rax)
142346071: 0f 11 58 70                 	movups	%xmm3, 0x70(%rax)
142346075: c6 07 05                    	movb	$0x5, (%rdi)
142346078: 48 c7 47 08 02 00 00 00     	movq	$0x2, 0x8(%rdi)
142346080: 48 89 47 10                 	movq	%rax, 0x10(%rdi)
142346084: 48 c7 47 18 02 00 00 00     	movq	$0x2, 0x18(%rdi)
14234608c: 48 c7 47 20 01 00 00 00     	movq	$0x1, 0x20(%rdi)
142346094: 48 c7 47 28 00 00 00 00     	movq	$0x0, 0x28(%rdi)
14234609c: f2 0f 11 77 30              	movsd	%xmm6, 0x30(%rdi)
1423460a1: c6 47 38 00                 	movb	$0x0, 0x38(%rdi)
1423460a5: 48 8d 05 1d 88 d0 00        	leaq	0xd0881d(%rip), %rax    # 0x14304e8c9
1423460ac: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
1423460b3: 48 c7 85 b0 00 00 00 0c 00 00 00    	movq	$0xc, 0xb0(%rbp)
1423460be: c6 85 b8 00 00 00 01        	movb	$0x1, 0xb8(%rbp)
1423460c5: 48 c7 85 c0 00 00 00 01 00 00 00    	movq	$0x1, 0xc0(%rbp)
1423460d0: 48 89 bd c8 00 00 00        	movq	%rdi, 0xc8(%rbp)
1423460d7: 48 c7 85 d0 00 00 00 01 00 00 00    	movq	$0x1, 0xd0(%rbp)
1423460e2: 0f b6 05 20 eb 2e 01        	movzbl	0x12eeb20(%rip), %eax   # 0x143634c09
1423460e9: 44 0f b6 3b                 	movzbl	(%rbx), %r15d
1423460ed: b9 c0 00 00 00              	movl	$0xc0, %ecx
1423460f2: ba 08 00 00 00              	movl	$0x8, %edx
1423460f7: e8 14 db 2c fe              	callq	0x140613c10 <.text+0x612c10>
1423460fc: 45 84 ff                    	testb	%r15b, %r15b
1423460ff: 74 78                       	je	0x142346179 <.text+0x2345179>
142346101: 48 85 c0                    	testq	%rax, %rax
142346104: 0f 84 8f 11 00 00           	je	0x142347299 <.text+0x2346299>
14234610a: c6 00 01                    	movb	$0x1, (%rax)
14234610d: c7 40 08 0b 00 00 00        	movl	$0xb, 0x8(%rax)
142346114: c6 40 0c 38                 	movb	$0x38, 0xc(%rax)
142346118: 48 8d 0d b6 87 d0 00        	leaq	0xd087b6(%rip), %rcx    # 0x14304e8d5
14234611f: 48 89 48 20                 	movq	%rcx, 0x20(%rax)
142346123: 48 c7 40 28 16 00 00 00     	movq	$0x16, 0x28(%rax)
14234612b: f2 0f 11 70 30              	movsd	%xmm6, 0x30(%rax)
142346130: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
142346134: c6 40 40 07                 	movb	$0x7, 0x40(%rax)
142346138: 48 8d 0d 60 87 d0 00        	leaq	0xd08760(%rip), %rcx    # 0x14304e89f
14234613f: 48 89 48 60                 	movq	%rcx, 0x60(%rax)
142346143: 48 c7 40 68 2a 00 00 00     	movq	$0x2a, 0x68(%rax)
14234614b: f2 0f 11 70 70              	movsd	%xmm6, 0x70(%rax)
142346150: c6 40 78 00                 	movb	$0x0, 0x78(%rax)
142346154: c6 80 80 00 00 00 07        	movb	$0x7, 0x80(%rax)
14234615b: 48 8d 0d b0 87 d0 00        	leaq	0xd087b0(%rip), %rcx    # 0x14304e912
142346162: 48 89 88 a0 00 00 00        	movq	%rcx, 0xa0(%rax)
142346169: 48 c7 80 a8 00 00 00 27 00 00 00    	movq	$0x27, 0xa8(%rax)
142346174: e9 8f 00 00 00              	jmp	0x142346208 <.text+0x2345208>
142346179: 48 85 c0                    	testq	%rax, %rax
14234617c: 0f 84 2b 11 00 00           	je	0x1423472ad <.text+0x23462ad>
142346182: c6 00 01                    	movb	$0x1, (%rax)
142346185: c7 40 08 0b 00 00 00        	movl	$0xb, 0x8(%rax)
14234618c: c6 40 0c 38                 	movb	$0x38, 0xc(%rax)
142346190: 48 8d 0d 3e 87 d0 00        	leaq	0xd0873e(%rip), %rcx    # 0x14304e8d5
142346197: 48 89 48 20                 	movq	%rcx, 0x20(%rax)
14234619b: 48 c7 40 28 16 00 00 00     	movq	$0x16, 0x28(%rax)
1423461a3: f2 0f 11 70 30              	movsd	%xmm6, 0x30(%rax)
1423461a8: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
1423461ac: c6 40 40 01                 	movb	$0x1, 0x40(%rax)
1423461b0: c7 40 48 0b 00 00 00        	movl	$0xb, 0x48(%rax)
1423461b7: c6 40 4c 39                 	movb	$0x39, 0x4c(%rax)
1423461bb: 48 8d 0d 29 87 d0 00        	leaq	0xd08729(%rip), %rcx    # 0x14304e8eb
1423461c2: 48 89 48 60                 	movq	%rcx, 0x60(%rax)
1423461c6: 48 c7 40 68 15 00 00 00     	movq	$0x15, 0x68(%rax)
1423461ce: f2 0f 11 70 70              	movsd	%xmm6, 0x70(%rax)
1423461d3: c6 40 78 00                 	movb	$0x0, 0x78(%rax)
1423461d7: c6 80 80 00 00 00 01        	movb	$0x1, 0x80(%rax)
1423461de: c7 80 88 00 00 00 0b 00 00 00       	movl	$0xb, 0x88(%rax)
1423461e8: c6 80 8c 00 00 00 3a        	movb	$0x3a, 0x8c(%rax)
1423461ef: 48 8d 0d 0a 87 d0 00        	leaq	0xd0870a(%rip), %rcx    # 0x14304e900
1423461f6: 48 89 88 a0 00 00 00        	movq	%rcx, 0xa0(%rax)
1423461fd: 48 c7 80 a8 00 00 00 12 00 00 00    	movq	$0x12, 0xa8(%rax)
142346208: f2 0f 11 b0 b0 00 00 00     	movsd	%xmm6, 0xb0(%rax)
142346210: c6 80 b8 00 00 00 00        	movb	$0x0, 0xb8(%rax)
142346217: 48 c7 85 70 02 00 00 01 00 00 00    	movq	$0x1, 0x270(%rbp)
142346222: 48 c7 85 78 02 00 00 00 00 00 00    	movq	$0x0, 0x278(%rbp)
14234622d: f2 0f 11 b5 80 02 00 00     	movsd	%xmm6, 0x280(%rbp)
142346235: c6 85 50 02 00 00 04        	movb	$0x4, 0x250(%rbp)
14234623c: 48 c7 85 58 02 00 00 03 00 00 00    	movq	$0x3, 0x258(%rbp)
142346247: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
14234624e: 48 c7 85 68 02 00 00 03 00 00 00    	movq	$0x3, 0x268(%rbp)
142346259: c6 85 88 02 00 00 00        	movb	$0x0, 0x288(%rbp)
142346260: 0f b6 05 a2 e9 2e 01        	movzbl	0x12ee9a2(%rip), %eax   # 0x143634c09
142346267: 44 0f b6 3b                 	movzbl	(%rbx), %r15d
14234626b: b9 c0 00 00 00              	movl	$0xc0, %ecx
142346270: ba 08 00 00 00              	movl	$0x8, %edx
142346275: e8 96 d9 2c fe              	callq	0x140613c10 <.text+0x612c10>
14234627a: 45 84 ff                    	testb	%r15b, %r15b
14234627d: 74 78                       	je	0x1423462f7 <.text+0x23452f7>
14234627f: 48 85 c0                    	testq	%rax, %rax
142346282: 0f 84 39 10 00 00           	je	0x1423472c1 <.text+0x23462c1>
142346288: c6 00 01                    	movb	$0x1, (%rax)
14234628b: c7 40 08 0b 00 00 00        	movl	$0xb, 0x8(%rax)
142346292: c6 40 0c 3c                 	movb	$0x3c, 0xc(%rax)
142346296: 48 8d 0d 9c 86 d0 00        	leaq	0xd0869c(%rip), %rcx    # 0x14304e939
14234629d: 48 89 48 20                 	movq	%rcx, 0x20(%rax)
1423462a1: 48 c7 40 28 0f 00 00 00     	movq	$0xf, 0x28(%rax)
1423462a9: f2 0f 11 70 30              	movsd	%xmm6, 0x30(%rax)
1423462ae: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
1423462b2: c6 40 40 07                 	movb	$0x7, 0x40(%rax)
1423462b6: 48 8d 0d a6 86 d0 00        	leaq	0xd086a6(%rip), %rcx    # 0x14304e963
1423462bd: 48 89 48 60                 	movq	%rcx, 0x60(%rax)
1423462c1: 48 c7 40 68 21 00 00 00     	movq	$0x21, 0x68(%rax)
1423462c9: f2 0f 11 70 70              	movsd	%xmm6, 0x70(%rax)
1423462ce: c6 40 78 00                 	movb	$0x0, 0x78(%rax)
1423462d2: c6 80 80 00 00 00 07        	movb	$0x7, 0x80(%rax)
1423462d9: 48 8d 0d a4 86 d0 00        	leaq	0xd086a4(%rip), %rcx    # 0x14304e984
1423462e0: 48 89 88 a0 00 00 00        	movq	%rcx, 0xa0(%rax)
1423462e7: 48 c7 80 a8 00 00 00 24 00 00 00    	movq	$0x24, 0xa8(%rax)
1423462f2: e9 8f 00 00 00              	jmp	0x142346386 <.text+0x2345386>
1423462f7: 48 85 c0                    	testq	%rax, %rax
1423462fa: 0f 84 d5 0f 00 00           	je	0x1423472d5 <.text+0x23462d5>
142346300: c6 00 01                    	movb	$0x1, (%rax)
142346303: c7 40 08 0b 00 00 00        	movl	$0xb, 0x8(%rax)
14234630a: c6 40 0c 3c                 	movb	$0x3c, 0xc(%rax)
14234630e: 48 8d 0d 24 86 d0 00        	leaq	0xd08624(%rip), %rcx    # 0x14304e939
142346315: 48 89 48 20                 	movq	%rcx, 0x20(%rax)
142346319: 48 c7 40 28 0f 00 00 00     	movq	$0xf, 0x28(%rax)
142346321: f2 0f 11 70 30              	movsd	%xmm6, 0x30(%rax)
142346326: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
14234632a: c6 40 40 01                 	movb	$0x1, 0x40(%rax)
14234632e: c7 40 48 0b 00 00 00        	movl	$0xb, 0x48(%rax)
142346335: c6 40 4c 3d                 	movb	$0x3d, 0x4c(%rax)
142346339: 48 8d 0d 08 86 d0 00        	leaq	0xd08608(%rip), %rcx    # 0x14304e948
142346340: 48 89 48 60                 	movq	%rcx, 0x60(%rax)
142346344: 48 c7 40 68 0c 00 00 00     	movq	$0xc, 0x68(%rax)
14234634c: f2 0f 11 70 70              	movsd	%xmm6, 0x70(%rax)
142346351: c6 40 78 00                 	movb	$0x0, 0x78(%rax)
142346355: c6 80 80 00 00 00 01        	movb	$0x1, 0x80(%rax)
14234635c: c7 80 88 00 00 00 0b 00 00 00       	movl	$0xb, 0x88(%rax)
142346366: c6 80 8c 00 00 00 3b        	movb	$0x3b, 0x8c(%rax)
14234636d: 48 8d 0d e0 85 d0 00        	leaq	0xd085e0(%rip), %rcx    # 0x14304e954
142346374: 48 89 88 a0 00 00 00        	movq	%rcx, 0xa0(%rax)
14234637b: 48 c7 80 a8 00 00 00 0f 00 00 00    	movq	$0xf, 0xa8(%rax)
142346386: f2 0f 11 b0 b0 00 00 00     	movsd	%xmm6, 0xb0(%rax)
14234638e: c6 80 b8 00 00 00 00        	movb	$0x0, 0xb8(%rax)
142346395: 48 c7 85 b0 01 00 00 01 00 00 00    	movq	$0x1, 0x1b0(%rbp)
1423463a0: 48 c7 85 b8 01 00 00 00 00 00 00    	movq	$0x0, 0x1b8(%rbp)
1423463ab: f2 0f 11 b5 c0 01 00 00     	movsd	%xmm6, 0x1c0(%rbp)
1423463b3: c6 85 90 01 00 00 04        	movb	$0x4, 0x190(%rbp)
1423463ba: 48 c7 85 98 01 00 00 03 00 00 00    	movq	$0x3, 0x198(%rbp)
1423463c5: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
1423463cc: 48 c7 85 a8 01 00 00 03 00 00 00    	movq	$0x3, 0x1a8(%rbp)
1423463d7: c6 85 c8 01 00 00 00        	movb	$0x0, 0x1c8(%rbp)
1423463de: 0f b6 05 24 e8 2e 01        	movzbl	0x12ee824(%rip), %eax   # 0x143634c09
1423463e5: b9 40 00 00 00              	movl	$0x40, %ecx
1423463ea: ba 08 00 00 00              	movl	$0x8, %edx
1423463ef: e8 1c d8 2c fe              	callq	0x140613c10 <.text+0x612c10>
1423463f4: 48 85 c0                    	testq	%rax, %rax
1423463f7: 0f 84 d3 0d 00 00           	je	0x1423471d0 <.text+0x23461d0>
1423463fd: 48 89 c7                    	movq	%rax, %rdi
142346400: 0f b6 05 02 e8 2e 01        	movzbl	0x12ee802(%rip), %eax   # 0x143634c09
142346407: b9 80 00 00 00              	movl	$0x80, %ecx
14234640c: ba 08 00 00 00              	movl	$0x8, %edx
142346411: e8 fa d7 2c fe              	callq	0x140613c10 <.text+0x612c10>
142346416: 48 85 c0                    	testq	%rax, %rax
142346419: 0f 84 c5 0d 00 00           	je	0x1423471e4 <.text+0x23461e4>
14234641f: 0f 10 85 50 02 00 00        	movups	0x250(%rbp), %xmm0
142346426: 0f 10 8d 60 02 00 00        	movups	0x260(%rbp), %xmm1
14234642d: 0f 10 95 70 02 00 00        	movups	0x270(%rbp), %xmm2
142346434: 0f 10 9d 80 02 00 00        	movups	0x280(%rbp), %xmm3
14234643b: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
14234643f: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
142346443: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
142346447: 0f 11 00                    	movups	%xmm0, (%rax)
14234644a: 0f 10 85 90 01 00 00        	movups	0x190(%rbp), %xmm0
142346451: 0f 10 8d a0 01 00 00        	movups	0x1a0(%rbp), %xmm1
142346458: 0f 10 95 b0 01 00 00        	movups	0x1b0(%rbp), %xmm2
14234645f: 0f 10 9d c0 01 00 00        	movups	0x1c0(%rbp), %xmm3
142346466: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
14234646a: 0f 11 48 50                 	movups	%xmm1, 0x50(%rax)
14234646e: 0f 11 50 60                 	movups	%xmm2, 0x60(%rax)
142346472: 0f 11 58 70                 	movups	%xmm3, 0x70(%rax)
142346476: c6 07 05                    	movb	$0x5, (%rdi)
142346479: 48 c7 47 08 02 00 00 00     	movq	$0x2, 0x8(%rdi)
142346481: 48 89 47 10                 	movq	%rax, 0x10(%rdi)
142346485: 48 c7 47 18 02 00 00 00     	movq	$0x2, 0x18(%rdi)
14234648d: 48 c7 47 20 01 00 00 00     	movq	$0x1, 0x20(%rdi)
142346495: 48 c7 47 28 00 00 00 00     	movq	$0x0, 0x28(%rdi)
14234649d: f2 0f 11 77 30              	movsd	%xmm6, 0x30(%rdi)
1423464a2: c6 47 38 00                 	movb	$0x0, 0x38(%rdi)
1423464a6: 48 8d 05 fb 84 d0 00        	leaq	0xd084fb(%rip), %rax    # 0x14304e9a8
1423464ad: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
1423464b1: 48 c7 85 80 00 00 00 12 00 00 00    	movq	$0x12, 0x80(%rbp)
1423464bc: c6 85 88 00 00 00 02        	movb	$0x2, 0x88(%rbp)
1423464c3: 48 c7 85 90 00 00 00 01 00 00 00    	movq	$0x1, 0x90(%rbp)
1423464ce: 48 89 bd 98 00 00 00        	movq	%rdi, 0x98(%rbp)
1423464d5: 48 c7 85 a0 00 00 00 01 00 00 00    	movq	$0x1, 0xa0(%rbp)
1423464e0: 48 8d 05 d3 84 d0 00        	leaq	0xd084d3(%rip), %rax    # 0x14304e9ba
1423464e7: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1423464eb: 48 c7 45 50 15 00 00 00     	movq	$0x15, 0x50(%rbp)
1423464f3: c6 45 58 03                 	movb	$0x3, 0x58(%rbp)
1423464f7: 48 c7 45 60 00 00 00 00     	movq	$0x0, 0x60(%rbp)
1423464ff: 48 c7 45 68 08 00 00 00     	movq	$0x8, 0x68(%rbp)
142346507: 48 c7 45 70 00 00 00 00     	movq	$0x0, 0x70(%rbp)
14234650f: 80 3b 00                    	cmpb	$0x0, (%rbx)
142346512: 0f 84 fa 00 00 00           	je	0x142346612 <.text+0x2345612>
142346518: 0f b6 05 ea e6 2e 01        	movzbl	0x12ee6ea(%rip), %eax   # 0x143634c09
14234651f: b9 00 01 00 00              	movl	$0x100, %ecx            # imm = 0x100
142346524: ba 08 00 00 00              	movl	$0x8, %edx
142346529: e8 e2 d6 2c fe              	callq	0x140613c10 <.text+0x612c10>
14234652e: 48 85 c0                    	testq	%rax, %rax
142346531: 0f 84 b2 0d 00 00           	je	0x1423472e9 <.text+0x23462e9>
142346537: 48 89 c7                    	movq	%rax, %rdi
14234653a: c6 00 01                    	movb	$0x1, (%rax)
14234653d: c7 40 08 09 00 00 00        	movl	$0x9, 0x8(%rax)
142346544: c6 40 0c 01                 	movb	$0x1, 0xc(%rax)
142346548: 4c 89 70 20                 	movq	%r14, 0x20(%rax)
14234654c: 48 c7 40 28 0c 00 00 00     	movq	$0xc, 0x28(%rax)
142346554: f2 0f 11 70 30              	movsd	%xmm6, 0x30(%rax)
142346559: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
14234655d: c6 40 40 01                 	movb	$0x1, 0x40(%rax)
142346561: c7 40 48 09 00 00 00        	movl	$0x9, 0x48(%rax)
142346568: c6 40 4c 02                 	movb	$0x2, 0x4c(%rax)
14234656c: 48 8d 05 74 84 d0 00        	leaq	0xd08474(%rip), %rax    # 0x14304e9e7
142346573: 48 89 47 60                 	movq	%rax, 0x60(%rdi)
142346577: 48 c7 47 68 0e 00 00 00     	movq	$0xe, 0x68(%rdi)
14234657f: f2 0f 11 77 70              	movsd	%xmm6, 0x70(%rdi)
142346584: c6 47 78 00                 	movb	$0x0, 0x78(%rdi)
142346588: c6 87 80 00 00 00 01        	movb	$0x1, 0x80(%rdi)
14234658f: c7 87 88 00 00 00 09 00 00 00       	movl	$0x9, 0x88(%rdi)
142346599: c6 87 8c 00 00 00 05        	movb	$0x5, 0x8c(%rdi)
1423465a0: 48 8d 05 5c 84 d0 00        	leaq	0xd0845c(%rip), %rax    # 0x14304ea03
1423465a7: 48 89 87 a0 00 00 00        	movq	%rax, 0xa0(%rdi)
1423465ae: 48 c7 87 a8 00 00 00 0e 00 00 00    	movq	$0xe, 0xa8(%rdi)
1423465b9: f2 0f 11 b7 b0 00 00 00     	movsd	%xmm6, 0xb0(%rdi)
1423465c1: c6 87 b8 00 00 00 00        	movb	$0x0, 0xb8(%rdi)
1423465c8: c6 87 c0 00 00 00 01        	movb	$0x1, 0xc0(%rdi)
1423465cf: c7 87 c8 00 00 00 09 00 00 00       	movl	$0x9, 0xc8(%rdi)
1423465d9: c6 87 cc 00 00 00 03        	movb	$0x3, 0xcc(%rdi)
1423465e0: 48 8d 05 0e 84 d0 00        	leaq	0xd0840e(%rip), %rax    # 0x14304e9f5
1423465e7: 48 89 87 e0 00 00 00        	movq	%rax, 0xe0(%rdi)
1423465ee: 48 c7 87 e8 00 00 00 0e 00 00 00    	movq	$0xe, 0xe8(%rdi)
1423465f9: f2 0f 11 b7 f0 00 00 00     	movsd	%xmm6, 0xf0(%rdi)
142346601: c6 87 f8 00 00 00 00        	movb	$0x0, 0xf8(%rdi)
142346608: b8 04 00 00 00              	movl	$0x4, %eax
14234660d: e9 81 03 00 00              	jmp	0x142346993 <.text+0x2345993>
142346612: 0f b6 05 f0 e5 2e 01        	movzbl	0x12ee5f0(%rip), %eax   # 0x143634c09
142346619: b9 40 00 00 00              	movl	$0x40, %ecx
14234661e: ba 08 00 00 00              	movl	$0x8, %edx
142346623: e8 e8 d5 2c fe              	callq	0x140613c10 <.text+0x612c10>
142346628: 48 85 c0                    	testq	%rax, %rax
14234662b: 0f 84 cc 0c 00 00           	je	0x1423472fd <.text+0x23462fd>
142346631: 66 c7 00 02 09              	movw	$0x902, (%rax)          # imm = 0x902
142346636: 48 8d 0d 92 83 d0 00        	leaq	0xd08392(%rip), %rcx    # 0x14304e9cf
14234663d: 48 89 48 20                 	movq	%rcx, 0x20(%rax)
142346641: 48 c7 40 28 18 00 00 00     	movq	$0x18, 0x28(%rax)
142346649: f2 0f 11 78 30              	movsd	%xmm7, 0x30(%rax)
14234664e: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
142346652: 48 c7 85 68 01 00 00 01 00 00 00    	movq	$0x1, 0x168(%rbp)
14234665d: 48 c7 85 70 01 00 00 00 00 00 00    	movq	$0x0, 0x170(%rbp)
142346668: f2 0f 11 b5 78 01 00 00     	movsd	%xmm6, 0x178(%rbp)
142346670: c6 85 48 01 00 00 04        	movb	$0x4, 0x148(%rbp)
142346677: 48 c7 85 50 01 00 00 01 00 00 00    	movq	$0x1, 0x150(%rbp)
142346682: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
142346689: 48 c7 85 60 01 00 00 01 00 00 00    	movq	$0x1, 0x160(%rbp)
142346694: c6 85 80 01 00 00 00        	movb	$0x0, 0x180(%rbp)
14234669b: 0f b6 05 67 e5 2e 01        	movzbl	0x12ee567(%rip), %eax   # 0x143634c09
1423466a2: b9 c0 00 00 00              	movl	$0xc0, %ecx
1423466a7: ba 08 00 00 00              	movl	$0x8, %edx
1423466ac: e8 5f d5 2c fe              	callq	0x140613c10 <.text+0x612c10>
1423466b1: 48 85 c0                    	testq	%rax, %rax
1423466b4: 0f 84 54 0c 00 00           	je	0x14234730e <.text+0x234630e>
1423466ba: c6 00 01                    	movb	$0x1, (%rax)
1423466bd: c7 40 08 09 00 00 00        	movl	$0x9, 0x8(%rax)
1423466c4: c6 40 0c 01                 	movb	$0x1, 0xc(%rax)
1423466c8: 4c 89 70 20                 	movq	%r14, 0x20(%rax)
1423466cc: 48 c7 40 28 0c 00 00 00     	movq	$0xc, 0x28(%rax)
1423466d4: f2 0f 11 70 30              	movsd	%xmm6, 0x30(%rax)
1423466d9: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
1423466dd: c6 40 40 01                 	movb	$0x1, 0x40(%rax)
1423466e1: c7 40 48 09 00 00 00        	movl	$0x9, 0x48(%rax)
1423466e8: c6 40 4c 02                 	movb	$0x2, 0x4c(%rax)
1423466ec: 48 8d 0d f4 82 d0 00        	leaq	0xd082f4(%rip), %rcx    # 0x14304e9e7
1423466f3: 48 89 48 60                 	movq	%rcx, 0x60(%rax)
1423466f7: 48 c7 40 68 0e 00 00 00     	movq	$0xe, 0x68(%rax)
1423466ff: f2 0f 11 70 70              	movsd	%xmm6, 0x70(%rax)
142346704: c6 40 78 00                 	movb	$0x0, 0x78(%rax)
142346708: c6 80 80 00 00 00 01        	movb	$0x1, 0x80(%rax)
14234670f: c7 80 88 00 00 00 09 00 00 00       	movl	$0x9, 0x88(%rax)
142346719: c6 80 8c 00 00 00 03        	movb	$0x3, 0x8c(%rax)
142346720: 48 8d 0d ce 82 d0 00        	leaq	0xd082ce(%rip), %rcx    # 0x14304e9f5
142346727: 48 89 88 a0 00 00 00        	movq	%rcx, 0xa0(%rax)
14234672e: 48 c7 80 a8 00 00 00 0e 00 00 00    	movq	$0xe, 0xa8(%rax)
142346739: f2 0f 11 b0 b0 00 00 00     	movsd	%xmm6, 0xb0(%rax)
142346741: c6 80 b8 00 00 00 00        	movb	$0x0, 0xb8(%rax)
142346748: 48 c7 85 70 02 00 00 01 00 00 00    	movq	$0x1, 0x270(%rbp)
142346753: 48 c7 85 78 02 00 00 00 00 00 00    	movq	$0x0, 0x278(%rbp)
14234675e: f2 0f 11 b5 80 02 00 00     	movsd	%xmm6, 0x280(%rbp)
142346766: c6 85 50 02 00 00 04        	movb	$0x4, 0x250(%rbp)
14234676d: 48 c7 85 58 02 00 00 03 00 00 00    	movq	$0x3, 0x258(%rbp)
142346778: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
14234677f: 48 c7 85 68 02 00 00 03 00 00 00    	movq	$0x3, 0x268(%rbp)
14234678a: c6 85 88 02 00 00 00        	movb	$0x0, 0x288(%rbp)
142346791: 0f b6 05 71 e4 2e 01        	movzbl	0x12ee471(%rip), %eax   # 0x143634c09
142346798: b9 c0 00 00 00              	movl	$0xc0, %ecx
14234679d: ba 08 00 00 00              	movl	$0x8, %edx
1423467a2: e8 69 d4 2c fe              	callq	0x140613c10 <.text+0x612c10>
1423467a7: 48 85 c0                    	testq	%rax, %rax
1423467aa: 0f 84 6f 0b 00 00           	je	0x14234731f <.text+0x234631f>
1423467b0: c6 00 01                    	movb	$0x1, (%rax)
1423467b3: c7 40 08 09 00 00 00        	movl	$0x9, 0x8(%rax)
1423467ba: c6 40 0c 05                 	movb	$0x5, 0xc(%rax)
1423467be: 48 8d 0d 3e 82 d0 00        	leaq	0xd0823e(%rip), %rcx    # 0x14304ea03
1423467c5: 48 89 48 20                 	movq	%rcx, 0x20(%rax)
1423467c9: 48 c7 40 28 0e 00 00 00     	movq	$0xe, 0x28(%rax)
1423467d1: f2 0f 11 70 30              	movsd	%xmm6, 0x30(%rax)
1423467d6: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
1423467da: c6 40 40 01                 	movb	$0x1, 0x40(%rax)
1423467de: c7 40 48 09 00 00 00        	movl	$0x9, 0x48(%rax)
1423467e5: c6 40 4c 06                 	movb	$0x6, 0x4c(%rax)
1423467e9: 48 8d 0d 21 82 d0 00        	leaq	0xd08221(%rip), %rcx    # 0x14304ea11
1423467f0: 48 89 48 60                 	movq	%rcx, 0x60(%rax)
1423467f4: 48 c7 40 68 0c 00 00 00     	movq	$0xc, 0x68(%rax)
1423467fc: f2 0f 11 70 70              	movsd	%xmm6, 0x70(%rax)
142346801: c6 40 78 00                 	movb	$0x0, 0x78(%rax)
142346805: c6 80 80 00 00 00 01        	movb	$0x1, 0x80(%rax)
14234680c: c7 80 88 00 00 00 09 00 00 00       	movl	$0x9, 0x88(%rax)
142346816: c6 80 8c 00 00 00 04        	movb	$0x4, 0x8c(%rax)
14234681d: 48 8d 0d f9 81 d0 00        	leaq	0xd081f9(%rip), %rcx    # 0x14304ea1d
142346824: 48 89 88 a0 00 00 00        	movq	%rcx, 0xa0(%rax)
14234682b: 48 c7 80 a8 00 00 00 0d 00 00 00    	movq	$0xd, 0xa8(%rax)
142346836: f2 0f 11 b0 b0 00 00 00     	movsd	%xmm6, 0xb0(%rax)
14234683e: c6 80 b8 00 00 00 00        	movb	$0x0, 0xb8(%rax)
142346845: 48 c7 85 b0 01 00 00 01 00 00 00    	movq	$0x1, 0x1b0(%rbp)
142346850: 48 c7 85 b8 01 00 00 00 00 00 00    	movq	$0x0, 0x1b8(%rbp)
14234685b: f2 0f 11 b5 c0 01 00 00     	movsd	%xmm6, 0x1c0(%rbp)
142346863: c6 85 90 01 00 00 04        	movb	$0x4, 0x190(%rbp)
14234686a: 48 c7 85 98 01 00 00 03 00 00 00    	movq	$0x3, 0x198(%rbp)
142346875: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
14234687c: 48 c7 85 a8 01 00 00 03 00 00 00    	movq	$0x3, 0x1a8(%rbp)
142346887: c6 85 c8 01 00 00 00        	movb	$0x0, 0x1c8(%rbp)
14234688e: 0f b6 05 74 e3 2e 01        	movzbl	0x12ee374(%rip), %eax   # 0x143634c09
142346895: b9 40 00 00 00              	movl	$0x40, %ecx
14234689a: ba 08 00 00 00              	movl	$0x8, %edx
14234689f: e8 6c d3 2c fe              	callq	0x140613c10 <.text+0x612c10>
1423468a4: 48 85 c0                    	testq	%rax, %rax
1423468a7: 0f 84 83 0a 00 00           	je	0x142347330 <.text+0x2346330>
1423468ad: 48 89 c7                    	movq	%rax, %rdi
1423468b0: 0f b6 05 52 e3 2e 01        	movzbl	0x12ee352(%rip), %eax   # 0x143634c09
1423468b7: b9 c0 00 00 00              	movl	$0xc0, %ecx
1423468bc: ba 08 00 00 00              	movl	$0x8, %edx
1423468c1: e8 4a d3 2c fe              	callq	0x140613c10 <.text+0x612c10>
1423468c6: 48 85 c0                    	testq	%rax, %rax
1423468c9: 0f 84 72 0a 00 00           	je	0x142347341 <.text+0x2346341>
1423468cf: 0f 10 85 48 01 00 00        	movups	0x148(%rbp), %xmm0
1423468d6: 0f 10 8d 58 01 00 00        	movups	0x158(%rbp), %xmm1
1423468dd: 0f 10 95 68 01 00 00        	movups	0x168(%rbp), %xmm2
1423468e4: 0f 10 9d 78 01 00 00        	movups	0x178(%rbp), %xmm3
1423468eb: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1423468ef: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1423468f3: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1423468f7: 0f 11 00                    	movups	%xmm0, (%rax)
1423468fa: 0f 10 85 90 01 00 00        	movups	0x190(%rbp), %xmm0
142346901: 0f 10 8d a0 01 00 00        	movups	0x1a0(%rbp), %xmm1
142346908: 0f 10 95 b0 01 00 00        	movups	0x1b0(%rbp), %xmm2
14234690f: 0f 10 9d c0 01 00 00        	movups	0x1c0(%rbp), %xmm3
142346916: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
14234691a: 0f 11 48 50                 	movups	%xmm1, 0x50(%rax)
14234691e: 0f 11 50 60                 	movups	%xmm2, 0x60(%rax)
142346922: 0f 11 58 70                 	movups	%xmm3, 0x70(%rax)
142346926: 0f 10 85 50 02 00 00        	movups	0x250(%rbp), %xmm0
14234692d: 0f 10 8d 60 02 00 00        	movups	0x260(%rbp), %xmm1
142346934: 0f 10 95 70 02 00 00        	movups	0x270(%rbp), %xmm2
14234693b: 0f 10 9d 80 02 00 00        	movups	0x280(%rbp), %xmm3
142346942: 0f 11 80 80 00 00 00        	movups	%xmm0, 0x80(%rax)
142346949: 0f 11 88 90 00 00 00        	movups	%xmm1, 0x90(%rax)
142346950: 0f 11 90 a0 00 00 00        	movups	%xmm2, 0xa0(%rax)
142346957: 0f 11 98 b0 00 00 00        	movups	%xmm3, 0xb0(%rax)
14234695e: c6 07 05                    	movb	$0x5, (%rdi)
142346961: 48 c7 47 08 03 00 00 00     	movq	$0x3, 0x8(%rdi)
142346969: 48 89 47 10                 	movq	%rax, 0x10(%rdi)
14234696d: 48 c7 47 18 03 00 00 00     	movq	$0x3, 0x18(%rdi)
142346975: 48 c7 47 20 01 00 00 00     	movq	$0x1, 0x20(%rdi)
14234697d: 48 c7 47 28 00 00 00 00     	movq	$0x0, 0x28(%rdi)
142346985: f2 0f 11 77 30              	movsd	%xmm6, 0x30(%rdi)
14234698a: c6 47 38 00                 	movb	$0x0, 0x38(%rdi)
14234698e: b8 01 00 00 00              	movl	$0x1, %eax
142346993: 48 8d 0d 90 80 d0 00        	leaq	0xd08090(%rip), %rcx    # 0x14304ea2a
14234699a: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
1423469a1: 48 c7 85 10 01 00 00 0e 00 00 00    	movq	$0xe, 0x110(%rbp)
1423469ac: c6 85 18 01 00 00 04        	movb	$0x4, 0x118(%rbp)
1423469b3: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1423469ba: 48 89 bd 28 01 00 00        	movq	%rdi, 0x128(%rbp)
1423469c1: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
1423469c8: 48 8d 05 69 80 d0 00        	leaq	0xd08069(%rip), %rax    # 0x14304ea38
1423469cf: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
1423469d6: 48 c7 85 50 01 00 00 0f 00 00 00    	movq	$0xf, 0x150(%rbp)
1423469e1: c6 85 58 01 00 00 06        	movb	$0x6, 0x158(%rbp)
1423469e8: 48 c7 85 60 01 00 00 00 00 00 00    	movq	$0x0, 0x160(%rbp)
1423469f3: 48 c7 85 68 01 00 00 08 00 00 00    	movq	$0x8, 0x168(%rbp)
1423469fe: 48 c7 85 70 01 00 00 00 00 00 00    	movq	$0x0, 0x170(%rbp)
142346a09: 0f b6 05 f9 e1 2e 01        	movzbl	0x12ee1f9(%rip), %eax   # 0x143634c09
142346a10: b9 40 00 00 00              	movl	$0x40, %ecx
142346a15: ba 08 00 00 00              	movl	$0x8, %edx
142346a1a: e8 f1 d1 2c fe              	callq	0x140613c10 <.text+0x612c10>
142346a1f: 48 85 c0                    	testq	%rax, %rax
142346a22: 0f 84 d7 07 00 00           	je	0x1423471ff <.text+0x23461ff>
142346a28: 49 89 c6                    	movq	%rax, %r14
142346a2b: 0f b6 05 d7 e1 2e 01        	movzbl	0x12ee1d7(%rip), %eax   # 0x143634c09
142346a32: bf 08 00 00 00              	movl	$0x8, %edi
142346a37: b9 80 00 00 00              	movl	$0x80, %ecx
142346a3c: ba 08 00 00 00              	movl	$0x8, %edx
142346a41: e8 ca d1 2c fe              	callq	0x140613c10 <.text+0x612c10>
142346a46: 48 85 c0                    	testq	%rax, %rax
142346a49: 0f 84 c4 07 00 00           	je	0x142347213 <.text+0x2346213>
142346a4f: 66 c7 00 02 0c              	movw	$0xc02, (%rax)          # imm = 0xC02
142346a54: 48 8d 0d ec 7f d0 00        	leaq	0xd07fec(%rip), %rcx    # 0x14304ea47
142346a5b: 48 89 48 20                 	movq	%rcx, 0x20(%rax)
142346a5f: 48 c7 40 28 18 00 00 00     	movq	$0x18, 0x28(%rax)
142346a67: f2 0f 11 78 30              	movsd	%xmm7, 0x30(%rax)
142346a6c: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
142346a70: 66 c7 40 40 03 0c           	movw	$0xc03, 0x40(%rax)      # imm = 0xC03
142346a76: 48 8d 0d e2 7f d0 00        	leaq	0xd07fe2(%rip), %rcx    # 0x14304ea5f
142346a7d: 48 89 48 60                 	movq	%rcx, 0x60(%rax)
142346a81: 48 c7 40 68 14 00 00 00     	movq	$0x14, 0x68(%rax)
142346a89: f2 0f 11 70 70              	movsd	%xmm6, 0x70(%rax)
142346a8e: c6 40 78 00                 	movb	$0x0, 0x78(%rax)
142346a92: 41 c6 06 04                 	movb	$0x4, (%r14)
142346a96: 49 c7 46 08 02 00 00 00     	movq	$0x2, 0x8(%r14)
142346a9e: 49 89 46 10                 	movq	%rax, 0x10(%r14)
142346aa2: 49 c7 46 18 02 00 00 00     	movq	$0x2, 0x18(%r14)
142346aaa: 49 c7 46 20 01 00 00 00     	movq	$0x1, 0x20(%r14)
142346ab2: 49 c7 46 28 00 00 00 00     	movq	$0x0, 0x28(%r14)
142346aba: f2 41 0f 11 76 30           	movsd	%xmm6, 0x30(%r14)
142346ac0: 41 c6 46 38 00              	movb	$0x0, 0x38(%r14)
142346ac5: 48 8d 05 37 7f d0 00        	leaq	0xd07f37(%rip), %rax    # 0x14304ea03
142346acc: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
142346ad3: 48 c7 85 58 02 00 00 0e 00 00 00    	movq	$0xe, 0x258(%rbp)
142346ade: c6 85 60 02 00 00 09        	movb	$0x9, 0x260(%rbp)
142346ae5: 48 c7 85 68 02 00 00 01 00 00 00    	movq	$0x1, 0x268(%rbp)
142346af0: 4c 89 b5 70 02 00 00        	movq	%r14, 0x270(%rbp)
142346af7: 48 c7 85 78 02 00 00 01 00 00 00    	movq	$0x1, 0x278(%rbp)
142346b02: 80 3b 00                    	cmpb	$0x0, (%rbx)
142346b05: b8 21 00 00 00              	movl	$0x21, %eax
142346b0a: b9 0c 00 00 00              	movl	$0xc, %ecx
142346b0f: 48 0f 45 c8                 	cmovneq	%rax, %rcx
142346b13: 48 8d 05 59 7f d0 00        	leaq	0xd07f59(%rip), %rax    # 0x14304ea73
142346b1a: 48 8d 15 f0 7e d0 00        	leaq	0xd07ef0(%rip), %rdx    # 0x14304ea11
142346b21: 48 0f 45 d0                 	cmovneq	%rax, %rdx
142346b25: 48 89 95 90 01 00 00        	movq	%rdx, 0x190(%rbp)
142346b2c: 48 89 8d 98 01 00 00        	movq	%rcx, 0x198(%rbp)
142346b33: c6 85 a0 01 00 00 05        	movb	$0x5, 0x1a0(%rbp)
142346b3a: 48 c7 85 a8 01 00 00 00 00 00 00    	movq	$0x0, 0x1a8(%rbp)
142346b45: 48 c7 85 b0 01 00 00 08 00 00 00    	movq	$0x8, 0x1b0(%rbp)
142346b50: 48 c7 85 b8 01 00 00 00 00 00 00    	movq	$0x0, 0x1b8(%rbp)
142346b5b: b8 22 00 00 00              	movl	$0x22, %eax
142346b60: 41 be 0d 00 00 00           	movl	$0xd, %r14d
142346b66: 4c 0f 45 f0                 	cmovneq	%rax, %r14
142346b6a: 48 8d 05 23 7f d0 00        	leaq	0xd07f23(%rip), %rax    # 0x14304ea94
142346b71: 4c 8d 3d a5 7e d0 00        	leaq	0xd07ea5(%rip), %r15    # 0x14304ea1d
142346b78: 4c 0f 45 f8                 	cmovneq	%rax, %r15
142346b7c: 74 07                       	je	0x142346b85 <.text+0x2345b85>
142346b7e: 31 c0                       	xorl	%eax, %eax
142346b80: e9 b9 00 00 00              	jmp	0x142346c3e <.text+0x2345c3e>
142346b85: 0f b6 05 7d e0 2e 01        	movzbl	0x12ee07d(%rip), %eax   # 0x143634c09
142346b8c: b9 40 00 00 00              	movl	$0x40, %ecx
142346b91: ba 08 00 00 00              	movl	$0x8, %edx
142346b96: e8 75 d0 2c fe              	callq	0x140613c10 <.text+0x612c10>
142346b9b: 48 85 c0                    	testq	%rax, %rax
142346b9e: 0f 84 b5 07 00 00           	je	0x142347359 <.text+0x2346359>
142346ba4: 48 89 c7                    	movq	%rax, %rdi
142346ba7: 0f b6 05 5b e0 2e 01        	movzbl	0x12ee05b(%rip), %eax   # 0x143634c09
142346bae: b9 80 00 00 00              	movl	$0x80, %ecx
142346bb3: ba 08 00 00 00              	movl	$0x8, %edx
142346bb8: e8 53 d0 2c fe              	callq	0x140613c10 <.text+0x612c10>
142346bbd: 48 85 c0                    	testq	%rax, %rax
142346bc0: 0f 84 a4 07 00 00           	je	0x14234736a <.text+0x234636a>
142346bc6: 66 c7 00 02 0b              	movw	$0xb02, (%rax)          # imm = 0xB02
142346bcb: 48 8d 0d e4 7e d0 00        	leaq	0xd07ee4(%rip), %rcx    # 0x14304eab6
142346bd2: 48 89 48 20                 	movq	%rcx, 0x20(%rax)
142346bd6: 48 c7 40 28 17 00 00 00     	movq	$0x17, 0x28(%rax)
142346bde: f2 0f 11 78 30              	movsd	%xmm7, 0x30(%rax)
142346be3: c6 40 38 00                 	movb	$0x0, 0x38(%rax)
142346be7: 66 c7 40 40 03 0b           	movw	$0xb03, 0x40(%rax)      # imm = 0xB03
142346bed: 48 8d 0d d9 7e d0 00        	leaq	0xd07ed9(%rip), %rcx    # 0x14304eacd
142346bf4: 48 89 48 60                 	movq	%rcx, 0x60(%rax)
142346bf8: 48 c7 40 68 13 00 00 00     	movq	$0x13, 0x68(%rax)
142346c00: f2 0f 11 70 70              	movsd	%xmm6, 0x70(%rax)
142346c05: c6 40 78 00                 	movb	$0x0, 0x78(%rax)
142346c09: c6 07 04                    	movb	$0x4, (%rdi)
142346c0c: 48 c7 47 08 02 00 00 00     	movq	$0x2, 0x8(%rdi)
142346c14: 48 89 47 10                 	movq	%rax, 0x10(%rdi)
142346c18: 48 c7 47 18 02 00 00 00     	movq	$0x2, 0x18(%rdi)
142346c20: 48 c7 47 20 01 00 00 00     	movq	$0x1, 0x20(%rdi)
142346c28: 48 c7 47 28 00 00 00 00     	movq	$0x0, 0x28(%rdi)
142346c30: f2 0f 11 77 30              	movsd	%xmm6, 0x30(%rdi)
142346c35: c6 47 38 00                 	movb	$0x0, 0x38(%rdi)
142346c39: b8 01 00 00 00              	movl	$0x1, %eax
142346c3e: 48 8d 8e e8 00 00 00        	leaq	0xe8(%rsi), %rcx
142346c45: 48 89 8d 40 02 00 00        	movq	%rcx, 0x240(%rbp)
142346c4c: 0f 10 85 d8 00 00 00        	movups	0xd8(%rbp), %xmm0
142346c53: 0f 10 8d e8 00 00 00        	movups	0xe8(%rbp), %xmm1
142346c5a: 0f 10 95 f8 00 00 00        	movups	0xf8(%rbp), %xmm2
142346c61: 41 0f 11 54 24 20           	movups	%xmm2, 0x20(%r12)
142346c67: 41 0f 11 4c 24 10           	movups	%xmm1, 0x10(%r12)
142346c6d: 41 0f 11 04 24              	movups	%xmm0, (%r12)
142346c72: 0f 10 85 a8 00 00 00        	movups	0xa8(%rbp), %xmm0
142346c79: 0f 10 8d b8 00 00 00        	movups	0xb8(%rbp), %xmm1
142346c80: 0f 10 95 c8 00 00 00        	movups	0xc8(%rbp), %xmm2
142346c87: 41 0f 11 54 24 50           	movups	%xmm2, 0x50(%r12)
142346c8d: 41 0f 11 4c 24 40           	movups	%xmm1, 0x40(%r12)
142346c93: 41 0f 11 44 24 30           	movups	%xmm0, 0x30(%r12)
142346c99: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
142346c9d: 0f 10 8d 88 00 00 00        	movups	0x88(%rbp), %xmm1
142346ca4: 0f 10 95 98 00 00 00        	movups	0x98(%rbp), %xmm2
142346cab: 41 0f 11 94 24 80 00 00 00  	movups	%xmm2, 0x80(%r12)
142346cb4: 41 0f 11 4c 24 70           	movups	%xmm1, 0x70(%r12)
142346cba: 41 0f 11 44 24 60           	movups	%xmm0, 0x60(%r12)
142346cc0: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
142346cc4: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
142346cc8: 0f 10 55 68                 	movups	0x68(%rbp), %xmm2
142346ccc: 41 0f 11 84 24 90 00 00 00  	movups	%xmm0, 0x90(%r12)
142346cd5: 41 0f 11 8c 24 a0 00 00 00  	movups	%xmm1, 0xa0(%r12)
142346cde: 41 0f 11 94 24 b0 00 00 00  	movups	%xmm2, 0xb0(%r12)
142346ce7: 0f 10 85 08 01 00 00        	movups	0x108(%rbp), %xmm0
142346cee: 0f 10 8d 18 01 00 00        	movups	0x118(%rbp), %xmm1
142346cf5: 0f 10 95 28 01 00 00        	movups	0x128(%rbp), %xmm2
142346cfc: 41 0f 11 8c 24 d0 00 00 00  	movups	%xmm1, 0xd0(%r12)
142346d05: 41 0f 11 94 24 e0 00 00 00  	movups	%xmm2, 0xe0(%r12)
142346d0e: 41 0f 11 84 24 c0 00 00 00  	movups	%xmm0, 0xc0(%r12)
142346d17: 0f 10 85 48 01 00 00        	movups	0x148(%rbp), %xmm0
142346d1e: 0f 10 8d 58 01 00 00        	movups	0x158(%rbp), %xmm1
142346d25: 0f 10 95 68 01 00 00        	movups	0x168(%rbp), %xmm2
142346d2c: 41 0f 11 84 24 f0 00 00 00  	movups	%xmm0, 0xf0(%r12)
142346d35: 41 0f 11 8c 24 00 01 00 00  	movups	%xmm1, 0x100(%r12)
142346d3e: 41 0f 11 94 24 10 01 00 00  	movups	%xmm2, 0x110(%r12)
142346d47: 0f 10 85 50 02 00 00        	movups	0x250(%rbp), %xmm0
142346d4e: 0f 10 8d 60 02 00 00        	movups	0x260(%rbp), %xmm1
142346d55: 0f 10 95 70 02 00 00        	movups	0x270(%rbp), %xmm2
142346d5c: 41 0f 11 84 24 20 01 00 00  	movups	%xmm0, 0x120(%r12)
142346d65: 41 0f 11 8c 24 30 01 00 00  	movups	%xmm1, 0x130(%r12)
142346d6e: 41 0f 11 94 24 40 01 00 00  	movups	%xmm2, 0x140(%r12)
142346d77: 0f 10 85 90 01 00 00        	movups	0x190(%rbp), %xmm0
142346d7e: 0f 10 8d a0 01 00 00        	movups	0x1a0(%rbp), %xmm1
142346d85: 0f 10 95 b0 01 00 00        	movups	0x1b0(%rbp), %xmm2
142346d8c: 41 0f 11 84 24 50 01 00 00  	movups	%xmm0, 0x150(%r12)
142346d95: 41 0f 11 94 24 70 01 00 00  	movups	%xmm2, 0x170(%r12)
142346d9e: 41 0f 11 8c 24 60 01 00 00  	movups	%xmm1, 0x160(%r12)
142346da7: 4d 89 bc 24 80 01 00 00     	movq	%r15, 0x180(%r12)
142346daf: 4d 89 b4 24 88 01 00 00     	movq	%r14, 0x188(%r12)
142346db7: 41 c6 84 24 90 01 00 00 08  	movb	$0x8, 0x190(%r12)
142346dc0: 49 89 84 24 98 01 00 00     	movq	%rax, 0x198(%r12)
142346dc8: 49 89 bc 24 a0 01 00 00     	movq	%rdi, 0x1a0(%r12)
142346dd0: 49 89 84 24 a8 01 00 00     	movq	%rax, 0x1a8(%r12)
142346dd8: 48 8d 05 01 7d d0 00        	leaq	0xd07d01(%rip), %rax    # 0x14304eae0
142346ddf: 49 89 84 24 b0 01 00 00     	movq	%rax, 0x1b0(%r12)
142346de7: 49 c7 84 24 b8 01 00 00 11 00 00 00 	movq	$0x11, 0x1b8(%r12)
142346df3: 41 c6 84 24 c0 01 00 00 0a  	movb	$0xa, 0x1c0(%r12)
142346dfc: 49 c7 84 24 c8 01 00 00 00 00 00 00 	movq	$0x0, 0x1c8(%r12)
142346e08: 49 c7 84 24 d0 01 00 00 08 00 00 00 	movq	$0x8, 0x1d0(%r12)
142346e14: 49 c7 84 24 d8 01 00 00 00 00 00 00 	movq	$0x0, 0x1d8(%r12)
142346e20: 48 8d 05 ca 7c d0 00        	leaq	0xd07cca(%rip), %rax    # 0x14304eaf1
142346e27: 49 89 84 24 e0 01 00 00     	movq	%rax, 0x1e0(%r12)
142346e2f: 49 c7 84 24 e8 01 00 00 13 00 00 00 	movq	$0x13, 0x1e8(%r12)
142346e3b: 41 c6 84 24 f0 01 00 00 0b  	movb	$0xb, 0x1f0(%r12)
142346e44: 49 c7 84 24 f8 01 00 00 00 00 00 00 	movq	$0x0, 0x1f8(%r12)
142346e50: 49 c7 84 24 00 02 00 00 08 00 00 00 	movq	$0x8, 0x200(%r12)
142346e5c: 49 c7 84 24 08 02 00 00 00 00 00 00 	movq	$0x0, 0x208(%r12)
142346e68: 4c 89 e0                    	movq	%r12, %rax
142346e6b: 48 05 10 02 00 00           	addq	$0x210, %rax            # imm = 0x210
142346e71: 4c 89 a5 90 01 00 00        	movq	%r12, 0x190(%rbp)
142346e78: 4c 89 a5 98 01 00 00        	movq	%r12, 0x198(%rbp)
142346e7f: 48 c7 85 a0 01 00 00 0b 00 00 00    	movq	$0xb, 0x1a0(%rbp)
142346e8a: 48 89 85 a8 01 00 00        	movq	%rax, 0x1a8(%rbp)
142346e91: 48 8d bd 50 01 00 00        	leaq	0x150(%rbp), %rdi
142346e98: 48 81 c6 f0 00 00 00        	addq	$0xf0, %rsi
142346e9f: 48 8d 9d 08 01 00 00        	leaq	0x108(%rbp), %rbx
142346ea6: 4c 8d 75 18                 	leaq	0x18(%rbp), %r14
142346eaa: 4c 8d bd 50 02 00 00        	leaq	0x250(%rbp), %r15
142346eb1: 49 39 c4                    	cmpq	%rax, %r12
142346eb4: 0f 84 e4 00 00 00           	je	0x142346f9e <.text+0x2345f9e>
142346eba: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
142346ec0: 49 8d 44 24 30              	leaq	0x30(%r12), %rax
142346ec5: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
142346ecc: 49 8b 44 24 18              	movq	0x18(%r12), %rax
142346ed1: 48 89 c1                    	movq	%rax, %rcx
142346ed4: 48 f7 d9                    	negq	%rcx
142346ed7: 0f 80 c1 00 00 00           	jo	0x142346f9e <.text+0x2345f9e>
142346edd: 4c 89 e2                    	movq	%r12, %rdx
142346ee0: 45 8a 64 24 10              	movb	0x10(%r12), %r12b
142346ee5: 4c 8b 0a                    	movq	(%rdx), %r9
142346ee8: 48 8b 4a 08                 	movq	0x8(%rdx), %rcx
142346eec: 44 88 a5 08 01 00 00        	movb	%r12b, 0x108(%rbp)
142346ef3: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
142346efa: 0f 10 42 20                 	movups	0x20(%rdx), %xmm0
142346efe: 0f 11 07                    	movups	%xmm0, (%rdi)
142346f01: c6 85 a5 02 00 00 01        	movb	$0x1, 0x2a5(%rbp)
142346f08: 48 89 5c 24 30              	movq	%rbx, 0x30(%rsp)
142346f0d: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
142346f14: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
142346f19: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
142346f1e: c7 44 24 38 00 00 00 00     	movl	$0x0, 0x38(%rsp)
142346f26: 4c 89 f1                    	movq	%r14, %rcx
142346f29: 4c 89 ea                    	movq	%r13, %rdx
142346f2c: 45 89 e0                    	movl	%r12d, %r8d
142346f2f: e8 9c c6 2d 00              	callq	0x1426235d0 <.text+0x26225d0>
142346f34: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
142346f3b: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
142346f42: 0f 10 85 48 01 00 00        	movups	0x148(%rbp), %xmm0
142346f49: 0f 29 85 50 02 00 00        	movaps	%xmm0, 0x250(%rbp)
142346f50: c6 85 a3 02 00 00 01        	movb	$0x1, 0x2a3(%rbp)
142346f57: 48 89 f1                    	movq	%rsi, %rcx
142346f5a: e8 11 71 7c fe              	callq	0x140b0e070 <.text+0xb0d070>
142346f5f: c6 85 a3 02 00 00 00        	movb	$0x0, 0x2a3(%rbp)
142346f66: 48 8b 8d 98 02 00 00        	movq	0x298(%rbp), %rcx
142346f6d: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
142346f72: 88 44 24 20                 	movb	%al, 0x20(%rsp)
142346f76: 4c 89 f1                    	movq	%r14, %rcx
142346f79: 4c 89 ea                    	movq	%r13, %rdx
142346f7c: 45 89 e0                    	movl	%r12d, %r8d
142346f7f: 4d 89 f9                    	movq	%r15, %r9
142346f82: e8 59 62 b0 ff              	callq	0x141e4d1e0 <.text+0x1e4c1e0>
142346f87: 4c 8b a5 98 01 00 00        	movq	0x198(%rbp), %r12
142346f8e: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
142346f95: 49 39 c4                    	cmpq	%rax, %r12
142346f98: 0f 85 22 ff ff ff           	jne	0x142346ec0 <.text+0x2345ec0>
142346f9e: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
142346fa5: e8 c6 c6 c5 fe              	callq	0x140fa3670 <.text+0xfa2670>
142346faa: 4c 8b b5 98 02 00 00        	movq	0x298(%rbp), %r14
142346fb1: 48 89 f1                    	movq	%rsi, %rcx
142346fb4: e8 b7 70 7c fe              	callq	0x140b0e070 <.text+0xb0d070>
142346fb9: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
142346fbd: 89 c2                       	movl	%eax, %edx
142346fbf: 4d 89 f0                    	movq	%r14, %r8
142346fc2: e8 b9 68 b0 ff              	callq	0x141e4d880 <.text+0x1e4c880>
142346fc7: 48 8b 45 18                 	movq	0x18(%rbp), %rax
142346fcb: 48 89 85 48 02 00 00        	movq	%rax, 0x248(%rbp)
142346fd2: 48 8b 45 20                 	movq	0x20(%rbp), %rax
142346fd6: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
142346fdd: 0f 10 45 28                 	movups	0x28(%rbp), %xmm0
142346fe1: 0f 29 85 50 02 00 00        	movaps	%xmm0, 0x250(%rbp)
142346fe8: 0f 10 45 38                 	movups	0x38(%rbp), %xmm0
142346fec: 0f 29 85 60 02 00 00        	movaps	%xmm0, 0x260(%rbp)
142346ff3: 8b bd 8c 01 00 00           	movl	0x18c(%rbp), %edi
142346ff9: 48 8b 9d 38 01 00 00        	movq	0x138(%rbp), %rbx
142347000: 89 3b                       	movl	%edi, (%rbx)
142347002: c6 85 a4 02 00 00 01        	movb	$0x1, 0x2a4(%rbp)
142347009: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
14234700d: 4c 89 ea                    	movq	%r13, %rdx
142347010: 4c 8b 85 40 02 00 00        	movq	0x240(%rbp), %r8
142347017: 4d 89 f1                    	movq	%r14, %r9
14234701a: e8 c1 d6 2d 00              	callq	0x1426246e0 <.text+0x26236e0>
14234701f: 89 3b                       	movl	%edi, (%rbx)
142347021: c6 85 a7 02 00 00 01        	movb	$0x1, 0x2a7(%rbp)
142347028: c6 85 a6 02 00 00 01        	movb	$0x1, 0x2a6(%rbp)
14234702f: 48 8d 4d 04                 	leaq	0x4(%rbp), %rcx
142347033: 4c 89 ea                    	movq	%r13, %rdx
142347036: 4d 89 f0                    	movq	%r14, %r8
142347039: e8 32 68 c6 fe              	callq	0x140fad870 <.text+0xfac870>
14234703e: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
142347045: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
14234704c: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
142347053: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
14234705a: 0f 28 85 50 02 00 00        	movaps	0x250(%rbp), %xmm0
142347061: 0f 28 8d 60 02 00 00        	movaps	0x260(%rbp), %xmm1
142347068: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
14234706f: 0f 11 8d b0 01 00 00        	movups	%xmm1, 0x1b0(%rbp)
142347076: c6 85 a7 02 00 00 00        	movb	$0x0, 0x2a7(%rbp)
14234707d: c6 85 a6 02 00 00 01        	movb	$0x1, 0x2a6(%rbp)
142347084: 4c 8d 05 cd 76 d0 00        	leaq	0xd076cd(%rip), %r8     # 0x14304e758
14234708b: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
142347092: 4c 89 f1                    	movq	%r14, %rcx
142347095: e8 66 92 c9 fe              	callq	0x140fe0300 <.text+0xfdf300>
14234709a: c6 85 a7 02 00 00 00        	movb	$0x0, 0x2a7(%rbp)
1423470a1: c6 85 a6 02 00 00 00        	movb	$0x0, 0x2a6(%rbp)
1423470a8: 4c 8d 05 c1 76 d0 00        	leaq	0xd076c1(%rip), %r8     # 0x14304e770
1423470af: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
1423470b3: 4c 89 f1                    	movq	%r14, %rcx
1423470b6: e8 f5 8d c9 fe              	callq	0x140fdfeb0 <.text+0xfdeeb0>
1423470bb: 49 8b 7e 08                 	movq	0x8(%r14), %rdi
1423470bf: 48 85 ff                    	testq	%rdi, %rdi
1423470c2: 75 04                       	jne	0x1423470c8 <.text+0x23460c8>
1423470c4: 49 8b 7e 10                 	movq	0x10(%r14), %rdi
1423470c8: 48 8b 07                    	movq	(%rdi), %rax
1423470cb: 48 8b 77 10                 	movq	0x10(%rdi), %rsi
1423470cf: 48 29 f0                    	subq	%rsi, %rax
1423470d2: 48 83 f8 1b                 	cmpq	$0x1b, %rax
1423470d6: 76 49                       	jbe	0x142347121 <.text+0x2346121>
1423470d8: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1423470dc: 48 8d 0d fd 85 c9 fe        	leaq	-0x1367a03(%rip), %rcx  # 0x140fdf6e0 <.text+0xfde6e0>
1423470e3: 48 89 0c 30                 	movq	%rcx, (%rax,%rsi)
1423470e7: 0f 10 45 04                 	movups	0x4(%rbp), %xmm0
1423470eb: 0f 11 44 30 08              	movups	%xmm0, 0x8(%rax,%rsi)
1423470f0: 8b 4d 14                    	movl	0x14(%rbp), %ecx
1423470f3: 89 4c 30 18                 	movl	%ecx, 0x18(%rax,%rsi)
1423470f7: 48 83 c6 1c                 	addq	$0x1c, %rsi
1423470fb: 48 89 77 10                 	movq	%rsi, 0x10(%rdi)
1423470ff: 0f 28 b5 b0 02 00 00        	movaps	0x2b0(%rbp), %xmm6
142347106: 0f 28 bd c0 02 00 00        	movaps	0x2c0(%rbp), %xmm7
14234710d: 48 81 c4 58 03 00 00        	addq	$0x358, %rsp            # imm = 0x358
142347114: 5b                          	popq	%rbx
142347115: 5f                          	popq	%rdi
142347116: 5e                          	popq	%rsi
142347117: 41 5c                       	popq	%r12
142347119: 41 5d                       	popq	%r13
14234711b: 41 5e                       	popq	%r14
14234711d: 41 5f                       	popq	%r15
14234711f: 5d                          	popq	%rbp
142347120: c3                          	retq
142347121: c6 85 a7 02 00 00 00        	movb	$0x0, 0x2a7(%rbp)
142347128: c6 85 a6 02 00 00 00        	movb	$0x0, 0x2a6(%rbp)
14234712f: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
142347138: 41 b8 1c 00 00 00           	movl	$0x1c, %r8d
14234713e: 41 b9 01 00 00 00           	movl	$0x1, %r9d
142347144: 48 89 f9                    	movq	%rdi, %rcx
142347147: 48 89 f2                    	movq	%rsi, %rdx
14234714a: e8 81 e7 55 00              	callq	0x1428a58d0 <.text+0x28a48d0>
14234714f: eb 87                       	jmp	0x1423470d8 <.text+0x23460d8>
142347151: b9 08 00 00 00              	movl	$0x8, %ecx
142347156: ba 10 02 00 00              	movl	$0x210, %edx            # imm = 0x210
14234715b: e8 a0 1e 59 00              	callq	0x1428d9000 <.text+0x28d8000>
142347160: e9 1b 02 00 00              	jmp	0x142347380 <.text+0x2346380>
142347165: b9 08 00 00 00              	movl	$0x8, %ecx
14234716a: ba c0 00 00 00              	movl	$0xc0, %edx
14234716f: e8 8c 1e 59 00              	callq	0x1428d9000 <.text+0x28d8000>
142347174: e9 07 02 00 00              	jmp	0x142347380 <.text+0x2346380>
142347179: b9 08 00 00 00              	movl	$0x8, %ecx
14234717e: ba 80 00 00 00              	movl	$0x80, %edx
142347183: e8 78 1e 59 00              	callq	0x1428d9000 <.text+0x28d8000>
142347188: e9 f3 01 00 00              	jmp	0x142347380 <.text+0x2346380>
14234718d: b9 08 00 00 00              	movl	$0x8, %ecx
142347192: ba 40 00 00 00              	movl	$0x40, %edx
142347197: e8 64 1e 59 00              	callq	0x1428d9000 <.text+0x28d8000>
14234719c: e9 df 01 00 00              	jmp	0x142347380 <.text+0x2346380>
1423471a1: 48 89 bd 98 02 00 00        	movq	%rdi, 0x298(%rbp)
1423471a8: b9 08 00 00 00              	movl	$0x8, %ecx
1423471ad: ba 80 00 00 00              	movl	$0x80, %edx
1423471b2: e8 49 1e 59 00              	callq	0x1428d9000 <.text+0x28d8000>
1423471b7: e9 c4 01 00 00              	jmp	0x142347380 <.text+0x2346380>
1423471bc: b9 08 00 00 00              	movl	$0x8, %ecx
1423471c1: ba 80 00 00 00              	movl	$0x80, %edx
1423471c6: e8 35 1e 59 00              	callq	0x1428d9000 <.text+0x28d8000>
1423471cb: e9 b0 01 00 00              	jmp	0x142347380 <.text+0x2346380>
1423471d0: b9 08 00 00 00              	movl	$0x8, %ecx
1423471d5: ba 40 00 00 00              	movl	$0x40, %edx
1423471da: e8 21 1e 59 00              	callq	0x1428d9000 <.text+0x28d8000>
1423471df: e9 9c 01 00 00              	jmp	0x142347380 <.text+0x2346380>
1423471e4: 48 89 bd 98 02 00 00        	movq	%rdi, 0x298(%rbp)
1423471eb: b9 08 00 00 00              	movl	$0x8, %ecx
1423471f0: ba 80 00 00 00              	movl	$0x80, %edx
1423471f5: e8 06 1e 59 00              	callq	0x1428d9000 <.text+0x28d8000>
1423471fa: e9 81 01 00 00              	jmp	0x142347380 <.text+0x2346380>
1423471ff: b9 08 00 00 00              	movl	$0x8, %ecx
142347204: ba 40 00 00 00              	movl	$0x40, %edx
142347209: e8 f2 1d 59 00              	callq	0x1428d9000 <.text+0x28d8000>
14234720e: e9 6d 01 00 00              	jmp	0x142347380 <.text+0x2346380>
142347213: 4c 89 b5 98 02 00 00        	movq	%r14, 0x298(%rbp)
14234721a: b9 08 00 00 00              	movl	$0x8, %ecx
14234721f: ba 80 00 00 00              	movl	$0x80, %edx
142347224: e8 d7 1d 59 00              	callq	0x1428d9000 <.text+0x28d8000>
142347229: e9 52 01 00 00              	jmp	0x142347380 <.text+0x2346380>
14234722e: b9 08 00 00 00              	movl	$0x8, %ecx
142347233: ba c0 00 00 00              	movl	$0xc0, %edx
142347238: e8 c3 1d 59 00              	callq	0x1428d9000 <.text+0x28d8000>
14234723d: e9 3e 01 00 00              	jmp	0x142347380 <.text+0x2346380>
142347242: b9 08 00 00 00              	movl	$0x8, %ecx
142347247: ba 00 01 00 00              	movl	$0x100, %edx            # imm = 0x100
14234724c: e8 af 1d 59 00              	callq	0x1428d9000 <.text+0x28d8000>
142347251: e9 2a 01 00 00              	jmp	0x142347380 <.text+0x2346380>
142347256: b9 08 00 00 00              	movl	$0x8, %ecx
14234725b: ba 40 00 00 00              	movl	$0x40, %edx
142347260: e8 9b 1d 59 00              	callq	0x1428d9000 <.text+0x28d8000>
142347265: e9 16 01 00 00              	jmp	0x142347380 <.text+0x2346380>
14234726a: b9 08 00 00 00              	movl	$0x8, %ecx
14234726f: ba 40 00 00 00              	movl	$0x40, %edx
142347274: e8 87 1d 59 00              	callq	0x1428d9000 <.text+0x28d8000>
142347279: e9 02 01 00 00              	jmp	0x142347380 <.text+0x2346380>
14234727e: 48 89 bd 98 02 00 00        	movq	%rdi, 0x298(%rbp)
142347285: b9 08 00 00 00              	movl	$0x8, %ecx
14234728a: ba 80 00 00 00              	movl	$0x80, %edx
14234728f: e8 6c 1d 59 00              	callq	0x1428d9000 <.text+0x28d8000>
142347294: e9 e7 00 00 00              	jmp	0x142347380 <.text+0x2346380>
142347299: b9 08 00 00 00              	movl	$0x8, %ecx
14234729e: ba c0 00 00 00              	movl	$0xc0, %edx
1423472a3: e8 58 1d 59 00              	callq	0x1428d9000 <.text+0x28d8000>
1423472a8: e9 d3 00 00 00              	jmp	0x142347380 <.text+0x2346380>
1423472ad: b9 08 00 00 00              	movl	$0x8, %ecx
1423472b2: ba c0 00 00 00              	movl	$0xc0, %edx
1423472b7: e8 44 1d 59 00              	callq	0x1428d9000 <.text+0x28d8000>
1423472bc: e9 bf 00 00 00              	jmp	0x142347380 <.text+0x2346380>
1423472c1: b9 08 00 00 00              	movl	$0x8, %ecx
1423472c6: ba c0 00 00 00              	movl	$0xc0, %edx
1423472cb: e8 30 1d 59 00              	callq	0x1428d9000 <.text+0x28d8000>
1423472d0: e9 ab 00 00 00              	jmp	0x142347380 <.text+0x2346380>
1423472d5: b9 08 00 00 00              	movl	$0x8, %ecx
1423472da: ba c0 00 00 00              	movl	$0xc0, %edx
1423472df: e8 1c 1d 59 00              	callq	0x1428d9000 <.text+0x28d8000>
1423472e4: e9 97 00 00 00              	jmp	0x142347380 <.text+0x2346380>
1423472e9: b9 08 00 00 00              	movl	$0x8, %ecx
1423472ee: ba 00 01 00 00              	movl	$0x100, %edx            # imm = 0x100
1423472f3: e8 08 1d 59 00              	callq	0x1428d9000 <.text+0x28d8000>
1423472f8: e9 83 00 00 00              	jmp	0x142347380 <.text+0x2346380>
1423472fd: b9 08 00 00 00              	movl	$0x8, %ecx
142347302: ba 40 00 00 00              	movl	$0x40, %edx
142347307: e8 f4 1c 59 00              	callq	0x1428d9000 <.text+0x28d8000>
14234730c: eb 72                       	jmp	0x142347380 <.text+0x2346380>
14234730e: b9 08 00 00 00              	movl	$0x8, %ecx
142347313: ba c0 00 00 00              	movl	$0xc0, %edx
142347318: e8 e3 1c 59 00              	callq	0x1428d9000 <.text+0x28d8000>
14234731d: eb 61                       	jmp	0x142347380 <.text+0x2346380>
14234731f: b9 08 00 00 00              	movl	$0x8, %ecx
142347324: ba c0 00 00 00              	movl	$0xc0, %edx
142347329: e8 d2 1c 59 00              	callq	0x1428d9000 <.text+0x28d8000>
14234732e: eb 50                       	jmp	0x142347380 <.text+0x2346380>
142347330: b9 08 00 00 00              	movl	$0x8, %ecx
142347335: ba 40 00 00 00              	movl	$0x40, %edx
14234733a: e8 c1 1c 59 00              	callq	0x1428d9000 <.text+0x28d8000>
14234733f: eb 3f                       	jmp	0x142347380 <.text+0x2346380>
142347341: 48 89 bd 98 02 00 00        	movq	%rdi, 0x298(%rbp)
142347348: b9 08 00 00 00              	movl	$0x8, %ecx
14234734d: ba c0 00 00 00              	movl	$0xc0, %edx
142347352: e8 a9 1c 59 00              	callq	0x1428d9000 <.text+0x28d8000>
142347357: eb 27                       	jmp	0x142347380 <.text+0x2346380>
142347359: b9 08 00 00 00              	movl	$0x8, %ecx
14234735e: ba 40 00 00 00              	movl	$0x40, %edx
142347363: e8 98 1c 59 00              	callq	0x1428d9000 <.text+0x28d8000>
142347368: eb 16                       	jmp	0x142347380 <.text+0x2346380>
14234736a: 48 89 bd 98 02 00 00        	movq	%rdi, 0x298(%rbp)
142347371: b9 08 00 00 00              	movl	$0x8, %ecx
142347376: ba 80 00 00 00              	movl	$0x80, %edx
14234737b: e8 80 1c 59 00              	callq	0x1428d9000 <.text+0x28d8000>
142347380: 0f 0b                       	ud2
142347382: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
142347390: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347395: 55                          	pushq	%rbp
142347396: 41 57                       	pushq	%r15
142347398: 41 56                       	pushq	%r14
14234739a: 41 55                       	pushq	%r13
14234739c: 41 54                       	pushq	%r12
14234739e: 56                          	pushq	%rsi
14234739f: 57                          	pushq	%rdi
1423473a0: 53                          	pushq	%rbx
1423473a1: 48 83 ec 68                 	subq	$0x68, %rsp
1423473a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1423473ac: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1423473b1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1423473b6: 48 8b 85 40 02 00 00        	movq	0x240(%rbp), %rax
1423473bd: 48 89 c6                    	movq	%rax, %rsi
1423473c0: 48 8b 80 98 00 00 00        	movq	0x98(%rax), %rax
1423473c7: 48 85 c0                    	testq	%rax, %rax
1423473ca: 74 1a                       	je	0x1423473e6 <.text+0x23463e6>
1423473cc: 48 8b 8e a0 00 00 00        	movq	0xa0(%rsi), %rcx
1423473d3: 48 c1 e0 03                 	shlq	$0x3, %rax
1423473d7: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1423473db: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1423473e1: e8 3a c8 2c fe              	callq	0x140613c20 <.text+0x612c20>
1423473e6: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1423473ed: 41 b8 b0 00 00 00           	movl	$0xb0, %r8d
1423473f3: 48 89 f1                    	movq	%rsi, %rcx
1423473f6: e8 05 c8 5d 00              	callq	0x142923c00 <.text+0x2922c00>
1423473fb: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347400: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347405: 48 83 c4 68                 	addq	$0x68, %rsp
142347409: 5b                          	popq	%rbx
14234740a: 5f                          	popq	%rdi
14234740b: 5e                          	popq	%rsi
14234740c: 41 5c                       	popq	%r12
14234740e: 41 5d                       	popq	%r13
142347410: 41 5e                       	popq	%r14
142347412: 41 5f                       	popq	%r15
142347414: 5d                          	popq	%rbp
142347415: c3                          	retq
142347416: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
142347420: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347425: 55                          	pushq	%rbp
142347426: 41 57                       	pushq	%r15
142347428: 41 56                       	pushq	%r14
14234742a: 41 55                       	pushq	%r13
14234742c: 41 54                       	pushq	%r12
14234742e: 56                          	pushq	%rsi
14234742f: 57                          	pushq	%rdi
142347430: 53                          	pushq	%rbx
142347431: 48 83 ec 68                 	subq	$0x68, %rsp
142347435: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234743c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347441: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347446: 48 8b 55 18                 	movq	0x18(%rbp), %rdx
14234744a: 48 85 d2                    	testq	%rdx, %rdx
14234744d: 74 13                       	je	0x142347462 <.text+0x2346462>
14234744f: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
142347453: 48 c1 e2 05                 	shlq	$0x5, %rdx
142347457: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14234745d: e8 be c7 2c fe              	callq	0x140613c20 <.text+0x612c20>
142347462: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347467: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14234746c: 48 83 c4 68                 	addq	$0x68, %rsp
142347470: 5b                          	popq	%rbx
142347471: 5f                          	popq	%rdi
142347472: 5e                          	popq	%rsi
142347473: 41 5c                       	popq	%r12
142347475: 41 5d                       	popq	%r13
142347477: 41 5e                       	popq	%r14
142347479: 41 5f                       	popq	%r15
14234747b: 5d                          	popq	%rbp
14234747c: c3                          	retq
14234747d: 0f 1f 00                    	nopl	(%rax)
142347480: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347485: 55                          	pushq	%rbp
142347486: 41 57                       	pushq	%r15
142347488: 41 56                       	pushq	%r14
14234748a: 41 55                       	pushq	%r13
14234748c: 41 54                       	pushq	%r12
14234748e: 56                          	pushq	%rsi
14234748f: 57                          	pushq	%rdi
142347490: 53                          	pushq	%rbx
142347491: 48 83 ec 68                 	subq	$0x68, %rsp
142347495: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234749c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1423474a1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1423474a6: ba 10 02 00 00              	movl	$0x210, %edx            # imm = 0x210
1423474ab: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1423474b1: 48 8b 8d 48 02 00 00        	movq	0x248(%rbp), %rcx
1423474b8: e8 63 c7 2c fe              	callq	0x140613c20 <.text+0x612c20>
1423474bd: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1423474c2: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1423474c7: 48 83 c4 68                 	addq	$0x68, %rsp
1423474cb: 5b                          	popq	%rbx
1423474cc: 5f                          	popq	%rdi
1423474cd: 5e                          	popq	%rsi
1423474ce: 41 5c                       	popq	%r12
1423474d0: 41 5d                       	popq	%r13
1423474d2: 41 5e                       	popq	%r14
1423474d4: 41 5f                       	popq	%r15
1423474d6: 5d                          	popq	%rbp
1423474d7: c3                          	retq
1423474d8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1423474e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1423474e5: 55                          	pushq	%rbp
1423474e6: 41 57                       	pushq	%r15
1423474e8: 41 56                       	pushq	%r14
1423474ea: 41 55                       	pushq	%r13
1423474ec: 41 54                       	pushq	%r12
1423474ee: 56                          	pushq	%rsi
1423474ef: 57                          	pushq	%rdi
1423474f0: 53                          	pushq	%rbx
1423474f1: 48 83 ec 68                 	subq	$0x68, %rsp
1423474f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1423474fc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347501: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347506: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
14234750d: e8 ce 31 c6 fe              	callq	0x140faa6e0 <.text+0xfa96e0>
142347512: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347517: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14234751c: 48 83 c4 68                 	addq	$0x68, %rsp
142347520: 5b                          	popq	%rbx
142347521: 5f                          	popq	%rdi
142347522: 5e                          	popq	%rsi
142347523: 41 5c                       	popq	%r12
142347525: 41 5d                       	popq	%r13
142347527: 41 5e                       	popq	%r14
142347529: 41 5f                       	popq	%r15
14234752b: 5d                          	popq	%rbp
14234752c: c3                          	retq
14234752d: 0f 1f 00                    	nopl	(%rax)
142347530: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347535: 55                          	pushq	%rbp
142347536: 41 57                       	pushq	%r15
142347538: 41 56                       	pushq	%r14
14234753a: 41 55                       	pushq	%r13
14234753c: 41 54                       	pushq	%r12
14234753e: 56                          	pushq	%rsi
14234753f: 57                          	pushq	%rdi
142347540: 53                          	pushq	%rbx
142347541: 48 83 ec 68                 	subq	$0x68, %rsp
142347545: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234754c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347551: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347556: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
14234755d: e8 7e 31 c6 fe              	callq	0x140faa6e0 <.text+0xfa96e0>
142347562: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347567: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14234756c: 48 83 c4 68                 	addq	$0x68, %rsp
142347570: 5b                          	popq	%rbx
142347571: 5f                          	popq	%rdi
142347572: 5e                          	popq	%rsi
142347573: 41 5c                       	popq	%r12
142347575: 41 5d                       	popq	%r13
142347577: 41 5e                       	popq	%r14
142347579: 41 5f                       	popq	%r15
14234757b: 5d                          	popq	%rbp
14234757c: c3                          	retq
14234757d: 0f 1f 00                    	nopl	(%rax)
142347580: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347585: 55                          	pushq	%rbp
142347586: 41 57                       	pushq	%r15
142347588: 41 56                       	pushq	%r14
14234758a: 41 55                       	pushq	%r13
14234758c: 41 54                       	pushq	%r12
14234758e: 56                          	pushq	%rsi
14234758f: 57                          	pushq	%rdi
142347590: 53                          	pushq	%rbx
142347591: 48 83 ec 68                 	subq	$0x68, %rsp
142347595: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234759c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1423475a1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1423475a6: ba 40 00 00 00              	movl	$0x40, %edx
1423475ab: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1423475b1: 48 8b 8d 98 02 00 00        	movq	0x298(%rbp), %rcx
1423475b8: e8 63 c6 2c fe              	callq	0x140613c20 <.text+0x612c20>
1423475bd: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1423475c2: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1423475c7: 48 83 c4 68                 	addq	$0x68, %rsp
1423475cb: 5b                          	popq	%rbx
1423475cc: 5f                          	popq	%rdi
1423475cd: 5e                          	popq	%rsi
1423475ce: 41 5c                       	popq	%r12
1423475d0: 41 5d                       	popq	%r13
1423475d2: 41 5e                       	popq	%r14
1423475d4: 41 5f                       	popq	%r15
1423475d6: 5d                          	popq	%rbp
1423475d7: c3                          	retq
1423475d8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1423475e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1423475e5: 55                          	pushq	%rbp
1423475e6: 41 57                       	pushq	%r15
1423475e8: 41 56                       	pushq	%r14
1423475ea: 41 55                       	pushq	%r13
1423475ec: 41 54                       	pushq	%r12
1423475ee: 56                          	pushq	%rsi
1423475ef: 57                          	pushq	%rdi
1423475f0: 53                          	pushq	%rbx
1423475f1: 48 83 ec 68                 	subq	$0x68, %rsp
1423475f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1423475fc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347601: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347606: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14234760d: e8 de a3 c5 fe              	callq	0x140fa19f0 <.text+0xfa09f0>
142347612: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347617: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14234761c: 48 83 c4 68                 	addq	$0x68, %rsp
142347620: 5b                          	popq	%rbx
142347621: 5f                          	popq	%rdi
142347622: 5e                          	popq	%rsi
142347623: 41 5c                       	popq	%r12
142347625: 41 5d                       	popq	%r13
142347627: 41 5e                       	popq	%r14
142347629: 41 5f                       	popq	%r15
14234762b: 5d                          	popq	%rbp
14234762c: c3                          	retq
14234762d: 0f 1f 00                    	nopl	(%rax)
142347630: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347635: 55                          	pushq	%rbp
142347636: 41 57                       	pushq	%r15
142347638: 41 56                       	pushq	%r14
14234763a: 41 55                       	pushq	%r13
14234763c: 41 54                       	pushq	%r12
14234763e: 56                          	pushq	%rsi
14234763f: 57                          	pushq	%rdi
142347640: 53                          	pushq	%rbx
142347641: 48 83 ec 68                 	subq	$0x68, %rsp
142347645: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234764c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347651: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347656: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
14234765d: e8 7e 30 c6 fe              	callq	0x140faa6e0 <.text+0xfa96e0>
142347662: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347667: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14234766c: 48 83 c4 68                 	addq	$0x68, %rsp
142347670: 5b                          	popq	%rbx
142347671: 5f                          	popq	%rdi
142347672: 5e                          	popq	%rsi
142347673: 41 5c                       	popq	%r12
142347675: 41 5d                       	popq	%r13
142347677: 41 5e                       	popq	%r14
142347679: 41 5f                       	popq	%r15
14234767b: 5d                          	popq	%rbp
14234767c: c3                          	retq
14234767d: 0f 1f 00                    	nopl	(%rax)
142347680: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347685: 55                          	pushq	%rbp
142347686: 41 57                       	pushq	%r15
142347688: 41 56                       	pushq	%r14
14234768a: 41 55                       	pushq	%r13
14234768c: 41 54                       	pushq	%r12
14234768e: 56                          	pushq	%rsi
14234768f: 57                          	pushq	%rdi
142347690: 53                          	pushq	%rbx
142347691: 48 83 ec 68                 	subq	$0x68, %rsp
142347695: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234769c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1423476a1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1423476a6: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1423476ad: e8 2e 30 c6 fe              	callq	0x140faa6e0 <.text+0xfa96e0>
1423476b2: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1423476b7: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1423476bc: 48 83 c4 68                 	addq	$0x68, %rsp
1423476c0: 5b                          	popq	%rbx
1423476c1: 5f                          	popq	%rdi
1423476c2: 5e                          	popq	%rsi
1423476c3: 41 5c                       	popq	%r12
1423476c5: 41 5d                       	popq	%r13
1423476c7: 41 5e                       	popq	%r14
1423476c9: 41 5f                       	popq	%r15
1423476cb: 5d                          	popq	%rbp
1423476cc: c3                          	retq
1423476cd: 0f 1f 00                    	nopl	(%rax)
1423476d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1423476d5: 55                          	pushq	%rbp
1423476d6: 41 57                       	pushq	%r15
1423476d8: 41 56                       	pushq	%r14
1423476da: 41 55                       	pushq	%r13
1423476dc: 41 54                       	pushq	%r12
1423476de: 56                          	pushq	%rsi
1423476df: 57                          	pushq	%rdi
1423476e0: 53                          	pushq	%rbx
1423476e1: 48 83 ec 68                 	subq	$0x68, %rsp
1423476e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1423476ec: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1423476f1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1423476f6: ba 40 00 00 00              	movl	$0x40, %edx
1423476fb: 41 b8 08 00 00 00           	movl	$0x8, %r8d
142347701: 48 8b 8d 98 02 00 00        	movq	0x298(%rbp), %rcx
142347708: e8 13 c5 2c fe              	callq	0x140613c20 <.text+0x612c20>
14234770d: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347712: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347717: 48 83 c4 68                 	addq	$0x68, %rsp
14234771b: 5b                          	popq	%rbx
14234771c: 5f                          	popq	%rdi
14234771d: 5e                          	popq	%rsi
14234771e: 41 5c                       	popq	%r12
142347720: 41 5d                       	popq	%r13
142347722: 41 5e                       	popq	%r14
142347724: 41 5f                       	popq	%r15
142347726: 5d                          	popq	%rbp
142347727: c3                          	retq
142347728: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
142347730: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347735: 55                          	pushq	%rbp
142347736: 41 57                       	pushq	%r15
142347738: 41 56                       	pushq	%r14
14234773a: 41 55                       	pushq	%r13
14234773c: 41 54                       	pushq	%r12
14234773e: 56                          	pushq	%rsi
14234773f: 57                          	pushq	%rdi
142347740: 53                          	pushq	%rbx
142347741: 48 83 ec 68                 	subq	$0x68, %rsp
142347745: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234774c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347751: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347756: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
14234775d: e8 7e 2f c6 fe              	callq	0x140faa6e0 <.text+0xfa96e0>
142347762: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347767: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14234776c: 48 83 c4 68                 	addq	$0x68, %rsp
142347770: 5b                          	popq	%rbx
142347771: 5f                          	popq	%rdi
142347772: 5e                          	popq	%rsi
142347773: 41 5c                       	popq	%r12
142347775: 41 5d                       	popq	%r13
142347777: 41 5e                       	popq	%r14
142347779: 41 5f                       	popq	%r15
14234777b: 5d                          	popq	%rbp
14234777c: c3                          	retq
14234777d: 0f 1f 00                    	nopl	(%rax)
142347780: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347785: 55                          	pushq	%rbp
142347786: 41 57                       	pushq	%r15
142347788: 41 56                       	pushq	%r14
14234778a: 41 55                       	pushq	%r13
14234778c: 41 54                       	pushq	%r12
14234778e: 56                          	pushq	%rsi
14234778f: 57                          	pushq	%rdi
142347790: 53                          	pushq	%rbx
142347791: 48 83 ec 68                 	subq	$0x68, %rsp
142347795: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234779c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1423477a1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1423477a6: ba 40 00 00 00              	movl	$0x40, %edx
1423477ab: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1423477b1: 48 8b 8d 98 02 00 00        	movq	0x298(%rbp), %rcx
1423477b8: e8 63 c4 2c fe              	callq	0x140613c20 <.text+0x612c20>
1423477bd: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1423477c2: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1423477c7: 48 83 c4 68                 	addq	$0x68, %rsp
1423477cb: 5b                          	popq	%rbx
1423477cc: 5f                          	popq	%rdi
1423477cd: 5e                          	popq	%rsi
1423477ce: 41 5c                       	popq	%r12
1423477d0: 41 5d                       	popq	%r13
1423477d2: 41 5e                       	popq	%r14
1423477d4: 41 5f                       	popq	%r15
1423477d6: 5d                          	popq	%rbp
1423477d7: c3                          	retq
1423477d8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1423477e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1423477e5: 55                          	pushq	%rbp
1423477e6: 41 57                       	pushq	%r15
1423477e8: 41 56                       	pushq	%r14
1423477ea: 41 55                       	pushq	%r13
1423477ec: 41 54                       	pushq	%r12
1423477ee: 56                          	pushq	%rsi
1423477ef: 57                          	pushq	%rdi
1423477f0: 53                          	pushq	%rbx
1423477f1: 48 83 ec 68                 	subq	$0x68, %rsp
1423477f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1423477fc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347801: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347806: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
14234780d: e8 de a1 c5 fe              	callq	0x140fa19f0 <.text+0xfa09f0>
142347812: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347817: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14234781c: 48 83 c4 68                 	addq	$0x68, %rsp
142347820: 5b                          	popq	%rbx
142347821: 5f                          	popq	%rdi
142347822: 5e                          	popq	%rsi
142347823: 41 5c                       	popq	%r12
142347825: 41 5d                       	popq	%r13
142347827: 41 5e                       	popq	%r14
142347829: 41 5f                       	popq	%r15
14234782b: 5d                          	popq	%rbp
14234782c: c3                          	retq
14234782d: 0f 1f 00                    	nopl	(%rax)
142347830: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347835: 55                          	pushq	%rbp
142347836: 41 57                       	pushq	%r15
142347838: 41 56                       	pushq	%r14
14234783a: 41 55                       	pushq	%r13
14234783c: 41 54                       	pushq	%r12
14234783e: 56                          	pushq	%rsi
14234783f: 57                          	pushq	%rdi
142347840: 53                          	pushq	%rbx
142347841: 48 83 ec 68                 	subq	$0x68, %rsp
142347845: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234784c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347851: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347856: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
14234785d: e8 7e 2e c6 fe              	callq	0x140faa6e0 <.text+0xfa96e0>
142347862: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347867: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14234786c: 48 83 c4 68                 	addq	$0x68, %rsp
142347870: 5b                          	popq	%rbx
142347871: 5f                          	popq	%rdi
142347872: 5e                          	popq	%rsi
142347873: 41 5c                       	popq	%r12
142347875: 41 5d                       	popq	%r13
142347877: 41 5e                       	popq	%r14
142347879: 41 5f                       	popq	%r15
14234787b: 5d                          	popq	%rbp
14234787c: c3                          	retq
14234787d: 0f 1f 00                    	nopl	(%rax)
142347880: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347885: 55                          	pushq	%rbp
142347886: 41 57                       	pushq	%r15
142347888: 41 56                       	pushq	%r14
14234788a: 41 55                       	pushq	%r13
14234788c: 41 54                       	pushq	%r12
14234788e: 56                          	pushq	%rsi
14234788f: 57                          	pushq	%rdi
142347890: 53                          	pushq	%rbx
142347891: 48 83 ec 68                 	subq	$0x68, %rsp
142347895: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234789c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1423478a1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1423478a6: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1423478ad: e8 2e 2e c6 fe              	callq	0x140faa6e0 <.text+0xfa96e0>
1423478b2: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1423478b7: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1423478bc: 48 83 c4 68                 	addq	$0x68, %rsp
1423478c0: 5b                          	popq	%rbx
1423478c1: 5f                          	popq	%rdi
1423478c2: 5e                          	popq	%rsi
1423478c3: 41 5c                       	popq	%r12
1423478c5: 41 5d                       	popq	%r13
1423478c7: 41 5e                       	popq	%r14
1423478c9: 41 5f                       	popq	%r15
1423478cb: 5d                          	popq	%rbp
1423478cc: c3                          	retq
1423478cd: 0f 1f 00                    	nopl	(%rax)
1423478d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1423478d5: 55                          	pushq	%rbp
1423478d6: 41 57                       	pushq	%r15
1423478d8: 41 56                       	pushq	%r14
1423478da: 41 55                       	pushq	%r13
1423478dc: 41 54                       	pushq	%r12
1423478de: 56                          	pushq	%rsi
1423478df: 57                          	pushq	%rdi
1423478e0: 53                          	pushq	%rbx
1423478e1: 48 83 ec 68                 	subq	$0x68, %rsp
1423478e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1423478ec: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1423478f1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1423478f6: ba 40 00 00 00              	movl	$0x40, %edx
1423478fb: 41 b8 08 00 00 00           	movl	$0x8, %r8d
142347901: 48 8b 8d 98 02 00 00        	movq	0x298(%rbp), %rcx
142347908: e8 13 c3 2c fe              	callq	0x140613c20 <.text+0x612c20>
14234790d: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347912: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347917: 48 83 c4 68                 	addq	$0x68, %rsp
14234791b: 5b                          	popq	%rbx
14234791c: 5f                          	popq	%rdi
14234791d: 5e                          	popq	%rsi
14234791e: 41 5c                       	popq	%r12
142347920: 41 5d                       	popq	%r13
142347922: 41 5e                       	popq	%r14
142347924: 41 5f                       	popq	%r15
142347926: 5d                          	popq	%rbp
142347927: c3                          	retq
142347928: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
142347930: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347935: 55                          	pushq	%rbp
142347936: 41 57                       	pushq	%r15
142347938: 41 56                       	pushq	%r14
14234793a: 41 55                       	pushq	%r13
14234793c: 41 54                       	pushq	%r12
14234793e: 56                          	pushq	%rsi
14234793f: 57                          	pushq	%rdi
142347940: 53                          	pushq	%rbx
142347941: 48 83 ec 68                 	subq	$0x68, %rsp
142347945: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234794c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347951: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347956: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
14234795a: e8 91 a0 c5 fe              	callq	0x140fa19f0 <.text+0xfa09f0>
14234795f: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
142347966: e8 85 a0 c5 fe              	callq	0x140fa19f0 <.text+0xfa09f0>
14234796b: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347970: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347975: 48 83 c4 68                 	addq	$0x68, %rsp
142347979: 5b                          	popq	%rbx
14234797a: 5f                          	popq	%rdi
14234797b: 5e                          	popq	%rsi
14234797c: 41 5c                       	popq	%r12
14234797e: 41 5d                       	popq	%r13
142347980: 41 5e                       	popq	%r14
142347982: 41 5f                       	popq	%r15
142347984: 5d                          	popq	%rbp
142347985: c3                          	retq
142347986: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
142347990: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347995: 55                          	pushq	%rbp
142347996: 41 57                       	pushq	%r15
142347998: 41 56                       	pushq	%r14
14234799a: 41 55                       	pushq	%r13
14234799c: 41 54                       	pushq	%r12
14234799e: 56                          	pushq	%rsi
14234799f: 57                          	pushq	%rdi
1423479a0: 53                          	pushq	%rbx
1423479a1: 48 83 ec 68                 	subq	$0x68, %rsp
1423479a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1423479ac: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1423479b1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1423479b6: 48 8d 8d 48 01 00 00        	leaq	0x148(%rbp), %rcx
1423479bd: e8 1e 2d c6 fe              	callq	0x140faa6e0 <.text+0xfa96e0>
1423479c2: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1423479c7: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1423479cc: 48 83 c4 68                 	addq	$0x68, %rsp
1423479d0: 5b                          	popq	%rbx
1423479d1: 5f                          	popq	%rdi
1423479d2: 5e                          	popq	%rsi
1423479d3: 41 5c                       	popq	%r12
1423479d5: 41 5d                       	popq	%r13
1423479d7: 41 5e                       	popq	%r14
1423479d9: 41 5f                       	popq	%r15
1423479db: 5d                          	popq	%rbp
1423479dc: c3                          	retq
1423479dd: 0f 1f 00                    	nopl	(%rax)
1423479e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1423479e5: 55                          	pushq	%rbp
1423479e6: 41 57                       	pushq	%r15
1423479e8: 41 56                       	pushq	%r14
1423479ea: 41 55                       	pushq	%r13
1423479ec: 41 54                       	pushq	%r12
1423479ee: 56                          	pushq	%rsi
1423479ef: 57                          	pushq	%rdi
1423479f0: 53                          	pushq	%rbx
1423479f1: 48 83 ec 68                 	subq	$0x68, %rsp
1423479f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1423479fc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347a01: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347a06: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
142347a0d: e8 ce 2c c6 fe              	callq	0x140faa6e0 <.text+0xfa96e0>
142347a12: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347a17: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347a1c: 48 83 c4 68                 	addq	$0x68, %rsp
142347a20: 5b                          	popq	%rbx
142347a21: 5f                          	popq	%rdi
142347a22: 5e                          	popq	%rsi
142347a23: 41 5c                       	popq	%r12
142347a25: 41 5d                       	popq	%r13
142347a27: 41 5e                       	popq	%r14
142347a29: 41 5f                       	popq	%r15
142347a2b: 5d                          	popq	%rbp
142347a2c: c3                          	retq
142347a2d: 0f 1f 00                    	nopl	(%rax)
142347a30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347a35: 55                          	pushq	%rbp
142347a36: 41 57                       	pushq	%r15
142347a38: 41 56                       	pushq	%r14
142347a3a: 41 55                       	pushq	%r13
142347a3c: 41 54                       	pushq	%r12
142347a3e: 56                          	pushq	%rsi
142347a3f: 57                          	pushq	%rdi
142347a40: 53                          	pushq	%rbx
142347a41: 48 83 ec 68                 	subq	$0x68, %rsp
142347a45: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
142347a4c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347a51: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347a56: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
142347a5d: e8 7e 2c c6 fe              	callq	0x140faa6e0 <.text+0xfa96e0>
142347a62: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347a67: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347a6c: 48 83 c4 68                 	addq	$0x68, %rsp
142347a70: 5b                          	popq	%rbx
142347a71: 5f                          	popq	%rdi
142347a72: 5e                          	popq	%rsi
142347a73: 41 5c                       	popq	%r12
142347a75: 41 5d                       	popq	%r13
142347a77: 41 5e                       	popq	%r14
142347a79: 41 5f                       	popq	%r15
142347a7b: 5d                          	popq	%rbp
142347a7c: c3                          	retq
142347a7d: 0f 1f 00                    	nopl	(%rax)
142347a80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347a85: 55                          	pushq	%rbp
142347a86: 41 57                       	pushq	%r15
142347a88: 41 56                       	pushq	%r14
142347a8a: 41 55                       	pushq	%r13
142347a8c: 41 54                       	pushq	%r12
142347a8e: 56                          	pushq	%rsi
142347a8f: 57                          	pushq	%rdi
142347a90: 53                          	pushq	%rbx
142347a91: 48 83 ec 68                 	subq	$0x68, %rsp
142347a95: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
142347a9c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347aa1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347aa6: ba 40 00 00 00              	movl	$0x40, %edx
142347aab: 41 b8 08 00 00 00           	movl	$0x8, %r8d
142347ab1: 48 8b 8d 98 02 00 00        	movq	0x298(%rbp), %rcx
142347ab8: e8 63 c1 2c fe              	callq	0x140613c20 <.text+0x612c20>
142347abd: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347ac2: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347ac7: 48 83 c4 68                 	addq	$0x68, %rsp
142347acb: 5b                          	popq	%rbx
142347acc: 5f                          	popq	%rdi
142347acd: 5e                          	popq	%rsi
142347ace: 41 5c                       	popq	%r12
142347ad0: 41 5d                       	popq	%r13
142347ad2: 41 5e                       	popq	%r14
142347ad4: 41 5f                       	popq	%r15
142347ad6: 5d                          	popq	%rbp
142347ad7: c3                          	retq
142347ad8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
142347ae0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347ae5: 55                          	pushq	%rbp
142347ae6: 41 57                       	pushq	%r15
142347ae8: 41 56                       	pushq	%r14
142347aea: 41 55                       	pushq	%r13
142347aec: 41 54                       	pushq	%r12
142347aee: 56                          	pushq	%rsi
142347aef: 57                          	pushq	%rdi
142347af0: 53                          	pushq	%rbx
142347af1: 48 83 ec 68                 	subq	$0x68, %rsp
142347af5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
142347afc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347b01: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347b06: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
142347b0d: e8 de 9e c5 fe              	callq	0x140fa19f0 <.text+0xfa09f0>
142347b12: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
142347b19: e8 d2 9e c5 fe              	callq	0x140fa19f0 <.text+0xfa09f0>
142347b1e: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347b23: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347b28: 48 83 c4 68                 	addq	$0x68, %rsp
142347b2c: 5b                          	popq	%rbx
142347b2d: 5f                          	popq	%rdi
142347b2e: 5e                          	popq	%rsi
142347b2f: 41 5c                       	popq	%r12
142347b31: 41 5d                       	popq	%r13
142347b33: 41 5e                       	popq	%r14
142347b35: 41 5f                       	popq	%r15
142347b37: 5d                          	popq	%rbp
142347b38: c3                          	retq
142347b39: 0f 1f 80 00 00 00 00        	nopl	(%rax)
142347b40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347b45: 55                          	pushq	%rbp
142347b46: 41 57                       	pushq	%r15
142347b48: 41 56                       	pushq	%r14
142347b4a: 41 55                       	pushq	%r13
142347b4c: 41 54                       	pushq	%r12
142347b4e: 56                          	pushq	%rsi
142347b4f: 57                          	pushq	%rdi
142347b50: 53                          	pushq	%rbx
142347b51: 48 83 ec 68                 	subq	$0x68, %rsp
142347b55: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
142347b5c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347b61: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347b66: ba 40 00 00 00              	movl	$0x40, %edx
142347b6b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
142347b71: 48 8b 8d 98 02 00 00        	movq	0x298(%rbp), %rcx
142347b78: e8 a3 c0 2c fe              	callq	0x140613c20 <.text+0x612c20>
142347b7d: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347b82: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347b87: 48 83 c4 68                 	addq	$0x68, %rsp
142347b8b: 5b                          	popq	%rbx
142347b8c: 5f                          	popq	%rdi
142347b8d: 5e                          	popq	%rsi
142347b8e: 41 5c                       	popq	%r12
142347b90: 41 5d                       	popq	%r13
142347b92: 41 5e                       	popq	%r14
142347b94: 41 5f                       	popq	%r15
142347b96: 5d                          	popq	%rbp
142347b97: c3                          	retq
142347b98: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
142347ba0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347ba5: 55                          	pushq	%rbp
142347ba6: 41 57                       	pushq	%r15
142347ba8: 41 56                       	pushq	%r14
142347baa: 41 55                       	pushq	%r13
142347bac: 41 54                       	pushq	%r12
142347bae: 56                          	pushq	%rsi
142347baf: 57                          	pushq	%rdi
142347bb0: 53                          	pushq	%rbx
142347bb1: 48 83 ec 68                 	subq	$0x68, %rsp
142347bb5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
142347bbc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347bc1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347bc6: 48 8d 8d a8 01 00 00        	leaq	0x1a8(%rbp), %rcx
142347bcd: e8 1e 9e c5 fe              	callq	0x140fa19f0 <.text+0xfa09f0>
142347bd2: 48 8d 8d 68 02 00 00        	leaq	0x268(%rbp), %rcx
142347bd9: e8 12 9e c5 fe              	callq	0x140fa19f0 <.text+0xfa09f0>
142347bde: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347be3: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347be8: 48 83 c4 68                 	addq	$0x68, %rsp
142347bec: 5b                          	popq	%rbx
142347bed: 5f                          	popq	%rdi
142347bee: 5e                          	popq	%rsi
142347bef: 41 5c                       	popq	%r12
142347bf1: 41 5d                       	popq	%r13
142347bf3: 41 5e                       	popq	%r14
142347bf5: 41 5f                       	popq	%r15
142347bf7: 5d                          	popq	%rbp
142347bf8: c3                          	retq
142347bf9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
142347c00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347c05: 55                          	pushq	%rbp
142347c06: 41 57                       	pushq	%r15
142347c08: 41 56                       	pushq	%r14
142347c0a: 41 55                       	pushq	%r13
142347c0c: 41 54                       	pushq	%r12
142347c0e: 56                          	pushq	%rsi
142347c0f: 57                          	pushq	%rdi
142347c10: 53                          	pushq	%rbx
142347c11: 48 83 ec 68                 	subq	$0x68, %rsp
142347c15: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
142347c1c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347c21: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347c26: ba 40 00 00 00              	movl	$0x40, %edx
142347c2b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
142347c31: 48 8b 8d 98 02 00 00        	movq	0x298(%rbp), %rcx
142347c38: e8 e3 bf 2c fe              	callq	0x140613c20 <.text+0x612c20>
142347c3d: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347c42: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347c47: 48 83 c4 68                 	addq	$0x68, %rsp
142347c4b: 5b                          	popq	%rbx
142347c4c: 5f                          	popq	%rdi
142347c4d: 5e                          	popq	%rsi
142347c4e: 41 5c                       	popq	%r12
142347c50: 41 5d                       	popq	%r13
142347c52: 41 5e                       	popq	%r14
142347c54: 41 5f                       	popq	%r15
142347c56: 5d                          	popq	%rbp
142347c57: c3                          	retq
142347c58: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
142347c60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347c65: 55                          	pushq	%rbp
142347c66: 41 57                       	pushq	%r15
142347c68: 41 56                       	pushq	%r14
142347c6a: 41 55                       	pushq	%r13
142347c6c: 41 54                       	pushq	%r12
142347c6e: 56                          	pushq	%rsi
142347c6f: 57                          	pushq	%rdi
142347c70: 53                          	pushq	%rbx
142347c71: 48 83 ec 68                 	subq	$0x68, %rsp
142347c75: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
142347c7c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347c81: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347c86: 80 bd a3 02 00 00 00        	cmpb	$0x0, 0x2a3(%rbp)
142347c8d: 74 2e                       	je	0x142347cbd <.text+0x2346cbd>
142347c8f: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
142347c96: e8 55 9d c5 fe              	callq	0x140fa19f0 <.text+0xfa09f0>
142347c9b: c6 85 a5 02 00 00 00        	movb	$0x0, 0x2a5(%rbp)
142347ca2: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347ca7: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347cac: 48 83 c4 68                 	addq	$0x68, %rsp
142347cb0: 5b                          	popq	%rbx
142347cb1: 5f                          	popq	%rdi
142347cb2: 5e                          	popq	%rsi
142347cb3: 41 5c                       	popq	%r12
142347cb5: 41 5d                       	popq	%r13
142347cb7: 41 5e                       	popq	%r14
142347cb9: 41 5f                       	popq	%r15
142347cbb: 5d                          	popq	%rbp
142347cbc: c3                          	retq
142347cbd: c6 85 a5 02 00 00 00        	movb	$0x0, 0x2a5(%rbp)
142347cc4: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347cc9: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347cce: 48 83 c4 68                 	addq	$0x68, %rsp
142347cd2: 5b                          	popq	%rbx
142347cd3: 5f                          	popq	%rdi
142347cd4: 5e                          	popq	%rsi
142347cd5: 41 5c                       	popq	%r12
142347cd7: 41 5d                       	popq	%r13
142347cd9: 41 5e                       	popq	%r14
142347cdb: 41 5f                       	popq	%r15
142347cdd: 5d                          	popq	%rbp
142347cde: c3                          	retq
142347cdf: 90                          	nop
142347ce0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347ce5: 55                          	pushq	%rbp
142347ce6: 41 57                       	pushq	%r15
142347ce8: 41 56                       	pushq	%r14
142347cea: 41 55                       	pushq	%r13
142347cec: 41 54                       	pushq	%r12
142347cee: 56                          	pushq	%rsi
142347cef: 57                          	pushq	%rdi
142347cf0: 53                          	pushq	%rbx
142347cf1: 48 83 ec 68                 	subq	$0x68, %rsp
142347cf5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
142347cfc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347d01: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347d06: 80 bd a5 02 00 00 00        	cmpb	$0x0, 0x2a5(%rbp)
142347d0d: 74 27                       	je	0x142347d36 <.text+0x2346d36>
142347d0f: 48 8d 8d 48 01 00 00        	leaq	0x148(%rbp), %rcx
142347d16: e8 d5 9c c5 fe              	callq	0x140fa19f0 <.text+0xfa09f0>
142347d1b: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347d20: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347d25: 48 83 c4 68                 	addq	$0x68, %rsp
142347d29: 5b                          	popq	%rbx
142347d2a: 5f                          	popq	%rdi
142347d2b: 5e                          	popq	%rsi
142347d2c: 41 5c                       	popq	%r12
142347d2e: 41 5d                       	popq	%r13
142347d30: 41 5e                       	popq	%r14
142347d32: 41 5f                       	popq	%r15
142347d34: 5d                          	popq	%rbp
142347d35: c3                          	retq
142347d36: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347d3b: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347d40: 48 83 c4 68                 	addq	$0x68, %rsp
142347d44: 5b                          	popq	%rbx
142347d45: 5f                          	popq	%rdi
142347d46: 5e                          	popq	%rsi
142347d47: 41 5c                       	popq	%r12
142347d49: 41 5d                       	popq	%r13
142347d4b: 41 5e                       	popq	%r14
142347d4d: 41 5f                       	popq	%r15
142347d4f: 5d                          	popq	%rbp
142347d50: c3                          	retq
142347d51: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
142347d60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347d65: 55                          	pushq	%rbp
142347d66: 41 57                       	pushq	%r15
142347d68: 41 56                       	pushq	%r14
142347d6a: 41 55                       	pushq	%r13
142347d6c: 41 54                       	pushq	%r12
142347d6e: 56                          	pushq	%rsi
142347d6f: 57                          	pushq	%rdi
142347d70: 53                          	pushq	%rbx
142347d71: 48 83 ec 68                 	subq	$0x68, %rsp
142347d75: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
142347d7c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347d81: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347d86: 0f b6 9d a7 02 00 00        	movzbl	0x2a7(%rbp), %ebx
142347d8d: 80 bd a6 02 00 00 00        	cmpb	$0x0, 0x2a6(%rbp)
142347d94: 74 40                       	je	0x142347dd6 <.text+0x2346dd6>
142347d96: 48 8b 55 d0                 	movq	-0x30(%rbp), %rdx
142347d9a: 48 85 d2                    	testq	%rdx, %rdx
142347d9d: 74 13                       	je	0x142347db2 <.text+0x2346db2>
142347d9f: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
142347da3: 48 c1 e2 05                 	shlq	$0x5, %rdx
142347da7: 41 b8 08 00 00 00           	movl	$0x8, %r8d
142347dad: e8 6e be 2c fe              	callq	0x140613c20 <.text+0x612c20>
142347db2: 80 e3 01                    	andb	$0x1, %bl
142347db5: 88 9d a4 02 00 00           	movb	%bl, 0x2a4(%rbp)
142347dbb: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347dc0: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347dc5: 48 83 c4 68                 	addq	$0x68, %rsp
142347dc9: 5b                          	popq	%rbx
142347dca: 5f                          	popq	%rdi
142347dcb: 5e                          	popq	%rsi
142347dcc: 41 5c                       	popq	%r12
142347dce: 41 5d                       	popq	%r13
142347dd0: 41 5e                       	popq	%r14
142347dd2: 41 5f                       	popq	%r15
142347dd4: 5d                          	popq	%rbp
142347dd5: c3                          	retq
142347dd6: 80 e3 01                    	andb	$0x1, %bl
142347dd9: 88 9d a4 02 00 00           	movb	%bl, 0x2a4(%rbp)
142347ddf: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347de4: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347de9: 48 83 c4 68                 	addq	$0x68, %rsp
142347ded: 5b                          	popq	%rbx
142347dee: 5f                          	popq	%rdi
142347def: 5e                          	popq	%rsi
142347df0: 41 5c                       	popq	%r12
142347df2: 41 5d                       	popq	%r13
142347df4: 41 5e                       	popq	%r14
142347df6: 41 5f                       	popq	%r15
142347df8: 5d                          	popq	%rbp
142347df9: c3                          	retq
142347dfa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
142347e00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347e05: 55                          	pushq	%rbp
142347e06: 41 57                       	pushq	%r15
142347e08: 41 56                       	pushq	%r14
142347e0a: 41 55                       	pushq	%r13
142347e0c: 41 54                       	pushq	%r12
142347e0e: 56                          	pushq	%rsi
142347e0f: 57                          	pushq	%rdi
142347e10: 53                          	pushq	%rbx
142347e11: 48 83 ec 68                 	subq	$0x68, %rsp
142347e15: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
142347e1c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347e21: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347e26: 80 bd a4 02 00 00 00        	cmpb	$0x0, 0x2a4(%rbp)
142347e2d: 74 27                       	je	0x142347e56 <.text+0x2346e56>
142347e2f: 48 83 bd 48 02 00 00 00     	cmpq	$0x0, 0x248(%rbp)
142347e37: 74 1d                       	je	0x142347e56 <.text+0x2346e56>
142347e39: 48 8b 95 48 02 00 00        	movq	0x248(%rbp), %rdx
142347e40: 48 c1 e2 05                 	shlq	$0x5, %rdx
142347e44: 41 b8 08 00 00 00           	movl	$0x8, %r8d
142347e4a: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
142347e51: e8 ca bd 2c fe              	callq	0x140613c20 <.text+0x612c20>
142347e56: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347e5b: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347e60: 48 83 c4 68                 	addq	$0x68, %rsp
142347e64: 5b                          	popq	%rbx
142347e65: 5f                          	popq	%rdi
142347e66: 5e                          	popq	%rsi
142347e67: 41 5c                       	popq	%r12
142347e69: 41 5d                       	popq	%r13
142347e6b: 41 5e                       	popq	%r14
142347e6d: 41 5f                       	popq	%r15
142347e6f: 5d                          	popq	%rbp
142347e70: c3                          	retq
142347e71: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
142347e80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142347e85: 55                          	pushq	%rbp
142347e86: 41 57                       	pushq	%r15
142347e88: 41 56                       	pushq	%r14
142347e8a: 41 55                       	pushq	%r13
142347e8c: 41 54                       	pushq	%r12
142347e8e: 56                          	pushq	%rsi
142347e8f: 57                          	pushq	%rdi
142347e90: 53                          	pushq	%rbx
142347e91: 48 83 ec 68                 	subq	$0x68, %rsp
142347e95: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
142347e9c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
142347ea1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
142347ea6: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
142347ead: e8 be b7 c5 fe              	callq	0x140fa3670 <.text+0xfa2670>
142347eb2: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
142347eb7: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
142347ebc: 48 83 c4 68                 	addq	$0x68, %rsp
142347ec0: 5b                          	popq	%rbx
142347ec1: 5f                          	popq	%rdi
142347ec2: 5e                          	popq	%rsi
142347ec3: 41 5c                       	popq	%r12
142347ec5: 41 5d                       	popq	%r13
142347ec7: 41 5e                       	popq	%r14
142347ec9: 41 5f                       	popq	%r15
142347ecb: 5d                          	popq	%rbp
142347ecc: c3                          	retq
