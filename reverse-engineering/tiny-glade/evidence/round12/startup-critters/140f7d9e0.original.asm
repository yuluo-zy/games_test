
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f7d9e0: 55                          	pushq	%rbp
140f7d9e1: 41 56                       	pushq	%r14
140f7d9e3: 56                          	pushq	%rsi
140f7d9e4: 57                          	pushq	%rdi
140f7d9e5: 53                          	pushq	%rbx
140f7d9e6: 48 81 ec 10 03 00 00        	subq	$0x310, %rsp            # imm = 0x310
140f7d9ed: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140f7d9f5: 48 c7 85 88 02 00 00 fe ff ff ff    	movq	$-0x2, 0x288(%rbp)
140f7da00: 89 d3                       	movl	%edx, %ebx
140f7da02: 48 89 ce                    	movq	%rcx, %rsi
140f7da05: 0f b6 05 fd 71 6b 02        	movzbl	0x26b71fd(%rip), %eax   # 0x143634c09
140f7da0c: b9 e0 00 00 00              	movl	$0xe0, %ecx
140f7da11: ba 08 00 00 00              	movl	$0x8, %edx
140f7da16: e8 f5 61 69 ff              	callq	0x140613c10 <.text+0x612c10>
140f7da1b: 48 85 c0                    	testq	%rax, %rax
140f7da1e: 0f 84 98 02 00 00           	je	0x140f7dcbc <.text+0xf7ccbc>
140f7da24: 48 89 85 80 02 00 00        	movq	%rax, 0x280(%rbp)
140f7da2b: 0f 57 c0                    	xorps	%xmm0, %xmm0
140f7da2e: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
140f7da32: 0f 11 45 d8                 	movups	%xmm0, -0x28(%rbp)
140f7da36: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
140f7da3a: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
140f7da3e: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
140f7da42: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
140f7da49: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
140f7da4d: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
140f7da51: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
140f7da58: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
140f7da5f: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
140f7da66: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
140f7da70: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
140f7da78: 48 c7 45 a8 08 00 00 00     	movq	$0x8, -0x58(%rbp)
140f7da80: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
140f7da88: 48 c7 45 b8 10 00 00 00     	movq	$0x10, -0x48(%rbp)
140f7da90: 48 c7 45 d0 10 00 00 00     	movq	$0x10, -0x30(%rbp)
140f7da98: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
140f7daa0: 48 c7 45 00 10 00 00 00     	movq	$0x10, (%rbp)
140f7daa8: 48 c7 45 18 10 00 00 00     	movq	$0x10, 0x18(%rbp)
140f7dab0: 49 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
140f7daba: 4c 89 75 38                 	movq	%r14, 0x38(%rbp)
140f7dabe: 48 8d 05 65 e6 c2 01        	leaq	0x1c2e665(%rip), %rax   # 0x142bac12a
140f7dac5: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140f7dac9: 48 c7 45 48 33 00 00 00     	movq	$0x33, 0x48(%rbp)
140f7dad1: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
140f7dad9: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
140f7dae1: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
140f7daec: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
140f7daf7: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
140f7db02: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
140f7db0c: 66 c7 85 d4 00 00 00 01 00  	movw	$0x1, 0xd4(%rbp)
140f7db15: 4c 89 b5 d0 01 00 00        	movq	%r14, 0x1d0(%rbp)
140f7db1c: c7 85 08 02 00 00 00 00 00 00       	movl	$0x0, 0x208(%rbp)
140f7db26: 0f b6 05 dc 70 6b 02        	movzbl	0x26b70dc(%rip), %eax   # 0x143634c09
140f7db2d: b9 70 02 00 00              	movl	$0x270, %ecx            # imm = 0x270
140f7db32: ba 08 00 00 00              	movl	$0x8, %edx
140f7db37: e8 d4 60 69 ff              	callq	0x140613c10 <.text+0x612c10>
140f7db3c: 48 85 c0                    	testq	%rax, %rax
140f7db3f: 0f 84 86 01 00 00           	je	0x140f7dccb <.text+0xf7cccb>
140f7db45: 48 89 c7                    	movq	%rax, %rdi
140f7db48: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140f7db4c: 41 b8 70 02 00 00           	movl	$0x270, %r8d            # imm = 0x270
140f7db52: 48 89 c1                    	movq	%rax, %rcx
140f7db55: e8 a6 60 9a 01              	callq	0x142923c00 <.text+0x2922c00>
140f7db5a: 4c 8d 05 df e4 c2 01        	leaq	0x1c2e4df(%rip), %r8    # 0x142bac040
140f7db61: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
140f7db68: 48 89 fa                    	movq	%rdi, %rdx
140f7db6b: e8 a0 6f d6 ff              	callq	0x140ce4b10 <.text+0xce3b10>
140f7db70: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140f7db74: e8 77 3a e1 00              	callq	0x141d915f0 <.text+0x1d905f0>
140f7db79: 0f 10 85 70 02 00 00        	movups	0x270(%rbp), %xmm0
140f7db80: 48 8b 85 80 02 00 00        	movq	0x280(%rbp), %rax
140f7db87: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140f7db8b: 0f 10 85 60 02 00 00        	movups	0x260(%rbp), %xmm0
140f7db92: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140f7db96: 0f 10 85 50 02 00 00        	movups	0x250(%rbp), %xmm0
140f7db9d: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140f7dba1: 0f 10 85 10 02 00 00        	movups	0x210(%rbp), %xmm0
140f7dba8: 0f 10 8d 20 02 00 00        	movups	0x220(%rbp), %xmm1
140f7dbaf: 0f 10 95 30 02 00 00        	movups	0x230(%rbp), %xmm2
140f7dbb6: 0f 10 9d 40 02 00 00        	movups	0x240(%rbp), %xmm3
140f7dbbd: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140f7dbc1: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140f7dbc5: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140f7dbc9: 0f 11 00                    	movups	%xmm0, (%rax)
140f7dbcc: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140f7dbd0: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140f7dbd4: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
140f7dbd8: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
140f7dbdc: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140f7dbe0: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140f7dbe7: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140f7dbee: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140f7dbf5: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140f7dbf9: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140f7dc00: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140f7dc04: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140f7dc0b: 0f 10 45 00                 	movups	(%rbp), %xmm0
140f7dc0f: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140f7dc16: 48 c7 45 a8 02 00 00 00     	movq	$0x2, -0x58(%rbp)
140f7dc1e: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140f7dc22: 48 c7 45 b8 02 00 00 00     	movq	$0x2, -0x48(%rbp)
140f7dc2a: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
140f7dc32: 48 c7 45 c8 08 00 00 00     	movq	$0x8, -0x38(%rbp)
140f7dc3a: 0f 57 c0                    	xorps	%xmm0, %xmm0
140f7dc3d: 0f 11 45 d0                 	movups	%xmm0, -0x30(%rbp)
140f7dc41: 4c 89 75 a0                 	movq	%r14, -0x60(%rbp)
140f7dc45: 88 9d 10 02 00 00           	movb	%bl, 0x210(%rbp)
140f7dc4b: 48 8b 0d 1e e7 6a 02        	movq	0x26ae71e(%rip), %rcx   # 0x14362c370
140f7dc52: 4c 8d 05 e7 dc fe 01        	leaq	0x1fedce7(%rip), %r8    # 0x142f6b940
140f7dc59: 48 8d 95 10 02 00 00        	leaq	0x210(%rbp), %rdx
140f7dc60: e8 8b 4a e1 00              	callq	0x141d926f0 <.text+0x1d916f0>
140f7dc65: 49 89 d0                    	movq	%rdx, %r8
140f7dc68: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140f7dc6c: 48 89 c2                    	movq	%rax, %rdx
140f7dc6f: e8 1c 29 00 00              	callq	0x140f80590 <.text+0xf7f590>
140f7dc74: 0f 10 45 00                 	movups	(%rbp), %xmm0
140f7dc78: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
140f7dc7c: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140f7dc80: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
140f7dc84: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140f7dc88: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
140f7dc8c: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140f7dc90: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140f7dc94: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
140f7dc98: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
140f7dc9c: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
140f7dca0: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140f7dca4: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140f7dca8: 0f 11 06                    	movups	%xmm0, (%rsi)
140f7dcab: 48 89 f0                    	movq	%rsi, %rax
140f7dcae: 48 81 c4 10 03 00 00        	addq	$0x310, %rsp            # imm = 0x310
140f7dcb5: 5b                          	popq	%rbx
140f7dcb6: 5f                          	popq	%rdi
140f7dcb7: 5e                          	popq	%rsi
140f7dcb8: 41 5e                       	popq	%r14
140f7dcba: 5d                          	popq	%rbp
140f7dcbb: c3                          	retq
140f7dcbc: b9 08 00 00 00              	movl	$0x8, %ecx
140f7dcc1: ba e0 00 00 00              	movl	$0xe0, %edx
140f7dcc6: e8 35 b3 95 01              	callq	0x1428d9000 <.text+0x28d8000>
140f7dccb: b9 08 00 00 00              	movl	$0x8, %ecx
140f7dcd0: ba 70 02 00 00              	movl	$0x270, %edx            # imm = 0x270
140f7dcd5: e8 26 b3 95 01              	callq	0x1428d9000 <.text+0x28d8000>
140f7dcda: 0f 0b                       	ud2
140f7dcdc: 0f 1f 40 00                 	nopl	(%rax)
140f7dce0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7dce5: 55                          	pushq	%rbp
140f7dce6: 41 56                       	pushq	%r14
140f7dce8: 56                          	pushq	%rsi
140f7dce9: 57                          	pushq	%rdi
140f7dcea: 53                          	pushq	%rbx
140f7dceb: 48 83 ec 20                 	subq	$0x20, %rsp
140f7dcef: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7dcf6: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140f7dcfa: e8 b1 16 65 ff              	callq	0x1405cf3b0 <.text+0x5ce3b0>
140f7dcff: 90                          	nop
140f7dd00: 48 83 c4 20                 	addq	$0x20, %rsp
140f7dd04: 5b                          	popq	%rbx
140f7dd05: 5f                          	popq	%rdi
140f7dd06: 5e                          	popq	%rsi
140f7dd07: 41 5e                       	popq	%r14
140f7dd09: 5d                          	popq	%rbp
140f7dd0a: c3                          	retq
140f7dd0b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140f7dd10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7dd15: 55                          	pushq	%rbp
140f7dd16: 41 56                       	pushq	%r14
140f7dd18: 56                          	pushq	%rsi
140f7dd19: 57                          	pushq	%rdi
140f7dd1a: 53                          	pushq	%rbx
140f7dd1b: 48 83 ec 20                 	subq	$0x20, %rsp
140f7dd1f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7dd26: ba e0 00 00 00              	movl	$0xe0, %edx
140f7dd2b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140f7dd31: 48 8b 8d 80 02 00 00        	movq	0x280(%rbp), %rcx
140f7dd38: e8 e3 5e 69 ff              	callq	0x140613c20 <.text+0x612c20>
140f7dd3d: 90                          	nop
140f7dd3e: 48 83 c4 20                 	addq	$0x20, %rsp
140f7dd42: 5b                          	popq	%rbx
140f7dd43: 5f                          	popq	%rdi
140f7dd44: 5e                          	popq	%rsi
140f7dd45: 41 5e                       	popq	%r14
140f7dd47: 5d                          	popq	%rbp
140f7dd48: c3                          	retq
140f7dd49: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140f7dd50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7dd55: 55                          	pushq	%rbp
140f7dd56: 41 56                       	pushq	%r14
140f7dd58: 56                          	pushq	%rsi
140f7dd59: 57                          	pushq	%rdi
140f7dd5a: 53                          	pushq	%rbx
140f7dd5b: 48 83 ec 20                 	subq	$0x20, %rsp
140f7dd5f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7dd66: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
140f7dd6d: e8 0e bb 0a ff              	callq	0x140029880 <.text+0x28880>
140f7dd72: 90                          	nop
140f7dd73: 48 83 c4 20                 	addq	$0x20, %rsp
140f7dd77: 5b                          	popq	%rbx
140f7dd78: 5f                          	popq	%rdi
140f7dd79: 5e                          	popq	%rsi
140f7dd7a: 41 5e                       	popq	%r14
140f7dd7c: 5d                          	popq	%rbp
140f7dd7d: c3                          	retq
140f7dd7e: 66 90                       	nop
140f7dd80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7dd85: 55                          	pushq	%rbp
140f7dd86: 41 56                       	pushq	%r14
140f7dd88: 56                          	pushq	%rsi
140f7dd89: 57                          	pushq	%rdi
140f7dd8a: 53                          	pushq	%rbx
140f7dd8b: 48 83 ec 20                 	subq	$0x20, %rsp
140f7dd8f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7dd96: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140f7dd9a: e8 e1 ba 0a ff              	callq	0x140029880 <.text+0x28880>
140f7dd9f: 90                          	nop
140f7dda0: 48 83 c4 20                 	addq	$0x20, %rsp
140f7dda4: 5b                          	popq	%rbx
140f7dda5: 5f                          	popq	%rdi
140f7dda6: 5e                          	popq	%rsi
140f7dda7: 41 5e                       	popq	%r14
140f7dda9: 5d                          	popq	%rbp
140f7ddaa: c3                          	retq
