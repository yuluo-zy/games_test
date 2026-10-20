
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141db2d10: 55                          	pushq	%rbp
141db2d11: 56                          	pushq	%rsi
141db2d12: 57                          	pushq	%rdi
141db2d13: 53                          	pushq	%rbx
141db2d14: 48 81 ec 88 06 00 00        	subq	$0x688, %rsp            # imm = 0x688
141db2d1b: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141db2d23: 48 c7 85 00 06 00 00 fe ff ff ff    	movq	$-0x2, 0x600(%rbp)
141db2d2e: 48 89 ce                    	movq	%rcx, %rsi
141db2d31: 0f b6 05 d1 1e 88 01        	movzbl	0x1881ed1(%rip), %eax   # 0x143634c09
141db2d38: b9 c0 01 00 00              	movl	$0x1c0, %ecx            # imm = 0x1C0
141db2d3d: ba 08 00 00 00              	movl	$0x8, %edx
141db2d42: e8 c9 0e 86 fe              	callq	0x140613c10 <.text+0x612c10>
141db2d47: 48 85 c0                    	testq	%rax, %rax
141db2d4a: 0f 84 2d 06 00 00           	je	0x141db337d <.text+0x1db237d>
141db2d50: 48 89 85 f8 05 00 00        	movq	%rax, 0x5f8(%rbp)
141db2d57: 0f 57 c0                    	xorps	%xmm0, %xmm0
141db2d5a: 0f 11 85 18 01 00 00        	movups	%xmm0, 0x118(%rbp)
141db2d61: 0f 11 85 30 01 00 00        	movups	%xmm0, 0x130(%rbp)
141db2d68: 0f 11 85 48 01 00 00        	movups	%xmm0, 0x148(%rbp)
141db2d6f: 0f 11 85 60 01 00 00        	movups	%xmm0, 0x160(%rbp)
141db2d76: 0f 11 85 78 01 00 00        	movups	%xmm0, 0x178(%rbp)
141db2d7d: c7 85 88 01 00 00 00 00 00 00       	movl	$0x0, 0x188(%rbp)
141db2d87: 0f 11 85 b0 01 00 00        	movups	%xmm0, 0x1b0(%rbp)
141db2d8e: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
141db2d95: 0f 11 85 e0 01 00 00        	movups	%xmm0, 0x1e0(%rbp)
141db2d9c: 0f 11 85 f8 01 00 00        	movups	%xmm0, 0x1f8(%rbp)
141db2da3: 0f 11 85 10 02 00 00        	movups	%xmm0, 0x210(%rbp)
141db2daa: c7 85 20 02 00 00 00 00 00 00       	movl	$0x0, 0x220(%rbp)
141db2db4: 48 c7 85 f8 00 00 00 00 00 00 00    	movq	$0x0, 0xf8(%rbp)
141db2dbf: 48 c7 85 00 01 00 00 08 00 00 00    	movq	$0x8, 0x100(%rbp)
141db2dca: 48 c7 85 08 01 00 00 00 00 00 00    	movq	$0x0, 0x108(%rbp)
141db2dd5: 48 c7 85 10 01 00 00 10 00 00 00    	movq	$0x10, 0x110(%rbp)
141db2de0: 48 c7 85 28 01 00 00 10 00 00 00    	movq	$0x10, 0x128(%rbp)
141db2deb: 48 c7 85 40 01 00 00 10 00 00 00    	movq	$0x10, 0x140(%rbp)
141db2df6: 48 c7 85 58 01 00 00 10 00 00 00    	movq	$0x10, 0x158(%rbp)
141db2e01: 48 c7 85 70 01 00 00 10 00 00 00    	movq	$0x10, 0x170(%rbp)
141db2e0c: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
141db2e16: 48 89 9d 90 01 00 00        	movq	%rbx, 0x190(%rbp)
141db2e1d: 48 8d 05 ae d8 1b 01        	leaq	0x11bd8ae(%rip), %rax   # 0x142f706d2
141db2e24: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
141db2e2b: 48 c7 85 a0 01 00 00 2d 00 00 00    	movq	$0x2d, 0x1a0(%rbp)
141db2e36: 48 c7 85 a8 01 00 00 10 00 00 00    	movq	$0x10, 0x1a8(%rbp)
141db2e41: 48 c7 85 c0 01 00 00 10 00 00 00    	movq	$0x10, 0x1c0(%rbp)
141db2e4c: 48 c7 85 d8 01 00 00 10 00 00 00    	movq	$0x10, 0x1d8(%rbp)
141db2e57: 48 c7 85 f0 01 00 00 10 00 00 00    	movq	$0x10, 0x1f0(%rbp)
141db2e62: 48 c7 85 08 02 00 00 10 00 00 00    	movq	$0x10, 0x208(%rbp)
141db2e6d: c7 85 28 02 00 00 00 00 00 00       	movl	$0x0, 0x228(%rbp)
141db2e77: 66 c7 85 2c 02 00 00 01 00  	movw	$0x1, 0x22c(%rbp)
141db2e80: 48 89 9d 30 02 00 00        	movq	%rbx, 0x230(%rbp)
141db2e87: c7 85 f8 04 00 00 00 00 00 00       	movl	$0x0, 0x4f8(%rbp)
141db2e91: 0f b6 05 71 1d 88 01        	movzbl	0x1881d71(%rip), %eax   # 0x143634c09
141db2e98: b9 08 04 00 00              	movl	$0x408, %ecx            # imm = 0x408
141db2e9d: ba 08 00 00 00              	movl	$0x8, %edx
141db2ea2: e8 69 0d 86 fe              	callq	0x140613c10 <.text+0x612c10>
141db2ea7: 48 85 c0                    	testq	%rax, %rax
141db2eaa: 0f 84 dc 04 00 00           	je	0x141db338c <.text+0x1db238c>
141db2eb0: 48 89 c7                    	movq	%rax, %rdi
141db2eb3: 48 8d 95 f8 00 00 00        	leaq	0xf8(%rbp), %rdx
141db2eba: 41 b8 08 04 00 00           	movl	$0x408, %r8d            # imm = 0x408
141db2ec0: 48 89 c1                    	movq	%rax, %rcx
141db2ec3: e8 38 0d b7 00              	callq	0x142923c00 <.text+0x2922c00>
141db2ec8: 4c 8d 05 69 d5 1b 01        	leaq	0x11bd569(%rip), %r8    # 0x142f70438
141db2ecf: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
141db2ed6: 48 89 fa                    	movq	%rdi, %rdx
141db2ed9: e8 32 1c f3 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141db2ede: 0f 57 c0                    	xorps	%xmm0, %xmm0
141db2ee1: 0f 11 85 18 01 00 00        	movups	%xmm0, 0x118(%rbp)
141db2ee8: 0f 11 85 30 01 00 00        	movups	%xmm0, 0x130(%rbp)
141db2eef: 0f 11 85 48 01 00 00        	movups	%xmm0, 0x148(%rbp)
141db2ef6: 0f 11 85 60 01 00 00        	movups	%xmm0, 0x160(%rbp)
141db2efd: 0f 11 85 78 01 00 00        	movups	%xmm0, 0x178(%rbp)
141db2f04: c7 85 88 01 00 00 00 00 00 00       	movl	$0x0, 0x188(%rbp)
141db2f0e: 0f 11 85 b0 01 00 00        	movups	%xmm0, 0x1b0(%rbp)
141db2f15: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
141db2f1c: 0f 11 85 e0 01 00 00        	movups	%xmm0, 0x1e0(%rbp)
141db2f23: 0f 11 85 f8 01 00 00        	movups	%xmm0, 0x1f8(%rbp)
141db2f2a: 0f 11 85 10 02 00 00        	movups	%xmm0, 0x210(%rbp)
141db2f31: c7 85 20 02 00 00 00 00 00 00       	movl	$0x0, 0x220(%rbp)
141db2f3b: 48 c7 85 f8 00 00 00 00 00 00 00    	movq	$0x0, 0xf8(%rbp)
141db2f46: 48 c7 85 00 01 00 00 08 00 00 00    	movq	$0x8, 0x100(%rbp)
141db2f51: 48 c7 85 08 01 00 00 00 00 00 00    	movq	$0x0, 0x108(%rbp)
141db2f5c: 48 c7 85 10 01 00 00 10 00 00 00    	movq	$0x10, 0x110(%rbp)
141db2f67: 48 c7 85 28 01 00 00 10 00 00 00    	movq	$0x10, 0x128(%rbp)
141db2f72: 48 c7 85 40 01 00 00 10 00 00 00    	movq	$0x10, 0x140(%rbp)
141db2f7d: 48 c7 85 58 01 00 00 10 00 00 00    	movq	$0x10, 0x158(%rbp)
141db2f88: 48 c7 85 70 01 00 00 10 00 00 00    	movq	$0x10, 0x170(%rbp)
141db2f93: 48 89 9d 90 01 00 00        	movq	%rbx, 0x190(%rbp)
141db2f9a: 48 8d 05 ff d6 1b 01        	leaq	0x11bd6ff(%rip), %rax   # 0x142f706a0
141db2fa1: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
141db2fa8: 48 c7 85 a0 01 00 00 32 00 00 00    	movq	$0x32, 0x1a0(%rbp)
141db2fb3: 48 c7 85 a8 01 00 00 10 00 00 00    	movq	$0x10, 0x1a8(%rbp)
141db2fbe: 48 c7 85 c0 01 00 00 10 00 00 00    	movq	$0x10, 0x1c0(%rbp)
141db2fc9: 48 c7 85 d8 01 00 00 10 00 00 00    	movq	$0x10, 0x1d8(%rbp)
141db2fd4: 48 c7 85 f0 01 00 00 10 00 00 00    	movq	$0x10, 0x1f0(%rbp)
141db2fdf: 48 c7 85 08 02 00 00 10 00 00 00    	movq	$0x10, 0x208(%rbp)
141db2fea: c7 85 28 02 00 00 00 00 00 00       	movl	$0x0, 0x228(%rbp)
141db2ff4: 66 c7 85 2c 02 00 00 01 00  	movw	$0x1, 0x22c(%rbp)
141db2ffd: 48 89 9d 20 03 00 00        	movq	%rbx, 0x320(%rbp)
141db3004: c7 85 e0 04 00 00 00 00 00 00       	movl	$0x0, 0x4e0(%rbp)
141db300e: 0f b6 05 f4 1b 88 01        	movzbl	0x1881bf4(%rip), %eax   # 0x143634c09
141db3015: b9 f0 03 00 00              	movl	$0x3f0, %ecx            # imm = 0x3F0
141db301a: ba 08 00 00 00              	movl	$0x8, %edx
141db301f: e8 ec 0b 86 fe              	callq	0x140613c10 <.text+0x612c10>
141db3024: 48 85 c0                    	testq	%rax, %rax
141db3027: 0f 84 70 03 00 00           	je	0x141db339d <.text+0x1db239d>
141db302d: 48 89 c7                    	movq	%rax, %rdi
141db3030: 48 8d 95 f8 00 00 00        	leaq	0xf8(%rbp), %rdx
141db3037: 41 b8 f0 03 00 00           	movl	$0x3f0, %r8d            # imm = 0x3F0
141db303d: 48 89 c1                    	movq	%rax, %rcx
141db3040: e8 bb 0b b7 00              	callq	0x142923c00 <.text+0x2922c00>
141db3045: 4c 8d 05 a4 d4 1b 01        	leaq	0x11bd4a4(%rip), %r8    # 0x142f704f0
141db304c: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
141db3050: 48 89 fa                    	movq	%rdi, %rdx
141db3053: e8 b8 1a f3 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141db3058: 0f 57 c0                    	xorps	%xmm0, %xmm0
141db305b: 0f 11 85 18 01 00 00        	movups	%xmm0, 0x118(%rbp)
141db3062: 0f 11 85 30 01 00 00        	movups	%xmm0, 0x130(%rbp)
141db3069: 0f 11 85 48 01 00 00        	movups	%xmm0, 0x148(%rbp)
141db3070: 0f 11 85 60 01 00 00        	movups	%xmm0, 0x160(%rbp)
141db3077: 0f 11 85 78 01 00 00        	movups	%xmm0, 0x178(%rbp)
141db307e: c7 85 88 01 00 00 00 00 00 00       	movl	$0x0, 0x188(%rbp)
141db3088: 0f 11 85 b0 01 00 00        	movups	%xmm0, 0x1b0(%rbp)
141db308f: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
141db3096: 0f 11 85 e0 01 00 00        	movups	%xmm0, 0x1e0(%rbp)
141db309d: 0f 11 85 f8 01 00 00        	movups	%xmm0, 0x1f8(%rbp)
141db30a4: 0f 11 85 10 02 00 00        	movups	%xmm0, 0x210(%rbp)
141db30ab: c7 85 20 02 00 00 00 00 00 00       	movl	$0x0, 0x220(%rbp)
141db30b5: 48 c7 85 f8 00 00 00 00 00 00 00    	movq	$0x0, 0xf8(%rbp)
141db30c0: 48 c7 85 00 01 00 00 08 00 00 00    	movq	$0x8, 0x100(%rbp)
141db30cb: 48 c7 85 08 01 00 00 00 00 00 00    	movq	$0x0, 0x108(%rbp)
141db30d6: 48 c7 85 10 01 00 00 10 00 00 00    	movq	$0x10, 0x110(%rbp)
141db30e1: 48 c7 85 28 01 00 00 10 00 00 00    	movq	$0x10, 0x128(%rbp)
141db30ec: 48 c7 85 40 01 00 00 10 00 00 00    	movq	$0x10, 0x140(%rbp)
141db30f7: 48 c7 85 58 01 00 00 10 00 00 00    	movq	$0x10, 0x158(%rbp)
141db3102: 48 c7 85 70 01 00 00 10 00 00 00    	movq	$0x10, 0x170(%rbp)
141db310d: 48 89 9d 90 01 00 00        	movq	%rbx, 0x190(%rbp)
141db3114: 48 8d 05 62 d5 1b 01        	leaq	0x11bd562(%rip), %rax   # 0x142f7067d
141db311b: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
141db3122: 48 c7 85 a0 01 00 00 23 00 00 00    	movq	$0x23, 0x1a0(%rbp)
141db312d: 48 c7 85 a8 01 00 00 10 00 00 00    	movq	$0x10, 0x1a8(%rbp)
141db3138: 48 c7 85 c0 01 00 00 10 00 00 00    	movq	$0x10, 0x1c0(%rbp)
141db3143: 48 c7 85 d8 01 00 00 10 00 00 00    	movq	$0x10, 0x1d8(%rbp)
141db314e: 48 c7 85 f0 01 00 00 10 00 00 00    	movq	$0x10, 0x1f0(%rbp)
141db3159: 48 c7 85 08 02 00 00 10 00 00 00    	movq	$0x10, 0x208(%rbp)
141db3164: c7 85 28 02 00 00 00 00 00 00       	movl	$0x0, 0x228(%rbp)
141db316e: 66 c7 85 2c 02 00 00 01 00  	movw	$0x1, 0x22c(%rbp)
141db3177: 48 89 9d 20 03 00 00        	movq	%rbx, 0x320(%rbp)
141db317e: c7 85 f0 05 00 00 00 00 00 00       	movl	$0x0, 0x5f0(%rbp)
141db3188: 0f b6 05 7a 1a 88 01        	movzbl	0x1881a7a(%rip), %eax   # 0x143634c09
141db318f: b9 00 05 00 00              	movl	$0x500, %ecx            # imm = 0x500
141db3194: ba 08 00 00 00              	movl	$0x8, %edx
141db3199: e8 72 0a 86 fe              	callq	0x140613c10 <.text+0x612c10>
141db319e: 48 85 c0                    	testq	%rax, %rax
141db31a1: 0f 84 07 02 00 00           	je	0x141db33ae <.text+0x1db23ae>
141db31a7: 48 89 c7                    	movq	%rax, %rdi
141db31aa: 48 8d 95 f8 00 00 00        	leaq	0xf8(%rbp), %rdx
141db31b1: 41 b8 00 05 00 00           	movl	$0x500, %r8d            # imm = 0x500
141db31b7: 48 89 c1                    	movq	%rax, %rcx
141db31ba: e8 41 0a b7 00              	callq	0x142923c00 <.text+0x2922c00>
141db31bf: 4c 8d 05 e2 d3 1b 01        	leaq	0x11bd3e2(%rip), %r8    # 0x142f705a8
141db31c6: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
141db31ca: 48 89 fa                    	movq	%rdi, %rdx
141db31cd: e8 3e 19 f3 fe              	callq	0x140ce4b10 <.text+0xce3b10>
141db31d2: 48 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %rcx
141db31d9: e8 92 4b 00 00              	callq	0x141db7d70 <.text+0x1db6d70>
141db31de: 0f 10 85 e8 00 00 00        	movups	0xe8(%rbp), %xmm0
141db31e5: 48 8b 85 f8 05 00 00        	movq	0x5f8(%rbp), %rax
141db31ec: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
141db31f0: 0f 10 85 d8 00 00 00        	movups	0xd8(%rbp), %xmm0
141db31f7: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
141db31fb: 0f 10 85 c8 00 00 00        	movups	0xc8(%rbp), %xmm0
141db3202: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
141db3206: 0f 10 85 88 00 00 00        	movups	0x88(%rbp), %xmm0
141db320d: 0f 10 8d 98 00 00 00        	movups	0x98(%rbp), %xmm1
141db3214: 0f 10 95 a8 00 00 00        	movups	0xa8(%rbp), %xmm2
141db321b: 0f 10 9d b8 00 00 00        	movups	0xb8(%rbp), %xmm3
141db3222: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
141db3226: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
141db322a: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
141db322e: 0f 11 00                    	movups	%xmm0, (%rax)
141db3231: 0f 10 45 18                 	movups	0x18(%rbp), %xmm0
141db3235: 0f 10 4d 28                 	movups	0x28(%rbp), %xmm1
141db3239: 0f 10 55 38                 	movups	0x38(%rbp), %xmm2
141db323d: 0f 10 5d 48                 	movups	0x48(%rbp), %xmm3
141db3241: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
141db3245: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
141db324c: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
141db3253: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
141db325a: 0f 10 45 58                 	movups	0x58(%rbp), %xmm0
141db325e: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
141db3265: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
141db3269: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
141db3270: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
141db3274: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
141db327b: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
141db327f: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
141db3286: 0f 10 45 f8                 	movups	-0x8(%rbp), %xmm0
141db328a: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
141db3291: 0f 10 45 e8                 	movups	-0x18(%rbp), %xmm0
141db3295: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
141db329c: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
141db32a0: 0f 10 4d b8                 	movups	-0x48(%rbp), %xmm1
141db32a4: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
141db32a8: 0f 10 5d d8                 	movups	-0x28(%rbp), %xmm3
141db32ac: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
141db32b3: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
141db32ba: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
141db32c1: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
141db32c8: 0f 10 85 f8 00 00 00        	movups	0xf8(%rbp), %xmm0
141db32cf: 0f 10 8d 08 01 00 00        	movups	0x108(%rbp), %xmm1
141db32d6: 0f 10 95 18 01 00 00        	movups	0x118(%rbp), %xmm2
141db32dd: 0f 10 9d 28 01 00 00        	movups	0x128(%rbp), %xmm3
141db32e4: 0f 11 80 50 01 00 00        	movups	%xmm0, 0x150(%rax)
141db32eb: 0f 11 88 60 01 00 00        	movups	%xmm1, 0x160(%rax)
141db32f2: 0f 11 90 70 01 00 00        	movups	%xmm2, 0x170(%rax)
141db32f9: 0f 11 98 80 01 00 00        	movups	%xmm3, 0x180(%rax)
141db3300: 0f 10 85 38 01 00 00        	movups	0x138(%rbp), %xmm0
141db3307: 0f 11 80 90 01 00 00        	movups	%xmm0, 0x190(%rax)
141db330e: 0f 10 85 48 01 00 00        	movups	0x148(%rbp), %xmm0
141db3315: 0f 11 80 a0 01 00 00        	movups	%xmm0, 0x1a0(%rax)
141db331c: 0f 10 85 58 01 00 00        	movups	0x158(%rbp), %xmm0
141db3323: 0f 11 80 b0 01 00 00        	movups	%xmm0, 0x1b0(%rax)
141db332a: 48 c7 46 08 04 00 00 00     	movq	$0x4, 0x8(%rsi)
141db3332: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
141db3336: 48 c7 46 18 04 00 00 00     	movq	$0x4, 0x18(%rsi)
141db333e: 48 c7 46 20 00 00 00 00     	movq	$0x0, 0x20(%rsi)
141db3346: 48 c7 46 28 08 00 00 00     	movq	$0x8, 0x28(%rsi)
141db334e: 0f 57 c0                    	xorps	%xmm0, %xmm0
141db3351: 0f 11 46 30                 	movups	%xmm0, 0x30(%rsi)
141db3355: 0f 10 85 f8 00 00 00        	movups	0xf8(%rbp), %xmm0
141db335c: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
141db3360: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
141db3367: 48 89 46 50                 	movq	%rax, 0x50(%rsi)
141db336b: 48 89 1e                    	movq	%rbx, (%rsi)
141db336e: 48 89 f0                    	movq	%rsi, %rax
141db3371: 48 81 c4 88 06 00 00        	addq	$0x688, %rsp            # imm = 0x688
141db3378: 5b                          	popq	%rbx
141db3379: 5f                          	popq	%rdi
141db337a: 5e                          	popq	%rsi
141db337b: 5d                          	popq	%rbp
141db337c: c3                          	retq
141db337d: b9 08 00 00 00              	movl	$0x8, %ecx
141db3382: ba c0 01 00 00              	movl	$0x1c0, %edx            # imm = 0x1C0
141db3387: e8 74 5c b2 00              	callq	0x1428d9000 <.text+0x28d8000>
141db338c: b9 08 00 00 00              	movl	$0x8, %ecx
141db3391: ba 08 04 00 00              	movl	$0x408, %edx            # imm = 0x408
141db3396: e8 65 5c b2 00              	callq	0x1428d9000 <.text+0x28d8000>
141db339b: eb 20                       	jmp	0x141db33bd <.text+0x1db23bd>
141db339d: b9 08 00 00 00              	movl	$0x8, %ecx
141db33a2: ba f0 03 00 00              	movl	$0x3f0, %edx            # imm = 0x3F0
141db33a7: e8 54 5c b2 00              	callq	0x1428d9000 <.text+0x28d8000>
141db33ac: eb 0f                       	jmp	0x141db33bd <.text+0x1db23bd>
141db33ae: b9 08 00 00 00              	movl	$0x8, %ecx
141db33b3: ba 00 05 00 00              	movl	$0x500, %edx            # imm = 0x500
141db33b8: e8 43 5c b2 00              	callq	0x1428d9000 <.text+0x28d8000>
141db33bd: 0f 0b                       	ud2
141db33bf: 90                          	nop
141db33c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141db33c5: 55                          	pushq	%rbp
141db33c6: 56                          	pushq	%rsi
141db33c7: 57                          	pushq	%rdi
141db33c8: 53                          	pushq	%rbx
141db33c9: 48 83 ec 28                 	subq	$0x28, %rsp
141db33cd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141db33d4: 48 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %rcx
141db33db: e8 b0 eb fc ff              	callq	0x141d81f90 <.text+0x1d80f90>
141db33e0: 90                          	nop
141db33e1: 48 83 c4 28                 	addq	$0x28, %rsp
141db33e5: 5b                          	popq	%rbx
141db33e6: 5f                          	popq	%rdi
141db33e7: 5e                          	popq	%rsi
141db33e8: 5d                          	popq	%rbp
141db33e9: c3                          	retq
141db33ea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141db33f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141db33f5: 55                          	pushq	%rbp
141db33f6: 56                          	pushq	%rsi
141db33f7: 57                          	pushq	%rdi
141db33f8: 53                          	pushq	%rbx
141db33f9: 48 83 ec 28                 	subq	$0x28, %rsp
141db33fd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141db3404: ba c0 01 00 00              	movl	$0x1c0, %edx            # imm = 0x1C0
141db3409: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141db340f: 48 8b 8d f8 05 00 00        	movq	0x5f8(%rbp), %rcx
141db3416: e8 05 08 86 fe              	callq	0x140613c20 <.text+0x612c20>
141db341b: 90                          	nop
141db341c: 48 83 c4 28                 	addq	$0x28, %rsp
141db3420: 5b                          	popq	%rbx
141db3421: 5f                          	popq	%rdi
141db3422: 5e                          	popq	%rsi
141db3423: 5d                          	popq	%rbp
141db3424: c3                          	retq
141db3425: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
141db3430: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141db3435: 55                          	pushq	%rbp
141db3436: 56                          	pushq	%rsi
141db3437: 57                          	pushq	%rdi
141db3438: 53                          	pushq	%rbx
141db3439: 48 83 ec 28                 	subq	$0x28, %rsp
141db343d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141db3444: 48 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %rcx
141db344b: e8 60 01 fd ff              	callq	0x141d835b0 <.text+0x1d825b0>
141db3450: 90                          	nop
141db3451: 48 83 c4 28                 	addq	$0x28, %rsp
141db3455: 5b                          	popq	%rbx
141db3456: 5f                          	popq	%rdi
141db3457: 5e                          	popq	%rsi
141db3458: 5d                          	popq	%rbp
141db3459: c3                          	retq
141db345a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141db3460: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141db3465: 55                          	pushq	%rbp
141db3466: 56                          	pushq	%rsi
141db3467: 57                          	pushq	%rdi
141db3468: 53                          	pushq	%rbx
141db3469: 48 83 ec 28                 	subq	$0x28, %rsp
141db346d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141db3474: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
141db347b: e8 00 64 27 fe              	callq	0x140029880 <.text+0x28880>
141db3480: 90                          	nop
141db3481: 48 83 c4 28                 	addq	$0x28, %rsp
141db3485: 5b                          	popq	%rbx
141db3486: 5f                          	popq	%rdi
141db3487: 5e                          	popq	%rsi
141db3488: 5d                          	popq	%rbp
141db3489: c3                          	retq
141db348a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141db3490: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141db3495: 55                          	pushq	%rbp
141db3496: 56                          	pushq	%rsi
141db3497: 57                          	pushq	%rdi
141db3498: 53                          	pushq	%rbx
141db3499: 48 83 ec 28                 	subq	$0x28, %rsp
141db349d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141db34a4: 48 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %rcx
141db34ab: e8 70 e7 fc ff              	callq	0x141d81c20 <.text+0x1d80c20>
141db34b0: 90                          	nop
141db34b1: 48 83 c4 28                 	addq	$0x28, %rsp
141db34b5: 5b                          	popq	%rbx
141db34b6: 5f                          	popq	%rdi
141db34b7: 5e                          	popq	%rsi
141db34b8: 5d                          	popq	%rbp
141db34b9: c3                          	retq
141db34ba: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141db34c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141db34c5: 55                          	pushq	%rbp
141db34c6: 56                          	pushq	%rsi
141db34c7: 57                          	pushq	%rdi
141db34c8: 53                          	pushq	%rbx
141db34c9: 48 83 ec 28                 	subq	$0x28, %rsp
141db34cd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141db34d4: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
141db34d8: e8 a3 63 27 fe              	callq	0x140029880 <.text+0x28880>
141db34dd: 90                          	nop
141db34de: 48 83 c4 28                 	addq	$0x28, %rsp
141db34e2: 5b                          	popq	%rbx
141db34e3: 5f                          	popq	%rdi
141db34e4: 5e                          	popq	%rsi
141db34e5: 5d                          	popq	%rbp
141db34e6: c3                          	retq
141db34e7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141db34f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141db34f5: 55                          	pushq	%rbp
141db34f6: 56                          	pushq	%rsi
141db34f7: 57                          	pushq	%rdi
141db34f8: 53                          	pushq	%rbx
141db34f9: 48 83 ec 28                 	subq	$0x28, %rsp
141db34fd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141db3504: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
141db3508: e8 73 63 27 fe              	callq	0x140029880 <.text+0x28880>
141db350d: 90                          	nop
141db350e: 48 83 c4 28                 	addq	$0x28, %rsp
141db3512: 5b                          	popq	%rbx
141db3513: 5f                          	popq	%rdi
141db3514: 5e                          	popq	%rsi
141db3515: 5d                          	popq	%rbp
141db3516: c3                          	retq
