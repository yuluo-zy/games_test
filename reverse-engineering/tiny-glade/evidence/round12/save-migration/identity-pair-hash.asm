
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14145a8e0: 56                          	pushq	%rsi
14145a8e1: 57                          	pushq	%rdi
14145a8e2: 48 83 ec 78                 	subq	$0x78, %rsp
14145a8e6: 48 b8 75 65 73 70 65 6d 6f 73       	movabsq	$0x736f6d6570736575, %rax # imm = 0x736F6D6570736575
14145a8f0: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14145a8f5: 48 b8 61 72 65 6e 65 67 79 6c       	movabsq	$0x6c7967656e657261, %rax # imm = 0x6C7967656E657261
14145a8ff: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14145a904: 48 b8 6d 6f 64 6e 61 72 6f 64       	movabsq	$0x646f72616e646f6d, %rax # imm = 0x646F72616E646F6D
14145a90e: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
14145a913: 48 b8 73 65 74 79 62 64 65 74       	movabsq	$0x7465646279746573, %rax # imm = 0x7465646279746573
14145a91d: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
14145a922: 0f 57 c0                    	xorps	%xmm0, %xmm0
14145a925: 0f 11 44 24 50              	movups	%xmm0, 0x50(%rsp)
14145a92a: 0f 11 44 24 60              	movups	%xmm0, 0x60(%rsp)
14145a92f: 48 c7 44 24 70 00 00 00 00  	movq	$0x0, 0x70(%rsp)
14145a938: 48 8b 01                    	movq	(%rcx), %rax
14145a93b: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
14145a93f: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14145a944: 48 8d 74 24 30              	leaq	0x30(%rsp), %rsi
14145a949: 48 8d 54 24 28              	leaq	0x28(%rsp), %rdx
14145a94e: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14145a954: 48 89 f1                    	movq	%rsi, %rcx
14145a957: e8 04 b8 ca fe              	callq	0x140106160 <.text+0x105160>
14145a95c: 48 8b 07                    	movq	(%rdi), %rax
14145a95f: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14145a964: 48 8d 54 24 28              	leaq	0x28(%rsp), %rdx
14145a969: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14145a96f: 48 89 f1                    	movq	%rsi, %rcx
14145a972: e8 e9 b7 ca fe              	callq	0x140106160 <.text+0x105160>
14145a977: 4c 8b 4c 24 60              	movq	0x60(%rsp), %r9
14145a97c: 49 c1 e1 38                 	shlq	$0x38, %r9
14145a980: 4c 0b 4c 24 68              	orq	0x68(%rsp), %r9
14145a985: 48 8b 54 24 40              	movq	0x40(%rsp), %rdx
14145a98a: 48 8b 4c 24 48              	movq	0x48(%rsp), %rcx
14145a98f: 4c 31 c9                    	xorq	%r9, %rcx
14145a992: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
14145a997: 48 01 d0                    	addq	%rdx, %rax
14145a99a: 4c 8b 44 24 38              	movq	0x38(%rsp), %r8
14145a99f: 48 c1 c2 0d                 	rolq	$0xd, %rdx
14145a9a3: 49 01 c8                    	addq	%rcx, %r8
14145a9a6: 48 31 c2                    	xorq	%rax, %rdx
14145a9a9: 48 c1 c1 10                 	rolq	$0x10, %rcx
14145a9ad: 4c 31 c1                    	xorq	%r8, %rcx
14145a9b0: 48 c1 c0 20                 	rolq	$0x20, %rax
14145a9b4: 49 01 d0                    	addq	%rdx, %r8
14145a9b7: 48 01 c8                    	addq	%rcx, %rax
14145a9ba: 48 c1 c2 11                 	rolq	$0x11, %rdx
14145a9be: 4c 31 c2                    	xorq	%r8, %rdx
14145a9c1: 48 c1 c1 15                 	rolq	$0x15, %rcx
14145a9c5: 49 c1 c0 20                 	rolq	$0x20, %r8
14145a9c9: 48 31 c1                    	xorq	%rax, %rcx
14145a9cc: 4c 31 c8                    	xorq	%r9, %rax
14145a9cf: 49 81 f0 ff 00 00 00        	xorq	$0xff, %r8
14145a9d6: 48 01 d0                    	addq	%rdx, %rax
14145a9d9: 49 01 c8                    	addq	%rcx, %r8
14145a9dc: 48 c1 c2 0d                 	rolq	$0xd, %rdx
14145a9e0: 48 31 c2                    	xorq	%rax, %rdx
14145a9e3: 48 c1 c1 10                 	rolq	$0x10, %rcx
14145a9e7: 48 c1 c0 20                 	rolq	$0x20, %rax
14145a9eb: 4c 31 c1                    	xorq	%r8, %rcx
14145a9ee: 49 01 d0                    	addq	%rdx, %r8
14145a9f1: 48 c1 c2 11                 	rolq	$0x11, %rdx
14145a9f5: 48 01 c8                    	addq	%rcx, %rax
14145a9f8: 4c 31 c2                    	xorq	%r8, %rdx
14145a9fb: 48 c1 c1 15                 	rolq	$0x15, %rcx
14145a9ff: 48 31 c1                    	xorq	%rax, %rcx
14145aa02: 49 c1 c0 20                 	rolq	$0x20, %r8
14145aa06: 48 01 d0                    	addq	%rdx, %rax
14145aa09: 49 01 c8                    	addq	%rcx, %r8
14145aa0c: 48 c1 c2 0d                 	rolq	$0xd, %rdx
14145aa10: 48 31 c2                    	xorq	%rax, %rdx
14145aa13: 48 c1 c1 10                 	rolq	$0x10, %rcx
14145aa17: 48 c1 c0 20                 	rolq	$0x20, %rax
14145aa1b: 4c 31 c1                    	xorq	%r8, %rcx
14145aa1e: 49 01 d0                    	addq	%rdx, %r8
14145aa21: 48 c1 c2 11                 	rolq	$0x11, %rdx
14145aa25: 48 01 c8                    	addq	%rcx, %rax
14145aa28: 4c 31 c2                    	xorq	%r8, %rdx
14145aa2b: 48 c1 c1 15                 	rolq	$0x15, %rcx
14145aa2f: 48 31 c1                    	xorq	%rax, %rcx
14145aa32: 49 c1 c0 20                 	rolq	$0x20, %r8
14145aa36: 48 01 d0                    	addq	%rdx, %rax
14145aa39: 49 01 c8                    	addq	%rcx, %r8
14145aa3c: 48 c1 c2 0d                 	rolq	$0xd, %rdx
14145aa40: 48 31 c2                    	xorq	%rax, %rdx
14145aa43: 48 c1 c1 10                 	rolq	$0x10, %rcx
14145aa47: 4c 31 c1                    	xorq	%r8, %rcx
14145aa4a: 49 01 d0                    	addq	%rdx, %r8
14145aa4d: 48 c1 c2 11                 	rolq	$0x11, %rdx
14145aa51: 48 c1 c1 15                 	rolq	$0x15, %rcx
14145aa55: 4c 89 c0                    	movq	%r8, %rax
14145aa58: 48 c1 c0 20                 	rolq	$0x20, %rax
14145aa5c: 48 31 d0                    	xorq	%rdx, %rax
14145aa5f: 48 31 c8                    	xorq	%rcx, %rax
14145aa62: 4c 31 c0                    	xorq	%r8, %rax
14145aa65: 48 83 c4 78                 	addq	$0x78, %rsp
14145aa69: 5f                          	popq	%rdi
14145aa6a: 5e                          	popq	%rsi
14145aa6b: c3                          	retq
