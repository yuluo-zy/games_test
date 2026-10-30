
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403d6ce0: 55                          	pushq	%rbp
1403d6ce1: 56                          	pushq	%rsi
1403d6ce2: 57                          	pushq	%rdi
1403d6ce3: 53                          	pushq	%rbx
1403d6ce4: 48 81 ec e8 03 00 00        	subq	$0x3e8, %rsp            # imm = 0x3E8
1403d6ceb: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1403d6cf3: 48 c7 85 60 03 00 00 fe ff ff ff    	movq	$-0x2, 0x360(%rbp)
1403d6cfe: 48 89 ce                    	movq	%rcx, %rsi
1403d6d01: 0f 10 42 60                 	movups	0x60(%rdx), %xmm0
1403d6d05: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
1403d6d0c: 0f 10 42 50                 	movups	0x50(%rdx), %xmm0
1403d6d10: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
1403d6d17: 0f 10 42 40                 	movups	0x40(%rdx), %xmm0
1403d6d1b: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
1403d6d22: 0f 10 02                    	movups	(%rdx), %xmm0
1403d6d25: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
1403d6d29: 0f 10 52 20                 	movups	0x20(%rdx), %xmm2
1403d6d2d: 0f 10 5a 30                 	movups	0x30(%rdx), %xmm3
1403d6d31: 0f 29 9d 20 01 00 00        	movaps	%xmm3, 0x120(%rbp)
1403d6d38: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
1403d6d3f: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
1403d6d46: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1403d6d4d: 0f b6 05 b5 de 25 03        	movzbl	0x325deb5(%rip), %eax   # 0x143634c09
1403d6d54: b9 50 01 00 00              	movl	$0x150, %ecx            # imm = 0x150
1403d6d59: ba 08 00 00 00              	movl	$0x8, %edx
1403d6d5e: e8 ad ce 23 00              	callq	0x140613c10 <.text+0x612c10>
1403d6d63: 48 85 c0                    	testq	%rax, %rax
1403d6d66: 0f 84 1a 05 00 00           	je	0x1403d7286 <.text+0x3d6286>
1403d6d6c: 48 89 85 50 03 00 00        	movq	%rax, 0x350(%rbp)
1403d6d73: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403d6d76: 0f 11 85 80 01 00 00        	movups	%xmm0, 0x180(%rbp)
1403d6d7d: 0f 11 85 98 01 00 00        	movups	%xmm0, 0x198(%rbp)
1403d6d84: 0f 11 85 b0 01 00 00        	movups	%xmm0, 0x1b0(%rbp)
1403d6d8b: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
1403d6d92: 0f 11 85 e0 01 00 00        	movups	%xmm0, 0x1e0(%rbp)
1403d6d99: c7 85 f0 01 00 00 00 00 00 00       	movl	$0x0, 0x1f0(%rbp)
1403d6da3: 0f 11 85 18 02 00 00        	movups	%xmm0, 0x218(%rbp)
1403d6daa: 0f 11 85 30 02 00 00        	movups	%xmm0, 0x230(%rbp)
1403d6db1: 0f 11 85 48 02 00 00        	movups	%xmm0, 0x248(%rbp)
1403d6db8: 0f 11 85 60 02 00 00        	movups	%xmm0, 0x260(%rbp)
1403d6dbf: 0f 11 85 78 02 00 00        	movups	%xmm0, 0x278(%rbp)
1403d6dc6: c7 85 88 02 00 00 00 00 00 00       	movl	$0x0, 0x288(%rbp)
1403d6dd0: 48 c7 85 60 01 00 00 00 00 00 00    	movq	$0x0, 0x160(%rbp)
1403d6ddb: 48 c7 85 68 01 00 00 08 00 00 00    	movq	$0x8, 0x168(%rbp)
1403d6de6: 48 c7 85 70 01 00 00 00 00 00 00    	movq	$0x0, 0x170(%rbp)
1403d6df1: 48 c7 85 78 01 00 00 10 00 00 00    	movq	$0x10, 0x178(%rbp)
1403d6dfc: 48 c7 85 90 01 00 00 10 00 00 00    	movq	$0x10, 0x190(%rbp)
1403d6e07: 48 c7 85 a8 01 00 00 10 00 00 00    	movq	$0x10, 0x1a8(%rbp)
1403d6e12: 48 c7 85 c0 01 00 00 10 00 00 00    	movq	$0x10, 0x1c0(%rbp)
1403d6e1d: 48 c7 85 d8 01 00 00 10 00 00 00    	movq	$0x10, 0x1d8(%rbp)
1403d6e28: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
1403d6e32: 48 89 9d f8 01 00 00        	movq	%rbx, 0x1f8(%rbp)
1403d6e39: 48 8d 05 4a c0 5f 02        	leaq	0x25fc04a(%rip), %rax   # 0x1429d2e8a
1403d6e40: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1403d6e47: 48 c7 85 08 02 00 00 33 00 00 00    	movq	$0x33, 0x208(%rbp)
1403d6e52: 48 c7 85 10 02 00 00 10 00 00 00    	movq	$0x10, 0x210(%rbp)
1403d6e5d: 48 c7 85 28 02 00 00 10 00 00 00    	movq	$0x10, 0x228(%rbp)
1403d6e68: 48 c7 85 40 02 00 00 10 00 00 00    	movq	$0x10, 0x240(%rbp)
1403d6e73: 48 c7 85 58 02 00 00 10 00 00 00    	movq	$0x10, 0x258(%rbp)
1403d6e7e: 48 c7 85 70 02 00 00 10 00 00 00    	movq	$0x10, 0x270(%rbp)
1403d6e89: c7 85 90 02 00 00 00 00 00 00       	movl	$0x0, 0x290(%rbp)
1403d6e93: 66 c7 85 94 02 00 00 01 00  	movw	$0x1, 0x294(%rbp)
1403d6e9c: 48 89 9d 98 02 00 00        	movq	%rbx, 0x298(%rbp)
1403d6ea3: c7 85 48 03 00 00 00 00 00 00       	movl	$0x0, 0x348(%rbp)
1403d6ead: 0f b6 05 55 dd 25 03        	movzbl	0x325dd55(%rip), %eax   # 0x143634c09
1403d6eb4: b9 f0 01 00 00              	movl	$0x1f0, %ecx            # imm = 0x1F0
1403d6eb9: ba 08 00 00 00              	movl	$0x8, %edx
1403d6ebe: e8 4d cd 23 00              	callq	0x140613c10 <.text+0x612c10>
1403d6ec3: 48 85 c0                    	testq	%rax, %rax
1403d6ec6: 0f 84 d2 03 00 00           	je	0x1403d729e <.text+0x3d629e>
1403d6ecc: 48 89 c7                    	movq	%rax, %rdi
1403d6ecf: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
1403d6ed6: 41 b8 f0 01 00 00           	movl	$0x1f0, %r8d            # imm = 0x1F0
1403d6edc: 48 89 c1                    	movq	%rax, %rcx
1403d6edf: e8 1c cd 54 02              	callq	0x142923c00 <.text+0x2922c00>
1403d6ee4: c6 85 5f 03 00 00 01        	movb	$0x1, 0x35f(%rbp)
1403d6eeb: 4c 8d 05 f6 bd 5f 02        	leaq	0x25fbdf6(%rip), %r8    # 0x1429d2ce8
1403d6ef2: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1403d6ef9: 48 89 fa                    	movq	%rdi, %rdx
1403d6efc: e8 0f dc 90 00              	callq	0x140ce4b10 <.text+0xce3b10>
1403d6f01: 0f 28 85 50 01 00 00        	movaps	0x150(%rbp), %xmm0
1403d6f08: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
1403d6f0f: 0f 28 85 40 01 00 00        	movaps	0x140(%rbp), %xmm0
1403d6f16: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
1403d6f1d: 0f 28 85 30 01 00 00        	movaps	0x130(%rbp), %xmm0
1403d6f24: 0f 29 85 a0 01 00 00        	movaps	%xmm0, 0x1a0(%rbp)
1403d6f2b: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
1403d6f32: 0f 28 8d 00 01 00 00        	movaps	0x100(%rbp), %xmm1
1403d6f39: 0f 28 95 10 01 00 00        	movaps	0x110(%rbp), %xmm2
1403d6f40: 0f 28 9d 20 01 00 00        	movaps	0x120(%rbp), %xmm3
1403d6f47: 0f 29 9d 90 01 00 00        	movaps	%xmm3, 0x190(%rbp)
1403d6f4e: 0f 29 95 80 01 00 00        	movaps	%xmm2, 0x180(%rbp)
1403d6f55: 0f 29 8d 70 01 00 00        	movaps	%xmm1, 0x170(%rbp)
1403d6f5c: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
1403d6f63: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1403d6f67: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
1403d6f6e: e8 1d d3 cf ff              	callq	0x1400d4290 <.text+0xd3290>
1403d6f73: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403d6f76: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
1403d6f7d: 0f 11 85 b8 01 00 00        	movups	%xmm0, 0x1b8(%rbp)
1403d6f84: 0f 11 85 d0 01 00 00        	movups	%xmm0, 0x1d0(%rbp)
1403d6f8b: 0f 11 85 e8 01 00 00        	movups	%xmm0, 0x1e8(%rbp)
1403d6f92: 0f 11 85 00 02 00 00        	movups	%xmm0, 0x200(%rbp)
1403d6f99: c7 85 10 02 00 00 00 00 00 00       	movl	$0x0, 0x210(%rbp)
1403d6fa3: 0f 11 85 38 02 00 00        	movups	%xmm0, 0x238(%rbp)
1403d6faa: 0f 11 85 50 02 00 00        	movups	%xmm0, 0x250(%rbp)
1403d6fb1: 0f 11 85 68 02 00 00        	movups	%xmm0, 0x268(%rbp)
1403d6fb8: 0f 11 85 80 02 00 00        	movups	%xmm0, 0x280(%rbp)
1403d6fbf: 0f 11 85 98 02 00 00        	movups	%xmm0, 0x298(%rbp)
1403d6fc6: c7 85 a8 02 00 00 00 00 00 00       	movl	$0x0, 0x2a8(%rbp)
1403d6fd0: 48 c7 85 60 01 00 00 00 00 00 00    	movq	$0x0, 0x160(%rbp)
1403d6fdb: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
1403d6fe6: 48 c7 85 88 01 00 00 08 00 00 00    	movq	$0x8, 0x188(%rbp)
1403d6ff1: 48 c7 85 90 01 00 00 00 00 00 00    	movq	$0x0, 0x190(%rbp)
1403d6ffc: 48 c7 85 98 01 00 00 10 00 00 00    	movq	$0x10, 0x198(%rbp)
1403d7007: 48 c7 85 b0 01 00 00 10 00 00 00    	movq	$0x10, 0x1b0(%rbp)
1403d7012: 48 c7 85 c8 01 00 00 10 00 00 00    	movq	$0x10, 0x1c8(%rbp)
1403d701d: 48 c7 85 e0 01 00 00 10 00 00 00    	movq	$0x10, 0x1e0(%rbp)
1403d7028: 48 c7 85 f8 01 00 00 10 00 00 00    	movq	$0x10, 0x1f8(%rbp)
1403d7033: 48 89 9d 18 02 00 00        	movq	%rbx, 0x218(%rbp)
1403d703a: 48 8d 05 ac bd 5f 02        	leaq	0x25fbdac(%rip), %rax   # 0x1429d2ded
1403d7041: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
1403d7048: 48 c7 85 28 02 00 00 34 00 00 00    	movq	$0x34, 0x228(%rbp)
1403d7053: 48 c7 85 30 02 00 00 10 00 00 00    	movq	$0x10, 0x230(%rbp)
1403d705e: 48 c7 85 48 02 00 00 10 00 00 00    	movq	$0x10, 0x248(%rbp)
1403d7069: 48 c7 85 60 02 00 00 10 00 00 00    	movq	$0x10, 0x260(%rbp)
1403d7074: 48 c7 85 78 02 00 00 10 00 00 00    	movq	$0x10, 0x278(%rbp)
1403d707f: 48 c7 85 90 02 00 00 10 00 00 00    	movq	$0x10, 0x290(%rbp)
1403d708a: c7 85 b0 02 00 00 00 00 00 00       	movl	$0x0, 0x2b0(%rbp)
1403d7094: 66 c7 85 b4 02 00 00 01 00  	movw	$0x1, 0x2b4(%rbp)
1403d709d: c7 85 b8 02 00 00 00 00 00 00       	movl	$0x0, 0x2b8(%rbp)
1403d70a7: 0f b6 05 5b db 25 03        	movzbl	0x325db5b(%rip), %eax   # 0x143634c09
1403d70ae: b9 60 01 00 00              	movl	$0x160, %ecx            # imm = 0x160
1403d70b3: ba 08 00 00 00              	movl	$0x8, %edx
1403d70b8: e8 53 cb 23 00              	callq	0x140613c10 <.text+0x612c10>
1403d70bd: 48 85 c0                    	testq	%rax, %rax
1403d70c0: 0f 84 e9 01 00 00           	je	0x1403d72af <.text+0x3d62af>
1403d70c6: 48 89 c7                    	movq	%rax, %rdi
1403d70c9: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
1403d70d0: 41 b8 60 01 00 00           	movl	$0x160, %r8d            # imm = 0x160
1403d70d6: 48 89 c1                    	movq	%rax, %rcx
1403d70d9: e8 22 cb 54 02              	callq	0x142923c00 <.text+0x2922c00>
1403d70de: 4c 8d 05 93 ba 5f 02        	leaq	0x25fba93(%rip), %r8    # 0x1429d2b78
1403d70e5: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
1403d70e9: 48 89 fa                    	movq	%rdi, %rdx
1403d70ec: e8 1f da 90 00              	callq	0x140ce4b10 <.text+0xce3b10>
1403d70f1: 0f 10 85 e0 00 00 00        	movups	0xe0(%rbp), %xmm0
1403d70f8: 48 8b 85 50 03 00 00        	movq	0x350(%rbp), %rax
1403d70ff: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
1403d7103: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
1403d710a: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
1403d710e: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
1403d7115: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
1403d7119: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
1403d7120: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
1403d7127: 0f 10 95 a0 00 00 00        	movups	0xa0(%rbp), %xmm2
1403d712e: 0f 10 9d b0 00 00 00        	movups	0xb0(%rbp), %xmm3
1403d7135: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1403d7139: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1403d713d: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1403d7141: 0f 11 00                    	movups	%xmm0, (%rax)
1403d7144: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
1403d7148: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
1403d714c: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
1403d7150: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
1403d7154: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
1403d7158: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
1403d715f: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
1403d7166: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
1403d716d: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
1403d7171: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
1403d7178: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
1403d717c: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
1403d7183: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
1403d7187: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
1403d718e: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
1403d7192: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
1403d7196: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
1403d719a: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
1403d719e: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
1403d71a5: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
1403d71ac: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
1403d71b3: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
1403d71ba: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
1403d71be: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
1403d71c5: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
1403d71c9: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
1403d71d0: 0f 10 45 00                 	movups	(%rbp), %xmm0
1403d71d4: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
1403d71db: 48 c7 85 68 01 00 00 03 00 00 00    	movq	$0x3, 0x168(%rbp)
1403d71e6: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1403d71ed: 48 c7 85 78 01 00 00 03 00 00 00    	movq	$0x3, 0x178(%rbp)
1403d71f8: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
1403d7203: 48 c7 85 88 01 00 00 08 00 00 00    	movq	$0x8, 0x188(%rbp)
1403d720e: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403d7211: 0f 11 85 90 01 00 00        	movups	%xmm0, 0x190(%rbp)
1403d7218: 48 89 9d 60 01 00 00        	movq	%rbx, 0x160(%rbp)
1403d721f: 48 8d 8d 98 01 00 00        	leaq	0x198(%rbp), %rcx
1403d7226: e8 a5 93 d9 ff              	callq	0x1401705d0 <.text+0x16f5d0>
1403d722b: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
1403d7232: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
1403d7236: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
1403d723d: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
1403d7241: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
1403d7248: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
1403d724c: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
1403d7253: 0f 10 8d 70 01 00 00        	movups	0x170(%rbp), %xmm1
1403d725a: 0f 10 95 80 01 00 00        	movups	0x180(%rbp), %xmm2
1403d7261: 0f 10 9d 90 01 00 00        	movups	0x190(%rbp), %xmm3
1403d7268: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
1403d726c: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
1403d7270: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
1403d7274: 0f 11 06                    	movups	%xmm0, (%rsi)
1403d7277: 48 89 f0                    	movq	%rsi, %rax
1403d727a: 48 81 c4 e8 03 00 00        	addq	$0x3e8, %rsp            # imm = 0x3E8
1403d7281: 5b                          	popq	%rbx
1403d7282: 5f                          	popq	%rdi
1403d7283: 5e                          	popq	%rsi
1403d7284: 5d                          	popq	%rbp
1403d7285: c3                          	retq
1403d7286: c6 85 5e 03 00 00 01        	movb	$0x1, 0x35e(%rbp)
1403d728d: b9 08 00 00 00              	movl	$0x8, %ecx
1403d7292: ba 50 01 00 00              	movl	$0x150, %edx            # imm = 0x150
1403d7297: e8 64 1d 50 02              	callq	0x1428d9000 <.text+0x28d8000>
1403d729c: eb 20                       	jmp	0x1403d72be <.text+0x3d62be>
1403d729e: b9 08 00 00 00              	movl	$0x8, %ecx
1403d72a3: ba f0 01 00 00              	movl	$0x1f0, %edx            # imm = 0x1F0
1403d72a8: e8 53 1d 50 02              	callq	0x1428d9000 <.text+0x28d8000>
1403d72ad: eb 0f                       	jmp	0x1403d72be <.text+0x3d62be>
1403d72af: b9 08 00 00 00              	movl	$0x8, %ecx
1403d72b4: ba 60 01 00 00              	movl	$0x160, %edx            # imm = 0x160
1403d72b9: e8 42 1d 50 02              	callq	0x1428d9000 <.text+0x28d8000>
