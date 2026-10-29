
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407e7dd0: 55                          	pushq	%rbp
1407e7dd1: 41 57                       	pushq	%r15
1407e7dd3: 41 56                       	pushq	%r14
1407e7dd5: 41 55                       	pushq	%r13
1407e7dd7: 41 54                       	pushq	%r12
1407e7dd9: 56                          	pushq	%rsi
1407e7dda: 57                          	pushq	%rdi
1407e7ddb: 53                          	pushq	%rbx
1407e7ddc: 48 81 ec a8 01 00 00        	subq	$0x1a8, %rsp            # imm = 0x1A8
1407e7de3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1407e7deb: 44 0f 29 95 10 01 00 00     	movaps	%xmm10, 0x110(%rbp)
1407e7df3: 44 0f 29 8d 00 01 00 00     	movaps	%xmm9, 0x100(%rbp)
1407e7dfb: 44 0f 29 85 f0 00 00 00     	movaps	%xmm8, 0xf0(%rbp)
1407e7e03: 0f 29 bd e0 00 00 00        	movaps	%xmm7, 0xe0(%rbp)
1407e7e0a: 0f 29 b5 d0 00 00 00        	movaps	%xmm6, 0xd0(%rbp)
1407e7e11: 48 c7 85 c8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xc8(%rbp)
1407e7e1c: 48 89 ce                    	movq	%rcx, %rsi
1407e7e1f: 89 95 a4 00 00 00           	movl	%edx, 0xa4(%rbp)
1407e7e25: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
1407e7e30: 48 c7 85 b0 00 00 00 04 00 00 00    	movq	$0x4, 0xb0(%rbp)
1407e7e3b: 48 c7 85 b8 00 00 00 00 00 00 00    	movq	$0x0, 0xb8(%rbp)
1407e7e46: 48 8b 09                    	movq	(%rcx), %rcx
1407e7e49: 48 8b 7e 08                 	movq	0x8(%rsi), %rdi
1407e7e4d: 4c 8b 41 08                 	movq	0x8(%rcx), %r8
1407e7e51: 48 8b 41 10                 	movq	0x10(%rcx), %rax
1407e7e55: 49 8d 04 c0                 	leaq	(%r8,%rax,8), %rax
1407e7e59: 48 c7 45 d0 00 00 00 00     	movq	$0x0, -0x30(%rbp)
1407e7e61: 4c 8d 7d d8                 	leaq	-0x28(%rbp), %r15
1407e7e65: 4c 89 45 d8                 	movq	%r8, -0x28(%rbp)
1407e7e69: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
1407e7e6d: 48 8d 5d e8                 	leaq	-0x18(%rbp), %rbx
1407e7e71: 48 89 7d e8                 	movq	%rdi, -0x18(%rbp)
1407e7e75: 0f 10 46 10                 	movups	0x10(%rsi), %xmm0
1407e7e79: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
1407e7e7d: 48 8d 95 a4 00 00 00        	leaq	0xa4(%rbp), %rdx
1407e7e84: 48 89 55 00                 	movq	%rdx, (%rbp)
1407e7e88: 48 89 4d 08                 	movq	%rcx, 0x8(%rbp)
1407e7e8c: 0f 10 46 20                 	movups	0x20(%rsi), %xmm0
1407e7e90: 0f 11 45 10                 	movups	%xmm0, 0x10(%rbp)
1407e7e94: 0f 10 46 30                 	movups	0x30(%rsi), %xmm0
1407e7e98: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
1407e7e9c: 0f 10 46 40                 	movups	0x40(%rsi), %xmm0
1407e7ea0: 0f 11 45 30                 	movups	%xmm0, 0x30(%rbp)
1407e7ea4: c7 45 40 00 00 00 00        	movl	$0x0, 0x40(%rbp)
1407e7eab: 4c 8d 75 c4                 	leaq	-0x3c(%rbp), %r14
1407e7eaf: 4c 8d 6d ac                 	leaq	-0x54(%rbp), %r13
1407e7eb3: eb 33                       	jmp	0x1407e7ee8 <.text+0x7e6ee8>
1407e7eb5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1407e7ec0: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1407e7ec7: 4b 8d 0c 64                 	leaq	(%r12,%r12,2), %rcx
1407e7ecb: 44 0f 13 04 88              	movlps	%xmm8, (%rax,%rcx,4)
1407e7ed0: f3 0f 11 7c 88 08           	movss	%xmm7, 0x8(%rax,%rcx,4)
1407e7ed6: 49 ff c4                    	incq	%r12
1407e7ed9: 4c 89 a5 b8 00 00 00        	movq	%r12, 0xb8(%rbp)
1407e7ee0: 4c 8b 45 d8                 	movq	-0x28(%rbp), %r8
1407e7ee4: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
1407e7ee8: 49 39 c0                    	cmpq	%rax, %r8
1407e7eeb: 0f 84 f3 01 00 00           	je	0x1407e80e4 <.text+0x7e70e4>
1407e7ef1: 49 8d 40 08                 	leaq	0x8(%r8), %rax
1407e7ef5: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1407e7ef9: c6 85 c7 00 00 00 01        	movb	$0x1, 0xc7(%rbp)
1407e7f00: 4c 89 f1                    	movq	%r14, %rcx
1407e7f03: 48 89 da                    	movq	%rbx, %rdx
1407e7f06: e8 a5 4a ac 00              	callq	0x1412ac9b0 <.text+0x12ab9b0>
1407e7f0b: 83 7d 40 01                 	cmpl	$0x1, 0x40(%rbp)
1407e7f0f: 75 3f                       	jne	0x1407e7f50 <.text+0x7e6f50>
1407e7f11: 41 8b 87 80 00 00 00        	movl	0x80(%r15), %eax
1407e7f18: 41 89 47 74                 	movl	%eax, 0x74(%r15)
1407e7f1c: 49 8b 47 78                 	movq	0x78(%r15), %rax
1407e7f20: 49 89 47 6c                 	movq	%rax, 0x6c(%r15)
1407e7f24: 41 8b 46 08                 	movl	0x8(%r14), %eax
1407e7f28: 41 89 87 80 00 00 00        	movl	%eax, 0x80(%r15)
1407e7f2f: 49 8b 06                    	movq	(%r14), %rax
1407e7f32: 49 89 47 78                 	movq	%rax, 0x78(%r15)
1407e7f36: f3 44 0f 10 4d 4c           	movss	0x4c(%rbp), %xmm9
1407e7f3c: f2 44 0f 10 55 44           	movsd	0x44(%rbp), %xmm10
1407e7f42: f2 44 0f 10 45 c4           	movsd	-0x3c(%rbp), %xmm8
1407e7f48: f3 0f 10 7d cc              	movss	-0x34(%rbp), %xmm7
1407e7f4d: eb 62                       	jmp	0x1407e7fb1 <.text+0x7e6fb1>
1407e7f4f: 90                          	nop
1407e7f50: 41 8b 46 08                 	movl	0x8(%r14), %eax
1407e7f54: 41 89 45 08                 	movl	%eax, 0x8(%r13)
1407e7f58: 49 8b 06                    	movq	(%r14), %rax
1407e7f5b: 49 89 45 00                 	movq	%rax, (%r13)
1407e7f5f: c7 45 a8 01 00 00 00        	movl	$0x1, -0x58(%rbp)
1407e7f66: 4c 89 7d b8                 	movq	%r15, -0x48(%rbp)
1407e7f6a: c6 85 c7 00 00 00 01        	movb	$0x1, 0xc7(%rbp)
1407e7f71: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
1407e7f75: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
1407e7f79: e8 f2 d9 a8 00              	callq	0x141275970 <.text+0x1274970>
1407e7f7e: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
1407e7f82: 0f 10 4d 6c                 	movups	0x6c(%rbp), %xmm1
1407e7f86: 41 0f 11 4f 74              	movups	%xmm1, 0x74(%r15)
1407e7f8b: 41 0f 11 47 68              	movups	%xmm0, 0x68(%r15)
1407e7f90: 83 7d 40 01                 	cmpl	$0x1, 0x40(%rbp)
1407e7f94: 0f 85 4a 01 00 00           	jne	0x1407e80e4 <.text+0x7e70e4>
1407e7f9a: f3 44 0f 10 4d 4c           	movss	0x4c(%rbp), %xmm9
1407e7fa0: f2 44 0f 10 55 44           	movsd	0x44(%rbp), %xmm10
1407e7fa6: f2 44 0f 10 45 50           	movsd	0x50(%rbp), %xmm8
1407e7fac: f3 0f 10 7d 58              	movss	0x58(%rbp), %xmm7
1407e7fb1: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
1407e7fb5: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
1407e7fb9: 48 89 4d d0                 	movq	%rcx, -0x30(%rbp)
1407e7fbd: 48 85 c0                    	testq	%rax, %rax
1407e7fc0: 75 4b                       	jne	0x1407e800d <.text+0x7e700d>
1407e7fc2: 4c 8b a5 b8 00 00 00        	movq	0xb8(%rbp), %r12
1407e7fc9: 4c 3b a5 a8 00 00 00        	cmpq	0xa8(%rbp), %r12
1407e7fd0: 75 1a                       	jne	0x1407e7fec <.text+0x7e6fec>
1407e7fd2: c6 85 c7 00 00 00 01        	movb	$0x1, 0xc7(%rbp)
1407e7fd9: 48 8d 8d a8 00 00 00        	leaq	0xa8(%rbp), %rcx
1407e7fe0: 48 8d 15 99 1d 2a 02        	leaq	0x22a1d99(%rip), %rdx   # 0x142a89d80
1407e7fe7: e8 94 71 3b 00              	callq	0x140b9f180 <.text+0xb9e180>
1407e7fec: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1407e7ff3: 4b 8d 0c 64                 	leaq	(%r12,%r12,2), %rcx
1407e7ff7: 44 0f 13 14 88              	movlps	%xmm10, (%rax,%rcx,4)
1407e7ffc: f3 44 0f 11 4c 88 08        	movss	%xmm9, 0x8(%rax,%rcx,4)
1407e8003: 49 ff c4                    	incq	%r12
1407e8006: 4c 89 a5 b8 00 00 00        	movq	%r12, 0xb8(%rbp)
1407e800d: c6 85 c7 00 00 00 01        	movb	$0x1, 0xc7(%rbp)
1407e8014: 48 89 f9                    	movq	%rdi, %rcx
1407e8017: e8 04 26 34 00              	callq	0x140b2a620 <.text+0xb29620>
1407e801c: f3 41 0f 16 c8              	movshdup	%xmm8, %xmm1            # xmm1 = xmm8[1,1,3,3]
1407e8021: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
1407e8024: 77 33                       	ja	0x1407e8059 <.text+0x7e7059>
1407e8026: 4c 8b a5 b8 00 00 00        	movq	0xb8(%rbp), %r12
1407e802d: 4c 3b a5 a8 00 00 00        	cmpq	0xa8(%rbp), %r12
1407e8034: 0f 85 86 fe ff ff           	jne	0x1407e7ec0 <.text+0x7e6ec0>
1407e803a: c6 85 c7 00 00 00 01        	movb	$0x1, 0xc7(%rbp)
1407e8041: 48 8d 8d a8 00 00 00        	leaq	0xa8(%rbp), %rcx
1407e8048: 48 8d 15 49 1d 2a 02        	leaq	0x22a1d49(%rip), %rdx   # 0x142a89d98
1407e804f: e8 2c 71 3b 00              	callq	0x140b9f180 <.text+0xb9e180>
1407e8054: e9 67 fe ff ff              	jmp	0x1407e7ec0 <.text+0x7e6ec0>
1407e8059: c6 85 c7 00 00 00 01        	movb	$0x1, 0xc7(%rbp)
1407e8060: 48 89 f9                    	movq	%rdi, %rcx
1407e8063: e8 b8 25 34 00              	callq	0x140b2a620 <.text+0xb29620>
1407e8068: 0f 28 f0                    	movaps	%xmm0, %xmm6
1407e806b: 48 8b bd b8 00 00 00        	movq	0xb8(%rbp), %rdi
1407e8072: 48 3b bd a8 00 00 00        	cmpq	0xa8(%rbp), %rdi
1407e8079: 75 1a                       	jne	0x1407e8095 <.text+0x7e7095>
1407e807b: c6 85 c7 00 00 00 01        	movb	$0x1, 0xc7(%rbp)
1407e8082: 48 8d 15 27 1d 2a 02        	leaq	0x22a1d27(%rip), %rdx   # 0x142a89db0
1407e8089: 48 8d 8d a8 00 00 00        	leaq	0xa8(%rbp), %rcx
1407e8090: e8 eb 70 3b 00              	callq	0x140b9f180 <.text+0xb9e180>
1407e8095: 45 0f 5c c2                 	subps	%xmm10, %xmm8
1407e8099: f3 41 0f 5c f9              	subss	%xmm9, %xmm7
1407e809e: f3 41 0f 16 c2              	movshdup	%xmm10, %xmm0           # xmm0 = xmm10[1,1,3,3]
1407e80a3: f3 0f 5c f0                 	subss	%xmm0, %xmm6
1407e80a7: f3 41 0f 16 c0              	movshdup	%xmm8, %xmm0            # xmm0 = xmm8[1,1,3,3]
1407e80ac: f3 0f 5e f0                 	divss	%xmm0, %xmm6
1407e80b0: f3 0f 12 c6                 	movsldup	%xmm6, %xmm0            # xmm0 = xmm6[0,0,2,2]
1407e80b4: 41 0f 59 c0                 	mulps	%xmm8, %xmm0
1407e80b8: f3 0f 59 f7                 	mulss	%xmm7, %xmm6
1407e80bc: 41 0f 58 c2                 	addps	%xmm10, %xmm0
1407e80c0: f3 41 0f 58 f1              	addss	%xmm9, %xmm6
1407e80c5: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1407e80cc: 48 8d 0c 7f                 	leaq	(%rdi,%rdi,2), %rcx
1407e80d0: 0f 13 04 88                 	movlps	%xmm0, (%rax,%rcx,4)
1407e80d4: f3 0f 11 74 88 08           	movss	%xmm6, 0x8(%rax,%rcx,4)
1407e80da: 48 ff c7                    	incq	%rdi
1407e80dd: 48 89 bd b8 00 00 00        	movq	%rdi, 0xb8(%rbp)
1407e80e4: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
1407e80eb: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1407e80ef: 0f 10 85 a8 00 00 00        	movups	0xa8(%rbp), %xmm0
1407e80f6: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
1407e80fa: c6 85 c7 00 00 00 00        	movb	$0x0, 0xc7(%rbp)
1407e8101: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
1407e8105: 48 8d 55 60                 	leaq	0x60(%rbp), %rdx
1407e8109: 45 31 c0                    	xorl	%r8d, %r8d
1407e810c: e8 cf 1b a8 00              	callq	0x141269ce0 <.text+0x1268ce0>
1407e8111: 31 c0                       	xorl	%eax, %eax
1407e8113: 48 3b 45 d0                 	cmpq	-0x30(%rbp), %rax
1407e8117: 0f 80 b4 00 00 00           	jo	0x1407e81d1 <.text+0x7e71d1>
1407e811d: 48 8b 45 00                 	movq	(%rbp), %rax
1407e8121: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
1407e8128: 0f 10 45 d0                 	movups	-0x30(%rbp), %xmm0
1407e812c: 0f 10 4d e0                 	movups	-0x20(%rbp), %xmm1
1407e8130: 0f 10 55 f0                 	movups	-0x10(%rbp), %xmm2
1407e8134: 0f 29 95 80 00 00 00        	movaps	%xmm2, 0x80(%rbp)
1407e813b: 0f 29 4d 70                 	movaps	%xmm1, 0x70(%rbp)
1407e813f: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
1407e8143: 48 8b 76 50                 	movq	0x50(%rsi), %rsi
1407e8147: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
1407e814b: 48 3b 3e                    	cmpq	(%rsi), %rdi
1407e814e: 75 0f                       	jne	0x1407e815f <.text+0x7e715f>
1407e8150: 48 8d 15 89 1c 2a 02        	leaq	0x22a1c89(%rip), %rdx   # 0x142a89de0
1407e8157: 48 89 f1                    	movq	%rsi, %rcx
1407e815a: e8 71 98 e4 ff              	callq	0x1406319d0 <.text+0x6309d0>
1407e815f: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1407e8163: 48 6b cf 38                 	imulq	$0x38, %rdi, %rcx
1407e8167: 0f 28 45 60                 	movaps	0x60(%rbp), %xmm0
1407e816b: 0f 28 4d 70                 	movaps	0x70(%rbp), %xmm1
1407e816f: 0f 28 95 80 00 00 00        	movaps	0x80(%rbp), %xmm2
1407e8176: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
1407e817d: 48 89 54 08 30              	movq	%rdx, 0x30(%rax,%rcx)
1407e8182: 0f 11 54 08 20              	movups	%xmm2, 0x20(%rax,%rcx)
1407e8187: 0f 11 4c 08 10              	movups	%xmm1, 0x10(%rax,%rcx)
1407e818c: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
1407e8190: 48 ff c7                    	incq	%rdi
1407e8193: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
1407e8197: 0f 28 b5 d0 00 00 00        	movaps	0xd0(%rbp), %xmm6
1407e819e: 0f 28 bd e0 00 00 00        	movaps	0xe0(%rbp), %xmm7
1407e81a5: 44 0f 28 85 f0 00 00 00     	movaps	0xf0(%rbp), %xmm8
1407e81ad: 44 0f 28 8d 00 01 00 00     	movaps	0x100(%rbp), %xmm9
1407e81b5: 44 0f 28 95 10 01 00 00     	movaps	0x110(%rbp), %xmm10
1407e81bd: 48 81 c4 a8 01 00 00        	addq	$0x1a8, %rsp            # imm = 0x1A8
1407e81c4: 5b                          	popq	%rbx
1407e81c5: 5f                          	popq	%rdi
1407e81c6: 5e                          	popq	%rsi
1407e81c7: 41 5c                       	popq	%r12
1407e81c9: 41 5d                       	popq	%r13
1407e81cb: 41 5e                       	popq	%r14
1407e81cd: 41 5f                       	popq	%r15
1407e81cf: 5d                          	popq	%rbp
1407e81d0: c3                          	retq
1407e81d1: 0f 10 45 d8                 	movups	-0x28(%rbp), %xmm0
1407e81d5: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
1407e81d9: c6 85 c7 00 00 00 00        	movb	$0x0, 0xc7(%rbp)
1407e81e0: 48 8d 05 e1 1b 2a 02        	leaq	0x22a1be1(%rip), %rax   # 0x142a89dc8
1407e81e7: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407e81ec: 48 8d 0d 75 a1 43 02        	leaq	0x243a175(%rip), %rcx   # 0x142c22368
1407e81f3: 4c 8d 0d 4e a1 43 02        	leaq	0x243a14e(%rip), %r9    # 0x142c22348
1407e81fa: 4c 8d 45 60                 	leaq	0x60(%rbp), %r8
1407e81fe: ba 2b 00 00 00              	movl	$0x2b, %edx
1407e8203: e8 58 15 0f 02              	callq	0x1428d9760 <.text+0x28d8760>
1407e8208: 0f 0b                       	ud2
1407e820a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1407e8210: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407e8215: 55                          	pushq	%rbp
1407e8216: 41 57                       	pushq	%r15
1407e8218: 41 56                       	pushq	%r14
1407e821a: 41 55                       	pushq	%r13
1407e821c: 41 54                       	pushq	%r12
1407e821e: 56                          	pushq	%rsi
1407e821f: 57                          	pushq	%rdi
1407e8220: 53                          	pushq	%rbx
1407e8221: 48 83 ec 78                 	subq	$0x78, %rsp
1407e8225: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407e822c: 44 0f 29 54 24 20           	movaps	%xmm10, 0x20(%rsp)
1407e8232: 44 0f 29 4c 24 30           	movaps	%xmm9, 0x30(%rsp)
1407e8238: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
1407e823e: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
1407e8243: 0f 29 74 24 60              	movaps	%xmm6, 0x60(%rsp)
1407e8248: c6 85 c7 00 00 00 00        	movb	$0x0, 0xc7(%rbp)
1407e824f: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
1407e8253: e8 c8 37 96 ff              	callq	0x14014ba20 <.text+0x14aa20>
1407e8258: c6 85 c7 00 00 00 00        	movb	$0x0, 0xc7(%rbp)
1407e825f: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
1407e8264: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
1407e8269: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
1407e826f: 44 0f 28 4c 24 30           	movaps	0x30(%rsp), %xmm9
1407e8275: 44 0f 28 54 24 20           	movaps	0x20(%rsp), %xmm10
1407e827b: 48 83 c4 78                 	addq	$0x78, %rsp
1407e827f: 5b                          	popq	%rbx
1407e8280: 5f                          	popq	%rdi
1407e8281: 5e                          	popq	%rsi
1407e8282: 41 5c                       	popq	%r12
1407e8284: 41 5d                       	popq	%r13
1407e8286: 41 5e                       	popq	%r14
1407e8288: 41 5f                       	popq	%r15
1407e828a: 5d                          	popq	%rbp
1407e828b: c3                          	retq
1407e828c: 0f 1f 40 00                 	nopl	(%rax)
1407e8290: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407e8295: 55                          	pushq	%rbp
1407e8296: 41 57                       	pushq	%r15
1407e8298: 41 56                       	pushq	%r14
1407e829a: 41 55                       	pushq	%r13
1407e829c: 41 54                       	pushq	%r12
1407e829e: 56                          	pushq	%rsi
1407e829f: 57                          	pushq	%rdi
1407e82a0: 53                          	pushq	%rbx
1407e82a1: 48 83 ec 78                 	subq	$0x78, %rsp
1407e82a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407e82ac: 44 0f 29 54 24 20           	movaps	%xmm10, 0x20(%rsp)
1407e82b2: 44 0f 29 4c 24 30           	movaps	%xmm9, 0x30(%rsp)
1407e82b8: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
1407e82be: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
1407e82c3: 0f 29 74 24 60              	movaps	%xmm6, 0x60(%rsp)
1407e82c8: 80 bd c7 00 00 00 00        	cmpb	$0x0, 0xc7(%rbp)
1407e82cf: 74 26                       	je	0x1407e82f7 <.text+0x7e72f7>
1407e82d1: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
1407e82d8: 48 85 c0                    	testq	%rax, %rax
1407e82db: 74 1a                       	je	0x1407e82f7 <.text+0x7e72f7>
1407e82dd: 48 8b 8d b0 00 00 00        	movq	0xb0(%rbp), %rcx
1407e82e4: 48 c1 e0 02                 	shlq	$0x2, %rax
1407e82e8: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1407e82ec: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407e82f2: e8 29 b9 e2 ff              	callq	0x140613c20 <.text+0x612c20>
1407e82f7: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
1407e82fc: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
1407e8301: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
1407e8307: 44 0f 28 4c 24 30           	movaps	0x30(%rsp), %xmm9
1407e830d: 44 0f 28 54 24 20           	movaps	0x20(%rsp), %xmm10
1407e8313: 48 83 c4 78                 	addq	$0x78, %rsp
1407e8317: 5b                          	popq	%rbx
1407e8318: 5f                          	popq	%rdi
1407e8319: 5e                          	popq	%rsi
1407e831a: 41 5c                       	popq	%r12
1407e831c: 41 5d                       	popq	%r13
1407e831e: 41 5e                       	popq	%r14
1407e8320: 41 5f                       	popq	%r15
1407e8322: 5d                          	popq	%rbp
1407e8323: c3                          	retq
