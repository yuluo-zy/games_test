
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f7cb50: 55                          	pushq	%rbp
140f7cb51: 56                          	pushq	%rsi
140f7cb52: 57                          	pushq	%rdi
140f7cb53: 53                          	pushq	%rbx
140f7cb54: 48 81 ec 78 03 00 00        	subq	$0x378, %rsp            # imm = 0x378
140f7cb5b: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140f7cb63: 48 c7 85 f0 02 00 00 fe ff ff ff    	movq	$-0x2, 0x2f0(%rbp)
140f7cb6e: 48 89 ce                    	movq	%rcx, %rsi
140f7cb71: 0f b6 05 91 80 6b 02        	movzbl	0x26b8091(%rip), %eax   # 0x143634c09
140f7cb78: b9 50 01 00 00              	movl	$0x150, %ecx            # imm = 0x150
140f7cb7d: ba 08 00 00 00              	movl	$0x8, %edx
140f7cb82: e8 89 70 69 ff              	callq	0x140613c10 <.text+0x612c10>
140f7cb87: 48 85 c0                    	testq	%rax, %rax
140f7cb8a: 0f 84 9d 02 00 00           	je	0x140f7ce2d <.text+0xf7be2d>
140f7cb90: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
140f7cb97: 0f 57 c0                    	xorps	%xmm0, %xmm0
140f7cb9a: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
140f7cb9e: 0f 11 45 d8                 	movups	%xmm0, -0x28(%rbp)
140f7cba2: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
140f7cba6: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
140f7cbaa: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
140f7cbae: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
140f7cbb5: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
140f7cbb9: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
140f7cbbd: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
140f7cbc4: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
140f7cbcb: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
140f7cbd2: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
140f7cbdc: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
140f7cbe4: 48 c7 45 a8 08 00 00 00     	movq	$0x8, -0x58(%rbp)
140f7cbec: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
140f7cbf4: 48 c7 45 b8 10 00 00 00     	movq	$0x10, -0x48(%rbp)
140f7cbfc: 48 c7 45 d0 10 00 00 00     	movq	$0x10, -0x30(%rbp)
140f7cc04: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
140f7cc0c: 48 c7 45 00 10 00 00 00     	movq	$0x10, (%rbp)
140f7cc14: 48 c7 45 18 10 00 00 00     	movq	$0x10, 0x18(%rbp)
140f7cc1c: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140f7cc26: 48 89 5d 38                 	movq	%rbx, 0x38(%rbp)
140f7cc2a: 48 8d 05 c7 f4 c2 01        	leaq	0x1c2f4c7(%rip), %rax   # 0x142bac0f8
140f7cc31: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140f7cc35: 48 c7 45 48 32 00 00 00     	movq	$0x32, 0x48(%rbp)
140f7cc3d: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
140f7cc45: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
140f7cc4d: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
140f7cc58: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
140f7cc63: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
140f7cc6e: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
140f7cc78: 66 c7 85 d4 00 00 00 01 00  	movw	$0x1, 0xd4(%rbp)
140f7cc81: 48 89 9d d0 01 00 00        	movq	%rbx, 0x1d0(%rbp)
140f7cc88: c7 85 00 02 00 00 00 00 00 00       	movl	$0x0, 0x200(%rbp)
140f7cc92: 0f b6 05 70 7f 6b 02        	movzbl	0x26b7f70(%rip), %eax   # 0x143634c09
140f7cc99: b9 68 02 00 00              	movl	$0x268, %ecx            # imm = 0x268
140f7cc9e: ba 08 00 00 00              	movl	$0x8, %edx
140f7cca3: e8 68 6f 69 ff              	callq	0x140613c10 <.text+0x612c10>
140f7cca8: 48 85 c0                    	testq	%rax, %rax
140f7ccab: 0f 84 8b 01 00 00           	je	0x140f7ce3c <.text+0xf7be3c>
140f7ccb1: 48 89 c7                    	movq	%rax, %rdi
140f7ccb4: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140f7ccb8: 41 b8 68 02 00 00           	movl	$0x268, %r8d            # imm = 0x268
140f7ccbe: 48 89 c1                    	movq	%rax, %rcx
140f7ccc1: e8 3a 6f 9a 01              	callq	0x142923c00 <.text+0x2922c00>
140f7ccc6: 4c 8d 05 03 f2 c2 01        	leaq	0x1c2f203(%rip), %r8    # 0x142babed0
140f7cccd: 48 8d 8d 78 02 00 00        	leaq	0x278(%rbp), %rcx
140f7ccd4: 48 89 fa                    	movq	%rdi, %rdx
140f7ccd7: e8 34 7e d6 ff              	callq	0x140ce4b10 <.text+0xce3b10>
140f7ccdc: 48 8d 8d 08 02 00 00        	leaq	0x208(%rbp), %rcx
140f7cce3: e8 48 f5 e0 00              	callq	0x141d8c230 <.text+0x1d8b230>
140f7cce8: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140f7ccec: e8 2f 32 e1 00              	callq	0x141d8ff20 <.text+0x1d8ef20>
140f7ccf1: 0f 10 85 d8 02 00 00        	movups	0x2d8(%rbp), %xmm0
140f7ccf8: 48 8b 85 e8 02 00 00        	movq	0x2e8(%rbp), %rax
140f7ccff: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140f7cd03: 0f 10 85 c8 02 00 00        	movups	0x2c8(%rbp), %xmm0
140f7cd0a: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140f7cd0e: 0f 10 85 b8 02 00 00        	movups	0x2b8(%rbp), %xmm0
140f7cd15: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140f7cd19: 0f 10 85 78 02 00 00        	movups	0x278(%rbp), %xmm0
140f7cd20: 0f 10 8d 88 02 00 00        	movups	0x288(%rbp), %xmm1
140f7cd27: 0f 10 95 98 02 00 00        	movups	0x298(%rbp), %xmm2
140f7cd2e: 0f 10 9d a8 02 00 00        	movups	0x2a8(%rbp), %xmm3
140f7cd35: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140f7cd39: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140f7cd3d: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140f7cd41: 0f 11 00                    	movups	%xmm0, (%rax)
140f7cd44: 0f 10 85 08 02 00 00        	movups	0x208(%rbp), %xmm0
140f7cd4b: 0f 10 8d 18 02 00 00        	movups	0x218(%rbp), %xmm1
140f7cd52: 0f 10 95 28 02 00 00        	movups	0x228(%rbp), %xmm2
140f7cd59: 0f 10 9d 38 02 00 00        	movups	0x238(%rbp), %xmm3
140f7cd60: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140f7cd64: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140f7cd6b: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140f7cd72: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140f7cd79: 0f 10 85 48 02 00 00        	movups	0x248(%rbp), %xmm0
140f7cd80: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140f7cd87: 0f 10 85 58 02 00 00        	movups	0x258(%rbp), %xmm0
140f7cd8e: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140f7cd95: 0f 10 85 68 02 00 00        	movups	0x268(%rbp), %xmm0
140f7cd9c: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140f7cda3: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140f7cda7: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140f7cdab: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
140f7cdaf: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
140f7cdb3: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
140f7cdba: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
140f7cdc1: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
140f7cdc8: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
140f7cdcf: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140f7cdd3: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
140f7cdda: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140f7cdde: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
140f7cde5: 0f 10 45 00                 	movups	(%rbp), %xmm0
140f7cde9: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
140f7cdf0: 48 c7 46 08 03 00 00 00     	movq	$0x3, 0x8(%rsi)
140f7cdf8: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
140f7cdfc: 48 c7 46 18 03 00 00 00     	movq	$0x3, 0x18(%rsi)
140f7ce04: 48 c7 46 20 00 00 00 00     	movq	$0x0, 0x20(%rsi)
140f7ce0c: 48 c7 46 28 08 00 00 00     	movq	$0x8, 0x28(%rsi)
140f7ce14: 0f 57 c0                    	xorps	%xmm0, %xmm0
140f7ce17: 0f 11 46 30                 	movups	%xmm0, 0x30(%rsi)
140f7ce1b: 48 89 1e                    	movq	%rbx, (%rsi)
140f7ce1e: 48 89 f0                    	movq	%rsi, %rax
140f7ce21: 48 81 c4 78 03 00 00        	addq	$0x378, %rsp            # imm = 0x378
140f7ce28: 5b                          	popq	%rbx
140f7ce29: 5f                          	popq	%rdi
140f7ce2a: 5e                          	popq	%rsi
140f7ce2b: 5d                          	popq	%rbp
140f7ce2c: c3                          	retq
140f7ce2d: b9 08 00 00 00              	movl	$0x8, %ecx
140f7ce32: ba 50 01 00 00              	movl	$0x150, %edx            # imm = 0x150
140f7ce37: e8 c4 c1 95 01              	callq	0x1428d9000 <.text+0x28d8000>
140f7ce3c: b9 08 00 00 00              	movl	$0x8, %ecx
140f7ce41: ba 68 02 00 00              	movl	$0x268, %edx            # imm = 0x268
140f7ce46: e8 b5 c1 95 01              	callq	0x1428d9000 <.text+0x28d8000>
140f7ce4b: 0f 0b                       	ud2
140f7ce4d: 0f 1f 00                    	nopl	(%rax)
140f7ce50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7ce55: 55                          	pushq	%rbp
140f7ce56: 56                          	pushq	%rsi
140f7ce57: 57                          	pushq	%rdi
140f7ce58: 53                          	pushq	%rbx
140f7ce59: 48 83 ec 28                 	subq	$0x28, %rsp
140f7ce5d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7ce64: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140f7ce68: e8 43 25 65 ff              	callq	0x1405cf3b0 <.text+0x5ce3b0>
140f7ce6d: 90                          	nop
140f7ce6e: 48 83 c4 28                 	addq	$0x28, %rsp
140f7ce72: 5b                          	popq	%rbx
140f7ce73: 5f                          	popq	%rdi
140f7ce74: 5e                          	popq	%rsi
140f7ce75: 5d                          	popq	%rbp
140f7ce76: c3                          	retq
140f7ce77: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140f7ce80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7ce85: 55                          	pushq	%rbp
140f7ce86: 56                          	pushq	%rsi
140f7ce87: 57                          	pushq	%rdi
140f7ce88: 53                          	pushq	%rbx
140f7ce89: 48 83 ec 28                 	subq	$0x28, %rsp
140f7ce8d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7ce94: ba 50 01 00 00              	movl	$0x150, %edx            # imm = 0x150
140f7ce99: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140f7ce9f: 48 8b 8d e8 02 00 00        	movq	0x2e8(%rbp), %rcx
140f7cea6: e8 75 6d 69 ff              	callq	0x140613c20 <.text+0x612c20>
140f7ceab: 90                          	nop
140f7ceac: 48 83 c4 28                 	addq	$0x28, %rsp
140f7ceb0: 5b                          	popq	%rbx
140f7ceb1: 5f                          	popq	%rdi
140f7ceb2: 5e                          	popq	%rsi
140f7ceb3: 5d                          	popq	%rbp
140f7ceb4: c3                          	retq
140f7ceb5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140f7cec0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7cec5: 55                          	pushq	%rbp
140f7cec6: 56                          	pushq	%rsi
140f7cec7: 57                          	pushq	%rdi
140f7cec8: 53                          	pushq	%rbx
140f7cec9: 48 83 ec 28                 	subq	$0x28, %rsp
140f7cecd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7ced4: 48 8d 8d 78 02 00 00        	leaq	0x278(%rbp), %rcx
140f7cedb: e8 a0 c9 0a ff              	callq	0x140029880 <.text+0x28880>
140f7cee0: 90                          	nop
140f7cee1: 48 83 c4 28                 	addq	$0x28, %rsp
140f7cee5: 5b                          	popq	%rbx
140f7cee6: 5f                          	popq	%rdi
140f7cee7: 5e                          	popq	%rsi
140f7cee8: 5d                          	popq	%rbp
140f7cee9: c3                          	retq
140f7ceea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7cef0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7cef5: 55                          	pushq	%rbp
140f7cef6: 56                          	pushq	%rsi
140f7cef7: 57                          	pushq	%rdi
140f7cef8: 53                          	pushq	%rbx
140f7cef9: 48 83 ec 28                 	subq	$0x28, %rsp
140f7cefd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7cf04: 48 8d 8d 08 02 00 00        	leaq	0x208(%rbp), %rcx
140f7cf0b: e8 70 c9 0a ff              	callq	0x140029880 <.text+0x28880>
140f7cf10: 90                          	nop
140f7cf11: 48 83 c4 28                 	addq	$0x28, %rsp
140f7cf15: 5b                          	popq	%rbx
140f7cf16: 5f                          	popq	%rdi
140f7cf17: 5e                          	popq	%rsi
140f7cf18: 5d                          	popq	%rbp
140f7cf19: c3                          	retq
