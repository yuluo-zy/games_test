
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b67b50: 56                          	pushq	%rsi
140b67b51: 57                          	pushq	%rdi
140b67b52: 53                          	pushq	%rbx
140b67b53: 48 81 ec c0 00 00 00        	subq	$0xc0, %rsp
140b67b5a: 48 89 ce                    	movq	%rcx, %rsi
140b67b5d: 48 83 c1 38                 	addq	$0x38, %rcx
140b67b61: 48 ba 84 a7 2b 96 51 d2 1b 8e       	movabsq	$-0x71e42dae69d4587c, %rdx # imm = 0x8E1BD251962BA784
140b67b6b: 49 b8 9b 52 e6 c0 74 23 78 6a       	movabsq	$0x6a782374c0e6529b, %r8 # imm = 0x6A782374C0E6529B
140b67b75: e8 76 5b 57 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140b67b7a: 48 85 c0                    	testq	%rax, %rax
140b67b7d: 0f 84 9b 00 00 00           	je	0x140b67c1e <.text+0xb66c1e>
140b67b83: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140b67b8a: 0f 86 8e 00 00 00           	jbe	0x140b67c1e <.text+0xb66c1e>
140b67b90: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140b67b97: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140b67b9b: 48 85 c9                    	testq	%rcx, %rcx
140b67b9e: 74 7e                       	je	0x140b67c1e <.text+0xb66c1e>
140b67ba0: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
140b67ba7: 48 f7 d1                    	notq	%rcx
140b67baa: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140b67bae: 48 c1 e1 05                 	shlq	$0x5, %rcx
140b67bb2: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140b67bb8: 74 64                       	je	0x140b67c1e <.text+0xb66c1e>
140b67bba: 48 01 c8                    	addq	%rcx, %rax
140b67bbd: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140b67bc1: 8b 96 b0 05 00 00           	movl	0x5b0(%rsi), %edx
140b67bc7: 89 50 5c                    	movl	%edx, 0x5c(%rax)
140b67bca: e8 51 a0 b3 00              	callq	0x1416a1c20 <.text+0x16a0c20>
140b67bcf: 48 89 c7                    	movq	%rax, %rdi
140b67bd2: 48 8d 5c 24 50              	leaq	0x50(%rsp), %rbx
140b67bd7: 48 89 d9                    	movq	%rbx, %rcx
140b67bda: e8 d1 3c e3 ff              	callq	0x14099b8b0 <.text+0x99a8b0>
140b67bdf: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140b67be4: 48 89 fa                    	movq	%rdi, %rdx
140b67be7: 49 89 d8                    	movq	%rbx, %r8
140b67bea: 45 31 c9                    	xorl	%r9d, %r9d
140b67bed: e8 3e 71 b3 00              	callq	0x14169ed30 <.text+0x169dd30>
140b67bf2: 48 8b 54 24 30              	movq	0x30(%rsp), %rdx
140b67bf7: 48 85 d2                    	testq	%rdx, %rdx
140b67bfa: 74 14                       	je	0x140b67c10 <.text+0xb66c10>
140b67bfc: 48 8b 4c 24 38              	movq	0x38(%rsp), %rcx
140b67c01: 48 c1 e2 04                 	shlq	$0x4, %rdx
140b67c05: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140b67c0b: e8 10 c0 aa ff              	callq	0x140613c20 <.text+0x612c20>
140b67c10: 48 89 f0                    	movq	%rsi, %rax
140b67c13: 48 81 c4 c0 00 00 00        	addq	$0xc0, %rsp
140b67c1a: 5b                          	popq	%rbx
140b67c1b: 5f                          	popq	%rdi
140b67c1c: 5e                          	popq	%rsi
140b67c1d: c3                          	retq
140b67c1e: 48 8d 05 9b b0 f8 01        	leaq	0x1f8b09b(%rip), %rax   # 0x142af2cc0
140b67c25: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140b67c2a: 48 c7 44 24 38 27 00 00 00  	movq	$0x27, 0x38(%rsp)
140b67c33: 48 8d 44 24 30              	leaq	0x30(%rsp), %rax
140b67c38: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b67c3d: 48 8d 05 4c 22 5a ff        	leaq	-0xa5ddb4(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140b67c44: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140b67c49: 48 8d 05 a0 b1 f8 01        	leaq	0x1f8b1a0(%rip), %rax   # 0x142af2df0
140b67c50: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140b67c55: 48 c7 44 24 58 02 00 00 00  	movq	$0x2, 0x58(%rsp)
140b67c5e: 48 c7 44 24 70 00 00 00 00  	movq	$0x0, 0x70(%rsp)
140b67c67: 48 8d 44 24 20              	leaq	0x20(%rsp), %rax
140b67c6c: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140b67c71: 48 c7 44 24 68 01 00 00 00  	movq	$0x1, 0x68(%rsp)
140b67c7a: 48 8d 15 27 b0 f8 01        	leaq	0x1f8b027(%rip), %rdx   # 0x142af2ca8
140b67c81: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
140b67c86: e8 05 17 d7 01              	callq	0x1428d9390 <.text+0x28d8390>
140b67c8b: cc                          	int3
