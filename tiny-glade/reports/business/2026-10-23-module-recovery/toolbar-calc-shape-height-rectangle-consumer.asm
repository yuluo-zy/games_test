
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141a41ba0: 55                          	pushq	%rbp
141a41ba1: 41 57                       	pushq	%r15
141a41ba3: 41 56                       	pushq	%r14
141a41ba5: 41 55                       	pushq	%r13
141a41ba7: 41 54                       	pushq	%r12
141a41ba9: 56                          	pushq	%rsi
141a41baa: 57                          	pushq	%rdi
141a41bab: 53                          	pushq	%rbx
141a41bac: 48 81 ec 68 03 00 00        	subq	$0x368, %rsp            # imm = 0x368
141a41bb3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141a41bbb: 44 0f 29 bd d0 02 00 00     	movaps	%xmm15, 0x2d0(%rbp)
141a41bc3: 44 0f 29 b5 c0 02 00 00     	movaps	%xmm14, 0x2c0(%rbp)
141a41bcb: 44 0f 29 ad b0 02 00 00     	movaps	%xmm13, 0x2b0(%rbp)
141a41bd3: 44 0f 29 a5 a0 02 00 00     	movaps	%xmm12, 0x2a0(%rbp)
141a41bdb: 44 0f 29 9d 90 02 00 00     	movaps	%xmm11, 0x290(%rbp)
141a41be3: 44 0f 29 95 80 02 00 00     	movaps	%xmm10, 0x280(%rbp)
141a41beb: 44 0f 29 8d 70 02 00 00     	movaps	%xmm9, 0x270(%rbp)
141a41bf3: 44 0f 29 85 60 02 00 00     	movaps	%xmm8, 0x260(%rbp)
141a41bfb: 0f 29 bd 50 02 00 00        	movaps	%xmm7, 0x250(%rbp)
141a41c02: 0f 29 b5 40 02 00 00        	movaps	%xmm6, 0x240(%rbp)
141a41c09: 48 c7 85 38 02 00 00 fe ff ff ff    	movq	$-0x2, 0x238(%rbp)
141a41c14: 4c 89 ce                    	movq	%r9, %rsi
141a41c17: 48 89 cf                    	movq	%rcx, %rdi
141a41c1a: 4c 8b 22                    	movq	(%rdx), %r12
141a41c1d: 49 83 3c 24 00              	cmpq	$0x0, (%r12)
141a41c22: 49 8b 18                    	movq	(%r8), %rbx
141a41c25: 0f 85 30 01 00 00           	jne	0x141a41d5b <.text+0x1a40d5b>
141a41c2b: 49 89 d7                    	movq	%rdx, %r15
141a41c2e: 48 8d 15 83 6f 5b 01        	leaq	0x15b6f83(%rip), %rdx   # 0x142ff8bb8
141a41c35: 48 89 d9                    	movq	%rbx, %rcx
141a41c38: e8 c3 40 77 00              	callq	0x1421b5d00 <.text+0x21b4d00>
141a41c3d: 48 85 c0                    	testq	%rax, %rax
141a41c40: 0f 84 61 1c 00 00           	je	0x141a438a7 <.text+0x1a428a7>
141a41c46: 49 89 c6                    	movq	%rax, %r14
141a41c49: 48 8d 05 a0 70 5b 01        	leaq	0x15b70a0(%rip), %rax   # 0x142ff8cf0
141a41c50: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
141a41c54: 48 c7 45 28 3d 00 00 00     	movq	$0x3d, 0x28(%rbp)
141a41c5c: 49 83 7e 18 3d              	cmpq	$0x3d, 0x18(%r14)
141a41c61: 0f 85 ec 1b 00 00           	jne	0x141a43853 <.text+0x1a42853>
141a41c67: 49 8b 56 10                 	movq	0x10(%r14), %rdx
141a41c6b: 48 8d 0d 7e 70 5b 01        	leaq	0x15b707e(%rip), %rcx   # 0x142ff8cf0
141a41c72: 41 b8 3d 00 00 00           	movl	$0x3d, %r8d
141a41c78: e8 73 1f ee 00              	callq	0x142923bf0 <.text+0x2922bf0>
141a41c7d: 85 c0                       	testl	%eax, %eax
141a41c7f: 0f 85 ce 1b 00 00           	jne	0x141a43853 <.text+0x1a42853>
141a41c85: 49 8b 0e                    	movq	(%r14), %rcx
141a41c88: 49 8b 46 08                 	movq	0x8(%r14), %rax
141a41c8c: ff 50 18                    	callq	*0x18(%rax)
141a41c8f: 48 85 c0                    	testq	%rax, %rax
141a41c92: 0f 84 0f 1c 00 00           	je	0x141a438a7 <.text+0x1a428a7>
141a41c98: 49 89 c6                    	movq	%rax, %r14
141a41c9b: 0f b6 05 67 2f bf 01        	movzbl	0x1bf2f67(%rip), %eax   # 0x143634c09
141a41ca2: b9 20 00 00 00              	movl	$0x20, %ecx
141a41ca7: ba 08 00 00 00              	movl	$0x8, %edx
141a41cac: e8 5f 1f bd fe              	callq	0x140613c10 <.text+0x612c10>
141a41cb1: 48 85 c0                    	testq	%rax, %rax
141a41cb4: 0f 84 05 1c 00 00           	je	0x141a438bf <.text+0x1a428bf>
141a41cba: 48 89 c2                    	movq	%rax, %rdx
141a41cbd: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
141a41cc4: 48 c7 40 08 04 00 00 00     	movq	$0x4, 0x8(%rax)
141a41ccc: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
141a41cd4: c6 40 18 00                 	movb	$0x0, 0x18(%rax)
141a41cd8: 4d 8b 46 60                 	movq	0x60(%r14), %r8
141a41cdc: eb 04                       	jmp	0x141a41ce2 <.text+0x1a40ce2>
141a41cde: 66 90                       	nop
141a41ce0: f3 90                       	pause
141a41ce2: 49 8b 40 08                 	movq	0x8(%r8), %rax
141a41ce6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
141a41cf0: 48 83 f8 ff                 	cmpq	$-0x1, %rax
141a41cf4: 74 ea                       	je	0x141a41ce0 <.text+0x1a40ce0>
141a41cf6: 48 85 c0                    	testq	%rax, %rax
141a41cf9: 0f 88 ec 1a 00 00           	js	0x141a437eb <.text+0x1a427eb>
141a41cff: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
141a41d03: f0                          	lock
141a41d04: 49 0f b1 48 08              	cmpxchgq	%rcx, 0x8(%r8)
141a41d09: 75 e5                       	jne	0x141a41cf0 <.text+0x1a40cf0>
141a41d0b: 4c 89 85 30 02 00 00        	movq	%r8, 0x230(%rbp)
141a41d12: 48 89 95 20 02 00 00        	movq	%rdx, 0x220(%rbp)
141a41d19: 41 8b 47 1c                 	movl	0x1c(%r15), %eax
141a41d1d: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
141a41d21: 89 01                       	movl	%eax, (%rcx)
141a41d23: 49 83 3c 24 00              	cmpq	$0x0, (%r12)
141a41d28: 4c 89 a5 28 02 00 00        	movq	%r12, 0x228(%rbp)
141a41d2f: 74 0c                       	je	0x141a41d3d <.text+0x1a40d3d>
141a41d31: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
141a41d38: e8 33 b1 ff ff              	callq	0x141a3ce70 <.text+0x1a3be70>
141a41d3d: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
141a41d44: 4c 8b a5 28 02 00 00        	movq	0x228(%rbp), %r12
141a41d4b: 49 89 04 24                 	movq	%rax, (%r12)
141a41d4f: 48 8b 85 20 02 00 00        	movq	0x220(%rbp), %rax
141a41d56: 49 89 44 24 08              	movq	%rax, 0x8(%r12)
141a41d5b: 48 8d 15 56 6e 5b 01        	leaq	0x15b6e56(%rip), %rdx   # 0x142ff8bb8
141a41d62: 48 89 d9                    	movq	%rbx, %rcx
141a41d65: e8 96 3f 77 00              	callq	0x1421b5d00 <.text+0x21b4d00>
141a41d6a: 48 85 c0                    	testq	%rax, %rax
141a41d6d: 0f 84 10 1b 00 00           	je	0x141a43883 <.text+0x1a42883>
141a41d73: 48 89 c3                    	movq	%rax, %rbx
141a41d76: 48 8d 05 73 6f 5b 01        	leaq	0x15b6f73(%rip), %rax   # 0x142ff8cf0
141a41d7d: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
141a41d81: 48 c7 45 28 3d 00 00 00     	movq	$0x3d, 0x28(%rbp)
141a41d89: 48 83 7b 18 3d              	cmpq	$0x3d, 0x18(%rbx)
141a41d8e: 0f 85 6a 1a 00 00           	jne	0x141a437fe <.text+0x1a427fe>
141a41d94: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141a41d98: 48 8d 0d 51 6f 5b 01        	leaq	0x15b6f51(%rip), %rcx   # 0x142ff8cf0
141a41d9f: 41 b8 3d 00 00 00           	movl	$0x3d, %r8d
141a41da5: e8 46 1e ee 00              	callq	0x142923bf0 <.text+0x2922bf0>
141a41daa: 85 c0                       	testl	%eax, %eax
141a41dac: 0f 85 4c 1a 00 00           	jne	0x141a437fe <.text+0x1a427fe>
141a41db2: 48 8b 0b                    	movq	(%rbx), %rcx
141a41db5: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141a41db9: ff 50 18                    	callq	*0x18(%rax)
141a41dbc: 48 85 c0                    	testq	%rax, %rax
141a41dbf: 0f 84 be 1a 00 00           	je	0x141a43883 <.text+0x1a42883>
141a41dc5: 49 83 3c 24 00              	cmpq	$0x0, (%r12)
141a41dca: 0f 84 bf 1a 00 00           	je	0x141a4388f <.text+0x1a4288f>
141a41dd0: 4d 8b 7c 24 08              	movq	0x8(%r12), %r15
141a41dd5: 4d 85 ff                    	testq	%r15, %r15
141a41dd8: 0f 84 bd 1a 00 00           	je	0x141a4389b <.text+0x1a4289b>
141a41dde: 48 89 c3                    	movq	%rax, %rbx
141a41de1: 49 8d 4f 18                 	leaq	0x18(%r15), %rcx
141a41de5: b2 01                       	movb	$0x1, %dl
141a41de7: 31 c0                       	xorl	%eax, %eax
141a41de9: f0                          	lock
141a41dea: 41 0f b0 57 18              	cmpxchgb	%dl, 0x18(%r15)
141a41def: 0f 85 39 1a 00 00           	jne	0x141a4382e <.text+0x1a4282e>
141a41df5: e8 76 fe 23 ff              	callq	0x140c81c70 <.text+0xc80c70>
141a41dfa: 49 8b 47 10                 	movq	0x10(%r15), %rax
141a41dfe: 48 85 c0                    	testq	%rax, %rax
141a41e01: 0f 94 c1                    	sete	%cl
141a41e04: 48 c1 e0 02                 	shlq	$0x2, %rax
141a41e08: 49 03 47 08                 	addq	0x8(%r15), %rax
141a41e0c: 48 83 c0 fc                 	addq	$-0x4, %rax
141a41e10: 0f 94 c2                    	sete	%dl
141a41e13: 08 ca                       	orb	%cl, %dl
141a41e15: 74 12                       	je	0x141a41e29 <.text+0x1a40e29>
141a41e17: 48 8b 43 28                 	movq	0x28(%rbx), %rax
141a41e1b: 48 85 c0                    	testq	%rax, %rax
141a41e1e: 0f 84 8f 1a 00 00           	je	0x141a438b3 <.text+0x1a428b3>
141a41e24: 8d 48 ff                    	leal	-0x1(%rax), %ecx
141a41e27: eb 06                       	jmp	0x141a41e2f <.text+0x1a40e2f>
141a41e29: 8b 08                       	movl	(%rax), %ecx
141a41e2b: 48 8b 43 28                 	movq	0x28(%rbx), %rax
141a41e2f: 48 8b 53 20                 	movq	0x20(%rbx), %rdx
141a41e33: 0f 10 43 30                 	movups	0x30(%rbx), %xmm0
141a41e37: 0f 10 4b 40                 	movups	0x40(%rbx), %xmm1
141a41e3b: 0f 10 53 50                 	movups	0x50(%rbx), %xmm2
141a41e3f: 89 4d 70                    	movl	%ecx, 0x70(%rbp)
141a41e42: 48 89 55 20                 	movq	%rdx, 0x20(%rbp)
141a41e46: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
141a41e4a: 0f 11 45 30                 	movups	%xmm0, 0x30(%rbp)
141a41e4e: 4c 89 7d 40                 	movq	%r15, 0x40(%rbp)
141a41e52: 0f 11 4d 48                 	movups	%xmm1, 0x48(%rbp)
141a41e56: 48 89 5d 58                 	movq	%rbx, 0x58(%rbp)
141a41e5a: 0f 11 55 60                 	movups	%xmm2, 0x60(%rbp)
141a41e5e: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
141a41e62: e8 79 44 77 00              	callq	0x1421b62e0 <.text+0x21b52e0>
141a41e67: 4c 8b 1f                    	movq	(%rdi), %r11
141a41e6a: 48 8b 47 08                 	movq	0x8(%rdi), %rax
141a41e6e: 49 8b 13                    	movq	(%r11), %rdx
141a41e71: 45 31 f6                    	xorl	%r14d, %r14d
141a41e74: 49 89 d0                    	movq	%rdx, %r8
141a41e77: 4c 2b 40 18                 	subq	0x18(%rax), %r8
141a41e7b: 4d 0f 42 c6                 	cmovbq	%r14, %r8
141a41e7f: 48 2b 50 38                 	subq	0x38(%rax), %rdx
141a41e83: 48 8b 8d 50 03 00 00        	movq	0x350(%rbp), %rcx
141a41e8a: 49 0f 42 d6                 	cmovbq	%r14, %rdx
141a41e8e: 4b 8d 3c 80                 	leaq	(%r8,%r8,4), %rdi
141a41e92: 48 c1 e7 03                 	shlq	$0x3, %rdi
141a41e96: 48 03 78 08                 	addq	0x8(%rax), %rdi
141a41e9a: 4c 8b 48 10                 	movq	0x10(%rax), %r9
141a41e9e: 4c 8b 50 30                 	movq	0x30(%rax), %r10
141a41ea2: 4d 29 c1                    	subq	%r8, %r9
141a41ea5: 4d 0f 42 ce                 	cmovbq	%r14, %r9
141a41ea9: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141a41eaf: 49 0f 42 f8                 	cmovbq	%r8, %rdi
141a41eb3: 4c 8d 24 92                 	leaq	(%rdx,%rdx,4), %r12
141a41eb7: 49 c1 e4 03                 	shlq	$0x3, %r12
141a41ebb: 4c 03 60 28                 	addq	0x28(%rax), %r12
141a41ebf: 49 29 d2                    	subq	%rdx, %r10
141a41ec2: 4d 0f 42 d6                 	cmovbq	%r14, %r10
141a41ec6: 4d 0f 42 e0                 	cmovbq	%r8, %r12
141a41eca: 48 8b 40 40                 	movq	0x40(%rax), %rax
141a41ece: 4c 29 d0                    	subq	%r10, %rax
141a41ed1: 4c 29 c8                    	subq	%r9, %rax
141a41ed4: 4c 89 5d f0                 	movq	%r11, -0x10(%rbp)
141a41ed8: 49 89 03                    	movq	%rax, (%r11)
141a41edb: 4b 8d 04 89                 	leaq	(%r9,%r9,4), %rax
141a41edf: 48 8d 04 c7                 	leaq	(%rdi,%rax,8), %rax
141a41ee3: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
141a41ee7: 4b 8d 04 92                 	leaq	(%r10,%r10,4), %rax
141a41eeb: 49 8d 04 c4                 	leaq	(%r12,%rax,8), %rax
141a41eef: 48 89 45 00                 	movq	%rax, (%rbp)
141a41ef3: 48 8b 01                    	movq	(%rcx), %rax
141a41ef6: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
141a41efa: 8b 46 1c                    	movl	0x1c(%rsi), %eax
141a41efd: 89 85 8c 00 00 00           	movl	%eax, 0x8c(%rbp)
141a41f03: 48 8b 06                    	movq	(%rsi), %rax
141a41f06: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
141a41f0a: 48 8b 46 10                 	movq	0x10(%rsi), %rax
141a41f0e: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
141a41f12: 48 89 fe                    	movq	%rdi, %rsi
141a41f15: 48 8d 9d 60 01 00 00        	leaq	0x160(%rbp), %rbx
141a41f1c: f3 44 0f 10 15 df 39 ee 00  	movss	0xee39df(%rip), %xmm10  # 0x142925904
141a41f25: eb 1a                       	jmp	0x141a41f41 <.text+0x1a40f41>
141a41f27: 41 08 c6                    	orb	%al, %r14b
141a41f2a: 48 ff c7                    	incq	%rdi
141a41f2d: 8b 85 8c 00 00 00           	movl	0x8c(%rbp), %eax
141a41f33: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
141a41f37: 89 01                       	movl	%eax, (%rcx)
141a41f39: 48 8b 45 10                 	movq	0x10(%rbp), %rax
141a41f3d: 48 89 78 08                 	movq	%rdi, 0x8(%rax)
141a41f41: 4c 89 e0                    	movq	%r12, %rax
141a41f44: 48 89 f1                    	movq	%rsi, %rcx
141a41f47: eb 20                       	jmp	0x141a41f69 <.text+0x1a40f69>
141a41f49: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141a41f50: 48 8d 71 28                 	leaq	0x28(%rcx), %rsi
141a41f54: 48 89 c7                    	movq	%rax, %rdi
141a41f57: 48 89 ca                    	movq	%rcx, %rdx
141a41f5a: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
141a41f5e: 48 ff 01                    	incq	(%rcx)
141a41f61: f6 02 01                    	testb	$0x1, (%rdx)
141a41f64: 48 89 f1                    	movq	%rsi, %rcx
141a41f67: 75 37                       	jne	0x141a41fa0 <.text+0x1a40fa0>
141a41f69: 48 85 c9                    	testq	%rcx, %rcx
141a41f6c: 74 06                       	je	0x141a41f74 <.text+0x1a40f74>
141a41f6e: 48 3b 4d f8                 	cmpq	-0x8(%rbp), %rcx
141a41f72: 75 dc                       	jne	0x141a41f50 <.text+0x1a40f50>
141a41f74: 48 85 c0                    	testq	%rax, %rax
141a41f77: 0f 84 1a 17 00 00           	je	0x141a43697 <.text+0x1a42697>
141a41f7d: 48 3b 45 00                 	cmpq	(%rbp), %rax
141a41f81: 0f 84 10 17 00 00           	je	0x141a43697 <.text+0x1a42697>
141a41f87: 48 8d 78 28                 	leaq	0x28(%rax), %rdi
141a41f8b: 31 f6                       	xorl	%esi, %esi
141a41f8d: 48 89 c2                    	movq	%rax, %rdx
141a41f90: 48 89 f8                    	movq	%rdi, %rax
141a41f93: eb c5                       	jmp	0x141a41f5a <.text+0x1a40f5a>
141a41f95: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
141a41fa0: 48 89 95 98 00 00 00        	movq	%rdx, 0x98(%rbp)
141a41fa7: 48 83 c2 04                 	addq	$0x4, %rdx
141a41fab: 48 8b 45 08                 	movq	0x8(%rbp), %rax
141a41faf: f3 0f 10 50 08              	movss	0x8(%rax), %xmm2
141a41fb4: f3 0f 10 78 0c              	movss	0xc(%rax), %xmm7
141a41fb9: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
141a41fc0: 48 89 95 28 02 00 00        	movq	%rdx, 0x228(%rbp)
141a41fc7: f3 0f 11 95 1c 02 00 00     	movss	%xmm2, 0x21c(%rbp)
141a41fcf: e8 1c 72 25 ff              	callq	0x140c991f0 <.text+0xc981f0>
141a41fd4: 48 89 bd b8 00 00 00        	movq	%rdi, 0xb8(%rbp)
141a41fdb: 4c 8b a5 28 01 00 00        	movq	0x128(%rbp), %r12
141a41fe2: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
141a41fe9: 48 85 c0                    	testq	%rax, %rax
141a41fec: 0f 84 fe 08 00 00           	je	0x141a428f0 <.text+0x1a418f0>
141a41ff2: 48 c1 e0 02                 	shlq	$0x2, %rax
141a41ff6: 4c 8d 3c 40                 	leaq	(%rax,%rax,2), %r15
141a41ffa: 45 31 ed                    	xorl	%r13d, %r13d
141a41ffd: 31 ff                       	xorl	%edi, %edi
141a41fff: 90                          	nop
141a42000: 43 8b 44 2c 08              	movl	0x8(%r12,%r13), %eax
141a42005: 89 85 c8 00 00 00           	movl	%eax, 0xc8(%rbp)
141a4200b: 4b 8b 04 2c                 	movq	(%r12,%r13), %rax
141a4200f: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
141a42016: 48 89 d9                    	movq	%rbx, %rcx
141a42019: 48 8b 95 28 02 00 00        	movq	0x228(%rbp), %rdx
141a42020: e8 4b 75 25 ff              	callq	0x140c99570 <.text+0xc98570>
141a42025: f3 0f 10 b5 60 01 00 00     	movss	0x160(%rbp), %xmm6
141a4202d: f3 0f 10 a5 64 01 00 00     	movss	0x164(%rbp), %xmm4
141a42035: f3 0f 10 9d 68 01 00 00     	movss	0x168(%rbp), %xmm3
141a4203d: f3 0f 10 95 6c 01 00 00     	movss	0x16c(%rbp), %xmm2
141a42045: f3 44 0f 10 85 70 01 00 00  	movss	0x170(%rbp), %xmm8
141a4204e: f2 0f 10 8d 74 01 00 00     	movsd	0x174(%rbp), %xmm1
141a42056: f3 0f 10 85 7c 01 00 00     	movss	0x17c(%rbp), %xmm0
141a4205e: f3 0f 10 ad 80 01 00 00     	movss	0x180(%rbp), %xmm5
141a42066: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
141a4206a: 44 0f 2e cd                 	ucomiss	%xmm5, %xmm9
141a4206e: 73 60                       	jae	0x141a420d0 <.text+0x1a410d0>
141a42070: f3 41 0f 58 f0              	addss	%xmm8, %xmm6
141a42075: f3 41 0f 58 ea              	addss	%xmm10, %xmm5
141a4207a: 44 0f 2e ce                 	ucomiss	%xmm6, %xmm9
141a4207e: 0f 83 9c 00 00 00           	jae	0x141a42120 <.text+0x1a41120>
141a42084: 0f 14 e6                    	unpcklps	%xmm6, %xmm4            # xmm4 = xmm4[0],xmm6[0],xmm4[1],xmm6[1]
141a42087: 0f c6 e1 14                 	shufps	$0x14, %xmm1, %xmm4     # xmm4 = xmm4[0,1],xmm1[1,0]
141a4208b: 0f 14 d8                    	unpcklps	%xmm0, %xmm3            # xmm3 = xmm3[0],xmm0[0],xmm3[1],xmm0[1]
141a4208e: 0f 14 d5                    	unpcklps	%xmm5, %xmm2            # xmm2 = xmm2[0],xmm5[0],xmm2[1],xmm5[1]
141a42091: 0f 16 d3                    	movlhps	%xmm3, %xmm2            # xmm2 = xmm2[0],xmm3[0]
141a42094: 0f 28 c4                    	movaps	%xmm4, %xmm0
141a42097: 0f 5c c2                    	subps	%xmm2, %xmm0
141a4209a: 0f 58 d4                    	addps	%xmm4, %xmm2
141a4209d: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
141a420a1: 0f c6 d0 01                 	shufps	$0x1, %xmm0, %xmm2      # xmm2 = xmm2[1,0],xmm0[0,0]
141a420a5: 0f c6 d0 e2                 	shufps	$0xe2, %xmm0, %xmm2     # xmm2 = xmm2[2,0],xmm0[2,3]
141a420a9: f3 0f 51 c9                 	sqrtss	%xmm1, %xmm1
141a420ad: f3 0f 10 05 bb 37 ee 00     	movss	0xee37bb(%rip), %xmm0   # 0x142925870
141a420b5: f3 0f 5e c1                 	divss	%xmm1, %xmm0
141a420b9: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141a420bd: 0f 59 c2                    	mulps	%xmm2, %xmm0
141a420c0: e9 c8 00 00 00              	jmp	0x141a4218d <.text+0x1a4118d>
141a420c5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
141a420d0: f3 44 0f 5c c6              	subss	%xmm6, %xmm8
141a420d5: 41 0f 28 f2                 	movaps	%xmm10, %xmm6
141a420d9: f3 0f 5c f5                 	subss	%xmm5, %xmm6
141a420dd: 45 0f 2e c8                 	ucomiss	%xmm8, %xmm9
141a420e1: 73 5d                       	jae	0x141a42140 <.text+0x1a41140>
141a420e3: 41 0f 16 e0                 	movlhps	%xmm8, %xmm4            # xmm4 = xmm4[0],xmm8[0]
141a420e7: 0f 28 e9                    	movaps	%xmm1, %xmm5
141a420ea: 0f c6 ec 24                 	shufps	$0x24, %xmm4, %xmm5     # xmm5 = xmm5[0,1],xmm4[2,0]
141a420ee: 0f 14 c3                    	unpcklps	%xmm3, %xmm0            # xmm0 = xmm0[0],xmm3[0],xmm0[1],xmm3[1]
141a420f1: 0f 16 d6                    	movlhps	%xmm6, %xmm2            # xmm2 = xmm2[0],xmm6[0]
141a420f4: 0f 5c c8                    	subps	%xmm0, %xmm1
141a420f7: 0f c6 c2 24                 	shufps	$0x24, %xmm2, %xmm0     # xmm0 = xmm0[0,1],xmm2[2,0]
141a420fb: 0f 58 c5                    	addps	%xmm5, %xmm0
141a420fe: 0f c6 c8 01                 	shufps	$0x1, %xmm0, %xmm1      # xmm1 = xmm1[1,0],xmm0[0,0]
141a42102: 0f c6 c8 e2                 	shufps	$0xe2, %xmm0, %xmm1     # xmm1 = xmm1[2,0],xmm0[2,3]
141a42106: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
141a42109: 0f 57 d2                    	xorps	%xmm2, %xmm2
141a4210c: f3 0f 51 d0                 	sqrtss	%xmm0, %xmm2
141a42110: eb 68                       	jmp	0x141a4217a <.text+0x1a4117a>
141a42112: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
141a42120: 0f 14 ec                    	unpcklps	%xmm4, %xmm5            # xmm5 = xmm5[0],xmm4[0],xmm5[1],xmm4[1]
141a42123: 0f 16 e9                    	movlhps	%xmm1, %xmm5            # xmm5 = xmm5[0],xmm1[0]
141a42126: 0f 14 c3                    	unpcklps	%xmm3, %xmm0            # xmm0 = xmm0[0],xmm3[0],xmm0[1],xmm3[1]
141a42129: 0f 14 f2                    	unpcklps	%xmm2, %xmm6            # xmm6 = xmm6[0],xmm2[0],xmm6[1],xmm2[1]
141a4212c: 0f 16 f0                    	movlhps	%xmm0, %xmm6            # xmm6 = xmm6[0],xmm0[0]
141a4212f: 0f 28 cd                    	movaps	%xmm5, %xmm1
141a42132: 0f 58 ce                    	addps	%xmm6, %xmm1
141a42135: 0f 5c ee                    	subps	%xmm6, %xmm5
141a42138: f2 0f 10 cd                 	movsd	%xmm5, %xmm1            # xmm1 = xmm5[0],xmm1[1]
141a4213c: eb 35                       	jmp	0x141a42173 <.text+0x1a41173>
141a4213e: 66 90                       	nop
141a42140: 0f 28 eb                    	movaps	%xmm3, %xmm5
141a42143: 0f 14 e9                    	unpcklps	%xmm1, %xmm5            # xmm5 = xmm5[0],xmm1[0],xmm5[1],xmm1[1]
141a42146: 0f 16 dc                    	movlhps	%xmm4, %xmm3            # xmm3 = xmm3[0],xmm4[0]
141a42149: 0f 16 f4                    	movlhps	%xmm4, %xmm6            # xmm6 = xmm6[0],xmm4[0]
141a4214c: 0f c6 ee 24                 	shufps	$0x24, %xmm6, %xmm5     # xmm5 = xmm5[0,1],xmm6[2,0]
141a42150: 0f c6 c1 d4                 	shufps	$0xd4, %xmm1, %xmm0     # xmm0 = xmm0[0,1],xmm1[1,3]
141a42154: 0f c6 ca c5                 	shufps	$0xc5, %xmm2, %xmm1     # xmm1 = xmm1[1,1],xmm2[0,3]
141a42158: 0f 58 cb                    	addps	%xmm3, %xmm1
141a4215b: 44 0f 16 c2                 	movlhps	%xmm2, %xmm8            # xmm8 = xmm8[0],xmm2[0]
141a4215f: 41 0f c6 c0 22              	shufps	$0x22, %xmm8, %xmm0     # xmm0 = xmm0[2,0],xmm8[2,0]
141a42164: 0f 5c e8                    	subps	%xmm0, %xmm5
141a42167: 0f c6 cd d8                 	shufps	$0xd8, %xmm5, %xmm1     # xmm1 = xmm1[0,2],xmm5[1,3]
141a4216b: 0f c6 c9 d8                 	shufps	$0xd8, %xmm1, %xmm1     # xmm1 = xmm1[0,2,1,3]
141a4216f: 0f c6 ed ff                 	shufps	$0xff, %xmm5, %xmm5     # xmm5 = xmm5[3,3,3,3]
141a42173: 0f 57 d2                    	xorps	%xmm2, %xmm2
141a42176: f3 0f 51 d5                 	sqrtss	%xmm5, %xmm2
141a4217a: f3 0f 10 05 ee 36 ee 00     	movss	0xee36ee(%rip), %xmm0   # 0x142925870
141a42182: f3 0f 5e c2                 	divss	%xmm2, %xmm0
141a42186: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141a4218a: 0f 59 c1                    	mulps	%xmm1, %xmm0
141a4218d: 8b 85 c8 00 00 00           	movl	0xc8(%rbp), %eax
141a42193: 89 85 e8 01 00 00           	movl	%eax, 0x1e8(%rbp)
141a42199: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
141a421a0: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
141a421a7: c7 85 ec 01 00 00 cd cc 4c 3e       	movl	$0x3e4ccccd, 0x1ec(%rbp) # imm = 0x3E4CCCCD
141a421b1: f3 0f 11 bd f0 01 00 00     	movss	%xmm7, 0x1f0(%rbp)
141a421b9: c7 85 f4 01 00 00 cd cc 4c 3e       	movl	$0x3e4ccccd, 0x1f4(%rbp) # imm = 0x3E4CCCCD
141a421c3: 0f 10 9d ec 01 00 00        	movups	0x1ec(%rbp), %xmm3
141a421ca: 0f 28 8d f0 01 00 00        	movaps	0x1f0(%rbp), %xmm1
141a421d1: 0f 28 95 e0 01 00 00        	movaps	0x1e0(%rbp), %xmm2
141a421d8: 0f 28 e0                    	movaps	%xmm0, %xmm4
141a421db: 0f c6 e0 ff                 	shufps	$0xff, %xmm0, %xmm4     # xmm4 = xmm4[3,3],xmm0[3,3]
141a421df: 44 0f 28 e0                 	movaps	%xmm0, %xmm12
141a421e3: 44 0f 58 e0                 	addps	%xmm0, %xmm12
141a421e7: 0f 28 f4                    	movaps	%xmm4, %xmm6
141a421ea: f3 41 0f 59 f4              	mulss	%xmm12, %xmm6
141a421ef: 41 0f 28 ec                 	movaps	%xmm12, %xmm5
141a421f3: 66 41 0f 15 ec              	unpckhpd	%xmm12, %xmm5           # xmm5 = xmm5[1],xmm12[1]
141a421f8: f3 0f 59 e5                 	mulss	%xmm5, %xmm4
141a421fc: 44 0f 28 f8                 	movaps	%xmm0, %xmm15
141a42200: 45 0f 59 fc                 	mulps	%xmm12, %xmm15
141a42204: f3 44 0f 16 c8              	movshdup	%xmm0, %xmm9            # xmm9 = xmm0[1,1,3,3]
141a42209: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
141a4220c: f3 41 0f 59 c4              	mulss	%xmm12, %xmm0
141a42211: 45 0f 28 c4                 	movaps	%xmm12, %xmm8
141a42215: 45 0f c6 c4 ff              	shufps	$0xff, %xmm12, %xmm8    # xmm8 = xmm8[3,3],xmm12[3,3]
141a4221a: f3 45 0f 59 c1              	mulss	%xmm9, %xmm8
141a4221f: f3 41 0f 59 e9              	mulss	%xmm9, %xmm5
141a42224: f3 45 0f 59 e1              	mulss	%xmm9, %xmm12
141a42229: 45 0f 28 f7                 	movaps	%xmm15, %xmm14
141a4222d: 66 45 0f 15 f7              	unpckhpd	%xmm15, %xmm14          # xmm14 = xmm14[1],xmm15[1]
141a42232: 45 0f 28 ce                 	movaps	%xmm14, %xmm9
141a42236: f3 45 0f 58 cf              	addss	%xmm15, %xmm9
141a4223b: 45 0f 28 da                 	movaps	%xmm10, %xmm11
141a4223f: f3 45 0f 5c d9              	subss	%xmm9, %xmm11
141a42244: 44 0f 28 cc                 	movaps	%xmm4, %xmm9
141a42248: f3 45 0f 58 cc              	addss	%xmm12, %xmm9
141a4224d: 45 0f 14 d9                 	unpcklps	%xmm9, %xmm11           # xmm11 = xmm11[0],xmm9[0],xmm11[1],xmm9[1]
141a42251: 44 0f 28 ce                 	movaps	%xmm6, %xmm9
141a42255: f3 44 0f 5c cd              	subss	%xmm5, %xmm9
141a4225a: 45 0f c6 d9 04              	shufps	$0x4, %xmm9, %xmm11     # xmm11 = xmm11[0,1],xmm9[0,0]
141a4225f: f3 41 0f 5c e4              	subss	%xmm12, %xmm4
141a42264: 45 0f 28 cf                 	movaps	%xmm15, %xmm9
141a42268: 45 0f c6 cf ff              	shufps	$0xff, %xmm15, %xmm9    # xmm9 = xmm9[3,3],xmm15[3,3]
141a4226d: f3 45 0f 58 f9              	addss	%xmm9, %xmm15
141a42272: 45 0f 28 e2                 	movaps	%xmm10, %xmm12
141a42276: f3 45 0f 5c e7              	subss	%xmm15, %xmm12
141a4227b: 41 0f 14 e4                 	unpcklps	%xmm12, %xmm4           # xmm4 = xmm4[0],xmm12[0],xmm4[1],xmm12[1]
141a4227f: 44 0f 28 e0                 	movaps	%xmm0, %xmm12
141a42283: f3 45 0f 58 e0              	addss	%xmm8, %xmm12
141a42288: 41 0f c6 e4 04              	shufps	$0x4, %xmm12, %xmm4     # xmm4 = xmm4[0,1],xmm12[0,0]
141a4228d: f3 0f 58 ee                 	addss	%xmm6, %xmm5
141a42291: f3 41 0f 5c c0              	subss	%xmm8, %xmm0
141a42296: 0f 14 e8                    	unpcklps	%xmm0, %xmm5            # xmm5 = xmm5[0],xmm0[0],xmm5[1],xmm0[1]
141a42299: f3 45 0f 58 ce              	addss	%xmm14, %xmm9
141a4229e: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
141a422a2: f3 41 0f 5c c1              	subss	%xmm9, %xmm0
141a422a7: 0f c6 e8 04                 	shufps	$0x4, %xmm0, %xmm5      # xmm5 = xmm5[0,1],xmm0[0,0]
141a422ab: 0f c6 db 00                 	shufps	$0x0, %xmm3, %xmm3      # xmm3 = xmm3[0,0,0,0]
141a422af: 41 0f 59 db                 	mulps	%xmm11, %xmm3
141a422b3: 0f 28 c1                    	movaps	%xmm1, %xmm0
141a422b6: 0f c6 c1 00                 	shufps	$0x0, %xmm1, %xmm0      # xmm0 = xmm0[0,0],xmm1[0,0]
141a422ba: 0f 59 c4                    	mulps	%xmm4, %xmm0
141a422bd: 0f c6 c9 55                 	shufps	$0x55, %xmm1, %xmm1     # xmm1 = xmm1[1,1,1,1]
141a422c1: 0f 59 cd                    	mulps	%xmm5, %xmm1
141a422c4: 0f c6 d2 a4                 	shufps	$0xa4, %xmm2, %xmm2     # xmm2 = xmm2[0,1,2,2]
141a422c8: 0f 29 9d 60 01 00 00        	movaps	%xmm3, 0x160(%rbp)
141a422cf: 0f 29 85 70 01 00 00        	movaps	%xmm0, 0x170(%rbp)
141a422d6: 0f 29 8d 80 01 00 00        	movaps	%xmm1, 0x180(%rbp)
141a422dd: 0f 29 95 90 01 00 00        	movaps	%xmm2, 0x190(%rbp)
141a422e4: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
141a422eb: 48 89 da                    	movq	%rbx, %rdx
141a422ee: e8 6d b2 e5 fe              	callq	0x14089d560 <.text+0x89c560>
141a422f3: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
141a422fa: 0f 10 8d d0 00 00 00        	movups	0xd0(%rbp), %xmm1
141a42301: 0f 10 95 e0 00 00 00        	movups	0xe0(%rbp), %xmm2
141a42308: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
141a4230f: 0f 29 8d 70 01 00 00        	movaps	%xmm1, 0x170(%rbp)
141a42316: 0f 29 95 80 01 00 00        	movaps	%xmm2, 0x180(%rbp)
141a4231d: 0f 28 05 7c 2b 0d 01        	movaps	0x10d2b7c(%rip), %xmm0  # 0x142b14ea0
141a42324: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
141a4232b: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
141a4232f: 48 89 da                    	movq	%rbx, %rdx
141a42332: e8 c9 3c 77 00              	callq	0x1421b6000 <.text+0x21b5000>
141a42337: 41 08 c6                    	orb	%al, %r14b
141a4233a: 48 ff c7                    	incq	%rdi
141a4233d: 49 83 c5 0c                 	addq	$0xc, %r13
141a42341: 4d 39 ef                    	cmpq	%r13, %r15
141a42344: 0f 85 b6 fc ff ff           	jne	0x141a42000 <.text+0x1a41000>
141a4234a: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
141a42351: 48 85 c0                    	testq	%rax, %rax
141a42354: 74 16                       	je	0x141a4236c <.text+0x1a4136c>
141a42356: 48 c1 e0 02                 	shlq	$0x2, %rax
141a4235a: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
141a4235e: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141a42364: 4c 89 e1                    	movq	%r12, %rcx
141a42367: e8 b4 18 bd fe              	callq	0x140613c20 <.text+0x612c20>
141a4236c: 48 8b 95 38 01 00 00        	movq	0x138(%rbp), %rdx
141a42373: 48 85 d2                    	testq	%rdx, %rdx
141a42376: 74 16                       	je	0x141a4238e <.text+0x1a4138e>
141a42378: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
141a4237f: 48 c1 e2 02                 	shlq	$0x2, %rdx
141a42383: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141a42389: e8 92 18 bd fe              	callq	0x140613c20 <.text+0x612c20>
141a4238e: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
141a42395: 48 8b 95 28 02 00 00        	movq	0x228(%rbp), %rdx
141a4239c: f3 0f 10 95 1c 02 00 00     	movss	0x21c(%rbp), %xmm2
141a423a4: e8 47 6e 25 ff              	callq	0x140c991f0 <.text+0xc981f0>
141a423a9: 4c 8b a5 b8 00 00 00        	movq	0xb8(%rbp), %r12
141a423b0: 48 8b 8d 28 01 00 00        	movq	0x128(%rbp), %rcx
141a423b7: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
141a423be: 48 85 c0                    	testq	%rax, %rax
141a423c1: 48 89 8d 90 00 00 00        	movq	%rcx, 0x90(%rbp)
141a423c8: 0f 84 47 05 00 00           	je	0x141a42915 <.text+0x1a41915>
141a423ce: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
141a423d2: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
141a423d6: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
141a423dd: b0 01                       	movb	$0x1, %al
141a423df: 0f 57 f6                    	xorps	%xmm6, %xmm6
141a423e2: eb 25                       	jmp	0x141a42409 <.text+0x1a41409>
141a423e4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141a423f0: 31 c0                       	xorl	%eax, %eax
141a423f2: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
141a423f9: 48 89 d1                    	movq	%rdx, %rcx
141a423fc: 48 3b 95 10 02 00 00        	cmpq	0x210(%rbp), %rdx
141a42403: 0f 84 0c 05 00 00           	je	0x141a42915 <.text+0x1a41915>
141a42409: 48 8d 51 0c                 	leaq	0xc(%rcx), %rdx
141a4240d: a8 01                       	testb	$0x1, %al
141a4240f: 74 2f                       	je	0x141a42440 <.text+0x1a41440>
141a42411: 48 3b 95 10 02 00 00        	cmpq	0x210(%rbp), %rdx
141a42418: 0f 84 e9 04 00 00           	je	0x141a42907 <.text+0x1a41907>
141a4241e: 48 8d 41 18                 	leaq	0x18(%rcx), %rax
141a42422: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
141a42429: 49 89 cd                    	movq	%rcx, %r13
141a4242c: 48 89 d1                    	movq	%rdx, %rcx
141a4242f: eb 1d                       	jmp	0x141a4244e <.text+0x1a4144e>
141a42431: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
141a42440: 48 89 95 30 02 00 00        	movq	%rdx, 0x230(%rbp)
141a42447: 4c 8b ad 20 02 00 00        	movq	0x220(%rbp), %r13
141a4244e: f3 41 0f 10 45 04           	movss	0x4(%r13), %xmm0
141a42454: f3 0f 5c 41 04              	subss	0x4(%rcx), %xmm0
141a42459: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
141a4245d: f3 41 0f 10 4d 08           	movss	0x8(%r13), %xmm1
141a42463: f3 41 0f 10 55 00           	movss	(%r13), %xmm2
141a42469: 0f 14 d1                    	unpcklps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0],xmm2[1],xmm1[1]
141a4246c: f3 0f 10 09                 	movss	(%rcx), %xmm1
141a42470: 48 89 8d 20 02 00 00        	movq	%rcx, 0x220(%rbp)
141a42477: f3 0f 10 59 08              	movss	0x8(%rcx), %xmm3
141a4247c: 0f 14 cb                    	unpcklps	%xmm3, %xmm1            # xmm1 = xmm1[0],xmm3[0],xmm1[1],xmm3[1]
141a4247f: 0f 5c d1                    	subps	%xmm1, %xmm2
141a42482: 0f 59 d2                    	mulps	%xmm2, %xmm2
141a42485: f3 0f 58 c2                 	addss	%xmm2, %xmm0
141a42489: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
141a4248d: f3 0f 58 c8                 	addss	%xmm0, %xmm1
141a42491: 0f 57 c0                    	xorps	%xmm0, %xmm0
141a42494: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
141a42498: f3 0f 5e 05 60 9e 47 01     	divss	0x1479e60(%rip), %xmm0  # 0x142ebc300
141a424a0: e8 3b 1a ee 00              	callq	0x142923ee0 <.text+0x2922ee0>
141a424a5: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
141a424aa: 48 89 c1                    	movq	%rax, %rcx
141a424ad: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
141a424b1: 0f 28 c8                    	movaps	%xmm0, %xmm1
141a424b4: f3 0f 5c 0d 20 7c f1 00     	subss	0xf17c20(%rip), %xmm1   # 0x14295a0dc
141a424bc: f3 48 0f 2c d1              	cvttss2si	%xmm1, %rdx
141a424c1: 48 21 ca                    	andq	%rcx, %rdx
141a424c4: 48 09 c2                    	orq	%rax, %rdx
141a424c7: 0f 2e c6                    	ucomiss	%xmm6, %xmm0
141a424ca: 41 bf 00 00 00 00           	movl	$0x0, %r15d
141a424d0: 4c 0f 43 fa                 	cmovaeq	%rdx, %r15
141a424d4: 0f 2e 05 05 7c f1 00        	ucomiss	0xf17c05(%rip), %xmm0   # 0x14295a0e0
141a424db: 48 c7 c0 ff ff ff ff        	movq	$-0x1, %rax
141a424e2: 4c 0f 47 f8                 	cmovaq	%rax, %r15
141a424e6: 49 83 ff 02                 	cmpq	$0x2, %r15
141a424ea: 0f 82 00 ff ff ff           	jb	0x141a423f0 <.text+0x1a413f0>
141a424f0: 4d 85 ff                    	testq	%r15, %r15
141a424f3: 78 0b                       	js	0x141a42500 <.text+0x1a41500>
141a424f5: f3 4d 0f 2a c7              	cvtsi2ss	%r15, %xmm8
141a424fa: eb 1d                       	jmp	0x141a42519 <.text+0x1a41519>
141a424fc: 0f 1f 40 00                 	nopl	(%rax)
141a42500: 4c 89 f8                    	movq	%r15, %rax
141a42503: 48 d1 e8                    	shrq	%rax
141a42506: 44 89 f9                    	movl	%r15d, %ecx
141a42509: 83 e1 01                    	andl	$0x1, %ecx
141a4250c: 48 09 c1                    	orq	%rax, %rcx
141a4250f: f3 4c 0f 2a c1              	cvtsi2ss	%rcx, %xmm8
141a42514: f3 45 0f 58 c0              	addss	%xmm8, %xmm8
141a42519: 4c 01 ff                    	addq	%r15, %rdi
141a4251c: 48 ff cf                    	decq	%rdi
141a4251f: 41 bc 01 00 00 00           	movl	$0x1, %r12d
141a42525: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
141a42530: 4d 85 e4                    	testq	%r12, %r12
141a42533: 78 0b                       	js	0x141a42540 <.text+0x1a41540>
141a42535: 0f 57 f6                    	xorps	%xmm6, %xmm6
141a42538: f3 49 0f 2a f4              	cvtsi2ss	%r12, %xmm6
141a4253d: eb 1c                       	jmp	0x141a4255b <.text+0x1a4155b>
141a4253f: 90                          	nop
141a42540: 4c 89 e0                    	movq	%r12, %rax
141a42543: 48 d1 e8                    	shrq	%rax
141a42546: 44 89 e1                    	movl	%r12d, %ecx
141a42549: 83 e1 01                    	andl	$0x1, %ecx
141a4254c: 48 09 c1                    	orq	%rax, %rcx
141a4254f: 0f 57 f6                    	xorps	%xmm6, %xmm6
141a42552: f3 48 0f 2a f1              	cvtsi2ss	%rcx, %xmm6
141a42557: f3 0f 58 f6                 	addss	%xmm6, %xmm6
141a4255b: 48 8b 85 20 02 00 00        	movq	0x220(%rbp), %rax
141a42562: f3 41 0f 5e f0              	divss	%xmm8, %xmm6
141a42567: 45 0f 28 e2                 	movaps	%xmm10, %xmm12
141a4256b: f3 44 0f 5c e6              	subss	%xmm6, %xmm12
141a42570: f3 45 0f 10 7d 08           	movss	0x8(%r13), %xmm15
141a42576: f3 45 0f 59 fc              	mulss	%xmm12, %xmm15
141a4257b: f2 45 0f 10 75 00           	movsd	(%r13), %xmm14
141a42581: f2 44 0f 10 28              	movsd	(%rax), %xmm13
141a42586: f3 44 0f 10 58 08           	movss	0x8(%rax), %xmm11
141a4258c: f3 44 0f 59 de              	mulss	%xmm6, %xmm11
141a42591: 48 89 d9                    	movq	%rbx, %rcx
141a42594: 48 8b 95 28 02 00 00        	movq	0x228(%rbp), %rdx
141a4259b: e8 d0 6f 25 ff              	callq	0x140c99570 <.text+0xc98570>
141a425a0: f3 41 0f 12 cc              	movsldup	%xmm12, %xmm1           # xmm1 = xmm12[0,0,2,2]
141a425a5: 41 0f 59 ce                 	mulps	%xmm14, %xmm1
141a425a9: f3 0f 12 c6                 	movsldup	%xmm6, %xmm0            # xmm0 = xmm6[0,0,2,2]
141a425ad: 41 0f 59 c5                 	mulps	%xmm13, %xmm0
141a425b1: 0f 58 c1                    	addps	%xmm1, %xmm0
141a425b4: f3 45 0f 58 fb              	addss	%xmm11, %xmm15
141a425b9: f3 44 0f 10 9d 60 01 00 00  	movss	0x160(%rbp), %xmm11
141a425c2: f3 0f 10 ad 64 01 00 00     	movss	0x164(%rbp), %xmm5
141a425ca: f3 0f 10 a5 68 01 00 00     	movss	0x168(%rbp), %xmm4
141a425d2: f3 0f 10 9d 6c 01 00 00     	movss	0x16c(%rbp), %xmm3
141a425da: f3 44 0f 10 a5 70 01 00 00  	movss	0x170(%rbp), %xmm12
141a425e3: f2 0f 10 95 74 01 00 00     	movsd	0x174(%rbp), %xmm2
141a425eb: f3 0f 10 8d 7c 01 00 00     	movss	0x17c(%rbp), %xmm1
141a425f3: f3 0f 10 b5 80 01 00 00     	movss	0x180(%rbp), %xmm6
141a425fb: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
141a425ff: 44 0f 2e ce                 	ucomiss	%xmm6, %xmm9
141a42603: 73 5b                       	jae	0x141a42660 <.text+0x1a41660>
141a42605: f3 45 0f 58 dc              	addss	%xmm12, %xmm11
141a4260a: f3 41 0f 58 f2              	addss	%xmm10, %xmm6
141a4260f: 45 0f 2e cb                 	ucomiss	%xmm11, %xmm9
141a42613: 0f 83 97 00 00 00           	jae	0x141a426b0 <.text+0x1a416b0>
141a42619: 41 0f 14 eb                 	unpcklps	%xmm11, %xmm5           # xmm5 = xmm5[0],xmm11[0],xmm5[1],xmm11[1]
141a4261d: 0f c6 ea 14                 	shufps	$0x14, %xmm2, %xmm5     # xmm5 = xmm5[0,1],xmm2[1,0]
141a42621: 0f 14 e1                    	unpcklps	%xmm1, %xmm4            # xmm4 = xmm4[0],xmm1[0],xmm4[1],xmm1[1]
141a42624: 0f 14 de                    	unpcklps	%xmm6, %xmm3            # xmm3 = xmm3[0],xmm6[0],xmm3[1],xmm6[1]
141a42627: 0f 16 dc                    	movlhps	%xmm4, %xmm3            # xmm3 = xmm3[0],xmm4[0]
141a4262a: 0f 28 cd                    	movaps	%xmm5, %xmm1
141a4262d: 0f 5c cb                    	subps	%xmm3, %xmm1
141a42630: 0f 58 dd                    	addps	%xmm5, %xmm3
141a42633: f3 0f 16 d3                 	movshdup	%xmm3, %xmm2            # xmm2 = xmm3[1,1,3,3]
141a42637: 0f c6 d9 01                 	shufps	$0x1, %xmm1, %xmm3      # xmm3 = xmm3[1,0],xmm1[0,0]
141a4263b: 0f c6 d9 e2                 	shufps	$0xe2, %xmm1, %xmm3     # xmm3 = xmm3[2,0],xmm1[2,3]
141a4263f: f3 0f 51 d2                 	sqrtss	%xmm2, %xmm2
141a42643: f3 0f 10 0d 25 32 ee 00     	movss	0xee3225(%rip), %xmm1   # 0x142925870
141a4264b: f3 0f 5e ca                 	divss	%xmm2, %xmm1
141a4264f: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
141a42653: 0f 59 cb                    	mulps	%xmm3, %xmm1
141a42656: e9 d4 00 00 00              	jmp	0x141a4272f <.text+0x1a4172f>
141a4265b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141a42660: f3 45 0f 5c e3              	subss	%xmm11, %xmm12
141a42665: 45 0f 28 da                 	movaps	%xmm10, %xmm11
141a42669: f3 44 0f 5c de              	subss	%xmm6, %xmm11
141a4266e: 45 0f 2e cc                 	ucomiss	%xmm12, %xmm9
141a42672: 73 6c                       	jae	0x141a426e0 <.text+0x1a416e0>
141a42674: 41 0f 16 ec                 	movlhps	%xmm12, %xmm5           # xmm5 = xmm5[0],xmm12[0]
141a42678: 0f 28 f2                    	movaps	%xmm2, %xmm6
141a4267b: 0f c6 f5 24                 	shufps	$0x24, %xmm5, %xmm6     # xmm6 = xmm6[0,1],xmm5[2,0]
141a4267f: 0f 14 cc                    	unpcklps	%xmm4, %xmm1            # xmm1 = xmm1[0],xmm4[0],xmm1[1],xmm4[1]
141a42682: 41 0f 16 db                 	movlhps	%xmm11, %xmm3           # xmm3 = xmm3[0],xmm11[0]
141a42686: 0f 5c d1                    	subps	%xmm1, %xmm2
141a42689: 0f c6 cb 24                 	shufps	$0x24, %xmm3, %xmm1     # xmm1 = xmm1[0,1],xmm3[2,0]
141a4268d: 0f 58 ce                    	addps	%xmm6, %xmm1
141a42690: 0f c6 d1 01                 	shufps	$0x1, %xmm1, %xmm2      # xmm2 = xmm2[1,0],xmm1[0,0]
141a42694: 0f c6 d1 e2                 	shufps	$0xe2, %xmm1, %xmm2     # xmm2 = xmm2[2,0],xmm1[2,3]
141a42698: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
141a4269b: 0f 57 db                    	xorps	%xmm3, %xmm3
141a4269e: f3 0f 51 d9                 	sqrtss	%xmm1, %xmm3
141a426a2: eb 78                       	jmp	0x141a4271c <.text+0x1a4171c>
141a426a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141a426b0: 0f 14 f5                    	unpcklps	%xmm5, %xmm6            # xmm6 = xmm6[0],xmm5[0],xmm6[1],xmm5[1]
141a426b3: 0f 16 f2                    	movlhps	%xmm2, %xmm6            # xmm6 = xmm6[0],xmm2[0]
141a426b6: 0f 14 cc                    	unpcklps	%xmm4, %xmm1            # xmm1 = xmm1[0],xmm4[0],xmm1[1],xmm4[1]
141a426b9: 44 0f 14 db                 	unpcklps	%xmm3, %xmm11           # xmm11 = xmm11[0],xmm3[0],xmm11[1],xmm3[1]
141a426bd: 44 0f 16 d9                 	movlhps	%xmm1, %xmm11           # xmm11 = xmm11[0],xmm1[0]
141a426c1: 0f 28 d6                    	movaps	%xmm6, %xmm2
141a426c4: 41 0f 58 d3                 	addps	%xmm11, %xmm2
141a426c8: 41 0f 5c f3                 	subps	%xmm11, %xmm6
141a426cc: f2 0f 10 d6                 	movsd	%xmm6, %xmm2            # xmm2 = xmm6[0],xmm2[1]
141a426d0: eb 43                       	jmp	0x141a42715 <.text+0x1a41715>
141a426d2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
141a426e0: 0f 28 f4                    	movaps	%xmm4, %xmm6
141a426e3: 0f 14 f2                    	unpcklps	%xmm2, %xmm6            # xmm6 = xmm6[0],xmm2[0],xmm6[1],xmm2[1]
141a426e6: 0f 16 e5                    	movlhps	%xmm5, %xmm4            # xmm4 = xmm4[0],xmm5[0]
141a426e9: 44 0f 16 dd                 	movlhps	%xmm5, %xmm11           # xmm11 = xmm11[0],xmm5[0]
141a426ed: 41 0f c6 f3 24              	shufps	$0x24, %xmm11, %xmm6    # xmm6 = xmm6[0,1],xmm11[2,0]
141a426f2: 0f c6 ca d4                 	shufps	$0xd4, %xmm2, %xmm1     # xmm1 = xmm1[0,1],xmm2[1,3]
141a426f6: 0f c6 d3 c5                 	shufps	$0xc5, %xmm3, %xmm2     # xmm2 = xmm2[1,1],xmm3[0,3]
141a426fa: 0f 58 d4                    	addps	%xmm4, %xmm2
141a426fd: 44 0f 16 e3                 	movlhps	%xmm3, %xmm12           # xmm12 = xmm12[0],xmm3[0]
141a42701: 41 0f c6 cc 22              	shufps	$0x22, %xmm12, %xmm1    # xmm1 = xmm1[2,0],xmm12[2,0]
141a42706: 0f 5c f1                    	subps	%xmm1, %xmm6
141a42709: 0f c6 d6 d8                 	shufps	$0xd8, %xmm6, %xmm2     # xmm2 = xmm2[0,2],xmm6[1,3]
141a4270d: 0f c6 d2 d8                 	shufps	$0xd8, %xmm2, %xmm2     # xmm2 = xmm2[0,2,1,3]
141a42711: 0f c6 f6 ff                 	shufps	$0xff, %xmm6, %xmm6     # xmm6 = xmm6[3,3,3,3]
141a42715: 0f 57 db                    	xorps	%xmm3, %xmm3
141a42718: f3 0f 51 de                 	sqrtss	%xmm6, %xmm3
141a4271c: f3 0f 10 0d 4c 31 ee 00     	movss	0xee314c(%rip), %xmm1   # 0x142925870
141a42724: f3 0f 5e cb                 	divss	%xmm3, %xmm1
141a42728: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
141a4272c: 0f 59 ca                    	mulps	%xmm2, %xmm1
141a4272f: 0f 13 85 e0 01 00 00        	movlps	%xmm0, 0x1e0(%rbp)
141a42736: f3 44 0f 11 bd e8 01 00 00  	movss	%xmm15, 0x1e8(%rbp)
141a4273f: c7 85 ec 01 00 00 eb 51 38 3e       	movl	$0x3e3851eb, 0x1ec(%rbp) # imm = 0x3E3851EB
141a42749: f3 0f 11 bd f0 01 00 00     	movss	%xmm7, 0x1f0(%rbp)
141a42751: c7 85 f4 01 00 00 eb 51 38 3e       	movl	$0x3e3851eb, 0x1f4(%rbp) # imm = 0x3E3851EB
141a4275b: 0f 10 9d ec 01 00 00        	movups	0x1ec(%rbp), %xmm3
141a42762: 0f 28 85 f0 01 00 00        	movaps	0x1f0(%rbp), %xmm0
141a42769: 0f 28 95 e0 01 00 00        	movaps	0x1e0(%rbp), %xmm2
141a42770: 0f 28 e1                    	movaps	%xmm1, %xmm4
141a42773: 0f c6 e1 ff                 	shufps	$0xff, %xmm1, %xmm4     # xmm4 = xmm4[3,3],xmm1[3,3]
141a42777: 44 0f 28 f9                 	movaps	%xmm1, %xmm15
141a4277b: 44 0f 58 f9                 	addps	%xmm1, %xmm15
141a4277f: 0f 28 f4                    	movaps	%xmm4, %xmm6
141a42782: f3 41 0f 59 f7              	mulss	%xmm15, %xmm6
141a42787: 41 0f 28 ef                 	movaps	%xmm15, %xmm5
141a4278b: 66 41 0f 15 ef              	unpckhpd	%xmm15, %xmm5           # xmm5 = xmm5[1],xmm15[1]
141a42790: f3 0f 59 e5                 	mulss	%xmm5, %xmm4
141a42794: 44 0f 28 f1                 	movaps	%xmm1, %xmm14
141a42798: 45 0f 59 f7                 	mulps	%xmm15, %xmm14
141a4279c: f3 44 0f 16 c9              	movshdup	%xmm1, %xmm9            # xmm9 = xmm1[1,1,3,3]
141a427a1: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
141a427a4: f3 41 0f 59 cf              	mulss	%xmm15, %xmm1
141a427a9: 45 0f 28 df                 	movaps	%xmm15, %xmm11
141a427ad: 45 0f c6 df ff              	shufps	$0xff, %xmm15, %xmm11   # xmm11 = xmm11[3,3],xmm15[3,3]
141a427b2: f3 45 0f 59 d9              	mulss	%xmm9, %xmm11
141a427b7: f3 41 0f 59 e9              	mulss	%xmm9, %xmm5
141a427bc: f3 45 0f 59 f9              	mulss	%xmm9, %xmm15
141a427c1: 45 0f 28 ee                 	movaps	%xmm14, %xmm13
141a427c5: 66 45 0f 15 ee              	unpckhpd	%xmm14, %xmm13          # xmm13 = xmm13[1],xmm14[1]
141a427ca: 45 0f 28 cd                 	movaps	%xmm13, %xmm9
141a427ce: f3 45 0f 58 ce              	addss	%xmm14, %xmm9
141a427d3: 45 0f 28 e2                 	movaps	%xmm10, %xmm12
141a427d7: f3 45 0f 5c e1              	subss	%xmm9, %xmm12
141a427dc: 44 0f 28 cc                 	movaps	%xmm4, %xmm9
141a427e0: f3 45 0f 58 cf              	addss	%xmm15, %xmm9
141a427e5: 45 0f 14 e1                 	unpcklps	%xmm9, %xmm12           # xmm12 = xmm12[0],xmm9[0],xmm12[1],xmm9[1]
141a427e9: 44 0f 28 ce                 	movaps	%xmm6, %xmm9
141a427ed: f3 44 0f 5c cd              	subss	%xmm5, %xmm9
141a427f2: 45 0f c6 e1 04              	shufps	$0x4, %xmm9, %xmm12     # xmm12 = xmm12[0,1],xmm9[0,0]
141a427f7: f3 41 0f 5c e7              	subss	%xmm15, %xmm4
141a427fc: 45 0f 28 ce                 	movaps	%xmm14, %xmm9
141a42800: 45 0f c6 ce ff              	shufps	$0xff, %xmm14, %xmm9    # xmm9 = xmm9[3,3],xmm14[3,3]
141a42805: f3 45 0f 58 f1              	addss	%xmm9, %xmm14
141a4280a: 45 0f 28 fa                 	movaps	%xmm10, %xmm15
141a4280e: f3 45 0f 5c fe              	subss	%xmm14, %xmm15
141a42813: 41 0f 14 e7                 	unpcklps	%xmm15, %xmm4           # xmm4 = xmm4[0],xmm15[0],xmm4[1],xmm15[1]
141a42817: 44 0f 28 f1                 	movaps	%xmm1, %xmm14
141a4281b: f3 45 0f 58 f3              	addss	%xmm11, %xmm14
141a42820: 41 0f c6 e6 04              	shufps	$0x4, %xmm14, %xmm4     # xmm4 = xmm4[0,1],xmm14[0,0]
141a42825: f3 0f 58 ee                 	addss	%xmm6, %xmm5
141a42829: f3 41 0f 5c cb              	subss	%xmm11, %xmm1
141a4282e: 0f 14 e9                    	unpcklps	%xmm1, %xmm5            # xmm5 = xmm5[0],xmm1[0],xmm5[1],xmm1[1]
141a42831: f3 45 0f 58 cd              	addss	%xmm13, %xmm9
141a42836: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
141a4283a: f3 41 0f 5c c9              	subss	%xmm9, %xmm1
141a4283f: 0f c6 e9 04                 	shufps	$0x4, %xmm1, %xmm5      # xmm5 = xmm5[0,1],xmm1[0,0]
141a42843: 0f c6 db 00                 	shufps	$0x0, %xmm3, %xmm3      # xmm3 = xmm3[0,0,0,0]
141a42847: 41 0f 59 dc                 	mulps	%xmm12, %xmm3
141a4284b: 0f 28 c8                    	movaps	%xmm0, %xmm1
141a4284e: 0f c6 c8 00                 	shufps	$0x0, %xmm0, %xmm1      # xmm1 = xmm1[0,0],xmm0[0,0]
141a42852: 0f 59 cc                    	mulps	%xmm4, %xmm1
141a42855: 0f c6 c0 55                 	shufps	$0x55, %xmm0, %xmm0     # xmm0 = xmm0[1,1,1,1]
141a42859: 0f 59 c5                    	mulps	%xmm5, %xmm0
141a4285c: 0f c6 d2 a4                 	shufps	$0xa4, %xmm2, %xmm2     # xmm2 = xmm2[0,1,2,2]
141a42860: 0f 29 9d 60 01 00 00        	movaps	%xmm3, 0x160(%rbp)
141a42867: 0f 29 8d 70 01 00 00        	movaps	%xmm1, 0x170(%rbp)
141a4286e: 0f 29 85 80 01 00 00        	movaps	%xmm0, 0x180(%rbp)
141a42875: 0f 29 95 90 01 00 00        	movaps	%xmm2, 0x190(%rbp)
141a4287c: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
141a42883: 48 89 da                    	movq	%rbx, %rdx
141a42886: e8 d5 ac e5 fe              	callq	0x14089d560 <.text+0x89c560>
141a4288b: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
141a42892: 0f 10 8d d0 00 00 00        	movups	0xd0(%rbp), %xmm1
141a42899: 0f 10 95 e0 00 00 00        	movups	0xe0(%rbp), %xmm2
141a428a0: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
141a428a7: 0f 29 8d 70 01 00 00        	movaps	%xmm1, 0x170(%rbp)
141a428ae: 0f 29 95 80 01 00 00        	movaps	%xmm2, 0x180(%rbp)
141a428b5: 0f 28 05 e4 25 0d 01        	movaps	0x10d25e4(%rip), %xmm0  # 0x142b14ea0
141a428bc: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
141a428c3: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
141a428c7: 48 89 da                    	movq	%rbx, %rdx
141a428ca: e8 31 37 77 00              	callq	0x1421b6000 <.text+0x21b5000>
141a428cf: 49 ff c4                    	incq	%r12
141a428d2: 41 08 c6                    	orb	%al, %r14b
141a428d5: 4d 39 e7                    	cmpq	%r12, %r15
141a428d8: 0f 85 52 fc ff ff           	jne	0x141a42530 <.text+0x1a41530>
141a428de: 4c 8b a5 b8 00 00 00        	movq	0xb8(%rbp), %r12
141a428e5: 0f 57 f6                    	xorps	%xmm6, %xmm6
141a428e8: e9 03 fb ff ff              	jmp	0x141a423f0 <.text+0x1a413f0>
141a428ed: 0f 1f 00                    	nopl	(%rax)
141a428f0: 31 ff                       	xorl	%edi, %edi
141a428f2: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
141a428f9: 48 85 c0                    	testq	%rax, %rax
141a428fc: 0f 85 54 fa ff ff           	jne	0x141a42356 <.text+0x1a41356>
141a42902: e9 65 fa ff ff              	jmp	0x141a4236c <.text+0x1a4136c>
141a42907: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
141a4290e: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
141a42915: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
141a4291c: 48 85 c0                    	testq	%rax, %rax
141a4291f: 74 1a                       	je	0x141a4293b <.text+0x1a4193b>
141a42921: 48 c1 e0 02                 	shlq	$0x2, %rax
141a42925: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
141a42929: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141a4292f: 48 8b 8d 90 00 00 00        	movq	0x90(%rbp), %rcx
141a42936: e8 e5 12 bd fe              	callq	0x140613c20 <.text+0x612c20>
141a4293b: 48 8b 95 38 01 00 00        	movq	0x138(%rbp), %rdx
141a42942: 48 85 d2                    	testq	%rdx, %rdx
141a42945: 74 16                       	je	0x141a4295d <.text+0x1a4195d>
141a42947: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
141a4294e: 48 c1 e2 02                 	shlq	$0x2, %rdx
141a42952: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141a42958: e8 c3 12 bd fe              	callq	0x140613c20 <.text+0x612c20>
141a4295d: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
141a42964: 0f 28 cf                    	movaps	%xmm7, %xmm1
141a42967: e8 14 26 0f ff              	callq	0x140b34f80 <.text+0xb33f80>
141a4296c: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
141a42973: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
141a4297a: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
141a4297e: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
141a42982: c7 45 78 00 00 00 00        	movl	$0x0, 0x78(%rbp)
141a42989: f3 0f 11 7d 7c              	movss	%xmm7, 0x7c(%rbp)
141a4298e: 48 89 8d 80 00 00 00        	movq	%rcx, 0x80(%rbp)
141a42995: 48 89 c8                    	movq	%rcx, %rax
141a42998: 48 c7 85 10 02 00 00 00 00 00 00    	movq	$0x0, 0x210(%rbp)
141a429a3: 48 89 b5 90 00 00 00        	movq	%rsi, 0x90(%rbp)
141a429aa: eb 07                       	jmp	0x141a429b3 <.text+0x1a419b3>
141a429ac: 0f 1f 40 00                 	nopl	(%rax)
141a429b0: 4c 89 f8                    	movq	%r15, %rax
141a429b3: 48 85 c0                    	testq	%rax, %rax
141a429b6: 74 18                       	je	0x141a429d0 <.text+0x1a419d0>
141a429b8: 48 3b 45 e8                 	cmpq	-0x18(%rbp), %rax
141a429bc: 74 12                       	je	0x141a429d0 <.text+0x1a419d0>
141a429be: 4c 8d 78 04                 	leaq	0x4(%rax), %r15
141a429c2: eb 35                       	jmp	0x141a429f9 <.text+0x1a419f9>
141a429c4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141a429d0: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
141a429d7: 48 83 f8 08                 	cmpq	$0x8, %rax
141a429db: 0f 84 ef 05 00 00           	je	0x141a42fd0 <.text+0x1a41fd0>
141a429e1: 48 89 c1                    	movq	%rax, %rcx
141a429e4: 48 01 e8                    	addq	%rbp, %rax
141a429e7: 48 83 c0 78                 	addq	$0x78, %rax
141a429eb: 48 83 c1 04                 	addq	$0x4, %rcx
141a429ef: 48 89 8d 10 02 00 00        	movq	%rcx, 0x210(%rbp)
141a429f6: 45 31 ff                    	xorl	%r15d, %r15d
141a429f9: f3 44 0f 10 00              	movss	(%rax), %xmm8
141a429fe: 0f 57 d2                    	xorps	%xmm2, %xmm2
141a42a01: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
141a42a08: 48 8b 95 28 02 00 00        	movq	0x228(%rbp), %rdx
141a42a0f: e8 dc 67 25 ff              	callq	0x140c991f0 <.text+0xc981f0>
141a42a14: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
141a42a1b: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
141a42a22: 4c 8b ad d0 00 00 00        	movq	0xd0(%rbp), %r13
141a42a29: 4d 85 ed                    	testq	%r13, %r13
141a42a2c: 0f 84 3e 05 00 00           	je	0x141a42f70 <.text+0x1a41f70>
141a42a32: 49 83 fd 01                 	cmpq	$0x1, %r13
141a42a36: 0f 84 34 05 00 00           	je	0x141a42f70 <.text+0x1a41f70>
141a42a3c: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
141a42a43: f3 0f 10 40 08              	movss	0x8(%rax), %xmm0
141a42a48: f3 0f 10 48 14              	movss	0x14(%rax), %xmm1
141a42a4d: 0f 28 f0                    	movaps	%xmm0, %xmm6
141a42a50: f3 0f 58 f1                 	addss	%xmm1, %xmm6
141a42a54: f2 0f 10 10                 	movsd	(%rax), %xmm2
141a42a58: f2 0f 10 58 0c              	movsd	0xc(%rax), %xmm3
141a42a5d: 44 0f 28 da                 	movaps	%xmm2, %xmm11
141a42a61: 44 0f 58 db                 	addps	%xmm3, %xmm11
141a42a65: 44 0f 59 1d 73 2e ee 00     	mulps	0xee2e73(%rip), %xmm11  # 0x1429258e0
141a42a6d: f3 0f 59 35 fb 2d ee 00     	mulss	0xee2dfb(%rip), %xmm6   # 0x142925870
141a42a75: f3 0f 5c da                 	subss	%xmm2, %xmm3
141a42a79: f3 0f 5c c8                 	subss	%xmm0, %xmm1
141a42a7d: 0f 28 c3                    	movaps	%xmm3, %xmm0
141a42a80: f3 0f 59 c3                 	mulss	%xmm3, %xmm0
141a42a84: 0f 28 d1                    	movaps	%xmm1, %xmm2
141a42a87: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
141a42a8b: f3 0f 58 d0                 	addss	%xmm0, %xmm2
141a42a8f: 0f 57 c0                    	xorps	%xmm0, %xmm0
141a42a92: f3 0f 51 c2                 	sqrtss	%xmm2, %xmm0
141a42a96: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
141a42a9a: f3 0f 5e d0                 	divss	%xmm0, %xmm2
141a42a9e: f3 0f 59 da                 	mulss	%xmm2, %xmm3
141a42aa2: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
141a42aa6: f3 0f 11 9d e0 01 00 00     	movss	%xmm3, 0x1e0(%rbp)
141a42aae: f3 0f 11 95 e4 01 00 00     	movss	%xmm2, 0x1e4(%rbp)
141a42ab6: 0f 57 d2                    	xorps	%xmm2, %xmm2
141a42ab9: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
141a42ac0: 48 8d 95 e0 01 00 00        	leaq	0x1e0(%rbp), %rdx
141a42ac7: e8 14 a3 25 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
141a42acc: 48 89 d9                    	movq	%rbx, %rcx
141a42acf: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
141a42ad6: e8 65 df 24 ff              	callq	0x140c90a40 <.text+0xc8fa40>
141a42adb: f3 44 0f 58 85 1c 02 00 00  	addss	0x21c(%rbp), %xmm8
141a42ae4: 45 0f 28 f8                 	movaps	%xmm8, %xmm15
141a42ae8: f3 44 0f 59 3d 9f db f1 00  	mulss	0xf1db9f(%rip), %xmm15  # 0x142960690
141a42af1: 45 0f 28 e7                 	movaps	%xmm15, %xmm12
141a42af5: 45 0f 14 e0                 	unpcklps	%xmm8, %xmm12           # xmm12 = xmm12[0],xmm8[0],xmm12[1],xmm8[1]
141a42af9: 45 0f 58 dc                 	addps	%xmm12, %xmm11
141a42afd: f3 41 0f 58 f7              	addss	%xmm15, %xmm6
141a42b02: 0f 28 9d 60 01 00 00        	movaps	0x160(%rbp), %xmm3
141a42b09: f3 0f 10 85 68 01 00 00     	movss	0x168(%rbp), %xmm0
141a42b11: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
141a42b18: f3 0f 10 08                 	movss	(%rax), %xmm1
141a42b1c: f3 0f 10 50 04              	movss	0x4(%rax), %xmm2
141a42b21: f3 0f 5c 48 0c              	subss	0xc(%rax), %xmm1
141a42b26: f3 0f 5c 50 10              	subss	0x10(%rax), %xmm2
141a42b2b: f3 0f 10 60 08              	movss	0x8(%rax), %xmm4
141a42b30: f3 0f 5c 60 14              	subss	0x14(%rax), %xmm4
141a42b35: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
141a42b39: f3 0f 59 d2                 	mulss	%xmm2, %xmm2
141a42b3d: f3 0f 58 d1                 	addss	%xmm1, %xmm2
141a42b41: f3 0f 59 e4                 	mulss	%xmm4, %xmm4
141a42b45: f3 0f 58 e2                 	addss	%xmm2, %xmm4
141a42b49: 0f 57 c9                    	xorps	%xmm1, %xmm1
141a42b4c: f3 0f 51 cc                 	sqrtss	%xmm4, %xmm1
141a42b50: 44 0f 13 9d 00 01 00 00     	movlps	%xmm11, 0x100(%rbp)
141a42b58: f3 0f 11 b5 08 01 00 00     	movss	%xmm6, 0x108(%rbp)
141a42b60: f2 0f 10 15 88 9b 04 01     	movsd	0x1049b88(%rip), %xmm2  # 0x142a8c6f0
141a42b68: f2 0f 11 95 0c 01 00 00     	movsd	%xmm2, 0x10c(%rbp)
141a42b70: f3 0f 11 8d 14 01 00 00     	movss	%xmm1, 0x114(%rbp)
141a42b78: 0f 28 f0                    	movaps	%xmm0, %xmm6
141a42b7b: f3 0f 58 f0                 	addss	%xmm0, %xmm6
141a42b7f: f3 0f 59 c6                 	mulss	%xmm6, %xmm0
141a42b83: 0f 28 d3                    	movaps	%xmm3, %xmm2
141a42b86: 0f 58 d3                    	addps	%xmm3, %xmm2
141a42b89: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
141a42b8d: f3 0f 59 cb                 	mulss	%xmm3, %xmm1
141a42b91: 0f 28 e3                    	movaps	%xmm3, %xmm4
141a42b94: f3 0f 12 ee                 	movsldup	%xmm6, %xmm5            # xmm5 = xmm6[0,0,2,2]
141a42b98: 0f 59 eb                    	mulps	%xmm3, %xmm5
141a42b9b: 0f c6 db ff                 	shufps	$0xff, %xmm3, %xmm3     # xmm3 = xmm3[3,3,3,3]
141a42b9f: 0f 59 e2                    	mulps	%xmm2, %xmm4
141a42ba2: 0f 59 d3                    	mulps	%xmm3, %xmm2
141a42ba5: 44 0f 28 c3                 	movaps	%xmm3, %xmm8
141a42ba9: f3 44 0f 59 c6              	mulss	%xmm6, %xmm8
141a42bae: f3 0f 16 f4                 	movshdup	%xmm4, %xmm6            # xmm6 = xmm4[1,1,3,3]
141a42bb2: 44 0f 28 de                 	movaps	%xmm6, %xmm11
141a42bb6: f3 44 0f 58 d8              	addss	%xmm0, %xmm11
141a42bbb: 41 0f 28 da                 	movaps	%xmm10, %xmm3
141a42bbf: f3 41 0f 5c db              	subss	%xmm11, %xmm3
141a42bc4: 44 0f 28 d9                 	movaps	%xmm1, %xmm11
141a42bc8: f3 45 0f 58 d8              	addss	%xmm8, %xmm11
141a42bcd: 41 0f 14 db                 	unpcklps	%xmm11, %xmm3           # xmm3 = xmm3[0],xmm11[0],xmm3[1],xmm11[1]
141a42bd1: f3 41 0f 5c c8              	subss	%xmm8, %xmm1
141a42bd6: f3 0f 58 c4                 	addss	%xmm4, %xmm0
141a42bda: 45 0f 28 c2                 	movaps	%xmm10, %xmm8
141a42bde: f3 44 0f 5c c0              	subss	%xmm0, %xmm8
141a42be3: 0f c6 d2 e1                 	shufps	$0xe1, %xmm2, %xmm2     # xmm2 = xmm2[1,0,2,3]
141a42be7: 41 0f 14 c8                 	unpcklps	%xmm8, %xmm1            # xmm1 = xmm1[0],xmm8[0],xmm1[1],xmm8[1]
141a42beb: 0f 28 c5                    	movaps	%xmm5, %xmm0
141a42bee: 0f 5c c2                    	subps	%xmm2, %xmm0
141a42bf1: 0f 58 d5                    	addps	%xmm5, %xmm2
141a42bf4: 0f c6 d8 04                 	shufps	$0x4, %xmm0, %xmm3      # xmm3 = xmm3[0,1],xmm0[0,0]
141a42bf8: 0f c6 ca 54                 	shufps	$0x54, %xmm2, %xmm1     # xmm1 = xmm1[0,1],xmm2[1,1]
141a42bfc: 0f c6 c2 01                 	shufps	$0x1, %xmm2, %xmm0      # xmm0 = xmm0[1,0],xmm2[0,0]
141a42c00: f3 0f 58 e6                 	addss	%xmm6, %xmm4
141a42c04: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
141a42c08: f3 0f 5c d4                 	subss	%xmm4, %xmm2
141a42c0c: 0f 10 a5 0c 01 00 00        	movups	0x10c(%rbp), %xmm4
141a42c13: 0f c6 c2 02                 	shufps	$0x2, %xmm2, %xmm0      # xmm0 = xmm0[2,0],xmm2[0,0]
141a42c17: 0f 28 95 10 01 00 00        	movaps	0x110(%rbp), %xmm2
141a42c1e: 0f c6 e4 00                 	shufps	$0x0, %xmm4, %xmm4      # xmm4 = xmm4[0,0,0,0]
141a42c22: 0f 59 e3                    	mulps	%xmm3, %xmm4
141a42c25: 0f 28 da                    	movaps	%xmm2, %xmm3
141a42c28: 0f c6 da 00                 	shufps	$0x0, %xmm2, %xmm3      # xmm3 = xmm3[0,0],xmm2[0,0]
141a42c2c: 0f 59 d9                    	mulps	%xmm1, %xmm3
141a42c2f: 0f c6 d2 55                 	shufps	$0x55, %xmm2, %xmm2     # xmm2 = xmm2[1,1,1,1]
141a42c33: 0f 59 d0                    	mulps	%xmm0, %xmm2
141a42c36: 0f 29 a5 60 01 00 00        	movaps	%xmm4, 0x160(%rbp)
141a42c3d: 0f 29 9d 70 01 00 00        	movaps	%xmm3, 0x170(%rbp)
141a42c44: 0f 29 95 80 01 00 00        	movaps	%xmm2, 0x180(%rbp)
141a42c4b: 0f 28 85 00 01 00 00        	movaps	0x100(%rbp), %xmm0
141a42c52: 0f c6 c0 a4                 	shufps	$0xa4, %xmm0, %xmm0     # xmm0 = xmm0[0,1,2,2]
141a42c56: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
141a42c5d: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
141a42c64: 48 89 da                    	movq	%rbx, %rdx
141a42c67: e8 f4 a8 e5 fe              	callq	0x14089d560 <.text+0x89c560>
141a42c6c: 0f 10 85 e0 01 00 00        	movups	0x1e0(%rbp), %xmm0
141a42c73: 0f 10 8d f0 01 00 00        	movups	0x1f0(%rbp), %xmm1
141a42c7a: 0f 10 95 00 02 00 00        	movups	0x200(%rbp), %xmm2
141a42c81: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
141a42c88: 0f 29 8d 30 01 00 00        	movaps	%xmm1, 0x130(%rbp)
141a42c8f: 0f 29 95 40 01 00 00        	movaps	%xmm2, 0x140(%rbp)
141a42c96: 0f 28 05 03 22 0d 01        	movaps	0x10d2203(%rip), %xmm0  # 0x142b14ea0
141a42c9d: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
141a42ca4: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
141a42ca8: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
141a42caf: e8 4c 33 77 00              	callq	0x1421b6000 <.text+0x21b5000>
141a42cb4: 41 08 c6                    	orb	%al, %r14b
141a42cb7: 48 ff c7                    	incq	%rdi
141a42cba: 49 83 fd 02                 	cmpq	$0x2, %r13
141a42cbe: 0f 84 ac 02 00 00           	je	0x141a42f70 <.text+0x1a41f70>
141a42cc4: 4a 8d 04 6d 00 00 00 00     	leaq	(,%r13,2), %rax
141a42ccc: 4c 01 e8                    	addq	%r13, %rax
141a42ccf: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
141a42cd6: 4c 8d 2c 81                 	leaq	(%rcx,%rax,4), %r13
141a42cda: 4c 8d 61 0c                 	leaq	0xc(%rcx), %r12
141a42cde: 48 8d 71 18                 	leaq	0x18(%rcx), %rsi
141a42ce2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
141a42cf0: f3 41 0f 10 44 24 08        	movss	0x8(%r12), %xmm0
141a42cf7: f3 0f 10 4e 08              	movss	0x8(%rsi), %xmm1
141a42cfc: 0f 28 f0                    	movaps	%xmm0, %xmm6
141a42cff: f3 0f 58 f1                 	addss	%xmm1, %xmm6
141a42d03: f2 41 0f 10 14 24           	movsd	(%r12), %xmm2
141a42d09: f2 0f 10 1e                 	movsd	(%rsi), %xmm3
141a42d0d: 44 0f 28 c2                 	movaps	%xmm2, %xmm8
141a42d11: 44 0f 58 c3                 	addps	%xmm3, %xmm8
141a42d15: 44 0f 59 05 c3 2b ee 00     	mulps	0xee2bc3(%rip), %xmm8   # 0x1429258e0
141a42d1d: f3 0f 59 35 4b 2b ee 00     	mulss	0xee2b4b(%rip), %xmm6   # 0x142925870
141a42d25: f3 0f 5c da                 	subss	%xmm2, %xmm3
141a42d29: f3 0f 5c c8                 	subss	%xmm0, %xmm1
141a42d2d: 0f 28 c3                    	movaps	%xmm3, %xmm0
141a42d30: f3 0f 59 c3                 	mulss	%xmm3, %xmm0
141a42d34: 0f 28 d1                    	movaps	%xmm1, %xmm2
141a42d37: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
141a42d3b: f3 0f 58 d0                 	addss	%xmm0, %xmm2
141a42d3f: 0f 57 c0                    	xorps	%xmm0, %xmm0
141a42d42: f3 0f 51 c2                 	sqrtss	%xmm2, %xmm0
141a42d46: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
141a42d4a: f3 0f 5e d0                 	divss	%xmm0, %xmm2
141a42d4e: f3 0f 59 da                 	mulss	%xmm2, %xmm3
141a42d52: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
141a42d56: f3 0f 11 9d e0 01 00 00     	movss	%xmm3, 0x1e0(%rbp)
141a42d5e: f3 0f 11 95 e4 01 00 00     	movss	%xmm2, 0x1e4(%rbp)
141a42d66: 0f 57 d2                    	xorps	%xmm2, %xmm2
141a42d69: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
141a42d70: 48 8d 95 e0 01 00 00        	leaq	0x1e0(%rbp), %rdx
141a42d77: e8 64 a0 25 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
141a42d7c: 48 89 d9                    	movq	%rbx, %rcx
141a42d7f: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
141a42d86: e8 b5 dc 24 ff              	callq	0x140c90a40 <.text+0xc8fa40>
141a42d8b: 45 0f 58 c4                 	addps	%xmm12, %xmm8
141a42d8f: f3 41 0f 58 f7              	addss	%xmm15, %xmm6
141a42d94: 0f 28 ad 60 01 00 00        	movaps	0x160(%rbp), %xmm5
141a42d9b: f3 0f 10 9d 68 01 00 00     	movss	0x168(%rbp), %xmm3
141a42da3: f3 41 0f 10 04 24           	movss	(%r12), %xmm0
141a42da9: f3 41 0f 10 4c 24 04        	movss	0x4(%r12), %xmm1
141a42db0: f3 0f 5c 06                 	subss	(%rsi), %xmm0
141a42db4: f3 0f 5c 4e 04              	subss	0x4(%rsi), %xmm1
141a42db9: f3 41 0f 10 54 24 08        	movss	0x8(%r12), %xmm2
141a42dc0: f3 0f 5c 56 08              	subss	0x8(%rsi), %xmm2
141a42dc5: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
141a42dc9: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
141a42dcd: f3 0f 58 c8                 	addss	%xmm0, %xmm1
141a42dd1: f3 0f 59 d2                 	mulss	%xmm2, %xmm2
141a42dd5: f3 0f 58 d1                 	addss	%xmm1, %xmm2
141a42dd9: 0f 57 c0                    	xorps	%xmm0, %xmm0
141a42ddc: f3 0f 51 c2                 	sqrtss	%xmm2, %xmm0
141a42de0: 44 0f 13 85 00 01 00 00     	movlps	%xmm8, 0x100(%rbp)
141a42de8: f3 0f 11 b5 08 01 00 00     	movss	%xmm6, 0x108(%rbp)
141a42df0: f2 0f 10 0d f8 98 04 01     	movsd	0x10498f8(%rip), %xmm1  # 0x142a8c6f0
141a42df8: f2 0f 11 8d 0c 01 00 00     	movsd	%xmm1, 0x10c(%rbp)
141a42e00: f3 0f 11 85 14 01 00 00     	movss	%xmm0, 0x114(%rbp)
141a42e08: 0f 10 95 0c 01 00 00        	movups	0x10c(%rbp), %xmm2
141a42e0f: 0f 28 85 10 01 00 00        	movaps	0x110(%rbp), %xmm0
141a42e16: 0f 28 8d 00 01 00 00        	movaps	0x100(%rbp), %xmm1
141a42e1d: 44 0f 28 eb                 	movaps	%xmm3, %xmm13
141a42e21: f3 44 0f 58 eb              	addss	%xmm3, %xmm13
141a42e26: f3 41 0f 59 dd              	mulss	%xmm13, %xmm3
141a42e2b: 0f 28 f5                    	movaps	%xmm5, %xmm6
141a42e2e: 0f 58 f5                    	addps	%xmm5, %xmm6
141a42e31: f3 0f 16 e6                 	movshdup	%xmm6, %xmm4            # xmm4 = xmm6[1,1,3,3]
141a42e35: f3 0f 59 e5                 	mulss	%xmm5, %xmm4
141a42e39: 44 0f 28 c5                 	movaps	%xmm5, %xmm8
141a42e3d: f3 45 0f 12 dd              	movsldup	%xmm13, %xmm11          # xmm11 = xmm13[0,0,2,2]
141a42e42: 44 0f 59 dd                 	mulps	%xmm5, %xmm11
141a42e46: 0f c6 ed ff                 	shufps	$0xff, %xmm5, %xmm5     # xmm5 = xmm5[3,3,3,3]
141a42e4a: 44 0f 59 c6                 	mulps	%xmm6, %xmm8
141a42e4e: 0f 59 f5                    	mulps	%xmm5, %xmm6
141a42e51: 44 0f 28 f5                 	movaps	%xmm5, %xmm14
141a42e55: f3 45 0f 59 f5              	mulss	%xmm13, %xmm14
141a42e5a: 0f c6 f6 e1                 	shufps	$0xe1, %xmm6, %xmm6     # xmm6 = xmm6[1,0,2,3]
141a42e5e: f3 45 0f 16 c8              	movshdup	%xmm8, %xmm9            # xmm9 = xmm8[1,1,3,3]
141a42e63: 41 0f 28 e9                 	movaps	%xmm9, %xmm5
141a42e67: f3 0f 58 eb                 	addss	%xmm3, %xmm5
141a42e6b: 45 0f 28 ea                 	movaps	%xmm10, %xmm13
141a42e6f: f3 44 0f 5c ed              	subss	%xmm5, %xmm13
141a42e74: 0f 28 ec                    	movaps	%xmm4, %xmm5
141a42e77: f3 41 0f 58 ee              	addss	%xmm14, %xmm5
141a42e7c: 44 0f 14 ed                 	unpcklps	%xmm5, %xmm13           # xmm13 = xmm13[0],xmm5[0],xmm13[1],xmm5[1]
141a42e80: 41 0f 28 eb                 	movaps	%xmm11, %xmm5
141a42e84: 0f 5c ee                    	subps	%xmm6, %xmm5
141a42e87: 44 0f c6 ed 04              	shufps	$0x4, %xmm5, %xmm13     # xmm13 = xmm13[0,1],xmm5[0,0]
141a42e8c: f3 41 0f 5c e6              	subss	%xmm14, %xmm4
141a42e91: f3 41 0f 58 d8              	addss	%xmm8, %xmm3
141a42e96: 45 0f 28 f2                 	movaps	%xmm10, %xmm14
141a42e9a: f3 44 0f 5c f3              	subss	%xmm3, %xmm14
141a42e9f: 41 0f 14 e6                 	unpcklps	%xmm14, %xmm4           # xmm4 = xmm4[0],xmm14[0],xmm4[1],xmm14[1]
141a42ea3: 41 0f 58 f3                 	addps	%xmm11, %xmm6
141a42ea7: 0f c6 e6 54                 	shufps	$0x54, %xmm6, %xmm4     # xmm4 = xmm4[0,1],xmm6[1,1]
141a42eab: 0f c6 ee 01                 	shufps	$0x1, %xmm6, %xmm5      # xmm5 = xmm5[1,0],xmm6[0,0]
141a42eaf: f3 45 0f 58 c1              	addss	%xmm9, %xmm8
141a42eb4: 41 0f 28 da                 	movaps	%xmm10, %xmm3
141a42eb8: f3 41 0f 5c d8              	subss	%xmm8, %xmm3
141a42ebd: 0f c6 eb 02                 	shufps	$0x2, %xmm3, %xmm5      # xmm5 = xmm5[2,0],xmm3[0,0]
141a42ec1: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
141a42ec5: 41 0f 59 d5                 	mulps	%xmm13, %xmm2
141a42ec9: 0f 28 d8                    	movaps	%xmm0, %xmm3
141a42ecc: 0f c6 d8 00                 	shufps	$0x0, %xmm0, %xmm3      # xmm3 = xmm3[0,0],xmm0[0,0]
141a42ed0: 0f 59 dc                    	mulps	%xmm4, %xmm3
141a42ed3: 0f c6 c0 55                 	shufps	$0x55, %xmm0, %xmm0     # xmm0 = xmm0[1,1,1,1]
141a42ed7: 0f 59 c5                    	mulps	%xmm5, %xmm0
141a42eda: 0f c6 c9 a4                 	shufps	$0xa4, %xmm1, %xmm1     # xmm1 = xmm1[0,1,2,2]
141a42ede: 0f 29 95 60 01 00 00        	movaps	%xmm2, 0x160(%rbp)
141a42ee5: 0f 29 9d 70 01 00 00        	movaps	%xmm3, 0x170(%rbp)
141a42eec: 0f 29 85 80 01 00 00        	movaps	%xmm0, 0x180(%rbp)
141a42ef3: 0f 29 8d 90 01 00 00        	movaps	%xmm1, 0x190(%rbp)
141a42efa: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
141a42f01: 48 89 da                    	movq	%rbx, %rdx
141a42f04: e8 57 a6 e5 fe              	callq	0x14089d560 <.text+0x89c560>
141a42f09: 0f 10 85 e0 01 00 00        	movups	0x1e0(%rbp), %xmm0
141a42f10: 0f 10 8d f0 01 00 00        	movups	0x1f0(%rbp), %xmm1
141a42f17: 0f 10 95 00 02 00 00        	movups	0x200(%rbp), %xmm2
141a42f1e: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
141a42f25: 0f 29 8d 30 01 00 00        	movaps	%xmm1, 0x130(%rbp)
141a42f2c: 0f 29 95 40 01 00 00        	movaps	%xmm2, 0x140(%rbp)
141a42f33: 0f 28 05 66 1f 0d 01        	movaps	0x10d1f66(%rip), %xmm0  # 0x142b14ea0
141a42f3a: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
141a42f41: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
141a42f45: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
141a42f4c: e8 af 30 77 00              	callq	0x1421b6000 <.text+0x21b5000>
141a42f51: 49 89 f4                    	movq	%rsi, %r12
141a42f54: 48 8d 4e 0c                 	leaq	0xc(%rsi), %rcx
141a42f58: 41 08 c6                    	orb	%al, %r14b
141a42f5b: 48 ff c7                    	incq	%rdi
141a42f5e: 48 89 ce                    	movq	%rcx, %rsi
141a42f61: 4c 39 e9                    	cmpq	%r13, %rcx
141a42f64: 0f 85 86 fd ff ff           	jne	0x141a42cf0 <.text+0x1a41cf0>
141a42f6a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141a42f70: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
141a42f77: 48 85 c0                    	testq	%rax, %rax
141a42f7a: 74 1a                       	je	0x141a42f96 <.text+0x1a41f96>
141a42f7c: 48 c1 e0 02                 	shlq	$0x2, %rax
141a42f80: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
141a42f84: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141a42f8a: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
141a42f91: e8 8a 0c bd fe              	callq	0x140613c20 <.text+0x612c20>
141a42f96: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
141a42f9d: 48 85 d2                    	testq	%rdx, %rdx
141a42fa0: 48 8b b5 90 00 00 00        	movq	0x90(%rbp), %rsi
141a42fa7: 4c 8b a5 b8 00 00 00        	movq	0xb8(%rbp), %r12
141a42fae: 0f 84 fc f9 ff ff           	je	0x141a429b0 <.text+0x1a419b0>
141a42fb4: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
141a42fbb: 48 c1 e2 02                 	shlq	$0x2, %rdx
141a42fbf: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141a42fc5: e8 56 0c bd fe              	callq	0x140613c20 <.text+0x612c20>
141a42fca: e9 e1 f9 ff ff              	jmp	0x141a429b0 <.text+0x1a419b0>
141a42fcf: 90                          	nop
141a42fd0: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
141a42fd7: 48 85 d2                    	testq	%rdx, %rdx
141a42fda: 74 16                       	je	0x141a42ff2 <.text+0x1a41ff2>
141a42fdc: 48 c1 e2 02                 	shlq	$0x2, %rdx
141a42fe0: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141a42fe6: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
141a42fed: e8 2e 0c bd fe              	callq	0x140613c20 <.text+0x612c20>
141a42ff2: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
141a42ff9: 80 79 1c 00                 	cmpb	$0x0, 0x1c(%rcx)
141a42ffd: 0f 85 2a ef ff ff           	jne	0x141a41f2d <.text+0x1a40f2d>
141a43003: e8 68 0c ea fe              	callq	0x1408e3c70 <.text+0x8e2c70>
141a43008: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
141a4300c: 48 8b 85 98 00 00 00        	movq	0x98(%rbp), %rax
141a43013: f3 0f 10 40 14              	movss	0x14(%rax), %xmm0
141a43018: f3 0f 10 48 18              	movss	0x18(%rax), %xmm1
141a4301d: e8 3e 0b ea fe              	callq	0x1408e3b60 <.text+0x8e2b60>
141a43022: 48 8b 85 98 00 00 00        	movq	0x98(%rbp), %rax
141a43029: f3 0f 10 50 14              	movss	0x14(%rax), %xmm2
141a4302e: f3 0f 10 58 18              	movss	0x18(%rax), %xmm3
141a43033: 0f 2e da                    	ucomiss	%xmm2, %xmm3
141a43036: 0f 97 c2                    	seta	%dl
141a43039: e8 c2 f6 e9 fe              	callq	0x1408e2700 <.text+0x8e1700>
141a4303e: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141a43045: 48 89 d9                    	movq	%rbx, %rcx
141a43048: e8 83 55 64 fe              	callq	0x1400885d0 <.text+0x875d0>
141a4304d: 48 8b 95 28 02 00 00        	movq	0x228(%rbp), %rdx
141a43054: 0f 10 02                    	movups	(%rdx), %xmm0
141a43057: 0f 29 85 a0 00 00 00        	movaps	%xmm0, 0xa0(%rbp)
141a4305e: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
141a43065: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
141a4306c: e8 7f 5c 25 ff              	callq	0x140c98cf0 <.text+0xc97cf0>
141a43071: 4c 8b bd c8 00 00 00        	movq	0xc8(%rbp), %r15
141a43078: 4c 8b ad d0 00 00 00        	movq	0xd0(%rbp), %r13
141a4307f: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
141a43086: 48 8d 95 a0 00 00 00        	leaq	0xa0(%rbp), %rdx
141a4308d: e8 5e 5c 25 ff              	callq	0x140c98cf0 <.text+0xc97cf0>
141a43092: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
141a43099: 4c 39 e8                    	cmpq	%r13, %rax
141a4309c: 4c 0f 42 e8                 	cmovbq	%rax, %r13
141a430a0: 4c 8b a5 28 01 00 00        	movq	0x128(%rbp), %r12
141a430a7: f3 0f 10 85 1c 02 00 00     	movss	0x21c(%rbp), %xmm0
141a430af: f3 0f 58 c7                 	addss	%xmm7, %xmm0
141a430b3: f3 44 0f 58 c0              	addss	%xmm0, %xmm8
141a430b8: 4d 85 ed                    	testq	%r13, %r13
141a430bb: 4c 89 bd 28 02 00 00        	movq	%r15, 0x228(%rbp)
141a430c2: 0f 84 81 02 00 00           	je	0x141a43349 <.text+0x1a42349>
141a430c8: 44 0f 28 c8                 	movaps	%xmm0, %xmm9
141a430cc: 4c 01 ef                    	addq	%r13, %rdi
141a430cf: 4c 89 a5 30 02 00 00        	movq	%r12, 0x230(%rbp)
141a430d6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
141a430e0: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
141a430e4: 4c 89 fa                    	movq	%r15, %rdx
141a430e7: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
141a430eb: e8 f0 9c 25 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
141a430f0: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
141a430f4: 4c 89 e2                    	movq	%r12, %rdx
141a430f7: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
141a430fb: e8 e0 9c 25 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
141a43100: f3 0f 10 75 e0              	movss	-0x20(%rbp), %xmm6
141a43105: f2 0f 10 7d d8              	movsd	-0x28(%rbp), %xmm7
141a4310a: f2 44 0f 10 65 c8           	movsd	-0x38(%rbp), %xmm12
141a43110: f3 44 0f 10 5d d0           	movss	-0x30(%rbp), %xmm11
141a43116: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
141a4311a: f3 0f 5c c6                 	subss	%xmm6, %xmm0
141a4311e: 0f 28 c8                    	movaps	%xmm0, %xmm1
141a43121: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
141a43125: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
141a43129: 0f 5c d7                    	subps	%xmm7, %xmm2
141a4312c: 0f 28 da                    	movaps	%xmm2, %xmm3
141a4312f: 0f 59 da                    	mulps	%xmm2, %xmm3
141a43132: f3 0f 16 e3                 	movshdup	%xmm3, %xmm4            # xmm4 = xmm3[1,1,3,3]
141a43136: f3 0f 58 e3                 	addss	%xmm3, %xmm4
141a4313a: f3 0f 58 e1                 	addss	%xmm1, %xmm4
141a4313e: 0f 57 c9                    	xorps	%xmm1, %xmm1
141a43141: f3 0f 51 cc                 	sqrtss	%xmm4, %xmm1
141a43145: 41 0f 28 da                 	movaps	%xmm10, %xmm3
141a43149: f3 0f 5e d9                 	divss	%xmm1, %xmm3
141a4314d: f3 0f 12 cb                 	movsldup	%xmm3, %xmm1            # xmm1 = xmm3[0,0,2,2]
141a43151: 0f 59 ca                    	mulps	%xmm2, %xmm1
141a43154: f3 0f 59 d8                 	mulss	%xmm0, %xmm3
141a43158: 0f 13 8d e0 01 00 00        	movlps	%xmm1, 0x1e0(%rbp)
141a4315f: f3 0f 11 9d e8 01 00 00     	movss	%xmm3, 0x1e8(%rbp)
141a43167: 48 89 d9                    	movq	%rbx, %rcx
141a4316a: 48 8d 95 e0 01 00 00        	leaq	0x1e0(%rbp), %rdx
141a43171: e8 ca d8 24 ff              	callq	0x140c90a40 <.text+0xc8fa40>
141a43176: 0f 28 c6                    	movaps	%xmm6, %xmm0
141a43179: f3 41 0f 58 c3              	addss	%xmm11, %xmm0
141a4317e: f3 0f 59 05 ea 26 ee 00     	mulss	0xee26ea(%rip), %xmm0   # 0x142925870
141a43186: 0f 28 cf                    	movaps	%xmm7, %xmm1
141a43189: 41 0f 58 cc                 	addps	%xmm12, %xmm1
141a4318d: 44 0f 28 bd 60 01 00 00     	movaps	0x160(%rbp), %xmm15
141a43195: f3 0f 10 9d 68 01 00 00     	movss	0x168(%rbp), %xmm3
141a4319d: 41 0f 5c fc                 	subps	%xmm12, %xmm7
141a431a1: f3 41 0f 5c f3              	subss	%xmm11, %xmm6
141a431a6: 0f 59 ff                    	mulps	%xmm7, %xmm7
141a431a9: f3 0f 16 d7                 	movshdup	%xmm7, %xmm2            # xmm2 = xmm7[1,1,3,3]
141a431ad: f3 0f 58 d7                 	addss	%xmm7, %xmm2
141a431b1: f3 0f 59 f6                 	mulss	%xmm6, %xmm6
141a431b5: f3 0f 58 f2                 	addss	%xmm2, %xmm6
141a431b9: 0f 57 d2                    	xorps	%xmm2, %xmm2
141a431bc: f3 0f 51 d6                 	sqrtss	%xmm6, %xmm2
141a431c0: 0f 59 0d 19 27 ee 00        	mulps	0xee2719(%rip), %xmm1   # 0x1429258e0
141a431c7: 0f 13 8d 00 01 00 00        	movlps	%xmm1, 0x100(%rbp)
141a431ce: f3 0f 11 85 08 01 00 00     	movss	%xmm0, 0x108(%rbp)
141a431d6: f2 0f 10 05 12 95 04 01     	movsd	0x1049512(%rip), %xmm0  # 0x142a8c6f0
141a431de: f2 0f 11 85 0c 01 00 00     	movsd	%xmm0, 0x10c(%rbp)
141a431e6: f3 0f 11 95 14 01 00 00     	movss	%xmm2, 0x114(%rbp)
141a431ee: 0f 10 95 0c 01 00 00        	movups	0x10c(%rbp), %xmm2
141a431f5: 0f 28 85 10 01 00 00        	movaps	0x110(%rbp), %xmm0
141a431fc: 0f 28 8d 00 01 00 00        	movaps	0x100(%rbp), %xmm1
141a43203: 0f 28 fb                    	movaps	%xmm3, %xmm7
141a43206: f3 0f 58 fb                 	addss	%xmm3, %xmm7
141a4320a: f3 0f 59 df                 	mulss	%xmm7, %xmm3
141a4320e: 41 0f 28 ef                 	movaps	%xmm15, %xmm5
141a43212: 41 0f 58 ef                 	addps	%xmm15, %xmm5
141a43216: f3 0f 16 e5                 	movshdup	%xmm5, %xmm4            # xmm4 = xmm5[1,1,3,3]
141a4321a: f3 41 0f 59 e7              	mulss	%xmm15, %xmm4
141a4321f: 41 0f 28 f7                 	movaps	%xmm15, %xmm6
141a43223: f3 44 0f 12 df              	movsldup	%xmm7, %xmm11           # xmm11 = xmm7[0,0,2,2]
141a43228: 45 0f 59 df                 	mulps	%xmm15, %xmm11
141a4322c: 45 0f c6 ff ff              	shufps	$0xff, %xmm15, %xmm15   # xmm15 = xmm15[3,3,3,3]
141a43231: 0f 59 f5                    	mulps	%xmm5, %xmm6
141a43234: 41 0f 59 ef                 	mulps	%xmm15, %xmm5
141a43238: f3 44 0f 59 ff              	mulss	%xmm7, %xmm15
141a4323d: 0f c6 ed e1                 	shufps	$0xe1, %xmm5, %xmm5     # xmm5 = xmm5[1,0,2,3]
141a43241: f3 44 0f 16 ee              	movshdup	%xmm6, %xmm13           # xmm13 = xmm6[1,1,3,3]
141a43246: 41 0f 28 fd                 	movaps	%xmm13, %xmm7
141a4324a: f3 0f 58 fb                 	addss	%xmm3, %xmm7
141a4324e: 45 0f 28 e2                 	movaps	%xmm10, %xmm12
141a43252: f3 44 0f 5c e7              	subss	%xmm7, %xmm12
141a43257: 0f 28 fc                    	movaps	%xmm4, %xmm7
141a4325a: f3 41 0f 58 ff              	addss	%xmm15, %xmm7
141a4325f: 44 0f 14 e7                 	unpcklps	%xmm7, %xmm12           # xmm12 = xmm12[0],xmm7[0],xmm12[1],xmm7[1]
141a43263: 41 0f 28 fb                 	movaps	%xmm11, %xmm7
141a43267: 0f 5c fd                    	subps	%xmm5, %xmm7
141a4326a: 44 0f c6 e7 04              	shufps	$0x4, %xmm7, %xmm12     # xmm12 = xmm12[0,1],xmm7[0,0]
141a4326f: f3 41 0f 5c e7              	subss	%xmm15, %xmm4
141a43274: f3 0f 58 de                 	addss	%xmm6, %xmm3
141a43278: 45 0f 28 f2                 	movaps	%xmm10, %xmm14
141a4327c: f3 44 0f 5c f3              	subss	%xmm3, %xmm14
141a43281: 41 0f 14 e6                 	unpcklps	%xmm14, %xmm4           # xmm4 = xmm4[0],xmm14[0],xmm4[1],xmm14[1]
141a43285: 41 0f 58 eb                 	addps	%xmm11, %xmm5
141a43289: 0f c6 e5 54                 	shufps	$0x54, %xmm5, %xmm4     # xmm4 = xmm4[0,1],xmm5[1,1]
141a4328d: 0f c6 fd 01                 	shufps	$0x1, %xmm5, %xmm7      # xmm7 = xmm7[1,0],xmm5[0,0]
141a43291: f3 41 0f 58 f5              	addss	%xmm13, %xmm6
141a43296: 41 0f 28 da                 	movaps	%xmm10, %xmm3
141a4329a: f3 0f 5c de                 	subss	%xmm6, %xmm3
141a4329e: 0f c6 fb 02                 	shufps	$0x2, %xmm3, %xmm7      # xmm7 = xmm7[2,0],xmm3[0,0]
141a432a2: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
141a432a6: 41 0f 59 d4                 	mulps	%xmm12, %xmm2
141a432aa: 0f 28 d8                    	movaps	%xmm0, %xmm3
141a432ad: 0f c6 d8 00                 	shufps	$0x0, %xmm0, %xmm3      # xmm3 = xmm3[0,0],xmm0[0,0]
141a432b1: 0f 59 dc                    	mulps	%xmm4, %xmm3
141a432b4: 0f c6 c0 55                 	shufps	$0x55, %xmm0, %xmm0     # xmm0 = xmm0[1,1,1,1]
141a432b8: 0f 59 c7                    	mulps	%xmm7, %xmm0
141a432bb: 0f c6 c9 a4                 	shufps	$0xa4, %xmm1, %xmm1     # xmm1 = xmm1[0,1,2,2]
141a432bf: 0f 29 95 60 01 00 00        	movaps	%xmm2, 0x160(%rbp)
141a432c6: 0f 29 9d 70 01 00 00        	movaps	%xmm3, 0x170(%rbp)
141a432cd: 0f 29 85 80 01 00 00        	movaps	%xmm0, 0x180(%rbp)
141a432d4: 0f 29 8d 90 01 00 00        	movaps	%xmm1, 0x190(%rbp)
141a432db: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
141a432e2: 48 89 da                    	movq	%rbx, %rdx
141a432e5: e8 76 a2 e5 fe              	callq	0x14089d560 <.text+0x89c560>
141a432ea: 0f 10 85 e0 01 00 00        	movups	0x1e0(%rbp), %xmm0
141a432f1: 0f 10 8d f0 01 00 00        	movups	0x1f0(%rbp), %xmm1
141a432f8: 0f 10 95 00 02 00 00        	movups	0x200(%rbp), %xmm2
141a432ff: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
141a43306: 0f 29 8d 70 01 00 00        	movaps	%xmm1, 0x170(%rbp)
141a4330d: 0f 29 95 80 01 00 00        	movaps	%xmm2, 0x180(%rbp)
141a43314: 0f 28 05 85 1b 0d 01        	movaps	0x10d1b85(%rip), %xmm0  # 0x142b14ea0
141a4331b: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
141a43322: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
141a43326: 48 89 da                    	movq	%rbx, %rdx
141a43329: e8 d2 2c 77 00              	callq	0x1421b6000 <.text+0x21b5000>
141a4332e: 41 08 c6                    	orb	%al, %r14b
141a43331: 49 83 c4 08                 	addq	$0x8, %r12
141a43335: 49 83 c7 08                 	addq	$0x8, %r15
141a43339: 49 ff cd                    	decq	%r13
141a4333c: 0f 85 9e fd ff ff           	jne	0x141a430e0 <.text+0x1a420e0>
141a43342: 4c 8b a5 30 02 00 00        	movq	0x230(%rbp), %r12
141a43349: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
141a43350: 48 85 d2                    	testq	%rdx, %rdx
141a43353: 74 12                       	je	0x141a43367 <.text+0x1a42367>
141a43355: 48 c1 e2 03                 	shlq	$0x3, %rdx
141a43359: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141a4335f: 4c 89 e1                    	movq	%r12, %rcx
141a43362: e8 b9 08 bd fe              	callq	0x140613c20 <.text+0x612c20>
141a43367: 48 8b 95 38 01 00 00        	movq	0x138(%rbp), %rdx
141a4336e: 48 85 d2                    	testq	%rdx, %rdx
141a43371: 4c 8b a5 b8 00 00 00        	movq	0xb8(%rbp), %r12
141a43378: 74 16                       	je	0x141a43390 <.text+0x1a42390>
141a4337a: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
141a43381: 48 c1 e2 02                 	shlq	$0x2, %rdx
141a43385: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141a4338b: e8 90 08 bd fe              	callq	0x140613c20 <.text+0x612c20>
141a43390: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
141a43397: 48 85 d2                    	testq	%rdx, %rdx
141a4339a: 74 16                       	je	0x141a433b2 <.text+0x1a423b2>
141a4339c: 48 c1 e2 03                 	shlq	$0x3, %rdx
141a433a0: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141a433a6: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
141a433ad: e8 6e 08 bd fe              	callq	0x140613c20 <.text+0x612c20>
141a433b2: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
141a433b9: 48 85 d2                    	testq	%rdx, %rdx
141a433bc: 74 16                       	je	0x141a433d4 <.text+0x1a423d4>
141a433be: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
141a433c5: 48 c1 e2 02                 	shlq	$0x2, %rdx
141a433c9: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141a433cf: e8 4c 08 bd fe              	callq	0x140613c20 <.text+0x612c20>
141a433d4: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
141a433db: e8 50 61 25 ff              	callq	0x140c99530 <.text+0xc98530>
141a433e0: 0f 2e 05 89 24 ee 00        	ucomiss	0xee2489(%rip), %xmm0   # 0x142925870
141a433e7: 0f 86 40 eb ff ff           	jbe	0x141a41f2d <.text+0x1a40f2d>
141a433ed: f3 0f 10 85 b4 00 00 00     	movss	0xb4(%rbp), %xmm0
141a433f5: 45 31 ff                    	xorl	%r15d, %r15d
141a433f8: 0f 2e 85 b0 00 00 00        	ucomiss	0xb0(%rbp), %xmm0
141a433ff: 41 0f 96 c7                 	setbe	%r15b
141a43403: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
141a4340a: 4c 89 fa                    	movq	%r15, %rdx
141a4340d: e8 ee 61 25 ff              	callq	0x140c99600 <.text+0xc98600>
141a43412: f3 0f 11 85 60 01 00 00     	movss	%xmm0, 0x160(%rbp)
141a4341a: f3 0f 11 8d 64 01 00 00     	movss	%xmm1, 0x164(%rbp)
141a43422: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
141a43426: 48 89 da                    	movq	%rbx, %rdx
141a43429: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
141a4342d: e8 ae 99 25 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
141a43432: 49 ff c7                    	incq	%r15
141a43435: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
141a4343c: 4c 89 fa                    	movq	%r15, %rdx
141a4343f: e8 bc 61 25 ff              	callq	0x140c99600 <.text+0xc98600>
141a43444: f3 0f 11 85 60 01 00 00     	movss	%xmm0, 0x160(%rbp)
141a4344c: f3 0f 11 8d 64 01 00 00     	movss	%xmm1, 0x164(%rbp)
141a43454: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
141a43458: 48 89 da                    	movq	%rbx, %rdx
141a4345b: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
141a4345f: e8 7c 99 25 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
141a43464: f3 0f 10 75 c0              	movss	-0x40(%rbp), %xmm6
141a43469: f2 0f 10 7d b8              	movsd	-0x48(%rbp), %xmm7
141a4346e: f2 44 0f 10 5d a8           	movsd	-0x58(%rbp), %xmm11
141a43474: f3 44 0f 10 45 b0           	movss	-0x50(%rbp), %xmm8
141a4347a: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
141a4347e: f3 0f 5c c6                 	subss	%xmm6, %xmm0
141a43482: 0f 28 c8                    	movaps	%xmm0, %xmm1
141a43485: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
141a43489: 41 0f 28 d3                 	movaps	%xmm11, %xmm2
141a4348d: 0f 5c d7                    	subps	%xmm7, %xmm2
141a43490: 0f 28 da                    	movaps	%xmm2, %xmm3
141a43493: 0f 59 da                    	mulps	%xmm2, %xmm3
141a43496: f3 0f 16 e3                 	movshdup	%xmm3, %xmm4            # xmm4 = xmm3[1,1,3,3]
141a4349a: f3 0f 58 e3                 	addss	%xmm3, %xmm4
141a4349e: f3 0f 58 e1                 	addss	%xmm1, %xmm4
141a434a2: 0f 57 c9                    	xorps	%xmm1, %xmm1
141a434a5: f3 0f 51 cc                 	sqrtss	%xmm4, %xmm1
141a434a9: 41 0f 28 da                 	movaps	%xmm10, %xmm3
141a434ad: f3 0f 5e d9                 	divss	%xmm1, %xmm3
141a434b1: f3 0f 12 cb                 	movsldup	%xmm3, %xmm1            # xmm1 = xmm3[0,0,2,2]
141a434b5: 0f 59 ca                    	mulps	%xmm2, %xmm1
141a434b8: f3 0f 59 d8                 	mulss	%xmm0, %xmm3
141a434bc: 0f 13 8d 20 01 00 00        	movlps	%xmm1, 0x120(%rbp)
141a434c3: f3 0f 11 9d 28 01 00 00     	movss	%xmm3, 0x128(%rbp)
141a434cb: 48 89 d9                    	movq	%rbx, %rcx
141a434ce: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
141a434d5: e8 66 d5 24 ff              	callq	0x140c90a40 <.text+0xc8fa40>
141a434da: 0f 28 c6                    	movaps	%xmm6, %xmm0
141a434dd: f3 41 0f 58 c0              	addss	%xmm8, %xmm0
141a434e2: f3 0f 59 05 86 23 ee 00     	mulss	0xee2386(%rip), %xmm0   # 0x142925870
141a434ea: 0f 28 cf                    	movaps	%xmm7, %xmm1
141a434ed: 41 0f 58 cb                 	addps	%xmm11, %xmm1
141a434f1: 44 0f 28 a5 60 01 00 00     	movaps	0x160(%rbp), %xmm12
141a434f9: f3 0f 10 9d 68 01 00 00     	movss	0x168(%rbp), %xmm3
141a43501: 41 0f 5c fb                 	subps	%xmm11, %xmm7
141a43505: f3 41 0f 5c f0              	subss	%xmm8, %xmm6
141a4350a: 0f 59 ff                    	mulps	%xmm7, %xmm7
141a4350d: f3 0f 16 d7                 	movshdup	%xmm7, %xmm2            # xmm2 = xmm7[1,1,3,3]
141a43511: f3 0f 58 d7                 	addss	%xmm7, %xmm2
141a43515: f3 0f 59 f6                 	mulss	%xmm6, %xmm6
141a43519: f3 0f 58 f2                 	addss	%xmm2, %xmm6
141a4351d: 0f 57 d2                    	xorps	%xmm2, %xmm2
141a43520: f3 0f 51 d6                 	sqrtss	%xmm6, %xmm2
141a43524: 0f 59 0d b5 23 ee 00        	mulps	0xee23b5(%rip), %xmm1   # 0x1429258e0
141a4352b: 0f 13 8d c0 00 00 00        	movlps	%xmm1, 0xc0(%rbp)
141a43532: f3 0f 11 85 c8 00 00 00     	movss	%xmm0, 0xc8(%rbp)
141a4353a: f2 0f 10 05 ae 91 04 01     	movsd	0x10491ae(%rip), %xmm0  # 0x142a8c6f0
141a43542: f2 0f 11 85 cc 00 00 00     	movsd	%xmm0, 0xcc(%rbp)
141a4354a: f3 0f 11 95 d4 00 00 00     	movss	%xmm2, 0xd4(%rbp)
141a43552: 0f 10 95 cc 00 00 00        	movups	0xcc(%rbp), %xmm2
141a43559: 0f 28 85 d0 00 00 00        	movaps	0xd0(%rbp), %xmm0
141a43560: 0f 28 8d c0 00 00 00        	movaps	0xc0(%rbp), %xmm1
141a43567: 0f 28 fb                    	movaps	%xmm3, %xmm7
141a4356a: f3 0f 58 fb                 	addss	%xmm3, %xmm7
141a4356e: f3 0f 59 df                 	mulss	%xmm7, %xmm3
141a43572: 41 0f 28 ec                 	movaps	%xmm12, %xmm5
141a43576: 41 0f 58 ec                 	addps	%xmm12, %xmm5
141a4357a: f3 0f 16 e5                 	movshdup	%xmm5, %xmm4            # xmm4 = xmm5[1,1,3,3]
141a4357e: f3 41 0f 59 e4              	mulss	%xmm12, %xmm4
141a43583: 41 0f 28 f4                 	movaps	%xmm12, %xmm6
141a43587: f3 44 0f 12 c7              	movsldup	%xmm7, %xmm8            # xmm8 = xmm7[0,0,2,2]
141a4358c: 45 0f 59 c4                 	mulps	%xmm12, %xmm8
141a43590: 45 0f c6 e4 ff              	shufps	$0xff, %xmm12, %xmm12   # xmm12 = xmm12[3,3,3,3]
141a43595: 0f 59 f5                    	mulps	%xmm5, %xmm6
141a43598: 41 0f 59 ec                 	mulps	%xmm12, %xmm5
141a4359c: f3 44 0f 59 e7              	mulss	%xmm7, %xmm12
141a435a1: 0f c6 ed e1                 	shufps	$0xe1, %xmm5, %xmm5     # xmm5 = xmm5[1,0,2,3]
141a435a5: f3 44 0f 16 ee              	movshdup	%xmm6, %xmm13           # xmm13 = xmm6[1,1,3,3]
141a435aa: 41 0f 28 fd                 	movaps	%xmm13, %xmm7
141a435ae: f3 0f 58 fb                 	addss	%xmm3, %xmm7
141a435b2: 45 0f 28 da                 	movaps	%xmm10, %xmm11
141a435b6: f3 44 0f 5c df              	subss	%xmm7, %xmm11
141a435bb: 0f 28 fc                    	movaps	%xmm4, %xmm7
141a435be: f3 41 0f 58 fc              	addss	%xmm12, %xmm7
141a435c3: 44 0f 14 df                 	unpcklps	%xmm7, %xmm11           # xmm11 = xmm11[0],xmm7[0],xmm11[1],xmm7[1]
141a435c7: 41 0f 28 f8                 	movaps	%xmm8, %xmm7
141a435cb: 0f 5c fd                    	subps	%xmm5, %xmm7
141a435ce: 44 0f c6 df 04              	shufps	$0x4, %xmm7, %xmm11     # xmm11 = xmm11[0,1],xmm7[0,0]
141a435d3: f3 41 0f 5c e4              	subss	%xmm12, %xmm4
141a435d8: f3 0f 58 de                 	addss	%xmm6, %xmm3
141a435dc: 45 0f 28 e2                 	movaps	%xmm10, %xmm12
141a435e0: f3 44 0f 5c e3              	subss	%xmm3, %xmm12
141a435e5: 41 0f 14 e4                 	unpcklps	%xmm12, %xmm4           # xmm4 = xmm4[0],xmm12[0],xmm4[1],xmm12[1]
141a435e9: 41 0f 58 e8                 	addps	%xmm8, %xmm5
141a435ed: 0f c6 e5 54                 	shufps	$0x54, %xmm5, %xmm4     # xmm4 = xmm4[0,1],xmm5[1,1]
141a435f1: 0f c6 fd 01                 	shufps	$0x1, %xmm5, %xmm7      # xmm7 = xmm7[1,0],xmm5[0,0]
141a435f5: f3 41 0f 58 f5              	addss	%xmm13, %xmm6
141a435fa: 41 0f 28 da                 	movaps	%xmm10, %xmm3
141a435fe: f3 0f 5c de                 	subss	%xmm6, %xmm3
141a43602: 0f c6 fb 02                 	shufps	$0x2, %xmm3, %xmm7      # xmm7 = xmm7[2,0],xmm3[0,0]
141a43606: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
141a4360a: 41 0f 59 d3                 	mulps	%xmm11, %xmm2
141a4360e: 0f 28 d8                    	movaps	%xmm0, %xmm3
141a43611: 0f c6 d8 00                 	shufps	$0x0, %xmm0, %xmm3      # xmm3 = xmm3[0,0],xmm0[0,0]
141a43615: 0f 59 dc                    	mulps	%xmm4, %xmm3
141a43618: 0f c6 c0 55                 	shufps	$0x55, %xmm0, %xmm0     # xmm0 = xmm0[1,1,1,1]
141a4361c: 0f 59 c7                    	mulps	%xmm7, %xmm0
141a4361f: 0f c6 c9 a4                 	shufps	$0xa4, %xmm1, %xmm1     # xmm1 = xmm1[0,1,2,2]
141a43623: 0f 29 95 60 01 00 00        	movaps	%xmm2, 0x160(%rbp)
141a4362a: 0f 29 9d 70 01 00 00        	movaps	%xmm3, 0x170(%rbp)
141a43631: 0f 29 85 80 01 00 00        	movaps	%xmm0, 0x180(%rbp)
141a43638: 0f 29 8d 90 01 00 00        	movaps	%xmm1, 0x190(%rbp)
141a4363f: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
141a43646: 48 89 da                    	movq	%rbx, %rdx
141a43649: e8 12 9f e5 fe              	callq	0x14089d560 <.text+0x89c560>
141a4364e: 0f 10 85 20 01 00 00        	movups	0x120(%rbp), %xmm0
141a43655: 0f 10 8d 30 01 00 00        	movups	0x130(%rbp), %xmm1
141a4365c: 0f 10 95 40 01 00 00        	movups	0x140(%rbp), %xmm2
141a43663: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
141a4366a: 0f 29 8d 70 01 00 00        	movaps	%xmm1, 0x170(%rbp)
141a43671: 0f 29 95 80 01 00 00        	movaps	%xmm2, 0x180(%rbp)
141a43678: 0f 28 05 21 18 0d 01        	movaps	0x10d1821(%rip), %xmm0  # 0x142b14ea0
141a4367f: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
141a43686: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
141a4368a: 48 89 da                    	movq	%rbx, %rdx
141a4368d: e8 6e 29 77 00              	callq	0x1421b6000 <.text+0x21b5000>
141a43692: e9 90 e8 ff ff              	jmp	0x141a41f27 <.text+0x1a40f27>
141a43697: 41 f6 c6 01                 	testb	$0x1, %r14b
141a4369b: 0f 84 c9 00 00 00           	je	0x141a4376a <.text+0x1a4276a>
141a436a1: 48 8b 05 00 8d be 01        	movq	0x1be8d00(%rip), %rax   # 0x14362c3a8
141a436a8: 48 8b 00                    	movq	(%rax), %rax
141a436ab: 48 85 c0                    	testq	%rax, %rax
141a436ae: 0f 84 b6 00 00 00           	je	0x141a4376a <.text+0x1a4276a>
141a436b4: 48 8d 0d 15 8d 47 01        	leaq	0x1478d15(%rip), %rcx   # 0x142ebc3d0
141a436bb: e8 c0 a6 c3 fe              	callq	0x14067dd80 <.text+0x67cd80>
141a436c0: 0f 10 00                    	movups	(%rax), %xmm0
141a436c3: 8b 40 10                    	movl	0x10(%rax), %eax
141a436c6: 48 c7 85 90 01 00 00 01 00 00 00    	movq	$0x1, 0x190(%rbp)
141a436d1: 48 8d 0d 10 8d 47 01        	leaq	0x1478d10(%rip), %rcx   # 0x142ebc3e8
141a436d8: 48 89 8d 98 01 00 00        	movq	%rcx, 0x198(%rbp)
141a436df: 48 c7 85 a0 01 00 00 22 00 00 00    	movq	$0x22, 0x1a0(%rbp)
141a436ea: 48 8d 15 cf 8c 47 01        	leaq	0x1478ccf(%rip), %rdx   # 0x142ebc3c0
141a436f1: 48 89 95 b0 01 00 00        	movq	%rdx, 0x1b0(%rbp)
141a436f8: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
141a43703: 48 c7 85 c0 01 00 00 08 00 00 00    	movq	$0x8, 0x1c0(%rbp)
141a4370e: 48 c7 85 60 01 00 00 00 00 00 00    	movq	$0x0, 0x160(%rbp)
141a43719: 0f 57 c9                    	xorps	%xmm1, %xmm1
141a4371c: 0f 11 8d c8 01 00 00        	movups	%xmm1, 0x1c8(%rbp)
141a43723: 48 89 8d 68 01 00 00        	movq	%rcx, 0x168(%rbp)
141a4372a: 48 c7 85 70 01 00 00 22 00 00 00    	movq	$0x22, 0x170(%rbp)
141a43735: 48 c7 85 78 01 00 00 00 00 00 00    	movq	$0x0, 0x178(%rbp)
141a43740: 0f 11 85 80 01 00 00        	movups	%xmm0, 0x180(%rbp)
141a43747: c7 85 a8 01 00 00 01 00 00 00       	movl	$0x1, 0x1a8(%rbp)
141a43751: 89 85 ac 01 00 00           	movl	%eax, 0x1ac(%rbp)
141a43757: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
141a4375e: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
141a43765: e8 c6 6b 30 ff              	callq	0x140d4a330 <.text+0xd49330>
141a4376a: 48 8b 7d 40                 	movq	0x40(%rbp), %rdi
141a4376e: 48 8d 77 18                 	leaq	0x18(%rdi), %rsi
141a43772: 48 89 f1                    	movq	%rsi, %rcx
141a43775: e8 96 e6 23 ff              	callq	0x140c81e10 <.text+0xc80e10>
141a4377a: 31 c9                       	xorl	%ecx, %ecx
141a4377c: b0 01                       	movb	$0x1, %al
141a4377e: f0                          	lock
141a4377f: 0f b0 4f 18                 	cmpxchgb	%cl, 0x18(%rdi)
141a43783: 0f 85 bb 00 00 00           	jne	0x141a43844 <.text+0x1a42844>
141a43789: 0f 28 b5 40 02 00 00        	movaps	0x240(%rbp), %xmm6
141a43790: 0f 28 bd 50 02 00 00        	movaps	0x250(%rbp), %xmm7
141a43797: 44 0f 28 85 60 02 00 00     	movaps	0x260(%rbp), %xmm8
141a4379f: 44 0f 28 8d 70 02 00 00     	movaps	0x270(%rbp), %xmm9
141a437a7: 44 0f 28 95 80 02 00 00     	movaps	0x280(%rbp), %xmm10
141a437af: 44 0f 28 9d 90 02 00 00     	movaps	0x290(%rbp), %xmm11
141a437b7: 44 0f 28 a5 a0 02 00 00     	movaps	0x2a0(%rbp), %xmm12
141a437bf: 44 0f 28 ad b0 02 00 00     	movaps	0x2b0(%rbp), %xmm13
141a437c7: 44 0f 28 b5 c0 02 00 00     	movaps	0x2c0(%rbp), %xmm14
141a437cf: 44 0f 28 bd d0 02 00 00     	movaps	0x2d0(%rbp), %xmm15
141a437d7: 48 81 c4 68 03 00 00        	addq	$0x368, %rsp            # imm = 0x368
141a437de: 5b                          	popq	%rbx
141a437df: 5f                          	popq	%rdi
141a437e0: 5e                          	popq	%rsi
141a437e1: 41 5c                       	popq	%r12
141a437e3: 41 5d                       	popq	%r13
141a437e5: 41 5e                       	popq	%r14
141a437e7: 41 5f                       	popq	%r15
141a437e9: 5d                          	popq	%rbp
141a437ea: c3                          	retq
141a437eb: 48 8d 0d 3e 52 5b 01        	leaq	0x15b523e(%rip), %rcx   # 0x142ff8a30
141a437f2: 48 8d 15 97 52 5b 01        	leaq	0x15b5297(%rip), %rdx   # 0x142ff8a90
141a437f9: e8 02 62 e6 00              	callq	0x1428a9a00 <.text+0x28a8a00>
141a437fe: 48 c7 85 60 01 00 00 00 00 00 00    	movq	$0x0, 0x160(%rbp)
141a43809: 48 83 c3 10                 	addq	$0x10, %rbx
141a4380d: 48 8d 05 14 54 5b 01        	leaq	0x15b5414(%rip), %rax   # 0x142ff8c28
141a43814: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141a43819: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
141a4381d: 4c 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %r9
141a43824: 31 c9                       	xorl	%ecx, %ecx
141a43826: 49 89 d8                    	movq	%rbx, %r8
141a43829: e8 13 73 e8 00              	callq	0x1428cab41 <.text+0x28c9b41>
141a4382e: 49 89 ce                    	movq	%rcx, %r14
141a43831: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
141a43837: e8 d4 99 e8 00              	callq	0x1428cd210 <.text+0x28cc210>
141a4383c: 4c 89 f1                    	movq	%r14, %rcx
141a4383f: e9 b1 e5 ff ff              	jmp	0x141a41df5 <.text+0x1a40df5>
141a43844: 48 89 f1                    	movq	%rsi, %rcx
141a43847: 31 d2                       	xorl	%edx, %edx
141a43849: e8 42 a3 e8 00              	callq	0x1428cdb90 <.text+0x28ccb90>
141a4384e: e9 36 ff ff ff              	jmp	0x141a43789 <.text+0x1a42789>
141a43853: 48 c7 85 60 01 00 00 00 00 00 00    	movq	$0x0, 0x160(%rbp)
141a4385e: 49 83 c6 10                 	addq	$0x10, %r14
141a43862: 48 8d 05 bf 53 5b 01        	leaq	0x15b53bf(%rip), %rax   # 0x142ff8c28
141a43869: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141a4386e: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
141a43872: 4c 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %r9
141a43879: 31 c9                       	xorl	%ecx, %ecx
141a4387b: 4d 89 f0                    	movq	%r14, %r8
141a4387e: e8 be 72 e8 00              	callq	0x1428cab41 <.text+0x28c9b41>
141a43883: 48 8d 0d ce 8a 47 01        	leaq	0x1478ace(%rip), %rcx   # 0x142ebc358
141a4388a: e8 81 5a e9 00              	callq	0x1428d9310 <.text+0x28d8310>
141a4388f: 48 8d 0d da 8a 47 01        	leaq	0x1478ada(%rip), %rcx   # 0x142ebc370
141a43896: e8 75 5a e9 00              	callq	0x1428d9310 <.text+0x28d8310>
141a4389b: 48 8d 0d 26 55 5b 01        	leaq	0x15b5526(%rip), %rcx   # 0x142ff8dc8
141a438a2: e8 69 5a e9 00              	callq	0x1428d9310 <.text+0x28d8310>
141a438a7: 48 8d 0d 92 8a 47 01        	leaq	0x1478a92(%rip), %rcx   # 0x142ebc340
141a438ae: e8 5d 5a e9 00              	callq	0x1428d9310 <.text+0x28d8310>
141a438b3: 48 8d 0d 26 55 5b 01        	leaq	0x15b5526(%rip), %rcx   # 0x142ff8de0
141a438ba: e8 51 5a e9 00              	callq	0x1428d9310 <.text+0x28d8310>
141a438bf: b9 08 00 00 00              	movl	$0x8, %ecx
141a438c4: ba 20 00 00 00              	movl	$0x20, %edx
141a438c9: e8 32 57 e9 00              	callq	0x1428d9000 <.text+0x28d8000>
141a438ce: cc                          	int3
141a438cf: 90                          	nop
141a438d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141a438d5: 55                          	pushq	%rbp
141a438d6: 41 57                       	pushq	%r15
141a438d8: 41 56                       	pushq	%r14
141a438da: 41 55                       	pushq	%r13
141a438dc: 41 54                       	pushq	%r12
141a438de: 56                          	pushq	%rsi
141a438df: 57                          	pushq	%rdi
141a438e0: 53                          	pushq	%rbx
141a438e1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
141a438e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141a438ef: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
141a438f5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
141a438fb: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
141a43901: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
141a43907: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
141a4390d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
141a43913: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
141a4391c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
141a43925: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
141a4392d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141a43935: 48 8b 7d 40                 	movq	0x40(%rbp), %rdi
141a43939: 48 8d 77 18                 	leaq	0x18(%rdi), %rsi
141a4393d: 48 89 f1                    	movq	%rsi, %rcx
141a43940: e8 cb e4 23 ff              	callq	0x140c81e10 <.text+0xc80e10>
141a43945: 31 c9                       	xorl	%ecx, %ecx
141a43947: b0 01                       	movb	$0x1, %al
141a43949: f0                          	lock
141a4394a: 0f b0 4f 18                 	cmpxchgb	%cl, 0x18(%rdi)
141a4394e: 75 5a                       	jne	0x141a439aa <.text+0x1a429aa>
141a43950: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
141a43958: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
141a43960: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
141a43969: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
141a43972: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
141a43978: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
141a4397e: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
141a43984: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
141a4398a: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
141a43990: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
141a43996: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
141a4399d: 5b                          	popq	%rbx
141a4399e: 5f                          	popq	%rdi
141a4399f: 5e                          	popq	%rsi
141a439a0: 41 5c                       	popq	%r12
141a439a2: 41 5d                       	popq	%r13
141a439a4: 41 5e                       	popq	%r14
141a439a6: 41 5f                       	popq	%r15
141a439a8: 5d                          	popq	%rbp
141a439a9: c3                          	retq
141a439aa: 48 89 f1                    	movq	%rsi, %rcx
141a439ad: 31 d2                       	xorl	%edx, %edx
141a439af: e8 dc a1 e8 00              	callq	0x1428cdb90 <.text+0x28ccb90>
141a439b4: eb 9a                       	jmp	0x141a43950 <.text+0x1a42950>
141a439b6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
141a439c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141a439c5: 55                          	pushq	%rbp
141a439c6: 41 57                       	pushq	%r15
141a439c8: 41 56                       	pushq	%r14
141a439ca: 41 55                       	pushq	%r13
141a439cc: 41 54                       	pushq	%r12
141a439ce: 56                          	pushq	%rsi
141a439cf: 57                          	pushq	%rdi
141a439d0: 53                          	pushq	%rbx
141a439d1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
141a439d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141a439df: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
141a439e5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
141a439eb: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
141a439f1: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
141a439f7: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
141a439fd: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
141a43a03: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
141a43a0c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
141a43a15: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
141a43a1d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141a43a25: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
141a43a2c: e8 ef 7f 70 fe              	callq	0x14014ba20 <.text+0x14aa20>
141a43a31: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
141a43a39: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
141a43a41: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
141a43a4a: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
141a43a53: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
141a43a59: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
141a43a5f: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
141a43a65: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
141a43a6b: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
141a43a71: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
141a43a77: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
141a43a7e: 5b                          	popq	%rbx
141a43a7f: 5f                          	popq	%rdi
141a43a80: 5e                          	popq	%rsi
141a43a81: 41 5c                       	popq	%r12
141a43a83: 41 5d                       	popq	%r13
141a43a85: 41 5e                       	popq	%r14
141a43a87: 41 5f                       	popq	%r15
141a43a89: 5d                          	popq	%rbp
141a43a8a: c3                          	retq
141a43a8b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141a43a90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141a43a95: 55                          	pushq	%rbp
141a43a96: 41 57                       	pushq	%r15
141a43a98: 41 56                       	pushq	%r14
141a43a9a: 41 55                       	pushq	%r13
141a43a9c: 41 54                       	pushq	%r12
141a43a9e: 56                          	pushq	%rsi
141a43a9f: 57                          	pushq	%rdi
141a43aa0: 53                          	pushq	%rbx
141a43aa1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
141a43aa8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141a43aaf: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
141a43ab5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
141a43abb: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
141a43ac1: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
141a43ac7: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
141a43acd: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
141a43ad3: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
141a43adc: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
141a43ae5: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
141a43aed: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141a43af5: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
141a43afc: e8 1f 7f 70 fe              	callq	0x14014ba20 <.text+0x14aa20>
141a43b01: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
141a43b09: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
141a43b11: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
141a43b1a: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
141a43b23: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
141a43b29: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
141a43b2f: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
141a43b35: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
141a43b3b: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
141a43b41: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
141a43b47: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
141a43b4e: 5b                          	popq	%rbx
141a43b4f: 5f                          	popq	%rdi
141a43b50: 5e                          	popq	%rsi
141a43b51: 41 5c                       	popq	%r12
141a43b53: 41 5d                       	popq	%r13
141a43b55: 41 5e                       	popq	%r14
141a43b57: 41 5f                       	popq	%r15
141a43b59: 5d                          	popq	%rbp
141a43b5a: c3                          	retq
141a43b5b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141a43b60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141a43b65: 55                          	pushq	%rbp
141a43b66: 41 57                       	pushq	%r15
141a43b68: 41 56                       	pushq	%r14
141a43b6a: 41 55                       	pushq	%r13
141a43b6c: 41 54                       	pushq	%r12
141a43b6e: 56                          	pushq	%rsi
141a43b6f: 57                          	pushq	%rdi
141a43b70: 53                          	pushq	%rbx
141a43b71: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
141a43b78: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141a43b7f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
141a43b85: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
141a43b8b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
141a43b91: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
141a43b97: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
141a43b9d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
141a43ba3: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
141a43bac: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
141a43bb5: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
141a43bbd: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141a43bc5: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
141a43bcc: 48 85 d2                    	testq	%rdx, %rdx
141a43bcf: 74 16                       	je	0x141a43be7 <.text+0x1a42be7>
141a43bd1: 48 c1 e2 02                 	shlq	$0x2, %rdx
141a43bd5: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141a43bdb: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
141a43be2: e8 39 00 bd fe              	callq	0x140613c20 <.text+0x612c20>
141a43be7: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
141a43bef: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
141a43bf7: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
141a43c00: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
141a43c09: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
141a43c0f: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
141a43c15: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
141a43c1b: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
141a43c21: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
141a43c27: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
141a43c2d: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
141a43c34: 5b                          	popq	%rbx
141a43c35: 5f                          	popq	%rdi
141a43c36: 5e                          	popq	%rsi
141a43c37: 41 5c                       	popq	%r12
141a43c39: 41 5d                       	popq	%r13
141a43c3b: 41 5e                       	popq	%r14
141a43c3d: 41 5f                       	popq	%r15
141a43c3f: 5d                          	popq	%rbp
141a43c40: c3                          	retq
141a43c41: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
141a43c50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141a43c55: 55                          	pushq	%rbp
141a43c56: 41 57                       	pushq	%r15
141a43c58: 41 56                       	pushq	%r14
141a43c5a: 41 55                       	pushq	%r13
141a43c5c: 41 54                       	pushq	%r12
141a43c5e: 56                          	pushq	%rsi
141a43c5f: 57                          	pushq	%rdi
141a43c60: 53                          	pushq	%rbx
141a43c61: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
141a43c68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141a43c6f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
141a43c75: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
141a43c7b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
141a43c81: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
141a43c87: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
141a43c8d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
141a43c93: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
141a43c9c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
141a43ca5: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
141a43cad: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141a43cb5: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
141a43cbc: 48 85 c0                    	testq	%rax, %rax
141a43cbf: 74 1a                       	je	0x141a43cdb <.text+0x1a42cdb>
141a43cc1: 48 c1 e0 02                 	shlq	$0x2, %rax
141a43cc5: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
141a43cc9: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141a43ccf: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
141a43cd6: e8 45 ff bc fe              	callq	0x140613c20 <.text+0x612c20>
141a43cdb: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
141a43ce2: 48 85 d2                    	testq	%rdx, %rdx
141a43ce5: 74 16                       	je	0x141a43cfd <.text+0x1a42cfd>
141a43ce7: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
141a43cee: 48 c1 e2 02                 	shlq	$0x2, %rdx
141a43cf2: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141a43cf8: e8 23 ff bc fe              	callq	0x140613c20 <.text+0x612c20>
141a43cfd: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
141a43d05: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
141a43d0d: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
141a43d16: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
141a43d1f: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
141a43d25: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
141a43d2b: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
141a43d31: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
141a43d37: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
141a43d3d: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
141a43d43: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
141a43d4a: 5b                          	popq	%rbx
141a43d4b: 5f                          	popq	%rdi
141a43d4c: 5e                          	popq	%rsi
141a43d4d: 41 5c                       	popq	%r12
141a43d4f: 41 5d                       	popq	%r13
141a43d51: 41 5e                       	popq	%r14
141a43d53: 41 5f                       	popq	%r15
141a43d55: 5d                          	popq	%rbp
141a43d56: c3                          	retq
141a43d57: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141a43d60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141a43d65: 55                          	pushq	%rbp
141a43d66: 41 57                       	pushq	%r15
141a43d68: 41 56                       	pushq	%r14
141a43d6a: 41 55                       	pushq	%r13
141a43d6c: 41 54                       	pushq	%r12
141a43d6e: 56                          	pushq	%rsi
141a43d6f: 57                          	pushq	%rdi
141a43d70: 53                          	pushq	%rbx
141a43d71: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
141a43d78: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141a43d7f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
141a43d85: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
141a43d8b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
141a43d91: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
141a43d97: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
141a43d9d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
141a43da3: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
141a43dac: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
141a43db5: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
141a43dbd: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141a43dc5: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
141a43dcc: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
141a43dd3: 48 89 01                    	movq	%rax, (%rcx)
141a43dd6: 48 8b 85 20 02 00 00        	movq	0x220(%rbp), %rax
141a43ddd: 48 89 41 08                 	movq	%rax, 0x8(%rcx)
141a43de1: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
141a43de9: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
141a43df1: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
141a43dfa: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
141a43e03: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
141a43e09: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
141a43e0f: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
141a43e15: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
141a43e1b: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
141a43e21: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
141a43e27: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
141a43e2e: 5b                          	popq	%rbx
141a43e2f: 5f                          	popq	%rdi
141a43e30: 5e                          	popq	%rsi
141a43e31: 41 5c                       	popq	%r12
141a43e33: 41 5d                       	popq	%r13
141a43e35: 41 5e                       	popq	%r14
141a43e37: 41 5f                       	popq	%r15
141a43e39: 5d                          	popq	%rbp
141a43e3a: c3                          	retq
141a43e3b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141a43e40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141a43e45: 55                          	pushq	%rbp
141a43e46: 41 57                       	pushq	%r15
141a43e48: 41 56                       	pushq	%r14
141a43e4a: 41 55                       	pushq	%r13
141a43e4c: 41 54                       	pushq	%r12
141a43e4e: 56                          	pushq	%rsi
141a43e4f: 57                          	pushq	%rdi
141a43e50: 53                          	pushq	%rbx
141a43e51: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
141a43e58: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141a43e5f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
141a43e65: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
141a43e6b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
141a43e71: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
141a43e77: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
141a43e7d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
141a43e83: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
141a43e8c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
141a43e95: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
141a43e9d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141a43ea5: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
141a43eac: e8 3f e6 b8 fe              	callq	0x1405d24f0 <.text+0x5d14f0>
141a43eb1: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
141a43eb9: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
141a43ec1: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
141a43eca: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
141a43ed3: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
141a43ed9: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
141a43edf: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
141a43ee5: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
141a43eeb: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
141a43ef1: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
141a43ef7: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
141a43efe: 5b                          	popq	%rbx
141a43eff: 5f                          	popq	%rdi
141a43f00: 5e                          	popq	%rsi
141a43f01: 41 5c                       	popq	%r12
141a43f03: 41 5d                       	popq	%r13
141a43f05: 41 5e                       	popq	%r14
141a43f07: 41 5f                       	popq	%r15
141a43f09: 5d                          	popq	%rbp
141a43f0a: c3                          	retq
141a43f0b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141a43f10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141a43f15: 55                          	pushq	%rbp
141a43f16: 41 57                       	pushq	%r15
141a43f18: 41 56                       	pushq	%r14
141a43f1a: 41 55                       	pushq	%r13
141a43f1c: 41 54                       	pushq	%r12
141a43f1e: 56                          	pushq	%rsi
141a43f1f: 57                          	pushq	%rdi
141a43f20: 53                          	pushq	%rbx
141a43f21: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
141a43f28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141a43f2f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
141a43f35: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
141a43f3b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
141a43f41: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
141a43f47: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
141a43f4d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
141a43f53: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
141a43f5c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
141a43f65: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
141a43f6d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141a43f75: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
141a43f7c: e8 6f e5 b8 fe              	callq	0x1405d24f0 <.text+0x5d14f0>
141a43f81: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
141a43f89: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
141a43f91: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
141a43f9a: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
141a43fa3: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
141a43fa9: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
141a43faf: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
141a43fb5: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
141a43fbb: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
141a43fc1: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
141a43fc7: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
141a43fce: 5b                          	popq	%rbx
141a43fcf: 5f                          	popq	%rdi
141a43fd0: 5e                          	popq	%rsi
141a43fd1: 41 5c                       	popq	%r12
141a43fd3: 41 5d                       	popq	%r13
141a43fd5: 41 5e                       	popq	%r14
141a43fd7: 41 5f                       	popq	%r15
141a43fd9: 5d                          	popq	%rbp
141a43fda: c3                          	retq
