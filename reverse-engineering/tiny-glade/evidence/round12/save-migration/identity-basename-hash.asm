
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14145aa70: 56                          	pushq	%rsi
14145aa71: 57                          	pushq	%rdi
14145aa72: 53                          	pushq	%rbx
14145aa73: 48 83 ec 70                 	subq	$0x70, %rsp
14145aa77: 48 b8 75 65 73 70 65 6d 6f 73       	movabsq	$0x736f6d6570736575, %rax # imm = 0x736F6D6570736575
14145aa81: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14145aa86: 48 b8 61 72 65 6e 65 67 79 6c       	movabsq	$0x6c7967656e657261, %rax # imm = 0x6C7967656E657261
14145aa90: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14145aa95: 48 b8 6d 6f 64 6e 61 72 6f 64       	movabsq	$0x646f72616e646f6d, %rax # imm = 0x646F72616E646F6D
14145aa9f: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14145aaa4: 48 b8 73 65 74 79 62 64 65 74       	movabsq	$0x7465646279746573, %rax # imm = 0x7465646279746573
14145aaae: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
14145aab3: 0f 57 c0                    	xorps	%xmm0, %xmm0
14145aab6: 0f 11 44 24 48              	movups	%xmm0, 0x48(%rsp)
14145aabb: 0f 11 44 24 58              	movups	%xmm0, 0x58(%rsp)
14145aac0: 48 c7 44 24 68 00 00 00 00  	movq	$0x0, 0x68(%rsp)
14145aac9: 48 8b 31                    	movq	(%rcx), %rsi
14145aacc: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
14145aad0: 31 c0                       	xorl	%eax, %eax
14145aad2: 48 85 f6                    	testq	%rsi, %rsi
14145aad5: 0f 95 c0                    	setne	%al
14145aad8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14145aadd: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
14145aae2: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
14145aae7: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14145aaed: e8 6e b6 ca fe              	callq	0x140106160 <.text+0x105160>
14145aaf2: 48 85 f6                    	testq	%rsi, %rsi
14145aaf5: 74 2b                       	je	0x14145ab22 <.text+0x1459b22>
14145aaf7: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
14145aafc: 48 8d 5c 24 28              	leaq	0x28(%rsp), %rbx
14145ab01: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
14145ab06: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14145ab0c: 48 89 d9                    	movq	%rbx, %rcx
14145ab0f: e8 4c b6 ca fe              	callq	0x140106160 <.text+0x105160>
14145ab14: 48 89 d9                    	movq	%rbx, %rcx
14145ab17: 48 89 f2                    	movq	%rsi, %rdx
14145ab1a: 49 89 f8                    	movq	%rdi, %r8
14145ab1d: e8 3e b6 ca fe              	callq	0x140106160 <.text+0x105160>
14145ab22: 4c 8b 4c 24 58              	movq	0x58(%rsp), %r9
14145ab27: 49 c1 e1 38                 	shlq	$0x38, %r9
14145ab2b: 4c 0b 4c 24 60              	orq	0x60(%rsp), %r9
14145ab30: 48 8b 54 24 38              	movq	0x38(%rsp), %rdx
14145ab35: 48 8b 4c 24 40              	movq	0x40(%rsp), %rcx
14145ab3a: 4c 31 c9                    	xorq	%r9, %rcx
14145ab3d: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
14145ab42: 48 01 d0                    	addq	%rdx, %rax
14145ab45: 4c 8b 44 24 30              	movq	0x30(%rsp), %r8
14145ab4a: 48 c1 c2 0d                 	rolq	$0xd, %rdx
14145ab4e: 49 01 c8                    	addq	%rcx, %r8
14145ab51: 48 31 c2                    	xorq	%rax, %rdx
14145ab54: 48 c1 c1 10                 	rolq	$0x10, %rcx
14145ab58: 4c 31 c1                    	xorq	%r8, %rcx
14145ab5b: 48 c1 c0 20                 	rolq	$0x20, %rax
14145ab5f: 49 01 d0                    	addq	%rdx, %r8
14145ab62: 48 01 c8                    	addq	%rcx, %rax
14145ab65: 48 c1 c2 11                 	rolq	$0x11, %rdx
14145ab69: 4c 31 c2                    	xorq	%r8, %rdx
14145ab6c: 48 c1 c1 15                 	rolq	$0x15, %rcx
14145ab70: 49 c1 c0 20                 	rolq	$0x20, %r8
14145ab74: 48 31 c1                    	xorq	%rax, %rcx
14145ab77: 4c 31 c8                    	xorq	%r9, %rax
14145ab7a: 49 81 f0 ff 00 00 00        	xorq	$0xff, %r8
14145ab81: 48 01 d0                    	addq	%rdx, %rax
14145ab84: 49 01 c8                    	addq	%rcx, %r8
14145ab87: 48 c1 c2 0d                 	rolq	$0xd, %rdx
14145ab8b: 48 31 c2                    	xorq	%rax, %rdx
14145ab8e: 48 c1 c1 10                 	rolq	$0x10, %rcx
14145ab92: 48 c1 c0 20                 	rolq	$0x20, %rax
14145ab96: 4c 31 c1                    	xorq	%r8, %rcx
14145ab99: 49 01 d0                    	addq	%rdx, %r8
14145ab9c: 48 c1 c2 11                 	rolq	$0x11, %rdx
14145aba0: 48 01 c8                    	addq	%rcx, %rax
14145aba3: 4c 31 c2                    	xorq	%r8, %rdx
14145aba6: 48 c1 c1 15                 	rolq	$0x15, %rcx
14145abaa: 48 31 c1                    	xorq	%rax, %rcx
14145abad: 49 c1 c0 20                 	rolq	$0x20, %r8
14145abb1: 48 01 d0                    	addq	%rdx, %rax
14145abb4: 49 01 c8                    	addq	%rcx, %r8
14145abb7: 48 c1 c2 0d                 	rolq	$0xd, %rdx
14145abbb: 48 31 c2                    	xorq	%rax, %rdx
14145abbe: 48 c1 c1 10                 	rolq	$0x10, %rcx
14145abc2: 48 c1 c0 20                 	rolq	$0x20, %rax
14145abc6: 4c 31 c1                    	xorq	%r8, %rcx
14145abc9: 49 01 d0                    	addq	%rdx, %r8
14145abcc: 48 c1 c2 11                 	rolq	$0x11, %rdx
14145abd0: 48 01 c8                    	addq	%rcx, %rax
14145abd3: 4c 31 c2                    	xorq	%r8, %rdx
14145abd6: 48 c1 c1 15                 	rolq	$0x15, %rcx
14145abda: 48 31 c1                    	xorq	%rax, %rcx
14145abdd: 49 c1 c0 20                 	rolq	$0x20, %r8
14145abe1: 48 01 d0                    	addq	%rdx, %rax
14145abe4: 49 01 c8                    	addq	%rcx, %r8
14145abe7: 48 c1 c2 0d                 	rolq	$0xd, %rdx
14145abeb: 48 31 c2                    	xorq	%rax, %rdx
14145abee: 48 c1 c1 10                 	rolq	$0x10, %rcx
14145abf2: 4c 31 c1                    	xorq	%r8, %rcx
14145abf5: 49 01 d0                    	addq	%rdx, %r8
14145abf8: 48 c1 c2 11                 	rolq	$0x11, %rdx
14145abfc: 48 c1 c1 15                 	rolq	$0x15, %rcx
14145ac00: 4c 89 c0                    	movq	%r8, %rax
14145ac03: 48 c1 c0 20                 	rolq	$0x20, %rax
14145ac07: 48 31 d0                    	xorq	%rdx, %rax
14145ac0a: 48 31 c8                    	xorq	%rcx, %rax
14145ac0d: 4c 31 c0                    	xorq	%r8, %rax
14145ac10: 48 83 c4 70                 	addq	$0x70, %rsp
14145ac14: 5b                          	popq	%rbx
14145ac15: 5f                          	popq	%rdi
14145ac16: 5e                          	popq	%rsi
14145ac17: c3                          	retq
