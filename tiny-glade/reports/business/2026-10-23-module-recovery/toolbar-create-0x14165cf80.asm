
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14165cf80: 55                          	pushq	%rbp
14165cf81: 41 57                       	pushq	%r15
14165cf83: 41 56                       	pushq	%r14
14165cf85: 41 55                       	pushq	%r13
14165cf87: 41 54                       	pushq	%r12
14165cf89: 56                          	pushq	%rsi
14165cf8a: 57                          	pushq	%rdi
14165cf8b: 53                          	pushq	%rbx
14165cf8c: 48 81 ec 88 01 00 00        	subq	$0x188, %rsp            # imm = 0x188
14165cf93: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14165cf9b: 0f 29 b5 f0 00 00 00        	movaps	%xmm6, 0xf0(%rbp)
14165cfa2: 48 c7 85 e8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xe8(%rbp)
14165cfad: 4d 89 ce                    	movq	%r9, %r14
14165cfb0: 4d 89 c7                    	movq	%r8, %r15
14165cfb3: 48 89 d7                    	movq	%rdx, %rdi
14165cfb6: 48 89 ce                    	movq	%rcx, %rsi
14165cfb9: 0f b6 05 49 7c fd 01        	movzbl	0x1fd7c49(%rip), %eax   # 0x143634c09
14165cfc0: b9 08 00 00 00              	movl	$0x8, %ecx
14165cfc5: ba 08 00 00 00              	movl	$0x8, %edx
14165cfca: e8 41 6c fb fe              	callq	0x140613c10 <.text+0x612c10>
14165cfcf: 48 85 c0                    	testq	%rax, %rax
14165cfd2: 0f 84 6a 03 00 00           	je	0x14165d342 <.text+0x165c342>
14165cfd8: 48 89 c3                    	movq	%rax, %rbx
14165cfdb: 0f b6 05 27 7c fd 01        	movzbl	0x1fd7c27(%rip), %eax   # 0x143634c09
14165cfe2: b9 10 00 00 00              	movl	$0x10, %ecx
14165cfe7: ba 04 00 00 00              	movl	$0x4, %edx
14165cfec: e8 1f 6c fb fe              	callq	0x140613c10 <.text+0x612c10>
14165cff1: 48 85 c0                    	testq	%rax, %rax
14165cff4: 48 89 9d e0 00 00 00        	movq	%rbx, 0xe0(%rbp)
14165cffb: 0f 84 50 03 00 00           	je	0x14165d351 <.text+0x165c351>
14165d001: 4c 89 bd d8 00 00 00        	movq	%r15, 0xd8(%rbp)
14165d008: 4c 89 b5 d0 00 00 00        	movq	%r14, 0xd0(%rbp)
14165d00f: 0f 10 07                    	movups	(%rdi), %xmm0
14165d012: 0f 11 00                    	movups	%xmm0, (%rax)
14165d015: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
14165d019: 48 8d 05 50 52 62 01        	leaq	0x1625250(%rip), %rax   # 0x142c82270
14165d020: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
14165d024: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
14165d02e: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
14165d032: 0f b6 05 d0 7b fd 01        	movzbl	0x1fd7bd0(%rip), %eax   # 0x143634c09
14165d039: b9 13 00 00 00              	movl	$0x13, %ecx
14165d03e: ba 01 00 00 00              	movl	$0x1, %edx
14165d043: e8 c8 6b fb fe              	callq	0x140613c10 <.text+0x612c10>
14165d048: 48 85 c0                    	testq	%rax, %rax
14165d04b: 0f 84 11 03 00 00           	je	0x14165d362 <.text+0x165c362>
14165d051: 49 89 c7                    	movq	%rax, %r15
14165d054: 8b 05 41 c4 43 01           	movl	0x143c441(%rip), %eax   # 0x142a9949b
14165d05a: 41 89 47 0f                 	movl	%eax, 0xf(%r15)
14165d05e: 0f 10 05 27 c4 43 01        	movups	0x143c427(%rip), %xmm0  # 0x142a9948c
14165d065: 41 0f 11 07                 	movups	%xmm0, (%r15)
14165d069: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
14165d06d: 0f 10 4d c0                 	movups	-0x40(%rbp), %xmm1
14165d071: 0f 10 55 d0                 	movups	-0x30(%rbp), %xmm2
14165d075: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
14165d079: 0f 29 4d 60                 	movaps	%xmm1, 0x60(%rbp)
14165d07d: 0f 29 55 70                 	movaps	%xmm2, 0x70(%rbp)
14165d081: 48 c7 85 bc 00 00 00 00 00 00 00    	movq	$0x0, 0xbc(%rbp)
14165d08c: 48 c7 85 c1 00 00 00 00 00 00 00    	movq	$0x0, 0xc1(%rbp)
14165d097: 0f 28 05 22 10 2e 01        	movaps	0x12e1022(%rip), %xmm0  # 0x14293e0c0
14165d09e: 0f 29 45 40                 	movaps	%xmm0, 0x40(%rbp)
14165d0a2: 66 c7 85 80 00 00 00 00 01  	movw	$0x100, 0x80(%rbp)      # imm = 0x100
14165d0ab: 48 b8 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rax # imm = 0x13F800000
14165d0b5: 48 89 85 84 00 00 00        	movq	%rax, 0x84(%rbp)
14165d0bc: c7 85 90 00 00 00 00 00 00 00       	movl	$0x0, 0x90(%rbp)
14165d0c6: c7 85 98 00 00 00 00 00 00 00       	movl	$0x0, 0x98(%rbp)
14165d0d0: 48 c7 85 a0 00 00 00 13 00 00 00    	movq	$0x13, 0xa0(%rbp)
14165d0db: 4c 89 bd a8 00 00 00        	movq	%r15, 0xa8(%rbp)
14165d0e2: 48 c7 85 b0 00 00 00 13 00 00 00    	movq	$0x13, 0xb0(%rbp)
14165d0ed: c7 85 b8 00 00 00 00 00 00 00       	movl	$0x0, 0xb8(%rbp)
14165d0f7: 66 c7 85 c9 00 00 00 00 00  	movw	$0x0, 0xc9(%rbp)
14165d100: 44 0f b6 77 10              	movzbl	0x10(%rdi), %r14d
14165d105: 44 89 f0                    	movl	%r14d, %eax
14165d108: 83 f0 01                    	xorl	$0x1, %eax
14165d10b: 4c 8d 24 c5 0b 00 00 00     	leaq	0xb(,%rax,8), %r12
14165d113: 0f b6 05 ef 7a fd 01        	movzbl	0x1fd7aef(%rip), %eax   # 0x143634c09
14165d11a: ba 01 00 00 00              	movl	$0x1, %edx
14165d11f: 4c 89 e1                    	movq	%r12, %rcx
14165d122: e8 e9 6a fb fe              	callq	0x140613c10 <.text+0x612c10>
14165d127: 48 85 c0                    	testq	%rax, %rax
14165d12a: 0f 84 4a 02 00 00           	je	0x14165d37a <.text+0x165c37a>
14165d130: 49 89 c5                    	movq	%rax, %r13
14165d133: 48 8b 9d 70 01 00 00        	movq	0x170(%rbp), %rbx
14165d13a: 48 8d 05 72 51 62 01        	leaq	0x1625172(%rip), %rax   # 0x142c822b3
14165d141: 48 8d 15 58 51 62 01        	leaq	0x1625158(%rip), %rdx   # 0x142c822a0
14165d148: 45 84 f6                    	testb	%r14b, %r14b
14165d14b: 48 0f 45 d0                 	cmovneq	%rax, %rdx
14165d14f: 4c 89 e9                    	movq	%r13, %rcx
14165d152: 4d 89 e0                    	movq	%r12, %r8
14165d155: e8 a6 6a 2c 01              	callq	0x142923c00 <.text+0x2922c00>
14165d15a: ba 13 00 00 00              	movl	$0x13, %edx
14165d15f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14165d165: 4c 89 f9                    	movq	%r15, %rcx
14165d168: e8 b3 6a fb fe              	callq	0x140613c20 <.text+0x612c20>
14165d16d: 4c 89 a5 a0 00 00 00        	movq	%r12, 0xa0(%rbp)
14165d174: 4c 89 ad a8 00 00 00        	movq	%r13, 0xa8(%rbp)
14165d17b: 4c 89 a5 b0 00 00 00        	movq	%r12, 0xb0(%rbp)
14165d182: 4c 8d 7d b0                 	leaq	-0x50(%rbp), %r15
14165d186: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
14165d18a: 41 b8 90 00 00 00           	movl	$0x90, %r8d
14165d190: 4c 89 f9                    	movq	%r15, %rcx
14165d193: e8 68 6a 2c 01              	callq	0x142923c00 <.text+0x2922c00>
14165d198: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
14165d19d: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
14165d1a1: 4c 89 fa                    	movq	%r15, %rdx
14165d1a4: 4c 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %r8
14165d1ab: 4c 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %r9
14165d1b2: e8 99 0b 24 ff              	callq	0x14089dd50 <.text+0x89cd50>
14165d1b7: 48 8b 5d 48                 	movq	0x48(%rbp), %rbx
14165d1bb: 48 85 db                    	testq	%rbx, %rbx
14165d1be: 75 04                       	jne	0x14165d1c4 <.text+0x165c1c4>
14165d1c0: 48 8b 5d 50                 	movq	0x50(%rbp), %rbx
14165d1c4: 4c 8b 7d 60                 	movq	0x60(%rbp), %r15
14165d1c8: 48 8b 03                    	movq	(%rbx), %rax
14165d1cb: 4c 8b 73 10                 	movq	0x10(%rbx), %r14
14165d1cf: 4c 29 f0                    	subq	%r14, %rax
14165d1d2: 48 83 f8 27                 	cmpq	$0x27, %rax
14165d1d6: 0f 86 fa 00 00 00           	jbe	0x14165d2d6 <.text+0x165c2d6>
14165d1dc: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14165d1e0: 48 8d 0d 09 f7 c7 ff        	leaq	-0x3808f7(%rip), %rcx   # 0x1412dc8f0 <.text+0x12db8f0>
14165d1e7: 4a 89 0c 30                 	movq	%rcx, (%rax,%r14)
14165d1eb: 4e 89 7c 30 08              	movq	%r15, 0x8(%rax,%r14)
14165d1f0: 48 b9 01 00 00 00 00 00 40 40       	movabsq	$0x4040000000000001, %rcx # imm = 0x4040000000000001
14165d1fa: 4a 89 4c 30 10              	movq	%rcx, 0x10(%rax,%r14)
14165d1ff: 42 c6 44 30 18 00           	movb	$0x0, 0x18(%rax,%r14)
14165d205: 4c 8d 25 24 7b f0 ff        	leaq	-0xf84dc(%rip), %r12    # 0x141564d30 <.text+0x1563d30>
14165d20c: 4e 89 64 30 20              	movq	%r12, 0x20(%rax,%r14)
14165d211: 49 83 c6 28                 	addq	$0x28, %r14
14165d215: 4c 89 73 10                 	movq	%r14, 0x10(%rbx)
14165d219: 48 89 f9                    	movq	%rdi, %rcx
14165d21c: e8 4f a7 63 ff              	callq	0x140c97970 <.text+0xc96970>
14165d221: 0f 28 f0                    	movaps	%xmm0, %xmm6
14165d224: 48 8b 03                    	movq	(%rbx), %rax
14165d227: 48 8b 7b 10                 	movq	0x10(%rbx), %rdi
14165d22b: 48 29 f8                    	subq	%rdi, %rax
14165d22e: 48 83 f8 1f                 	cmpq	$0x1f, %rax
14165d232: 0f 86 c3 00 00 00           	jbe	0x14165d2fb <.text+0x165c2fb>
14165d238: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14165d23c: 48 8d 0d 1d ba c7 ff        	leaq	-0x3845e3(%rip), %rcx   # 0x1412d8c60 <.text+0x12d7c60>
14165d243: 48 89 0c 38                 	movq	%rcx, (%rax,%rdi)
14165d247: 4c 89 64 38 08              	movq	%r12, 0x8(%rax,%rdi)
14165d24c: 4c 89 7c 38 10              	movq	%r15, 0x10(%rax,%rdi)
14165d251: f3 0f 11 74 38 18           	movss	%xmm6, 0x18(%rax,%rdi)
14165d257: c6 44 38 1c 00              	movb	$0x0, 0x1c(%rax,%rdi)
14165d25c: 48 8d 57 20                 	leaq	0x20(%rdi), %rdx
14165d260: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14165d264: 48 8b 03                    	movq	(%rbx), %rax
14165d267: 48 29 d0                    	subq	%rdx, %rax
14165d26a: 48 83 f8 1f                 	cmpq	$0x1f, %rax
14165d26e: 0f 86 ac 00 00 00           	jbe	0x14165d320 <.text+0x165c320>
14165d274: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14165d278: 48 8d 0d a1 cf c7 ff        	leaq	-0x38305f(%rip), %rcx   # 0x1412da220 <.text+0x12d9220>
14165d27f: 48 89 4c 38 20              	movq	%rcx, 0x20(%rax,%rdi)
14165d284: 4c 89 64 38 28              	movq	%r12, 0x28(%rax,%rdi)
14165d289: 4c 89 7c 38 30              	movq	%r15, 0x30(%rax,%rdi)
14165d28e: c6 44 38 38 00              	movb	$0x0, 0x38(%rax,%rdi)
14165d293: 48 83 c7 40                 	addq	$0x40, %rdi
14165d297: 48 89 7b 10                 	movq	%rdi, 0x10(%rbx)
14165d29b: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
14165d2a2: 4c 89 38                    	movq	%r15, (%rax)
14165d2a5: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
14165d2ac: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14165d2b0: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
14165d2b8: 48 89 f0                    	movq	%rsi, %rax
14165d2bb: 0f 28 b5 f0 00 00 00        	movaps	0xf0(%rbp), %xmm6
14165d2c2: 48 81 c4 88 01 00 00        	addq	$0x188, %rsp            # imm = 0x188
14165d2c9: 5b                          	popq	%rbx
14165d2ca: 5f                          	popq	%rdi
14165d2cb: 5e                          	popq	%rsi
14165d2cc: 41 5c                       	popq	%r12
14165d2ce: 41 5d                       	popq	%r13
14165d2d0: 41 5e                       	popq	%r14
14165d2d2: 41 5f                       	popq	%r15
14165d2d4: 5d                          	popq	%rbp
14165d2d5: c3                          	retq
14165d2d6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14165d2df: 41 b8 28 00 00 00           	movl	$0x28, %r8d
14165d2e5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14165d2eb: 48 89 d9                    	movq	%rbx, %rcx
14165d2ee: 4c 89 f2                    	movq	%r14, %rdx
14165d2f1: e8 6a c7 24 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14165d2f6: e9 e1 fe ff ff              	jmp	0x14165d1dc <.text+0x165c1dc>
14165d2fb: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14165d304: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14165d30a: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14165d310: 48 89 d9                    	movq	%rbx, %rcx
14165d313: 48 89 fa                    	movq	%rdi, %rdx
14165d316: e8 45 c7 24 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14165d31b: e9 18 ff ff ff              	jmp	0x14165d238 <.text+0x165c238>
14165d320: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14165d329: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14165d32f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14165d335: 48 89 d9                    	movq	%rbx, %rcx
14165d338: e8 23 c7 24 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14165d33d: e9 32 ff ff ff              	jmp	0x14165d274 <.text+0x165c274>
14165d342: b9 08 00 00 00              	movl	$0x8, %ecx
14165d347: ba 08 00 00 00              	movl	$0x8, %edx
14165d34c: e8 af bc 27 01              	callq	0x1428d9000 <.text+0x28d8000>
14165d351: b9 04 00 00 00              	movl	$0x4, %ecx
14165d356: ba 10 00 00 00              	movl	$0x10, %edx
14165d35b: e8 a0 bc 27 01              	callq	0x1428d9000 <.text+0x28d8000>
14165d360: eb 2c                       	jmp	0x14165d38e <.text+0x165c38e>
14165d362: 4c 8d 05 b7 bb 43 01        	leaq	0x143bbb7(%rip), %r8    # 0x142a98f20
14165d369: b9 01 00 00 00              	movl	$0x1, %ecx
14165d36e: ba 13 00 00 00              	movl	$0x13, %edx
14165d373: e8 6b bc 27 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14165d378: eb 14                       	jmp	0x14165d38e <.text+0x165c38e>
14165d37a: 4c 8d 05 9f bb 43 01        	leaq	0x143bb9f(%rip), %r8    # 0x142a98f20
14165d381: b9 01 00 00 00              	movl	$0x1, %ecx
14165d386: 4c 89 e2                    	movq	%r12, %rdx
14165d389: e8 55 bc 27 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14165d38e: 0f 0b                       	ud2
14165d390: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14165d395: 55                          	pushq	%rbp
14165d396: 41 57                       	pushq	%r15
14165d398: 41 56                       	pushq	%r14
14165d39a: 41 55                       	pushq	%r13
14165d39c: 41 54                       	pushq	%r12
14165d39e: 56                          	pushq	%rsi
14165d39f: 57                          	pushq	%rdi
14165d3a0: 53                          	pushq	%rbx
14165d3a1: 48 83 ec 38                 	subq	$0x38, %rsp
14165d3a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14165d3ac: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14165d3b1: ba 08 00 00 00              	movl	$0x8, %edx
14165d3b6: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14165d3bc: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
14165d3c3: e8 58 68 fb fe              	callq	0x140613c20 <.text+0x612c20>
14165d3c8: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14165d3cd: 48 83 c4 38                 	addq	$0x38, %rsp
14165d3d1: 5b                          	popq	%rbx
14165d3d2: 5f                          	popq	%rdi
14165d3d3: 5e                          	popq	%rsi
14165d3d4: 41 5c                       	popq	%r12
14165d3d6: 41 5d                       	popq	%r13
14165d3d8: 41 5e                       	popq	%r14
14165d3da: 41 5f                       	popq	%r15
14165d3dc: 5d                          	popq	%rbp
14165d3dd: c3                          	retq
14165d3de: 66 90                       	nop
14165d3e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14165d3e5: 55                          	pushq	%rbp
14165d3e6: 41 57                       	pushq	%r15
14165d3e8: 41 56                       	pushq	%r14
14165d3ea: 41 55                       	pushq	%r13
14165d3ec: 41 54                       	pushq	%r12
14165d3ee: 56                          	pushq	%rsi
14165d3ef: 57                          	pushq	%rdi
14165d3f0: 53                          	pushq	%rbx
14165d3f1: 48 83 ec 38                 	subq	$0x38, %rsp
14165d3f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14165d3fc: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14165d401: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14165d405: e8 96 d9 23 ff              	callq	0x14089ada0 <.text+0x899da0>
14165d40a: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14165d40f: 48 83 c4 38                 	addq	$0x38, %rsp
14165d413: 5b                          	popq	%rbx
14165d414: 5f                          	popq	%rdi
14165d415: 5e                          	popq	%rsi
14165d416: 41 5c                       	popq	%r12
14165d418: 41 5d                       	popq	%r13
14165d41a: 41 5e                       	popq	%r14
14165d41c: 41 5f                       	popq	%r15
14165d41e: 5d                          	popq	%rbp
14165d41f: c3                          	retq
14165d420: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14165d425: 55                          	pushq	%rbp
14165d426: 41 57                       	pushq	%r15
14165d428: 41 56                       	pushq	%r14
14165d42a: 41 55                       	pushq	%r13
14165d42c: 41 54                       	pushq	%r12
14165d42e: 56                          	pushq	%rsi
14165d42f: 57                          	pushq	%rdi
14165d430: 53                          	pushq	%rbx
14165d431: 48 83 ec 38                 	subq	$0x38, %rsp
14165d435: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14165d43c: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14165d441: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
14165d445: e8 06 65 23 ff              	callq	0x140893950 <.text+0x892950>
14165d44a: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14165d44f: 48 83 c4 38                 	addq	$0x38, %rsp
14165d453: 5b                          	popq	%rbx
14165d454: 5f                          	popq	%rdi
14165d455: 5e                          	popq	%rsi
14165d456: 41 5c                       	popq	%r12
14165d458: 41 5d                       	popq	%r13
14165d45a: 41 5e                       	popq	%r14
14165d45c: 41 5f                       	popq	%r15
14165d45e: 5d                          	popq	%rbp
14165d45f: c3                          	retq
