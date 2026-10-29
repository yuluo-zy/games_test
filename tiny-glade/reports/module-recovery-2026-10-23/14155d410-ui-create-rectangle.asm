
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14155d410: 41 57                       	pushq	%r15
14155d412: 41 56                       	pushq	%r14
14155d414: 41 55                       	pushq	%r13
14155d416: 41 54                       	pushq	%r12
14155d418: 56                          	pushq	%rsi
14155d419: 57                          	pushq	%rdi
14155d41a: 55                          	pushq	%rbp
14155d41b: 53                          	pushq	%rbx
14155d41c: 48 81 ec c8 01 00 00        	subq	$0x1c8, %rsp            # imm = 0x1C8
14155d423: 44 0f 29 84 24 b0 01 00 00  	movaps	%xmm8, 0x1b0(%rsp)
14155d42c: 0f 29 bc 24 a0 01 00 00     	movaps	%xmm7, 0x1a0(%rsp)
14155d434: 0f 29 b4 24 90 01 00 00     	movaps	%xmm6, 0x190(%rsp)
14155d43c: 4d 89 ce                    	movq	%r9, %r14
14155d43f: 49 8b 68 20                 	movq	0x20(%r8), %rbp
14155d443: 4d 8b 48 30                 	movq	0x30(%r8), %r9
14155d447: 45 8b 50 3c                 	movl	0x3c(%r8), %r10d
14155d44b: 48 8b 01                    	movq	(%rcx), %rax
14155d44e: 83 38 06                    	cmpl	$0x6, (%rax)
14155d451: 0f 85 18 05 00 00           	jne	0x14155d96f <.text+0x155c96f>
14155d457: 80 78 04 02                 	cmpb	$0x2, 0x4(%rax)
14155d45b: 0f 85 0e 05 00 00           	jne	0x14155d96f <.text+0x155c96f>
14155d461: 44 89 54 24 5c              	movl	%r10d, 0x5c(%rsp)
14155d466: 4c 89 8c 24 88 00 00 00     	movq	%r9, 0x88(%rsp)
14155d46e: 4c 8b 8c 24 70 02 00 00     	movq	0x270(%rsp), %r9
14155d476: 48 8b 8c 24 68 02 00 00     	movq	0x268(%rsp), %rcx
14155d47e: 48 8b 84 24 80 02 00 00     	movq	0x280(%rsp), %rax
14155d486: 49 8b 30                    	movq	(%r8), %rsi
14155d489: 48 8b 49 28                 	movq	0x28(%rcx), %rcx
14155d48d: 48 89 8c 24 80 00 00 00     	movq	%rcx, 0x80(%rsp)
14155d495: 49 8b 49 20                 	movq	0x20(%r9), %rcx
14155d499: 48 89 8c 24 78 01 00 00     	movq	%rcx, 0x178(%rsp)
14155d4a1: 49 8b 49 30                 	movq	0x30(%r9), %rcx
14155d4a5: 48 89 8c 24 70 01 00 00     	movq	%rcx, 0x170(%rsp)
14155d4ad: 41 8b 49 3c                 	movl	0x3c(%r9), %ecx
14155d4b1: 89 8c 24 a0 00 00 00        	movl	%ecx, 0xa0(%rsp)
14155d4b8: 48 8b 38                    	movq	(%rax), %rdi
14155d4bb: 48 8b 48 20                 	movq	0x20(%rax), %rcx
14155d4bf: 48 89 8c 24 90 00 00 00     	movq	%rcx, 0x90(%rsp)
14155d4c7: 48 8b 48 30                 	movq	0x30(%rax), %rcx
14155d4cb: 48 89 8c 24 68 01 00 00     	movq	%rcx, 0x168(%rsp)
14155d4d3: 8b 48 3c                    	movl	0x3c(%rax), %ecx
14155d4d6: 89 8c 24 9c 00 00 00        	movl	%ecx, 0x9c(%rsp)
14155d4dd: 48 8b 48 40                 	movq	0x40(%rax), %rcx
14155d4e1: 48 89 4c 24 68              	movq	%rcx, 0x68(%rsp)
14155d4e6: 48 8b 48 50                 	movq	0x50(%rax), %rcx
14155d4ea: 48 89 8c 24 60 01 00 00     	movq	%rcx, 0x160(%rsp)
14155d4f2: 8b 40 5c                    	movl	0x5c(%rax), %eax
14155d4f5: 89 84 24 98 00 00 00        	movl	%eax, 0x98(%rsp)
14155d4fc: 48 8b 0a                    	movq	(%rdx), %rcx
14155d4ff: 48 c7 84 24 b0 00 00 00 00 00 00 00 	movq	$0x0, 0xb0(%rsp)
14155d50b: 48 c7 84 24 b8 00 00 00 08 00 00 00 	movq	$0x8, 0xb8(%rsp)
14155d517: 48 c7 84 24 c0 00 00 00 00 00 00 00 	movq	$0x0, 0xc0(%rsp)
14155d523: c7 84 24 c8 00 00 00 00 00 00 00    	movl	$0x0, 0xc8(%rsp)
14155d52e: c7 84 24 e0 00 00 00 00 00 00 00    	movl	$0x0, 0xe0(%rsp)
14155d539: c7 84 24 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rsp)
14155d544: c7 84 24 10 01 00 00 00 00 00 00    	movl	$0x0, 0x110(%rsp)
14155d54f: c6 84 24 28 01 00 00 00     	movb	$0x0, 0x128(%rsp)
14155d557: c6 84 24 38 01 00 00 05     	movb	$0x5, 0x138(%rsp)
14155d55f: c7 84 24 3c 01 00 00 ff ff 7f 7f    	movl	$0x7f7fffff, 0x13c(%rsp) # imm = 0x7F7FFFFF
14155d56a: 66 c7 84 24 58 01 00 00 00 01       	movw	$0x100, 0x158(%rsp) # imm = 0x100
14155d574: 4d 8b 61 40                 	movq	0x40(%r9), %r12
14155d578: 4d 8b 69 50                 	movq	0x50(%r9), %r13
14155d57c: 41 8b 41 5c                 	movl	0x5c(%r9), %eax
14155d580: 89 44 24 24                 	movl	%eax, 0x24(%rsp)
14155d584: 48 8d 15 6d 0d 71 01        	leaq	0x1710d6d(%rip), %rdx   # 0x142c6e2f8
14155d58b: 4c 8d 8c 24 b0 00 00 00     	leaq	0xb0(%rsp), %r9
14155d593: 41 b8 15 00 00 00           	movl	$0x15, %r8d
14155d599: e8 32 61 3c ff              	callq	0x1409236d0 <.text+0x9226d0>
14155d59e: 49 89 c1                    	movq	%rax, %r9
14155d5a1: f3 0f 10 46 3c              	movss	0x3c(%rsi), %xmm0
14155d5a6: 0f 2e 05 1f 43 47 01        	ucomiss	0x147431f(%rip), %xmm0  # 0x1429d18cc
14155d5ad: 48 89 44 24 78              	movq	%rax, 0x78(%rsp)
14155d5b2: 48 89 54 24 70              	movq	%rdx, 0x70(%rsp)
14155d5b7: 0f 82 b0 00 00 00           	jb	0x14155d66d <.text+0x155c66d>
14155d5bd: 48 89 74 24 28              	movq	%rsi, 0x28(%rsp)
14155d5c2: 4c 89 e6                    	movq	%r12, %rsi
14155d5c5: 4d 89 f4                    	movq	%r14, %r12
14155d5c8: 4d 89 ee                    	movq	%r13, %r14
14155d5cb: 49 89 fd                    	movq	%rdi, %r13
14155d5ce: 48 8b 84 24 88 02 00 00     	movq	0x288(%rsp), %rax
14155d5d6: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
14155d5da: 48 89 8c 24 c0 00 00 00     	movq	%rcx, 0xc0(%rsp)
14155d5e2: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14155d5e6: 0f 29 84 24 b0 00 00 00     	movaps	%xmm0, 0xb0(%rsp)
14155d5ee: 8b 48 1c                    	movl	0x1c(%rax), %ecx
14155d5f1: 4c 8b 40 10                 	movq	0x10(%rax), %r8
14155d5f5: 41 89 08                    	movl	%ecx, (%r8)
14155d5f8: 48 8b 18                    	movq	(%rax), %rbx
14155d5fb: 4c 8b 7b 30                 	movq	0x30(%rbx), %r15
14155d5ff: 48 8b 7b 40                 	movq	0x40(%rbx), %rdi
14155d603: 4c 3b 7b 20                 	cmpq	0x20(%rbx), %r15
14155d607: 75 1a                       	jne	0x14155d623 <.text+0x155c623>
14155d609: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
14155d60d: 48 8d 15 34 cb 6d 01        	leaq	0x16dcb34(%rip), %rdx   # 0x142c3a148
14155d614: e8 97 54 00 ff              	callq	0x140562ab0 <.text+0x561ab0>
14155d619: 48 8b 54 24 70              	movq	0x70(%rsp), %rdx
14155d61e: 4c 8b 4c 24 78              	movq	0x78(%rsp), %r9
14155d623: 48 8b 43 28                 	movq	0x28(%rbx), %rax
14155d627: 4b 8d 0c bf                 	leaq	(%r15,%r15,4), %rcx
14155d62b: 48 89 3c c8                 	movq	%rdi, (%rax,%rcx,8)
14155d62f: c7 44 c8 08 01 00 00 00     	movl	$0x1, 0x8(%rax,%rcx,8)
14155d637: 4c 8b 84 24 c0 00 00 00     	movq	0xc0(%rsp), %r8
14155d63f: 4c 89 44 c8 1c              	movq	%r8, 0x1c(%rax,%rcx,8)
14155d644: 0f 28 84 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm0
14155d64c: 0f 11 44 c8 0c              	movups	%xmm0, 0xc(%rax,%rcx,8)
14155d651: 49 ff c7                    	incq	%r15
14155d654: 4c 89 7b 30                 	movq	%r15, 0x30(%rbx)
14155d658: 48 ff 43 40                 	incq	0x40(%rbx)
14155d65c: 4c 89 ef                    	movq	%r13, %rdi
14155d65f: 4d 89 f5                    	movq	%r14, %r13
14155d662: 4d 89 e6                    	movq	%r12, %r14
14155d665: 49 89 f4                    	movq	%rsi, %r12
14155d668: 48 8b 74 24 28              	movq	0x28(%rsp), %rsi
14155d66d: 4c 8b 9c 24 40 02 00 00     	movq	0x240(%rsp), %r11
14155d675: b3 02                       	movb	$0x2, %bl
14155d677: 41 80 79 18 02              	cmpb	$0x2, 0x18(%r9)
14155d67c: 0f 82 76 02 00 00           	jb	0x14155d8f8 <.text+0x155c8f8>
14155d682: 4c 8b bc 24 60 02 00 00     	movq	0x260(%rsp), %r15
14155d68a: 49 39 51 38                 	cmpq	%rdx, 0x38(%r9)
14155d68e: 0f 85 c0 00 00 00           	jne	0x14155d754 <.text+0x155c754>
14155d694: 41 80 79 19 03              	cmpb	$0x3, 0x19(%r9)
14155d699: 0f 85 b5 00 00 00           	jne	0x14155d754 <.text+0x155c754>
14155d69f: 48 89 7c 24 60              	movq	%rdi, 0x60(%rsp)
14155d6a4: 48 89 74 24 28              	movq	%rsi, 0x28(%rsp)
14155d6a9: 4c 89 ee                    	movq	%r13, %rsi
14155d6ac: 4d 89 e5                    	movq	%r12, %r13
14155d6af: 48 8b 84 24 58 02 00 00     	movq	0x258(%rsp), %rax
14155d6b7: 41 8b 4b 1c                 	movl	0x1c(%r11), %ecx
14155d6bb: 4d 8b 53 10                 	movq	0x10(%r11), %r10
14155d6bf: 41 89 0a                    	movl	%ecx, (%r10)
14155d6c2: 4d 8b 03                    	movq	(%r11), %r8
14155d6c5: 41 c6 40 0c 00              	movb	$0x0, 0xc(%r8)
14155d6ca: 48 8b 00                    	movq	(%rax), %rax
14155d6cd: f2 0f 10 00                 	movsd	(%rax), %xmm0
14155d6d1: f2 41 0f 11 40 04           	movsd	%xmm0, 0x4(%r8)
14155d6d7: 41 89 0a                    	movl	%ecx, (%r10)
14155d6da: 41 c7 00 00 00 00 00        	movl	$0x0, (%r8)
14155d6e1: 41 8b 47 1c                 	movl	0x1c(%r15), %eax
14155d6e5: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
14155d6e9: 89 01                       	movl	%eax, (%rcx)
14155d6eb: 49 8b 3f                    	movq	(%r15), %rdi
14155d6ee: 4c 8b 7f 30                 	movq	0x30(%rdi), %r15
14155d6f2: 4c 8b 67 40                 	movq	0x40(%rdi), %r12
14155d6f6: 4c 3b 7f 20                 	cmpq	0x20(%rdi), %r15
14155d6fa: 75 22                       	jne	0x14155d71e <.text+0x155c71e>
14155d6fc: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
14155d700: 48 8d 15 41 ca 6d 01        	leaq	0x16dca41(%rip), %rdx   # 0x142c3a148
14155d707: e8 64 51 00 ff              	callq	0x140562870 <.text+0x561870>
14155d70c: 4c 8b 9c 24 40 02 00 00     	movq	0x240(%rsp), %r11
14155d714: 48 8b 54 24 70              	movq	0x70(%rsp), %rdx
14155d719: 4c 8b 4c 24 78              	movq	0x78(%rsp), %r9
14155d71e: 48 8b 47 28                 	movq	0x28(%rdi), %rax
14155d722: 4e 89 24 f8                 	movq	%r12, (%rax,%r15,8)
14155d726: 49 ff c7                    	incq	%r15
14155d729: 4c 89 7f 30                 	movq	%r15, 0x30(%rdi)
14155d72d: 48 ff 47 40                 	incq	0x40(%rdi)
14155d731: 41 80 79 18 02              	cmpb	$0x2, 0x18(%r9)
14155d736: 4d 89 ec                    	movq	%r13, %r12
14155d739: 49 89 f5                    	movq	%rsi, %r13
14155d73c: 48 8b 74 24 28              	movq	0x28(%rsp), %rsi
14155d741: 48 8b 7c 24 60              	movq	0x60(%rsp), %rdi
14155d746: 4c 8b bc 24 60 02 00 00     	movq	0x260(%rsp), %r15
14155d74e: 0f 82 a4 01 00 00           	jb	0x14155d8f8 <.text+0x155c8f8>
14155d754: 49 39 51 38                 	cmpq	%rdx, 0x38(%r9)
14155d758: 0f 85 d4 00 00 00           	jne	0x14155d832 <.text+0x155c832>
14155d75e: 41 80 79 19 04              	cmpb	$0x4, 0x19(%r9)
14155d763: 0f 85 c9 00 00 00           	jne	0x14155d832 <.text+0x155c832>
14155d769: 48 89 7c 24 60              	movq	%rdi, 0x60(%rsp)
14155d76e: 48 89 74 24 28              	movq	%rsi, 0x28(%rsp)
14155d773: 4c 89 6c 24 50              	movq	%r13, 0x50(%rsp)
14155d778: 4d 89 e5                    	movq	%r12, %r13
14155d77b: 48 8b 84 24 48 02 00 00     	movq	0x248(%rsp), %rax
14155d783: 48 8b 00                    	movq	(%rax), %rax
14155d786: f2 0f 10 80 40 01 00 00     	movsd	0x140(%rax), %xmm0
14155d78e: 0f 59 c0                    	mulps	%xmm0, %xmm0
14155d791: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
14155d795: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14155d799: 0f 57 c0                    	xorps	%xmm0, %xmm0
14155d79c: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
14155d7a0: 41 8b 43 1c                 	movl	0x1c(%r11), %eax
14155d7a4: 49 8b 4b 10                 	movq	0x10(%r11), %rcx
14155d7a8: 89 01                       	movl	%eax, (%rcx)
14155d7aa: 4d 8b 03                    	movq	(%r11), %r8
14155d7ad: f3 41 0f 58 00              	addss	(%r8), %xmm0
14155d7b2: f3 41 0f 11 00              	movss	%xmm0, (%r8)
14155d7b7: 0f 2e 05 d6 6e 3d 01        	ucomiss	0x13d6ed6(%rip), %xmm0  # 0x142934694
14155d7be: 76 07                       	jbe	0x14155d7c7 <.text+0x155c7c7>
14155d7c0: 89 01                       	movl	%eax, (%rcx)
14155d7c2: 41 c6 40 0c 01              	movb	$0x1, 0xc(%r8)
14155d7c7: 41 8b 47 1c                 	movl	0x1c(%r15), %eax
14155d7cb: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
14155d7cf: 89 01                       	movl	%eax, (%rcx)
14155d7d1: 49 8b 3f                    	movq	(%r15), %rdi
14155d7d4: 4c 8b 7f 30                 	movq	0x30(%rdi), %r15
14155d7d8: 4c 8b 67 40                 	movq	0x40(%rdi), %r12
14155d7dc: 4c 3b 7f 20                 	cmpq	0x20(%rdi), %r15
14155d7e0: 75 20                       	jne	0x14155d802 <.text+0x155c802>
14155d7e2: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
14155d7e6: 48 8d 15 5b c9 6d 01        	leaq	0x16dc95b(%rip), %rdx   # 0x142c3a148
14155d7ed: 4c 89 de                    	movq	%r11, %rsi
14155d7f0: e8 7b 50 00 ff              	callq	0x140562870 <.text+0x561870>
14155d7f5: 49 89 f3                    	movq	%rsi, %r11
14155d7f8: 48 8b 54 24 70              	movq	0x70(%rsp), %rdx
14155d7fd: 4c 8b 4c 24 78              	movq	0x78(%rsp), %r9
14155d802: 48 8b 47 28                 	movq	0x28(%rdi), %rax
14155d806: 4e 89 24 f8                 	movq	%r12, (%rax,%r15,8)
14155d80a: 49 ff c7                    	incq	%r15
14155d80d: 4c 89 7f 30                 	movq	%r15, 0x30(%rdi)
14155d811: 48 ff 47 40                 	incq	0x40(%rdi)
14155d815: 41 80 79 18 02              	cmpb	$0x2, 0x18(%r9)
14155d81a: 4d 89 ec                    	movq	%r13, %r12
14155d81d: 4c 8b 6c 24 50              	movq	0x50(%rsp), %r13
14155d822: 48 8b 74 24 28              	movq	0x28(%rsp), %rsi
14155d827: 48 8b 7c 24 60              	movq	0x60(%rsp), %rdi
14155d82c: 0f 82 c6 00 00 00           	jb	0x14155d8f8 <.text+0x155c8f8>
14155d832: 49 39 51 38                 	cmpq	%rdx, 0x38(%r9)
14155d836: 0f 85 b1 00 00 00           	jne	0x14155d8ed <.text+0x155c8ed>
14155d83c: 41 80 79 19 01              	cmpb	$0x1, 0x19(%r9)
14155d841: 0f 87 a6 00 00 00           	ja	0x14155d8ed <.text+0x155c8ed>
14155d847: 48 89 6c 24 50              	movq	%rbp, 0x50(%rsp)
14155d84c: 48 89 74 24 28              	movq	%rsi, 0x28(%rsp)
14155d851: 4c 89 64 24 60              	movq	%r12, 0x60(%rsp)
14155d856: 4d 89 f4                    	movq	%r14, %r12
14155d859: 4d 89 ee                    	movq	%r13, %r14
14155d85c: 49 89 fd                    	movq	%rdi, %r13
14155d85f: 48 8b 84 24 50 02 00 00     	movq	0x250(%rsp), %rax
14155d867: 49 8b 0b                    	movq	(%r11), %rcx
14155d86a: f2 0f 10 71 04              	movsd	0x4(%rcx), %xmm6
14155d86f: 8b 48 1c                    	movl	0x1c(%rax), %ecx
14155d872: 4c 8b 40 10                 	movq	0x10(%rax), %r8
14155d876: 41 89 08                    	movl	%ecx, (%r8)
14155d879: 4c 8b 38                    	movq	(%rax), %r15
14155d87c: 49 8b 7f 30                 	movq	0x30(%r15), %rdi
14155d880: 49 8b 77 40                 	movq	0x40(%r15), %rsi
14155d884: 49 3b 7f 20                 	cmpq	0x20(%r15), %rdi
14155d888: 75 20                       	jne	0x14155d8aa <.text+0x155c8aa>
14155d88a: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
14155d88e: 48 8d 15 b3 c8 6d 01        	leaq	0x16dc8b3(%rip), %rdx   # 0x142c3a148
14155d895: 4c 89 dd                    	movq	%r11, %rbp
14155d898: e8 13 55 00 ff              	callq	0x140562db0 <.text+0x561db0>
14155d89d: 49 89 eb                    	movq	%rbp, %r11
14155d8a0: 48 8b 54 24 70              	movq	0x70(%rsp), %rdx
14155d8a5: 4c 8b 4c 24 78              	movq	0x78(%rsp), %r9
14155d8aa: 49 8b 47 28                 	movq	0x28(%r15), %rax
14155d8ae: 48 8d 0c 7f                 	leaq	(%rdi,%rdi,2), %rcx
14155d8b2: 48 89 34 c8                 	movq	%rsi, (%rax,%rcx,8)
14155d8b6: c7 44 c8 08 02 00 00 00     	movl	$0x2, 0x8(%rax,%rcx,8)
14155d8be: 0f 13 74 c8 0c              	movlps	%xmm6, 0xc(%rax,%rcx,8)
14155d8c3: 48 ff c7                    	incq	%rdi
14155d8c6: 49 89 7f 30                 	movq	%rdi, 0x30(%r15)
14155d8ca: 49 ff 47 40                 	incq	0x40(%r15)
14155d8ce: 41 80 79 18 02              	cmpb	$0x2, 0x18(%r9)
14155d8d3: 4c 89 ef                    	movq	%r13, %rdi
14155d8d6: 4d 89 f5                    	movq	%r14, %r13
14155d8d9: 4d 89 e6                    	movq	%r12, %r14
14155d8dc: 4c 8b 64 24 60              	movq	0x60(%rsp), %r12
14155d8e1: 48 8b 74 24 28              	movq	0x28(%rsp), %rsi
14155d8e6: 48 8b 6c 24 50              	movq	0x50(%rsp), %rbp
14155d8eb: 72 0b                       	jb	0x14155d8f8 <.text+0x155c8f8>
14155d8ed: 49 39 51 38                 	cmpq	%rdx, 0x38(%r9)
14155d8f1: 75 05                       	jne	0x14155d8f8 <.text+0x155c8f8>
14155d8f3: 41 0f b6 59 19              	movzbl	0x19(%r9), %ebx
14155d8f8: 4c 8b bc 24 38 02 00 00     	movq	0x238(%rsp), %r15
14155d900: 80 c3 fe                    	addb	$-0x2, %bl
14155d903: 0f b6 cb                    	movzbl	%bl, %ecx
14155d906: 80 f9 03                    	cmpb	$0x3, %cl
14155d909: b8 03 00 00 00              	movl	$0x3, %eax
14155d90e: 0f 42 c1                    	cmovbl	%ecx, %eax
14155d911: 3c 02                       	cmpb	$0x2, %al
14155d913: 73 69                       	jae	0x14155d97e <.text+0x155c97e>
14155d915: 80 7e 2c 02                 	cmpb	$0x2, 0x2c(%rsi)
14155d919: 0f 85 90 01 00 00           	jne	0x14155daaf <.text+0x155caaf>
14155d91f: 49 8b 16                    	movq	(%r14), %rdx
14155d922: 48 8d 8c 24 b0 00 00 00     	leaq	0xb0(%rsp), %rcx
14155d92a: e8 c1 94 73 ff              	callq	0x140c96df0 <.text+0xc95df0>
14155d92f: f3 0f 10 8c 24 b0 00 00 00  	movss	0xb0(%rsp), %xmm1
14155d938: f3 0f 10 84 24 b8 00 00 00  	movss	0xb8(%rsp), %xmm0
14155d941: 0f 28 d1                    	movaps	%xmm1, %xmm2
14155d944: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
14155d948: 0f 14 c8                    	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
14155d94b: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
14155d94f: f3 0f 58 c2                 	addss	%xmm2, %xmm0
14155d953: f3 0f 51 c0                 	sqrtss	%xmm0, %xmm0
14155d957: f3 0f 10 15 a5 7f 3c 01     	movss	0x13c7fa5(%rip), %xmm2  # 0x142925904
14155d95f: f3 0f 5e d0                 	divss	%xmm0, %xmm2
14155d963: f3 0f 12 c2                 	movsldup	%xmm2, %xmm0            # xmm0 = xmm2[0,0,2,2]
14155d967: 0f 59 c1                    	mulps	%xmm1, %xmm0
14155d96a: e9 7a 01 00 00              	jmp	0x14155dae9 <.text+0x155cae9>
14155d96f: 45 89 11                    	movl	%r10d, (%r9)
14155d972: c7 45 00 00 00 00 00        	movl	$0x0, (%rbp)
14155d979: e9 19 09 00 00              	jmp	0x14155e297 <.text+0x155d297>
14155d97e: 0f b6 c0                    	movzbl	%al, %eax
14155d981: 83 f8 02                    	cmpl	$0x2, %eax
14155d984: 0f 85 18 04 00 00           	jne	0x14155dda2 <.text+0x155cda2>
14155d98a: 49 8b 03                    	movq	(%r11), %rax
14155d98d: 0f 57 ff                    	xorps	%xmm7, %xmm7
14155d990: b3 01                       	movb	$0x1, %bl
14155d992: 80 78 0c 00                 	cmpb	$0x0, 0xc(%rax)
14155d996: 41 b0 01                    	movb	$0x1, %r8b
14155d999: 0f 84 d2 00 00 00           	je	0x14155da71 <.text+0x155ca71>
14155d99f: f3 0f 10 46 34              	movss	0x34(%rsi), %xmm0
14155d9a4: 48 8b 8c 24 80 00 00 00     	movq	0x80(%rsp), %rcx
14155d9ac: f3 0f 5c 41 04              	subss	0x4(%rcx), %xmm0
14155d9b1: f3 0f 5e 41 10              	divss	0x10(%rcx), %xmm0
14155d9b6: f3 0f 10 49 0c              	movss	0xc(%rcx), %xmm1
14155d9bb: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
14155d9bf: f3 0f 59 41 14              	mulss	0x14(%rcx), %xmm0
14155d9c4: f3 0f 58 09                 	addss	(%rcx), %xmm1
14155d9c8: f3 0f 58 41 08              	addss	0x8(%rcx), %xmm0
14155d9cd: f3 0f 5c 4d 18              	subss	0x18(%rbp), %xmm1
14155d9d2: f3 0f 5c 45 1c              	subss	0x1c(%rbp), %xmm0
14155d9d7: f3 0f 12 d0                 	movsldup	%xmm0, %xmm2            # xmm2 = xmm0[0,0,2,2]
14155d9db: f2 0f 10 7d 10              	movsd	0x10(%rbp), %xmm7
14155d9e0: f2 0f 10 45 20              	movsd	0x20(%rbp), %xmm0
14155d9e5: f3 0f 12 c9                 	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
14155d9e9: 0f 59 cf                    	mulps	%xmm7, %xmm1
14155d9ec: 0f c6 ff e1                 	shufps	$0xe1, %xmm7, %xmm7     # xmm7 = xmm7[1,0,2,3]
14155d9f0: 0f 59 fa                    	mulps	%xmm2, %xmm7
14155d9f3: 0f 28 d7                    	movaps	%xmm7, %xmm2
14155d9f6: 0f 58 d1                    	addps	%xmm1, %xmm2
14155d9f9: 0f 5c f9                    	subps	%xmm1, %xmm7
14155d9fc: 0f c6 fa 01                 	shufps	$0x1, %xmm2, %xmm7      # xmm7 = xmm7[1,0],xmm2[0,0]
14155da00: 0f c6 fa e2                 	shufps	$0xe2, %xmm2, %xmm7     # xmm7 = xmm7[2,0],xmm2[2,3]
14155da04: 0f 28 0d 75 0f 3d 01        	movaps	0x13d0f75(%rip), %xmm1  # 0x14292e980
14155da0b: 0f 54 f9                    	andps	%xmm1, %xmm7
14155da0e: 0f 58 ff                    	addps	%xmm7, %xmm7
14155da11: 0f 28 15 e8 ff 70 01        	movaps	0x170ffe8(%rip), %xmm2  # 0x142c6da00
14155da18: 0f 28 da                    	movaps	%xmm2, %xmm3
14155da1b: 0f 5f df                    	maxps	%xmm7, %xmm3
14155da1e: 0f c2 ff 03                 	cmpunordps	%xmm7, %xmm7
14155da22: 0f 54 d7                    	andps	%xmm7, %xmm2
14155da25: 0f 55 fb                    	andnps	%xmm3, %xmm7
14155da28: 0f 56 fa                    	orps	%xmm2, %xmm7
14155da2b: 0f 28 15 de ff 70 01        	movaps	0x170ffde(%rip), %xmm2  # 0x142c6da10
14155da32: 0f 28 da                    	movaps	%xmm2, %xmm3
14155da35: 0f 5d df                    	minps	%xmm7, %xmm3
14155da38: 0f c2 ff 03                 	cmpunordps	%xmm7, %xmm7
14155da3c: 0f 54 d7                    	andps	%xmm7, %xmm2
14155da3f: 0f 55 fb                    	andnps	%xmm3, %xmm7
14155da42: 0f 56 fa                    	orps	%xmm2, %xmm7
14155da45: f3 0f 11 7d 20              	movss	%xmm7, 0x20(%rbp)
14155da4a: 66 0f 70 d7 f5              	pshufd	$0xf5, %xmm7, %xmm2     # xmm2 = xmm7[1,1,3,3]
14155da4f: 0f 5c f8                    	subps	%xmm0, %xmm7
14155da52: 0f 54 f9                    	andps	%xmm1, %xmm7
14155da55: 48 8b 8c 24 88 00 00 00     	movq	0x88(%rsp), %rcx
14155da5d: 8b 54 24 5c                 	movl	0x5c(%rsp), %edx
14155da61: 89 11                       	movl	%edx, (%rcx)
14155da63: 66 0f 7e 55 24              	movd	%xmm2, 0x24(%rbp)
14155da68: 44 0f b6 40 0c              	movzbl	0xc(%rax), %r8d
14155da6d: 41 80 f0 01                 	xorb	$0x1, %r8b
14155da71: 4d 8b 37                    	movq	(%r15), %r14
14155da74: 48 8b 45 20                 	movq	0x20(%rbp), %rax
14155da78: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
14155da7d: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14155da81: 0f 29 44 24 30              	movaps	%xmm0, 0x30(%rsp)
14155da86: 41 80 e0 01                 	andb	$0x1, %r8b
14155da8a: 4c 8d bc 24 b0 00 00 00     	leaq	0xb0(%rsp), %r15
14155da92: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
14155da97: 4c 89 f9                    	movq	%r15, %rcx
14155da9a: e8 c1 09 00 00              	callq	0x14155e460 <.text+0x155d460>
14155da9f: 4c 89 f1                    	movq	%r14, %rcx
14155daa2: 4c 89 fa                    	movq	%r15, %rdx
14155daa5: e8 d6 ed f0 ff              	callq	0x14146c880 <.text+0x146b880>
14155daaa: e9 ab 00 00 00              	jmp	0x14155db5a <.text+0x155cb5a>
14155daaf: f3 0f 10 46 18              	movss	0x18(%rsi), %xmm0
14155dab4: f3 0f 10 4e 1c              	movss	0x1c(%rsi), %xmm1
14155dab9: 0f 28 15 30 7e 3c 01        	movaps	0x13c7e30(%rip), %xmm2  # 0x1429258f0
14155dac0: 0f 57 d1                    	xorps	%xmm1, %xmm2
14155dac3: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
14155dac7: 0f 14 d0                    	unpcklps	%xmm0, %xmm2            # xmm2 = xmm2[0],xmm0[0],xmm2[1],xmm0[1]
14155daca: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
14155dace: f3 0f 58 c1                 	addss	%xmm1, %xmm0
14155dad2: f3 0f 51 c0                 	sqrtss	%xmm0, %xmm0
14155dad6: f3 0f 10 0d 26 7e 3c 01     	movss	0x13c7e26(%rip), %xmm1  # 0x142925904
14155dade: f3 0f 5e c8                 	divss	%xmm0, %xmm1
14155dae2: f3 0f 12 c1                 	movsldup	%xmm1, %xmm0            # xmm0 = xmm1[0,0,2,2]
14155dae6: 0f 59 c2                    	mulps	%xmm2, %xmm0
14155dae9: 48 8b 84 24 88 00 00 00     	movq	0x88(%rsp), %rax
14155daf1: 8b 4c 24 5c                 	movl	0x5c(%rsp), %ecx
14155daf5: 89 08                       	movl	%ecx, (%rax)
14155daf7: 0f 13 45 10                 	movlps	%xmm0, 0x10(%rbp)
14155dafb: f3 0f 10 46 30              	movss	0x30(%rsi), %xmm0
14155db00: f3 0f 10 4e 38              	movss	0x38(%rsi), %xmm1
14155db05: 89 08                       	movl	%ecx, (%rax)
14155db07: f3 0f 11 45 18              	movss	%xmm0, 0x18(%rbp)
14155db0c: f3 0f 11 4d 1c              	movss	%xmm1, 0x1c(%rbp)
14155db11: f3 0f 10 46 34              	movss	0x34(%rsi), %xmm0
14155db16: 89 08                       	movl	%ecx, (%rax)
14155db18: f3 0f 11 45 28              	movss	%xmm0, 0x28(%rbp)
14155db1d: 4d 8b 37                    	movq	(%r15), %r14
14155db20: 48 8b 45 20                 	movq	0x20(%rbp), %rax
14155db24: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
14155db29: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14155db2d: 0f 29 44 24 30              	movaps	%xmm0, 0x30(%rsp)
14155db32: 4c 8d bc 24 b0 00 00 00     	leaq	0xb0(%rsp), %r15
14155db3a: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
14155db3f: b3 01                       	movb	$0x1, %bl
14155db41: 4c 89 f9                    	movq	%r15, %rcx
14155db44: 41 b0 01                    	movb	$0x1, %r8b
14155db47: e8 14 09 00 00              	callq	0x14155e460 <.text+0x155d460>
14155db4c: 4c 89 f1                    	movq	%r14, %rcx
14155db4f: 4c 89 fa                    	movq	%r15, %rdx
14155db52: e8 29 ed f0 ff              	callq	0x14146c880 <.text+0x146b880>
14155db57: 0f 57 ff                    	xorps	%xmm7, %xmm7
14155db5a: 48 8b 45 20                 	movq	0x20(%rbp), %rax
14155db5e: 48 89 84 24 c4 00 00 00     	movq	%rax, 0xc4(%rsp)
14155db66: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14155db6a: 0f 11 84 24 b4 00 00 00     	movups	%xmm0, 0xb4(%rsp)
14155db72: c7 84 24 b0 00 00 00 01 00 00 00    	movl	$0x1, 0xb0(%rsp)
14155db7d: 48 8d 94 24 b0 00 00 00     	leaq	0xb0(%rsp), %rdx
14155db85: 48 89 f9                    	movq	%rdi, %rcx
14155db88: e8 e3 4d 57 ff              	callq	0x140ad2970 <.text+0xad1970>
14155db8d: 84 c0                       	testb	%al, %al
14155db8f: 75 5a                       	jne	0x14155dbeb <.text+0x155cbeb>
14155db91: 48 8b 84 24 68 01 00 00     	movq	0x168(%rsp), %rax
14155db99: 8b 8c 24 9c 00 00 00        	movl	0x9c(%rsp), %ecx
14155dba0: 89 08                       	movl	%ecx, (%rax)
14155dba2: 48 8b 8c 24 90 00 00 00     	movq	0x90(%rsp), %rcx
14155dbaa: 48 8b 79 30                 	movq	0x30(%rcx), %rdi
14155dbae: 4c 8b 71 40                 	movq	0x40(%rcx), %r14
14155dbb2: 48 3b 79 20                 	cmpq	0x20(%rcx), %rdi
14155dbb6: 75 20                       	jne	0x14155dbd8 <.text+0x155cbd8>
14155dbb8: 48 8b 84 24 90 00 00 00     	movq	0x90(%rsp), %rax
14155dbc0: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14155dbc4: 48 8d 15 7d c5 6d 01        	leaq	0x16dc57d(%rip), %rdx   # 0x142c3a148
14155dbcb: e8 a0 4c 00 ff              	callq	0x140562870 <.text+0x561870>
14155dbd0: 48 8b 8c 24 90 00 00 00     	movq	0x90(%rsp), %rcx
14155dbd8: 48 8b 41 28                 	movq	0x28(%rcx), %rax
14155dbdc: 4c 89 34 f8                 	movq	%r14, (%rax,%rdi,8)
14155dbe0: 48 ff c7                    	incq	%rdi
14155dbe3: 48 89 79 30                 	movq	%rdi, 0x30(%rcx)
14155dbe7: 48 ff 41 40                 	incq	0x40(%rcx)
14155dbeb: 4c 8d 7d 10                 	leaq	0x10(%rbp), %r15
14155dbef: 84 db                       	testb	%bl, %bl
14155dbf1: 0f 84 31 01 00 00           	je	0x14155dd28 <.text+0x155cd28>
14155dbf7: 4c 89 6c 24 50              	movq	%r13, 0x50(%rsp)
14155dbfc: 4d 89 e5                    	movq	%r12, %r13
14155dbff: 41 0f 10 07                 	movups	(%r15), %xmm0
14155dc03: 0f 29 84 24 80 01 00 00     	movaps	%xmm0, 0x180(%rsp)
14155dc0b: f2 0f 10 75 20              	movsd	0x20(%rbp), %xmm6
14155dc10: 0f 58 35 b9 15 48 01        	addps	0x14815b9(%rip), %xmm6  # 0x1429df1d0
14155dc17: 8b 46 2c                    	movl	0x2c(%rsi), %eax
14155dc1a: 31 db                       	xorl	%ebx, %ebx
14155dc1c: 3c 02                       	cmpb	$0x2, %al
14155dc1e: 0f 45 d8                    	cmovnel	%eax, %ebx
14155dc21: 4d 89 fe                    	movq	%r15, %r14
14155dc24: 41 0f 10 07                 	movups	(%r15), %xmm0
14155dc28: 0f 29 84 24 b0 00 00 00     	movaps	%xmm0, 0xb0(%rsp)
14155dc30: 48 8b 94 24 70 02 00 00     	movq	0x270(%rsp), %rdx
14155dc38: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14155dc3b: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14155dc3f: 89 01                       	movl	%eax, (%rcx)
14155dc41: 48 8b 3a                    	movq	(%rdx), %rdi
14155dc44: 4c 8b 7f 30                 	movq	0x30(%rdi), %r15
14155dc48: 4c 8b 67 40                 	movq	0x40(%rdi), %r12
14155dc4c: 4c 3b 7f 20                 	cmpq	0x20(%rdi), %r15
14155dc50: 75 10                       	jne	0x14155dc62 <.text+0x155cc62>
14155dc52: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
14155dc56: 48 8d 15 eb c4 6d 01        	leaq	0x16dc4eb(%rip), %rdx   # 0x142c3a148
14155dc5d: e8 4e 4e 00 ff              	callq	0x140562ab0 <.text+0x561ab0>
14155dc62: 48 8b 47 28                 	movq	0x28(%rdi), %rax
14155dc66: 4b 8d 0c bf                 	leaq	(%r15,%r15,4), %rcx
14155dc6a: c7 04 c8 01 00 00 00        	movl	$0x1, (%rax,%rcx,8)
14155dc71: 0f 28 84 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm0
14155dc79: 0f 11 44 c8 04              	movups	%xmm0, 0x4(%rax,%rcx,8)
14155dc7e: 0f 13 74 c8 14              	movlps	%xmm6, 0x14(%rax,%rcx,8)
14155dc83: 88 5c c8 1c                 	movb	%bl, 0x1c(%rax,%rcx,8)
14155dc87: 4c 89 64 c8 20              	movq	%r12, 0x20(%rax,%rcx,8)
14155dc8c: 49 ff c7                    	incq	%r15
14155dc8f: 4c 89 7f 30                 	movq	%r15, 0x30(%rdi)
14155dc93: 48 ff 47 40                 	incq	0x40(%rdi)
14155dc97: f3 44 0f 10 46 3c           	movss	0x3c(%rsi), %xmm8
14155dc9d: 48 8b 84 24 70 01 00 00     	movq	0x170(%rsp), %rax
14155dca5: 8b 8c 24 a0 00 00 00        	movl	0xa0(%rsp), %ecx
14155dcac: 89 08                       	movl	%ecx, (%rax)
14155dcae: 4c 8b bc 24 78 01 00 00     	movq	0x178(%rsp), %r15
14155dcb6: 49 8b 7f 30                 	movq	0x30(%r15), %rdi
14155dcba: 49 8b 5f 40                 	movq	0x40(%r15), %rbx
14155dcbe: 49 3b 7f 20                 	cmpq	0x20(%r15), %rdi
14155dcc2: 75 10                       	jne	0x14155dcd4 <.text+0x155ccd4>
14155dcc4: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
14155dcc8: 48 8d 15 79 c4 6d 01        	leaq	0x16dc479(%rip), %rdx   # 0x142c3a148
14155dccf: e8 1c 4d 00 ff              	callq	0x1405629f0 <.text+0x5619f0>
14155dcd4: 49 8b 47 28                 	movq	0x28(%r15), %rax
14155dcd8: 48 8d 0c 7f                 	leaq	(%rdi,%rdi,2), %rcx
14155dcdc: 48 c1 e1 04                 	shlq	$0x4, %rcx
14155dce0: c7 04 08 01 00 00 00        	movl	$0x1, (%rax,%rcx)
14155dce7: 0f 28 84 24 80 01 00 00     	movaps	0x180(%rsp), %xmm0
14155dcef: 0f 11 44 08 04              	movups	%xmm0, 0x4(%rax,%rcx)
14155dcf4: 0f 13 74 08 14              	movlps	%xmm6, 0x14(%rax,%rcx)
14155dcf9: f3 44 0f 11 44 08 1c        	movss	%xmm8, 0x1c(%rax,%rcx)
14155dd00: c7 44 08 20 00 00 80 40     	movl	$0x40800000, 0x20(%rax,%rcx) # imm = 0x40800000
14155dd08: c6 44 08 24 00              	movb	$0x0, 0x24(%rax,%rcx)
14155dd0d: 48 89 5c 08 28              	movq	%rbx, 0x28(%rax,%rcx)
14155dd12: 48 ff c7                    	incq	%rdi
14155dd15: 49 89 7f 30                 	movq	%rdi, 0x30(%r15)
14155dd19: 49 ff 47 40                 	incq	0x40(%r15)
14155dd1d: 4d 89 ec                    	movq	%r13, %r12
14155dd20: 4c 8b 6c 24 50              	movq	0x50(%rsp), %r13
14155dd25: 4d 89 f7                    	movq	%r14, %r15
14155dd28: 48 8b 84 24 68 02 00 00     	movq	0x268(%rsp), %rax
14155dd30: 48 8b 10                    	movq	(%rax), %rdx
14155dd33: 48 8b 48 08                 	movq	0x8(%rax), %rcx
14155dd37: 4c 8b 02                    	movq	(%rdx), %r8
14155dd3a: 31 c0                       	xorl	%eax, %eax
14155dd3c: 4d 89 c1                    	movq	%r8, %r9
14155dd3f: 4c 2b 49 18                 	subq	0x18(%rcx), %r9
14155dd43: 4c 0f 42 c8                 	cmovbq	%rax, %r9
14155dd47: 4c 2b 41 38                 	subq	0x38(%rcx), %r8
14155dd4b: 4c 0f 42 c0                 	cmovbq	%rax, %r8
14155dd4f: 4c 8b 51 30                 	movq	0x30(%rcx), %r10
14155dd53: 4d 29 c2                    	subq	%r8, %r10
14155dd56: 4c 0f 42 d0                 	cmovbq	%rax, %r10
14155dd5a: 41 0f 96 c0                 	setbe	%r8b
14155dd5e: 4c 8b 59 10                 	movq	0x10(%rcx), %r11
14155dd62: 4d 29 cb                    	subq	%r9, %r11
14155dd65: 4c 0f 42 d8                 	cmovbq	%rax, %r11
14155dd69: 4c 8b 49 40                 	movq	0x40(%rcx), %r9
14155dd6d: 0f 96 c1                    	setbe	%cl
14155dd70: 4d 01 d3                    	addq	%r10, %r11
14155dd73: 44 20 c1                    	andb	%r8b, %cl
14155dd76: 4c 0f 44 d8                 	cmoveq	%rax, %r11
14155dd7a: 4d 29 d9                    	subq	%r11, %r9
14155dd7d: 4c 89 0a                    	movq	%r9, (%rdx)
14155dd80: 48 8b 54 24 78              	movq	0x78(%rsp), %rdx
14155dd85: 80 7a 18 02                 	cmpb	$0x2, 0x18(%rdx)
14155dd89: 0f 82 80 00 00 00           	jb	0x14155de0f <.text+0x155ce0f>
14155dd8f: 48 8b 44 24 70              	movq	0x70(%rsp), %rax
14155dd94: 48 39 42 38                 	cmpq	%rax, 0x38(%rdx)
14155dd98: 75 73                       	jne	0x14155de0d <.text+0x155ce0d>
14155dd9a: 0f b6 42 19                 	movzbl	0x19(%rdx), %eax
14155dd9e: 04 fe                       	addb	$-0x2, %al
14155dda0: eb 6d                       	jmp	0x14155de0f <.text+0x155ce0f>
14155dda2: 49 8b 03                    	movq	(%r11), %rax
14155dda5: 0f 57 ff                    	xorps	%xmm7, %xmm7
14155dda8: b3 01                       	movb	$0x1, %bl
14155ddaa: 80 78 0c 00                 	cmpb	$0x0, 0xc(%rax)
14155ddae: 0f 85 a6 fd ff ff           	jne	0x14155db5a <.text+0x155cb5a>
14155ddb4: 48 8b 45 20                 	movq	0x20(%rbp), %rax
14155ddb8: 48 89 84 24 c4 00 00 00     	movq	%rax, 0xc4(%rsp)
14155ddc0: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14155ddc4: 0f 11 84 24 b4 00 00 00     	movups	%xmm0, 0xb4(%rsp)
14155ddcc: c7 84 24 b0 00 00 00 01 00 00 00    	movl	$0x1, 0xb0(%rsp)
14155ddd7: 48 8d 94 24 b0 00 00 00     	leaq	0xb0(%rsp), %rdx
14155dddf: 48 89 f9                    	movq	%rdi, %rcx
14155dde2: e8 89 4b 57 ff              	callq	0x140ad2970 <.text+0xad1970>
14155dde7: 84 c0                       	testb	%al, %al
14155dde9: 0f 84 01 03 00 00           	je	0x14155e0f0 <.text+0x155d0f0>
14155ddef: 48 8b 84 24 30 02 00 00     	movq	0x230(%rsp), %rax
14155ddf7: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
14155ddfb: 80 7e 2c 02                 	cmpb	$0x2, 0x2c(%rsi)
14155ddff: 0f 85 bf 04 00 00           	jne	0x14155e2c4 <.text+0x155d2c4>
14155de05: 45 31 c9                    	xorl	%r9d, %r9d
14155de08: e9 c1 04 00 00              	jmp	0x14155e2ce <.text+0x155d2ce>
14155de0d: 31 c0                       	xorl	%eax, %eax
14155de0f: 0f b6 c0                    	movzbl	%al, %eax
14155de12: 3c 03                       	cmpb	$0x3, %al
14155de14: ba 03 00 00 00              	movl	$0x3, %edx
14155de19: 0f 42 d0                    	cmovbl	%eax, %edx
14155de1c: 0f b6 c2                    	movzbl	%dl, %eax
14155de1f: 48 8d 15 3e 0a 71 01        	leaq	0x1710a3e(%rip), %rdx   # 0x142c6e864
14155de26: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14155de2a: 48 01 d0                    	addq	%rdx, %rax
14155de2d: ff e0                       	jmpq	*%rax
14155de2f: 84 c9                       	testb	%cl, %cl
14155de31: 0f 85 60 04 00 00           	jne	0x14155e297 <.text+0x155d297>
14155de37: 48 89 f0                    	movq	%rsi, %rax
14155de3a: 48 8b b4 24 78 02 00 00     	movq	0x278(%rsp), %rsi
14155de42: 48 8d 55 18                 	leaq	0x18(%rbp), %rdx
14155de46: 48 89 c7                    	movq	%rax, %rdi
14155de49: f3 0f 10 50 3c              	movss	0x3c(%rax), %xmm2
14155de4e: 48 8d 8c 24 a4 00 00 00     	leaq	0xa4(%rsp), %rcx
14155de56: e8 85 ef 73 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
14155de5b: f3 0f 10 94 24 a4 00 00 00  	movss	0xa4(%rsp), %xmm2
14155de64: f3 0f 10 84 24 a8 00 00 00  	movss	0xa8(%rsp), %xmm0
14155de6d: f3 0f 10 8c 24 ac 00 00 00  	movss	0xac(%rsp), %xmm1
14155de76: f6 45 00 01                 	testb	$0x1, (%rbp)
14155de7a: 0f 85 ce 02 00 00           	jne	0x14155e14e <.text+0x155d14e>
14155de80: 0f 28 da                    	movaps	%xmm2, %xmm3
14155de83: 0f 28 e0                    	movaps	%xmm0, %xmm4
14155de86: 0f 28 e9                    	movaps	%xmm1, %xmm5
14155de89: e9 cf 02 00 00              	jmp	0x14155e15d <.text+0x155d15d>
14155de8e: 8b 46 38                    	movl	0x38(%rsi), %eax
14155de91: 89 44 24 3f                 	movl	%eax, 0x3f(%rsp)
14155de95: 48 8b 46 30                 	movq	0x30(%rsi), %rax
14155de99: 48 89 44 24 37              	movq	%rax, 0x37(%rsp)
14155de9e: 48 8d 8c 24 b4 00 00 00     	leaq	0xb4(%rsp), %rcx
14155dea6: f3 41 0f 6f 07              	movdqu	(%r15), %xmm0
14155deab: 49 8b 47 10                 	movq	0x10(%r15), %rax
14155deaf: 48 89 84 24 c4 00 00 00     	movq	%rax, 0xc4(%rsp)
14155deb7: f3 0f 7f 84 24 b4 00 00 00  	movdqu	%xmm0, 0xb4(%rsp)
14155dec0: c7 84 24 b0 00 00 00 01 00 00 00    	movl	$0x1, 0xb0(%rsp)
14155decb: e8 50 b6 73 ff              	callq	0x140c99520 <.text+0xc98520>
14155ded0: 66 0f 6f d0                 	movdqa	%xmm0, %xmm2
14155ded4: f3 0f 10 05 14 fb 70 01     	movss	0x170fb14(%rip), %xmm0  # 0x142c6d9f0
14155dedc: f3 0f 10 0d 10 fb 70 01     	movss	0x170fb10(%rip), %xmm1  # 0x142c6d9f4
14155dee4: e8 b7 25 73 ff              	callq	0x140c904a0 <.text+0xc8f4a0>
14155dee9: 0f 28 f0                    	movaps	%xmm0, %xmm6
14155deec: 8b 44 24 24                 	movl	0x24(%rsp), %eax
14155def0: 41 89 45 00                 	movl	%eax, (%r13)
14155def4: 49 8b 74 24 30              	movq	0x30(%r12), %rsi
14155def9: 49 8b 7c 24 40              	movq	0x40(%r12), %rdi
14155defe: 49 3b 74 24 20              	cmpq	0x20(%r12), %rsi
14155df03: 75 11                       	jne	0x14155df16 <.text+0x155cf16>
14155df05: 49 8d 4c 24 20              	leaq	0x20(%r12), %rcx
14155df0a: 48 8d 15 37 c2 6d 01        	leaq	0x16dc237(%rip), %rdx   # 0x142c3a148
14155df11: e8 1a 4d 00 ff              	callq	0x140562c30 <.text+0x561c30>
14155df16: 49 8b 44 24 28              	movq	0x28(%r12), %rax
14155df1b: 48 89 f1                    	movq	%rsi, %rcx
14155df1e: 48 c1 e1 05                 	shlq	$0x5, %rcx
14155df22: c6 04 08 02                 	movb	$0x2, (%rax,%rcx)
14155df26: 0f 10 44 24 30              	movups	0x30(%rsp), %xmm0
14155df2b: 0f 11 44 08 01              	movups	%xmm0, 0x1(%rax,%rcx)
14155df30: 8b 54 24 3f                 	movl	0x3f(%rsp), %edx
14155df34: e9 96 01 00 00              	jmp	0x14155e0cf <.text+0x155d0cf>
14155df39: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
14155df3d: 41 0f 2e f8                 	ucomiss	%xmm8, %xmm7
14155df41: 0f 86 ce 00 00 00           	jbe	0x14155e015 <.text+0x155d015>
14155df47: 8b 46 38                    	movl	0x38(%rsi), %eax
14155df4a: 89 44 24 38                 	movl	%eax, 0x38(%rsp)
14155df4e: 48 89 f3                    	movq	%rsi, %rbx
14155df51: 48 8b 46 30                 	movq	0x30(%rsi), %rax
14155df55: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14155df5a: 48 8d 8c 24 b4 00 00 00     	leaq	0xb4(%rsp), %rcx
14155df62: f3 41 0f 6f 07              	movdqu	(%r15), %xmm0
14155df67: 49 8b 47 10                 	movq	0x10(%r15), %rax
14155df6b: 48 89 84 24 c4 00 00 00     	movq	%rax, 0xc4(%rsp)
14155df73: f3 0f 7f 84 24 b4 00 00 00  	movdqu	%xmm0, 0xb4(%rsp)
14155df7c: c7 84 24 b0 00 00 00 01 00 00 00    	movl	$0x1, 0xb0(%rsp)
14155df87: e8 94 b5 73 ff              	callq	0x140c99520 <.text+0xc98520>
14155df8c: 66 0f 6f d0                 	movdqa	%xmm0, %xmm2
14155df90: f3 0f 10 05 58 fa 70 01     	movss	0x170fa58(%rip), %xmm0  # 0x142c6d9f0
14155df98: f3 0f 10 0d 54 fa 70 01     	movss	0x170fa54(%rip), %xmm1  # 0x142c6d9f4
14155dfa0: e8 fb 24 73 ff              	callq	0x140c904a0 <.text+0xc8f4a0>
14155dfa5: 0f 28 f0                    	movaps	%xmm0, %xmm6
14155dfa8: 8b 44 24 24                 	movl	0x24(%rsp), %eax
14155dfac: 41 89 45 00                 	movl	%eax, (%r13)
14155dfb0: 49 8b 74 24 30              	movq	0x30(%r12), %rsi
14155dfb5: 49 8b 7c 24 40              	movq	0x40(%r12), %rdi
14155dfba: 49 3b 74 24 20              	cmpq	0x20(%r12), %rsi
14155dfbf: 75 11                       	jne	0x14155dfd2 <.text+0x155cfd2>
14155dfc1: 49 8d 4c 24 20              	leaq	0x20(%r12), %rcx
14155dfc6: 48 8d 15 7b c1 6d 01        	leaq	0x16dc17b(%rip), %rdx   # 0x142c3a148
14155dfcd: e8 5e 4c 00 ff              	callq	0x140562c30 <.text+0x561c30>
14155dfd2: 49 8b 44 24 28              	movq	0x28(%r12), %rax
14155dfd7: 48 89 f1                    	movq	%rsi, %rcx
14155dfda: 48 c1 e1 05                 	shlq	$0x5, %rcx
14155dfde: c6 04 08 01                 	movb	$0x1, (%rax,%rcx)
14155dfe2: f3 0f 11 7c 08 04           	movss	%xmm7, 0x4(%rax,%rcx)
14155dfe8: 48 8b 54 24 30              	movq	0x30(%rsp), %rdx
14155dfed: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
14155dff2: 8b 54 24 38                 	movl	0x38(%rsp), %edx
14155dff6: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
14155dffa: f3 0f 11 74 08 14           	movss	%xmm6, 0x14(%rax,%rcx)
14155e000: 48 89 7c 08 18              	movq	%rdi, 0x18(%rax,%rcx)
14155e005: 48 ff c6                    	incq	%rsi
14155e008: 49 89 74 24 30              	movq	%rsi, 0x30(%r12)
14155e00d: 49 ff 44 24 40              	incq	0x40(%r12)
14155e012: 48 89 de                    	movq	%rbx, %rsi
14155e015: f3 0f 16 ff                 	movshdup	%xmm7, %xmm7            # xmm7 = xmm7[1,1,3,3]
14155e019: 41 0f 2e f8                 	ucomiss	%xmm8, %xmm7
14155e01d: 0f 86 74 02 00 00           	jbe	0x14155e297 <.text+0x155d297>
14155e023: 8b 46 38                    	movl	0x38(%rsi), %eax
14155e026: 89 44 24 38                 	movl	%eax, 0x38(%rsp)
14155e02a: 48 8b 46 30                 	movq	0x30(%rsi), %rax
14155e02e: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14155e033: 48 8d 8c 24 b4 00 00 00     	leaq	0xb4(%rsp), %rcx
14155e03b: f3 41 0f 6f 07              	movdqu	(%r15), %xmm0
14155e040: 49 8b 47 10                 	movq	0x10(%r15), %rax
14155e044: 48 89 84 24 c4 00 00 00     	movq	%rax, 0xc4(%rsp)
14155e04c: f3 0f 7f 84 24 b4 00 00 00  	movdqu	%xmm0, 0xb4(%rsp)
14155e055: c7 84 24 b0 00 00 00 01 00 00 00    	movl	$0x1, 0xb0(%rsp)
14155e060: e8 bb b4 73 ff              	callq	0x140c99520 <.text+0xc98520>
14155e065: 66 0f 6f d0                 	movdqa	%xmm0, %xmm2
14155e069: f3 0f 10 05 7f f9 70 01     	movss	0x170f97f(%rip), %xmm0  # 0x142c6d9f0
14155e071: f3 0f 10 0d 7b f9 70 01     	movss	0x170f97b(%rip), %xmm1  # 0x142c6d9f4
14155e079: e8 22 24 73 ff              	callq	0x140c904a0 <.text+0xc8f4a0>
14155e07e: 0f 28 f0                    	movaps	%xmm0, %xmm6
14155e081: 8b 44 24 24                 	movl	0x24(%rsp), %eax
14155e085: 41 89 45 00                 	movl	%eax, (%r13)
14155e089: 49 8b 74 24 30              	movq	0x30(%r12), %rsi
14155e08e: 49 8b 7c 24 40              	movq	0x40(%r12), %rdi
14155e093: 49 3b 74 24 20              	cmpq	0x20(%r12), %rsi
14155e098: 75 11                       	jne	0x14155e0ab <.text+0x155d0ab>
14155e09a: 49 8d 4c 24 20              	leaq	0x20(%r12), %rcx
14155e09f: 48 8d 15 a2 c0 6d 01        	leaq	0x16dc0a2(%rip), %rdx   # 0x142c3a148
14155e0a6: e8 85 4b 00 ff              	callq	0x140562c30 <.text+0x561c30>
14155e0ab: 49 8b 44 24 28              	movq	0x28(%r12), %rax
14155e0b0: 48 89 f1                    	movq	%rsi, %rcx
14155e0b3: 48 c1 e1 05                 	shlq	$0x5, %rcx
14155e0b7: c6 04 08 00                 	movb	$0x0, (%rax,%rcx)
14155e0bb: f3 0f 11 7c 08 04           	movss	%xmm7, 0x4(%rax,%rcx)
14155e0c1: 48 8b 54 24 30              	movq	0x30(%rsp), %rdx
14155e0c6: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
14155e0cb: 8b 54 24 38                 	movl	0x38(%rsp), %edx
14155e0cf: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
14155e0d3: f3 0f 11 74 08 14           	movss	%xmm6, 0x14(%rax,%rcx)
14155e0d9: 48 89 7c 08 18              	movq	%rdi, 0x18(%rax,%rcx)
14155e0de: 48 ff c6                    	incq	%rsi
14155e0e1: 49 89 74 24 30              	movq	%rsi, 0x30(%r12)
14155e0e6: 49 ff 44 24 40              	incq	0x40(%r12)
14155e0eb: e9 a7 01 00 00              	jmp	0x14155e297 <.text+0x155d297>
14155e0f0: 49 89 fe                    	movq	%rdi, %r14
14155e0f3: 48 8b 84 24 60 01 00 00     	movq	0x160(%rsp), %rax
14155e0fb: 8b 8c 24 98 00 00 00        	movl	0x98(%rsp), %ecx
14155e102: 89 08                       	movl	%ecx, (%rax)
14155e104: 48 8b 4c 24 68              	movq	0x68(%rsp), %rcx
14155e109: 48 8b 79 30                 	movq	0x30(%rcx), %rdi
14155e10d: 48 8b 59 40                 	movq	0x40(%rcx), %rbx
14155e111: 48 3b 79 20                 	cmpq	0x20(%rcx), %rdi
14155e115: 75 1a                       	jne	0x14155e131 <.text+0x155d131>
14155e117: 48 8b 44 24 68              	movq	0x68(%rsp), %rax
14155e11c: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14155e120: 48 8d 15 21 c0 6d 01        	leaq	0x16dc021(%rip), %rdx   # 0x142c3a148
14155e127: e8 44 47 00 ff              	callq	0x140562870 <.text+0x561870>
14155e12c: 48 8b 4c 24 68              	movq	0x68(%rsp), %rcx
14155e131: 48 8b 41 28                 	movq	0x28(%rcx), %rax
14155e135: 48 89 1c f8                 	movq	%rbx, (%rax,%rdi,8)
14155e139: 48 ff c7                    	incq	%rdi
14155e13c: 48 89 79 30                 	movq	%rdi, 0x30(%rcx)
14155e140: 48 ff 41 40                 	incq	0x40(%rcx)
14155e144: 31 db                       	xorl	%ebx, %ebx
14155e146: 4c 89 f7                    	movq	%r14, %rdi
14155e149: e9 0c fa ff ff              	jmp	0x14155db5a <.text+0x155cb5a>
14155e14e: f3 0f 10 5d 04              	movss	0x4(%rbp), %xmm3
14155e153: f3 0f 10 65 08              	movss	0x8(%rbp), %xmm4
14155e158: f3 0f 10 6d 0c              	movss	0xc(%rbp), %xmm5
14155e15d: f3 0f 5c d3                 	subss	%xmm3, %xmm2
14155e161: f3 0f 5c c4                 	subss	%xmm4, %xmm0
14155e165: f3 0f 5c cd                 	subss	%xmm5, %xmm1
14155e169: f3 0f 59 d2                 	mulss	%xmm2, %xmm2
14155e16d: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
14155e171: f3 0f 58 c2                 	addss	%xmm2, %xmm0
14155e175: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
14155e179: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14155e17d: f3 0f 51 f9                 	sqrtss	%xmm1, %xmm7
14155e181: 48 8b 06                    	movq	(%rsi), %rax
14155e184: f3 0f 5e 78 28              	divss	0x28(%rax), %xmm7
14155e189: 48 8d 75 04                 	leaq	0x4(%rbp), %rsi
14155e18d: 0f 57 c0                    	xorps	%xmm0, %xmm0
14155e190: 48 89 f9                    	movq	%rdi, %rcx
14155e193: f3 0f c2 47 3c 00           	cmpeqss	0x3c(%rdi), %xmm0
14155e199: 66 0f 7e c7                 	movd	%xmm0, %edi
14155e19d: 83 e7 01                    	andl	$0x1, %edi
14155e1a0: 48 8b 41 30                 	movq	0x30(%rcx), %rax
14155e1a4: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14155e1a9: 8b 41 38                    	movl	0x38(%rcx), %eax
14155e1ac: 89 44 24 38                 	movl	%eax, 0x38(%rsp)
14155e1b0: 48 8d 8c 24 b4 00 00 00     	leaq	0xb4(%rsp), %rcx
14155e1b8: f3 41 0f 6f 07              	movdqu	(%r15), %xmm0
14155e1bd: 49 8b 47 10                 	movq	0x10(%r15), %rax
14155e1c1: 48 89 84 24 c4 00 00 00     	movq	%rax, 0xc4(%rsp)
14155e1c9: f3 0f 7f 84 24 b4 00 00 00  	movdqu	%xmm0, 0xb4(%rsp)
14155e1d2: c7 84 24 b0 00 00 00 01 00 00 00    	movl	$0x1, 0xb0(%rsp)
14155e1dd: e8 3e b3 73 ff              	callq	0x140c99520 <.text+0xc98520>
14155e1e2: 66 0f 6f d0                 	movdqa	%xmm0, %xmm2
14155e1e6: f3 0f 10 05 02 f8 70 01     	movss	0x170f802(%rip), %xmm0  # 0x142c6d9f0
14155e1ee: f3 0f 10 0d fe f7 70 01     	movss	0x170f7fe(%rip), %xmm1  # 0x142c6d9f4
14155e1f6: e8 a5 22 73 ff              	callq	0x140c904a0 <.text+0xc8f4a0>
14155e1fb: 0f 28 f0                    	movaps	%xmm0, %xmm6
14155e1fe: 8b 44 24 24                 	movl	0x24(%rsp), %eax
14155e202: 41 89 45 00                 	movl	%eax, (%r13)
14155e206: 49 8b 5c 24 30              	movq	0x30(%r12), %rbx
14155e20b: 4d 8b 74 24 40              	movq	0x40(%r12), %r14
14155e210: 49 3b 5c 24 20              	cmpq	0x20(%r12), %rbx
14155e215: 75 11                       	jne	0x14155e228 <.text+0x155d228>
14155e217: 49 8d 4c 24 20              	leaq	0x20(%r12), %rcx
14155e21c: 48 8d 15 25 bf 6d 01        	leaq	0x16dbf25(%rip), %rdx   # 0x142c3a148
14155e223: e8 08 4a 00 ff              	callq	0x140562c30 <.text+0x561c30>
14155e228: 49 8b 44 24 28              	movq	0x28(%r12), %rax
14155e22d: 48 89 d9                    	movq	%rbx, %rcx
14155e230: 48 c1 e1 05                 	shlq	$0x5, %rcx
14155e234: c6 04 08 03                 	movb	$0x3, (%rax,%rcx)
14155e238: 40 88 7c 08 01              	movb	%dil, 0x1(%rax,%rcx)
14155e23d: f3 0f 11 7c 08 04           	movss	%xmm7, 0x4(%rax,%rcx)
14155e243: 48 8b 54 24 30              	movq	0x30(%rsp), %rdx
14155e248: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
14155e24d: 8b 54 24 38                 	movl	0x38(%rsp), %edx
14155e251: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
14155e255: f3 0f 11 74 08 14           	movss	%xmm6, 0x14(%rax,%rcx)
14155e25b: 4c 89 74 08 18              	movq	%r14, 0x18(%rax,%rcx)
14155e260: 48 ff c3                    	incq	%rbx
14155e263: 49 89 5c 24 30              	movq	%rbx, 0x30(%r12)
14155e268: 49 ff 44 24 40              	incq	0x40(%r12)
14155e26d: 48 8b 84 24 88 00 00 00     	movq	0x88(%rsp), %rax
14155e275: 8b 4c 24 5c                 	movl	0x5c(%rsp), %ecx
14155e279: 89 08                       	movl	%ecx, (%rax)
14155e27b: c7 45 00 01 00 00 00        	movl	$0x1, (%rbp)
14155e282: 8b 84 24 ac 00 00 00        	movl	0xac(%rsp), %eax
14155e289: 89 46 08                    	movl	%eax, 0x8(%rsi)
14155e28c: 48 8b 84 24 a4 00 00 00     	movq	0xa4(%rsp), %rax
14155e294: 48 89 06                    	movq	%rax, (%rsi)
14155e297: 0f 28 b4 24 90 01 00 00     	movaps	0x190(%rsp), %xmm6
14155e29f: 0f 28 bc 24 a0 01 00 00     	movaps	0x1a0(%rsp), %xmm7
14155e2a7: 44 0f 28 84 24 b0 01 00 00  	movaps	0x1b0(%rsp), %xmm8
14155e2b0: 48 81 c4 c8 01 00 00        	addq	$0x1c8, %rsp            # imm = 0x1C8
14155e2b7: 5b                          	popq	%rbx
14155e2b8: 5d                          	popq	%rbp
14155e2b9: 5f                          	popq	%rdi
14155e2ba: 5e                          	popq	%rsi
14155e2bb: 41 5c                       	popq	%r12
14155e2bd: 41 5d                       	popq	%r13
14155e2bf: 41 5e                       	popq	%r14
14155e2c1: 41 5f                       	popq	%r15
14155e2c3: c3                          	retq
14155e2c4: 4c 8b 56 20                 	movq	0x20(%rsi), %r10
14155e2c8: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14155e2ce: f3 0f 10 76 40              	movss	0x40(%rsi), %xmm6
14155e2d3: f3 44 0f 10 46 3c           	movss	0x3c(%rsi), %xmm8
14155e2d9: 49 8b 48 10                 	movq	0x10(%r8), %rcx
14155e2dd: 48 89 8c 24 c0 00 00 00     	movq	%rcx, 0xc0(%rsp)
14155e2e5: 41 0f 10 00                 	movups	(%r8), %xmm0
14155e2e9: 0f 29 84 24 b0 00 00 00     	movaps	%xmm0, 0xb0(%rsp)
14155e2f1: 8b 48 1c                    	movl	0x1c(%rax), %ecx
14155e2f4: 48 8b 50 10                 	movq	0x10(%rax), %rdx
14155e2f8: 89 0a                       	movl	%ecx, (%rdx)
14155e2fa: 4c 8b 38                    	movq	(%rax), %r15
14155e2fd: 4d 8b 77 30                 	movq	0x30(%r15), %r14
14155e301: 4d 8b 5f 40                 	movq	0x40(%r15), %r11
14155e305: 4d 3b 77 20                 	cmpq	0x20(%r15), %r14
14155e309: 75 3e                       	jne	0x14155e349 <.text+0x155d349>
14155e30b: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
14155e30f: 48 8d 15 32 be 6d 01        	leaq	0x16dbe32(%rip), %rdx   # 0x142c3a148
14155e316: 4c 89 44 24 28              	movq	%r8, 0x28(%rsp)
14155e31b: 4c 89 4c 24 50              	movq	%r9, 0x50(%rsp)
14155e320: 4c 89 54 24 68              	movq	%r10, 0x68(%rsp)
14155e325: 4c 89 9c 24 80 00 00 00     	movq	%r11, 0x80(%rsp)
14155e32d: e8 9e 36 0d ff              	callq	0x1406319d0 <.text+0x6309d0>
14155e332: 4c 8b 9c 24 80 00 00 00     	movq	0x80(%rsp), %r11
14155e33a: 4c 8b 54 24 68              	movq	0x68(%rsp), %r10
14155e33f: 4c 8b 4c 24 50              	movq	0x50(%rsp), %r9
14155e344: 4c 8b 44 24 28              	movq	0x28(%rsp), %r8
14155e349: 49 8b 47 28                 	movq	0x28(%r15), %rax
14155e34d: 49 6b ce 38                 	imulq	$0x38, %r14, %rcx
14155e351: 4c 89 0c 08                 	movq	%r9, (%rax,%rcx)
14155e355: 4c 89 54 08 08              	movq	%r10, 0x8(%rax,%rcx)
14155e35a: 0f 28 84 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm0
14155e362: 0f 11 44 08 10              	movups	%xmm0, 0x10(%rax,%rcx)
14155e367: 48 8b 94 24 c0 00 00 00     	movq	0xc0(%rsp), %rdx
14155e36f: 48 89 54 08 20              	movq	%rdx, 0x20(%rax,%rcx)
14155e374: f3 0f 11 74 08 28           	movss	%xmm6, 0x28(%rax,%rcx)
14155e37a: f3 44 0f 11 44 08 2c        	movss	%xmm8, 0x2c(%rax,%rcx)
14155e381: 4c 89 5c 08 30              	movq	%r11, 0x30(%rax,%rcx)
14155e386: 49 ff c6                    	incq	%r14
14155e389: 4d 89 77 30                 	movq	%r14, 0x30(%r15)
14155e38d: 49 ff 47 40                 	incq	0x40(%r15)
14155e391: 8b 46 38                    	movl	0x38(%rsi), %eax
14155e394: 89 44 24 3f                 	movl	%eax, 0x3f(%rsp)
14155e398: 48 8b 46 30                 	movq	0x30(%rsi), %rax
14155e39c: 48 89 44 24 37              	movq	%rax, 0x37(%rsp)
14155e3a1: 48 8d 8c 24 b4 00 00 00     	leaq	0xb4(%rsp), %rcx
14155e3a9: f3 41 0f 6f 00              	movdqu	(%r8), %xmm0
14155e3ae: 49 8b 40 10                 	movq	0x10(%r8), %rax
14155e3b2: 48 89 84 24 c4 00 00 00     	movq	%rax, 0xc4(%rsp)
14155e3ba: f3 0f 7f 84 24 b4 00 00 00  	movdqu	%xmm0, 0xb4(%rsp)
14155e3c3: c7 84 24 b0 00 00 00 01 00 00 00    	movl	$0x1, 0xb0(%rsp)
14155e3ce: e8 4d b1 73 ff              	callq	0x140c99520 <.text+0xc98520>
14155e3d3: 66 0f 6f d0                 	movdqa	%xmm0, %xmm2
14155e3d7: f3 0f 10 05 11 f6 70 01     	movss	0x170f611(%rip), %xmm0  # 0x142c6d9f0
14155e3df: f3 0f 10 0d 0d f6 70 01     	movss	0x170f60d(%rip), %xmm1  # 0x142c6d9f4
14155e3e7: e8 b4 20 73 ff              	callq	0x140c904a0 <.text+0xc8f4a0>
14155e3ec: 0f 28 f0                    	movaps	%xmm0, %xmm6
14155e3ef: 8b 44 24 24                 	movl	0x24(%rsp), %eax
14155e3f3: 41 89 45 00                 	movl	%eax, (%r13)
14155e3f7: 4d 8b 74 24 30              	movq	0x30(%r12), %r14
14155e3fc: 4d 8b 7c 24 40              	movq	0x40(%r12), %r15
14155e401: 4d 3b 74 24 20              	cmpq	0x20(%r12), %r14
14155e406: 75 11                       	jne	0x14155e419 <.text+0x155d419>
14155e408: 49 8d 4c 24 20              	leaq	0x20(%r12), %rcx
14155e40d: 48 8d 15 34 bd 6d 01        	leaq	0x16dbd34(%rip), %rdx   # 0x142c3a148
14155e414: e8 17 48 00 ff              	callq	0x140562c30 <.text+0x561c30>
14155e419: 49 8b 44 24 28              	movq	0x28(%r12), %rax
14155e41e: 4c 89 f1                    	movq	%r14, %rcx
14155e421: 48 c1 e1 05                 	shlq	$0x5, %rcx
14155e425: c6 04 08 07                 	movb	$0x7, (%rax,%rcx)
14155e429: 0f 10 44 24 30              	movups	0x30(%rsp), %xmm0
14155e42e: 0f 11 44 08 01              	movups	%xmm0, 0x1(%rax,%rcx)
14155e433: 8b 54 24 3f                 	movl	0x3f(%rsp), %edx
14155e437: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
14155e43b: f3 0f 11 74 08 14           	movss	%xmm6, 0x14(%rax,%rcx)
14155e441: 4c 89 7c 08 18              	movq	%r15, 0x18(%rax,%rcx)
14155e446: 49 ff c6                    	incq	%r14
14155e449: 4d 89 74 24 30              	movq	%r14, 0x30(%r12)
14155e44e: 49 ff 44 24 40              	incq	0x40(%r12)
14155e453: e9 02 f7 ff ff              	jmp	0x14155db5a <.text+0x155cb5a>
14155e458: cc                          	int3
14155e459: cc                          	int3
14155e45a: cc                          	int3
14155e45b: cc                          	int3
14155e45c: cc                          	int3
14155e45d: cc                          	int3
14155e45e: cc                          	int3
14155e45f: cc                          	int3
