
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140314a00: 41 56                       	pushq	%r14
140314a02: 56                          	pushq	%rsi
140314a03: 57                          	pushq	%rdi
140314a04: 53                          	pushq	%rbx
140314a05: 48 83 ec 38                 	subq	$0x38, %rsp
140314a09: 48 89 d7                    	movq	%rdx, %rdi
140314a0c: 48 89 ce                    	movq	%rcx, %rsi
140314a0f: 8b 01                       	movl	(%rcx), %eax
140314a11: 48 83 c2 08                 	addq	$0x8, %rdx
140314a15: 85 c0                       	testl	%eax, %eax
140314a17: 0f 84 25 01 00 00           	je	0x140314b42 <.text+0x313b42>
140314a1d: 83 f8 01                    	cmpl	$0x1, %eax
140314a20: 0f 85 2e 02 00 00           	jne	0x140314c54 <.text+0x313c54>
140314a26: 48 8b 1f                    	movq	(%rdi), %rbx
140314a29: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140314a2d: 48 39 03                    	cmpq	%rax, (%rbx)
140314a30: 0f 84 5f 02 00 00           	je	0x140314c95 <.text+0x313c95>
140314a36: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140314a3a: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140314a3e: 48 ff c0                    	incq	%rax
140314a41: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140314a45: 4c 8d 05 70 66 6a 02        	leaq	0x26a6670(%rip), %r8    # 0x1429bb0bc
140314a4c: 41 b9 08 00 00 00           	movl	$0x8, %r9d
140314a52: 48 89 f9                    	movq	%rdi, %rcx
140314a55: e8 a6 16 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140314a5a: 48 85 c0                    	testq	%rax, %rax
140314a5d: 0f 85 17 02 00 00           	jne	0x140314c7a <.text+0x313c7a>
140314a63: 48 8b 1f                    	movq	(%rdi), %rbx
140314a66: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314a6a: 48 39 13                    	cmpq	%rdx, (%rbx)
140314a6d: 0f 84 80 02 00 00           	je	0x140314cf3 <.text+0x313cf3>
140314a73: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140314a77: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140314a7b: 48 ff c2                    	incq	%rdx
140314a7e: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140314a82: 48 8b 1f                    	movq	(%rdi), %rbx
140314a85: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314a89: 48 39 13                    	cmpq	%rdx, (%rbx)
140314a8c: 0f 84 87 02 00 00           	je	0x140314d19 <.text+0x313d19>
140314a92: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140314a96: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140314a9a: 48 ff c2                    	incq	%rdx
140314a9d: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140314aa1: 4c 8d 4e 04                 	leaq	0x4(%rsi), %r9
140314aa5: 48 89 7c 24 28              	movq	%rdi, 0x28(%rsp)
140314aaa: c6 44 24 30 01              	movb	$0x1, 0x30(%rsp)
140314aaf: 48 8d 15 fa 65 6a 02        	leaq	0x26a65fa(%rip), %rdx   # 0x1429bb0b0
140314ab6: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
140314abb: 41 b8 07 00 00 00           	movl	$0x7, %r8d
140314ac1: e8 fa 4a 1a 00              	callq	0x1404b95c0 <.text+0x4b85c0>
140314ac6: 48 85 c0                    	testq	%rax, %rax
140314ac9: 0f 85 a1 01 00 00           	jne	0x140314c70 <.text+0x313c70>
140314acf: 48 83 c6 08                 	addq	$0x8, %rsi
140314ad3: 48 8d 15 dd 65 6a 02        	leaq	0x26a65dd(%rip), %rdx   # 0x1429bb0b7
140314ada: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
140314adf: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140314ae5: 49 89 f1                    	movq	%rsi, %r9
140314ae8: e8 d3 4a 1a 00              	callq	0x1404b95c0 <.text+0x4b85c0>
140314aed: 48 85 c0                    	testq	%rax, %rax
140314af0: 0f 85 7a 01 00 00           	jne	0x140314c70 <.text+0x313c70>
140314af6: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
140314afb: 48 8b 30                    	movq	(%rax), %rsi
140314afe: 80 7c 24 30 00              	cmpb	$0x0, 0x30(%rsp)
140314b03: 0f 84 18 01 00 00           	je	0x140314c21 <.text+0x313c21>
140314b09: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
140314b0d: 48 39 16                    	cmpq	%rdx, (%rsi)
140314b10: 0f 84 75 02 00 00           	je	0x140314d8b <.text+0x313d8b>
140314b16: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140314b1a: c6 04 10 7d                 	movb	$0x7d, (%rax,%rdx)
140314b1e: 48 ff c2                    	incq	%rdx
140314b21: 48 89 56 10                 	movq	%rdx, 0x10(%rsi)
140314b25: 48 39 16                    	cmpq	%rdx, (%rsi)
140314b28: 0f 84 00 01 00 00           	je	0x140314c2e <.text+0x313c2e>
140314b2e: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140314b32: c6 04 10 7d                 	movb	$0x7d, (%rax,%rdx)
140314b36: 48 ff c2                    	incq	%rdx
140314b39: 48 89 56 10                 	movq	%rdx, 0x10(%rsi)
140314b3d: e9 2c 01 00 00              	jmp	0x140314c6e <.text+0x313c6e>
140314b42: 48 8b 1f                    	movq	(%rdi), %rbx
140314b45: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140314b49: 48 39 03                    	cmpq	%rax, (%rbx)
140314b4c: 0f 84 72 01 00 00           	je	0x140314cc4 <.text+0x313cc4>
140314b52: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140314b56: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140314b5a: 48 ff c0                    	incq	%rax
140314b5d: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140314b61: 4c 8d 05 3e 65 6a 02        	leaq	0x26a653e(%rip), %r8    # 0x1429bb0a6
140314b68: 41 b9 0a 00 00 00           	movl	$0xa, %r9d
140314b6e: 48 89 f9                    	movq	%rdi, %rcx
140314b71: e8 8a 15 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140314b76: 48 85 c0                    	testq	%rax, %rax
140314b79: 0f 85 fb 00 00 00           	jne	0x140314c7a <.text+0x313c7a>
140314b7f: 48 8b 1f                    	movq	(%rdi), %rbx
140314b82: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314b86: 48 39 13                    	cmpq	%rdx, (%rbx)
140314b89: 0f 84 b0 01 00 00           	je	0x140314d3f <.text+0x313d3f>
140314b8f: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140314b93: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140314b97: 48 ff c2                    	incq	%rdx
140314b9a: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140314b9e: 48 8b 1f                    	movq	(%rdi), %rbx
140314ba1: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314ba5: 48 39 13                    	cmpq	%rdx, (%rbx)
140314ba8: 0f 84 b7 01 00 00           	je	0x140314d65 <.text+0x313d65>
140314bae: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140314bb2: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140314bb6: 48 ff c2                    	incq	%rdx
140314bb9: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140314bbd: 4c 8d 4e 04                 	leaq	0x4(%rsi), %r9
140314bc1: 48 89 7c 24 28              	movq	%rdi, 0x28(%rsp)
140314bc6: c6 44 24 30 01              	movb	$0x1, 0x30(%rsp)
140314bcb: 48 8d 15 de 64 6a 02        	leaq	0x26a64de(%rip), %rdx   # 0x1429bb0b0
140314bd2: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
140314bd7: 41 b8 07 00 00 00           	movl	$0x7, %r8d
140314bdd: e8 de 49 1a 00              	callq	0x1404b95c0 <.text+0x4b85c0>
140314be2: 48 85 c0                    	testq	%rax, %rax
140314be5: 0f 85 85 00 00 00           	jne	0x140314c70 <.text+0x313c70>
140314beb: 48 83 c6 08                 	addq	$0x8, %rsi
140314bef: 48 8d 15 c1 64 6a 02        	leaq	0x26a64c1(%rip), %rdx   # 0x1429bb0b7
140314bf6: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
140314bfb: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140314c01: 49 89 f1                    	movq	%rsi, %r9
140314c04: e8 b7 49 1a 00              	callq	0x1404b95c0 <.text+0x4b85c0>
140314c09: 48 85 c0                    	testq	%rax, %rax
140314c0c: 75 62                       	jne	0x140314c70 <.text+0x313c70>
140314c0e: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
140314c13: 48 8b 30                    	movq	(%rax), %rsi
140314c16: 80 7c 24 30 00              	cmpb	$0x0, 0x30(%rsp)
140314c1b: 0f 85 e8 fe ff ff           	jne	0x140314b09 <.text+0x313b09>
140314c21: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
140314c25: 48 39 16                    	cmpq	%rdx, (%rsi)
140314c28: 0f 85 00 ff ff ff           	jne	0x140314b2e <.text+0x313b2e>
140314c2e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140314c37: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140314c3d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140314c43: 48 89 f1                    	movq	%rsi, %rcx
140314c46: e8 95 dd 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140314c4b: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
140314c4f: e9 da fe ff ff              	jmp	0x140314b2e <.text+0x313b2e>
140314c54: 4c 8d 05 69 64 6a 02        	leaq	0x26a6469(%rip), %r8    # 0x1429bb0c4
140314c5b: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
140314c61: 48 89 f9                    	movq	%rdi, %rcx
140314c64: e8 97 14 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140314c69: 48 85 c0                    	testq	%rax, %rax
140314c6c: 75 16                       	jne	0x140314c84 <.text+0x313c84>
140314c6e: 31 c0                       	xorl	%eax, %eax
140314c70: 48 83 c4 38                 	addq	$0x38, %rsp
140314c74: 5b                          	popq	%rbx
140314c75: 5f                          	popq	%rdi
140314c76: 5e                          	popq	%rsi
140314c77: 41 5e                       	popq	%r14
140314c79: c3                          	retq
140314c7a: 48 89 c1                    	movq	%rax, %rcx
140314c7d: e8 1e 4c 5b 02              	callq	0x1428c98a0 <.text+0x28c88a0>
140314c82: eb ec                       	jmp	0x140314c70 <.text+0x313c70>
140314c84: 48 89 c1                    	movq	%rax, %rcx
140314c87: 48 83 c4 38                 	addq	$0x38, %rsp
140314c8b: 5b                          	popq	%rbx
140314c8c: 5f                          	popq	%rdi
140314c8d: 5e                          	popq	%rsi
140314c8e: 41 5e                       	popq	%r14
140314c90: e9 0b 4c 5b 02              	jmp	0x1428c98a0 <.text+0x28c88a0>
140314c95: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140314c9e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140314ca4: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140314caa: 48 89 d9                    	movq	%rbx, %rcx
140314cad: 49 89 d6                    	movq	%rdx, %r14
140314cb0: 48 89 c2                    	movq	%rax, %rdx
140314cb3: e8 28 dd 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140314cb8: 4c 89 f2                    	movq	%r14, %rdx
140314cbb: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140314cbf: e9 72 fd ff ff              	jmp	0x140314a36 <.text+0x313a36>
140314cc4: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140314ccd: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140314cd3: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140314cd9: 48 89 d9                    	movq	%rbx, %rcx
140314cdc: 49 89 d6                    	movq	%rdx, %r14
140314cdf: 48 89 c2                    	movq	%rax, %rdx
140314ce2: e8 f9 dc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140314ce7: 4c 89 f2                    	movq	%r14, %rdx
140314cea: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140314cee: e9 5f fe ff ff              	jmp	0x140314b52 <.text+0x313b52>
140314cf3: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140314cfc: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140314d02: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140314d08: 48 89 d9                    	movq	%rbx, %rcx
140314d0b: e8 d0 dc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140314d10: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314d14: e9 5a fd ff ff              	jmp	0x140314a73 <.text+0x313a73>
140314d19: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140314d22: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140314d28: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140314d2e: 48 89 d9                    	movq	%rbx, %rcx
140314d31: e8 aa dc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140314d36: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314d3a: e9 53 fd ff ff              	jmp	0x140314a92 <.text+0x313a92>
140314d3f: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140314d48: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140314d4e: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140314d54: 48 89 d9                    	movq	%rbx, %rcx
140314d57: e8 84 dc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140314d5c: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314d60: e9 2a fe ff ff              	jmp	0x140314b8f <.text+0x313b8f>
140314d65: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140314d6e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140314d74: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140314d7a: 48 89 d9                    	movq	%rbx, %rcx
140314d7d: e8 5e dc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140314d82: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314d86: e9 23 fe ff ff              	jmp	0x140314bae <.text+0x313bae>
140314d8b: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140314d94: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140314d9a: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140314da0: 48 89 f1                    	movq	%rsi, %rcx
140314da3: e8 38 dc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140314da8: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
140314dac: e9 65 fd ff ff              	jmp	0x140314b16 <.text+0x313b16>
140314db1: cc                          	int3
140314db2: cc                          	int3
140314db3: cc                          	int3
140314db4: cc                          	int3
140314db5: cc                          	int3
140314db6: cc                          	int3
140314db7: cc                          	int3
140314db8: cc                          	int3
140314db9: cc                          	int3
140314dba: cc                          	int3
140314dbb: cc                          	int3
140314dbc: cc                          	int3
140314dbd: cc                          	int3
140314dbe: cc                          	int3
140314dbf: cc                          	int3
140314dc0: 41 56                       	pushq	%r14
140314dc2: 56                          	pushq	%rsi
140314dc3: 57                          	pushq	%rdi
140314dc4: 53                          	pushq	%rbx
140314dc5: 48 83 ec 48                 	subq	$0x48, %rsp
140314dc9: 48 89 ce                    	movq	%rcx, %rsi
140314dcc: 48 8b 01                    	movq	(%rcx), %rax
140314dcf: 48 b9 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rcx # imm = 0x8000000000000001
140314dd9: 48 01 c1                    	addq	%rax, %rcx
140314ddc: 45 31 c0                    	xorl	%r8d, %r8d
140314ddf: 49 b9 10 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff0, %r9 # imm = 0x8000000000000010
140314de9: 4c 39 c8                    	cmpq	%r9, %rax
140314dec: 4c 0f 4c c1                 	cmovlq	%rcx, %r8
140314df0: 48 89 d7                    	movq	%rdx, %rdi
140314df3: 48 8d 05 02 65 6a 02        	leaq	0x26a6502(%rip), %rax   # 0x1429bb2fc
140314dfa: 4a 63 0c 80                 	movslq	(%rax,%r8,4), %rcx
140314dfe: 48 01 c1                    	addq	%rax, %rcx
140314e01: ff e1                       	jmpq	*%rcx
140314e03: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140314e07: 48 8b 1f                    	movq	(%rdi), %rbx
140314e0a: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140314e0e: 48 39 03                    	cmpq	%rax, (%rbx)
140314e11: 0f 84 70 11 00 00           	je	0x140315f87 <.text+0x314f87>
140314e17: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140314e1b: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140314e1f: 48 ff c0                    	incq	%rax
140314e22: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140314e26: 4c 8d 05 a2 62 6a 02        	leaq	0x26a62a2(%rip), %r8    # 0x1429bb0cf
140314e2d: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
140314e33: 48 89 f9                    	movq	%rdi, %rcx
140314e36: e8 c5 12 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140314e3b: 48 85 c0                    	testq	%rax, %rax
140314e3e: 0f 85 0e 0f 00 00           	jne	0x140315d52 <.text+0x314d52>
140314e44: 48 8b 1f                    	movq	(%rdi), %rbx
140314e47: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314e4b: 48 39 13                    	cmpq	%rdx, (%rbx)
140314e4e: 0f 84 dc 14 00 00           	je	0x140316330 <.text+0x315330>
140314e54: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140314e58: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140314e5c: 48 ff c2                    	incq	%rdx
140314e5f: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140314e63: 48 8b 1f                    	movq	(%rdi), %rbx
140314e66: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314e6a: 48 39 13                    	cmpq	%rdx, (%rbx)
140314e6d: 0f 84 e3 14 00 00           	je	0x140316356 <.text+0x315356>
140314e73: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140314e77: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140314e7b: 48 ff c2                    	incq	%rdx
140314e7e: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140314e82: 4c 8d 8e e0 00 00 00        	leaq	0xe0(%rsi), %r9
140314e89: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140314e8e: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140314e93: 48 8d 15 40 62 6a 02        	leaq	0x26a6240(%rip), %rdx   # 0x1429bb0da
140314e9a: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140314e9f: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140314ea5: e8 f6 c2 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140314eaa: 48 85 c0                    	testq	%rax, %rax
140314ead: 0f 85 a7 0e 00 00           	jne	0x140315d5a <.text+0x314d5a>
140314eb3: 4c 8d 8e e8 00 00 00        	leaq	0xe8(%rsi), %r9
140314eba: 48 8d 15 23 62 6a 02        	leaq	0x26a6223(%rip), %rdx   # 0x1429bb0e4
140314ec1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140314ec6: 41 b8 0c 00 00 00           	movl	$0xc, %r8d
140314ecc: e8 6f 6a 1a 00              	callq	0x1404bb940 <.text+0x4ba940>
140314ed1: 48 85 c0                    	testq	%rax, %rax
140314ed4: 0f 85 80 0e 00 00           	jne	0x140315d5a <.text+0x314d5a>
140314eda: 48 8d 15 0f 62 6a 02        	leaq	0x26a620f(%rip), %rdx   # 0x1429bb0f0
140314ee1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140314ee6: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140314eec: 49 89 f1                    	movq	%rsi, %r9
140314eef: e8 fc 3a 1a 00              	callq	0x1404b89f0 <.text+0x4b79f0>
140314ef4: 48 85 c0                    	testq	%rax, %rax
140314ef7: 0f 85 5d 0e 00 00           	jne	0x140315d5a <.text+0x314d5a>
140314efd: 48 83 c6 70                 	addq	$0x70, %rsi
140314f01: 48 8d 15 ee 61 6a 02        	leaq	0x26a61ee(%rip), %rdx   # 0x1429bb0f6
140314f08: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140314f0d: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140314f13: 49 89 f1                    	movq	%rsi, %r9
140314f16: e8 d5 3a 1a 00              	callq	0x1404b89f0 <.text+0x4b79f0>
140314f1b: e9 17 0e 00 00              	jmp	0x140315d37 <.text+0x314d37>
140314f20: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140314f24: 48 8b 1f                    	movq	(%rdi), %rbx
140314f27: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140314f2b: 48 39 03                    	cmpq	%rax, (%rbx)
140314f2e: 0f 84 7d 0e 00 00           	je	0x140315db1 <.text+0x314db1>
140314f34: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140314f38: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140314f3c: 48 ff c0                    	incq	%rax
140314f3f: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140314f43: 4c 8d 05 08 63 6a 02        	leaq	0x26a6308(%rip), %r8    # 0x1429bb252
140314f4a: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
140314f50: 48 89 f9                    	movq	%rdi, %rcx
140314f53: e8 a8 11 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140314f58: 48 85 c0                    	testq	%rax, %rax
140314f5b: 0f 85 f1 0d 00 00           	jne	0x140315d52 <.text+0x314d52>
140314f61: 48 8b 1f                    	movq	(%rdi), %rbx
140314f64: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314f68: 48 39 13                    	cmpq	%rdx, (%rbx)
140314f6b: 0f 84 5f 11 00 00           	je	0x1403160d0 <.text+0x3150d0>
140314f71: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140314f75: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140314f79: 48 ff c2                    	incq	%rdx
140314f7c: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140314f80: 48 8b 1f                    	movq	(%rdi), %rbx
140314f83: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314f87: 48 39 13                    	cmpq	%rdx, (%rbx)
140314f8a: 0f 84 66 11 00 00           	je	0x1403160f6 <.text+0x3150f6>
140314f90: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140314f94: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140314f98: 48 ff c2                    	incq	%rdx
140314f9b: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140314f9f: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140314fa3: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140314fa8: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140314fad: 48 8d 15 26 61 6a 02        	leaq	0x26a6126(%rip), %rdx   # 0x1429bb0da
140314fb4: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140314fb9: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140314fbf: e8 dc c1 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140314fc4: 48 85 c0                    	testq	%rax, %rax
140314fc7: 0f 85 8d 0d 00 00           	jne	0x140315d5a <.text+0x314d5a>
140314fcd: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
140314fd1: 48 8d 15 18 61 6a 02        	leaq	0x26a6118(%rip), %rdx   # 0x1429bb0f0
140314fd8: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140314fdd: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140314fe3: e8 a8 73 1a 00              	callq	0x1404bc390 <.text+0x4bb390>
140314fe8: 48 85 c0                    	testq	%rax, %rax
140314feb: 0f 85 69 0d 00 00           	jne	0x140315d5a <.text+0x314d5a>
140314ff1: 48 83 c6 11                 	addq	$0x11, %rsi
140314ff5: 48 8d 15 fa 60 6a 02        	leaq	0x26a60fa(%rip), %rdx   # 0x1429bb0f6
140314ffc: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315001: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140315007: e9 b8 06 00 00              	jmp	0x1403156c4 <.text+0x3146c4>
14031500c: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315010: 48 8b 1f                    	movq	(%rdi), %rbx
140315013: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315017: 48 39 03                    	cmpq	%rax, (%rbx)
14031501a: 0f 84 c0 0d 00 00           	je	0x140315de0 <.text+0x314de0>
140315020: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315024: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315028: 48 ff c0                    	incq	%rax
14031502b: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
14031502f: 4c 8d 05 da 61 6a 02        	leaq	0x26a61da(%rip), %r8    # 0x1429bb210
140315036: 41 b9 0f 00 00 00           	movl	$0xf, %r9d
14031503c: 48 89 f9                    	movq	%rdi, %rcx
14031503f: e8 bc 10 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315044: 48 85 c0                    	testq	%rax, %rax
140315047: 0f 85 05 0d 00 00           	jne	0x140315d52 <.text+0x314d52>
14031504d: 48 8b 1f                    	movq	(%rdi), %rbx
140315050: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315054: 48 39 13                    	cmpq	%rdx, (%rbx)
140315057: 0f 84 bf 10 00 00           	je	0x14031611c <.text+0x31511c>
14031505d: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315061: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315065: 48 ff c2                    	incq	%rdx
140315068: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14031506c: 48 8b 1f                    	movq	(%rdi), %rbx
14031506f: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315073: 48 39 13                    	cmpq	%rdx, (%rbx)
140315076: 0f 84 c6 10 00 00           	je	0x140316142 <.text+0x315142>
14031507c: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315080: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315084: 48 ff c2                    	incq	%rdx
140315087: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14031508b: 4c 8d 4e 78                 	leaq	0x78(%rsi), %r9
14031508f: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315094: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315099: 48 8d 15 3a 60 6a 02        	leaq	0x26a603a(%rip), %rdx   # 0x1429bb0da
1403150a0: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403150a5: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1403150ab: e8 f0 c0 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
1403150b0: 48 85 c0                    	testq	%rax, %rax
1403150b3: 0f 85 a1 0c 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403150b9: 48 8d 46 08                 	leaq	0x8(%rsi), %rax
1403150bd: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
1403150c2: 48 8d 15 56 61 6a 02        	leaq	0x26a6156(%rip), %rdx   # 0x1429bb21f
1403150c9: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403150ce: 4c 8d 4c 24 40              	leaq	0x40(%rsp), %r9
1403150d3: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403150d9: e8 f2 2b 1a 00              	callq	0x1404b7cd0 <.text+0x4b6cd0>
1403150de: 48 85 c0                    	testq	%rax, %rax
1403150e1: 0f 85 73 0c 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403150e7: 48 83 c6 40                 	addq	$0x40, %rsi
1403150eb: 48 89 74 24 40              	movq	%rsi, 0x40(%rsp)
1403150f0: 48 8d 15 30 61 6a 02        	leaq	0x26a6130(%rip), %rdx   # 0x1429bb227
1403150f7: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403150fc: 4c 8d 4c 24 40              	leaq	0x40(%rsp), %r9
140315101: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140315107: e8 c4 2b 1a 00              	callq	0x1404b7cd0 <.text+0x4b6cd0>
14031510c: e9 26 0c 00 00              	jmp	0x140315d37 <.text+0x314d37>
140315111: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315115: 48 8b 1f                    	movq	(%rdi), %rbx
140315118: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031511c: 48 39 03                    	cmpq	%rax, (%rbx)
14031511f: 0f 84 ea 0c 00 00           	je	0x140315e0f <.text+0x314e0f>
140315125: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315129: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
14031512d: 48 ff c0                    	incq	%rax
140315130: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315134: 4c 8d 05 71 60 6a 02        	leaq	0x26a6071(%rip), %r8    # 0x1429bb1ac
14031513b: 41 b9 18 00 00 00           	movl	$0x18, %r9d
140315141: 48 89 f9                    	movq	%rdi, %rcx
140315144: e8 b7 0f 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315149: 48 85 c0                    	testq	%rax, %rax
14031514c: 0f 85 00 0c 00 00           	jne	0x140315d52 <.text+0x314d52>
140315152: 48 8b 1f                    	movq	(%rdi), %rbx
140315155: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315159: 48 39 13                    	cmpq	%rdx, (%rbx)
14031515c: 0f 84 06 10 00 00           	je	0x140316168 <.text+0x315168>
140315162: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315166: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
14031516a: 48 ff c2                    	incq	%rdx
14031516d: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315171: 48 8b 1f                    	movq	(%rdi), %rbx
140315174: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315178: 48 39 13                    	cmpq	%rdx, (%rbx)
14031517b: 0f 84 0d 10 00 00           	je	0x14031618e <.text+0x31518e>
140315181: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315185: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315189: 48 ff c2                    	incq	%rdx
14031518c: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315190: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315194: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315199: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
14031519e: 48 8d 15 35 5f 6a 02        	leaq	0x26a5f35(%rip), %rdx   # 0x1429bb0da
1403151a5: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403151aa: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1403151b0: e8 eb bf 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
1403151b5: 48 85 c0                    	testq	%rax, %rax
1403151b8: 0f 85 9c 0b 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403151be: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
1403151c2: 48 8d 15 27 5f 6a 02        	leaq	0x26a5f27(%rip), %rdx   # 0x1429bb0f0
1403151c9: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403151ce: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1403151d4: e8 47 4a 1a 00              	callq	0x1404b9c20 <.text+0x4b8c20>
1403151d9: 48 85 c0                    	testq	%rax, %rax
1403151dc: 0f 85 78 0b 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403151e2: 48 83 c6 11                 	addq	$0x11, %rsi
1403151e6: 48 8d 15 09 5f 6a 02        	leaq	0x26a5f09(%rip), %rdx   # 0x1429bb0f6
1403151ed: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403151f2: 41 b8 05 00 00 00           	movl	$0x5, %r8d
1403151f8: 49 89 f1                    	movq	%rsi, %r9
1403151fb: e8 20 4a 1a 00              	callq	0x1404b9c20 <.text+0x4b8c20>
140315200: e9 32 0b 00 00              	jmp	0x140315d37 <.text+0x314d37>
140315205: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315209: 48 8b 1f                    	movq	(%rdi), %rbx
14031520c: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315210: 48 39 03                    	cmpq	%rax, (%rbx)
140315213: 0f 84 25 0c 00 00           	je	0x140315e3e <.text+0x314e3e>
140315219: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
14031521d: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315221: 48 ff c0                    	incq	%rax
140315224: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315228: 4c 8d 05 08 5f 6a 02        	leaq	0x26a5f08(%rip), %r8    # 0x1429bb137
14031522f: 41 b9 15 00 00 00           	movl	$0x15, %r9d
140315235: 48 89 f9                    	movq	%rdi, %rcx
140315238: e8 c3 0e 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
14031523d: 48 85 c0                    	testq	%rax, %rax
140315240: 0f 85 0c 0b 00 00           	jne	0x140315d52 <.text+0x314d52>
140315246: 48 8b 1f                    	movq	(%rdi), %rbx
140315249: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031524d: 48 39 13                    	cmpq	%rdx, (%rbx)
140315250: 0f 84 5e 0f 00 00           	je	0x1403161b4 <.text+0x3151b4>
140315256: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14031525a: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
14031525e: 48 ff c2                    	incq	%rdx
140315261: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315265: 48 8b 1f                    	movq	(%rdi), %rbx
140315268: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031526c: 48 39 13                    	cmpq	%rdx, (%rbx)
14031526f: 0f 84 65 0f 00 00           	je	0x1403161da <.text+0x3151da>
140315275: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315279: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
14031527d: 48 ff c2                    	incq	%rdx
140315280: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315284: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315288: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
14031528d: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315292: 48 8d 15 41 5e 6a 02        	leaq	0x26a5e41(%rip), %rdx   # 0x1429bb0da
140315299: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14031529e: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1403152a4: e8 f7 be 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
1403152a9: 48 85 c0                    	testq	%rax, %rax
1403152ac: 0f 85 a8 0a 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403152b2: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
1403152b6: 48 8d 15 33 5e 6a 02        	leaq	0x26a5e33(%rip), %rdx   # 0x1429bb0f0
1403152bd: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403152c2: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1403152c8: e8 53 78 1a 00              	callq	0x1404bcb20 <.text+0x4bbb20>
1403152cd: 48 85 c0                    	testq	%rax, %rax
1403152d0: 0f 85 84 0a 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403152d6: 48 83 c6 2c                 	addq	$0x2c, %rsi
1403152da: 48 8d 15 15 5e 6a 02        	leaq	0x26a5e15(%rip), %rdx   # 0x1429bb0f6
1403152e1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403152e6: 41 b8 05 00 00 00           	movl	$0x5, %r8d
1403152ec: 49 89 f1                    	movq	%rsi, %r9
1403152ef: e8 2c 78 1a 00              	callq	0x1404bcb20 <.text+0x4bbb20>
1403152f4: e9 3e 0a 00 00              	jmp	0x140315d37 <.text+0x314d37>
1403152f9: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
1403152fd: 48 8b 1f                    	movq	(%rdi), %rbx
140315300: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315304: 48 39 03                    	cmpq	%rax, (%rbx)
140315307: 0f 84 60 0b 00 00           	je	0x140315e6d <.text+0x314e6d>
14031530d: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315311: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315315: 48 ff c0                    	incq	%rax
140315318: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
14031531c: 4c 8d 05 a1 5e 6a 02        	leaq	0x26a5ea1(%rip), %r8    # 0x1429bb1c4
140315323: 41 b9 0e 00 00 00           	movl	$0xe, %r9d
140315329: 48 89 f9                    	movq	%rdi, %rcx
14031532c: e8 cf 0d 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315331: 48 85 c0                    	testq	%rax, %rax
140315334: 0f 85 18 0a 00 00           	jne	0x140315d52 <.text+0x314d52>
14031533a: 48 8b 1f                    	movq	(%rdi), %rbx
14031533d: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315341: 48 39 13                    	cmpq	%rdx, (%rbx)
140315344: 0f 84 b6 0e 00 00           	je	0x140316200 <.text+0x315200>
14031534a: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14031534e: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315352: 48 ff c2                    	incq	%rdx
140315355: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315359: 48 8b 1f                    	movq	(%rdi), %rbx
14031535c: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315360: 48 39 13                    	cmpq	%rdx, (%rbx)
140315363: 0f 84 bd 0e 00 00           	je	0x140316226 <.text+0x315226>
140315369: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14031536d: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315371: 48 ff c2                    	incq	%rdx
140315374: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315378: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
14031537c: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315381: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315386: 48 8d 15 4d 5d 6a 02        	leaq	0x26a5d4d(%rip), %rdx   # 0x1429bb0da
14031538d: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315392: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140315398: e8 03 be 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
14031539d: 48 85 c0                    	testq	%rax, %rax
1403153a0: 0f 85 b4 09 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403153a6: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
1403153aa: 48 8d 15 21 5e 6a 02        	leaq	0x26a5e21(%rip), %rdx   # 0x1429bb1d2
1403153b1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403153b6: 41 b8 17 00 00 00           	movl	$0x17, %r8d
1403153bc: e8 cf 6f 1a 00              	callq	0x1404bc390 <.text+0x4bb390>
1403153c1: 48 85 c0                    	testq	%rax, %rax
1403153c4: 0f 85 90 09 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403153ca: 48 83 c6 11                 	addq	$0x11, %rsi
1403153ce: 48 8d 15 14 5e 6a 02        	leaq	0x26a5e14(%rip), %rdx   # 0x1429bb1e9
1403153d5: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403153da: 41 b8 16 00 00 00           	movl	$0x16, %r8d
1403153e0: e9 df 02 00 00              	jmp	0x1403156c4 <.text+0x3146c4>
1403153e5: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
1403153e9: 48 8b 1f                    	movq	(%rdi), %rbx
1403153ec: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1403153f0: 48 39 03                    	cmpq	%rax, (%rbx)
1403153f3: 0f 84 a3 0a 00 00           	je	0x140315e9c <.text+0x314e9c>
1403153f9: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1403153fd: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315401: 48 ff c0                    	incq	%rax
140315404: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315408: 4c 8d 05 7b 5d 6a 02        	leaq	0x26a5d7b(%rip), %r8    # 0x1429bb18a
14031540f: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
140315415: 48 89 f9                    	movq	%rdi, %rcx
140315418: e8 e3 0c 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
14031541d: 48 85 c0                    	testq	%rax, %rax
140315420: 0f 85 2c 09 00 00           	jne	0x140315d52 <.text+0x314d52>
140315426: 48 8b 1f                    	movq	(%rdi), %rbx
140315429: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031542d: 48 39 13                    	cmpq	%rdx, (%rbx)
140315430: 0f 84 16 0e 00 00           	je	0x14031624c <.text+0x31524c>
140315436: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14031543a: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
14031543e: 48 ff c2                    	incq	%rdx
140315441: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315445: 48 8b 1f                    	movq	(%rdi), %rbx
140315448: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031544c: 48 39 13                    	cmpq	%rdx, (%rbx)
14031544f: 0f 84 1d 0e 00 00           	je	0x140316272 <.text+0x315272>
140315455: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315459: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
14031545d: 48 ff c2                    	incq	%rdx
140315460: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315464: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315468: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
14031546d: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315472: 48 8d 15 61 5c 6a 02        	leaq	0x26a5c61(%rip), %rdx   # 0x1429bb0da
140315479: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14031547e: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140315484: e8 17 bd 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140315489: 48 85 c0                    	testq	%rax, %rax
14031548c: 0f 85 c8 08 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315492: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
140315496: 48 8d 15 53 5c 6a 02        	leaq	0x26a5c53(%rip), %rdx   # 0x1429bb0f0
14031549d: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403154a2: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1403154a8: e8 53 5f 1a 00              	callq	0x1404bb400 <.text+0x4ba400>
1403154ad: 48 85 c0                    	testq	%rax, %rax
1403154b0: 0f 85 a4 08 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403154b6: 48 83 c6 11                 	addq	$0x11, %rsi
1403154ba: 48 8d 15 35 5c 6a 02        	leaq	0x26a5c35(%rip), %rdx   # 0x1429bb0f6
1403154c1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403154c6: 41 b8 05 00 00 00           	movl	$0x5, %r8d
1403154cc: 49 89 f1                    	movq	%rsi, %r9
1403154cf: e8 2c 5f 1a 00              	callq	0x1404bb400 <.text+0x4ba400>
1403154d4: e9 5e 08 00 00              	jmp	0x140315d37 <.text+0x314d37>
1403154d9: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
1403154dd: 48 8b 1f                    	movq	(%rdi), %rbx
1403154e0: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1403154e4: 48 39 03                    	cmpq	%rax, (%rbx)
1403154e7: 0f 84 de 09 00 00           	je	0x140315ecb <.text+0x314ecb>
1403154ed: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1403154f1: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1403154f5: 48 ff c0                    	incq	%rax
1403154f8: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
1403154fc: 4c 8d 05 1e 5c 6a 02        	leaq	0x26a5c1e(%rip), %r8    # 0x1429bb121
140315503: 41 b9 0a 00 00 00           	movl	$0xa, %r9d
140315509: 48 89 f9                    	movq	%rdi, %rcx
14031550c: e8 ef 0b 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315511: 48 85 c0                    	testq	%rax, %rax
140315514: 0f 85 38 08 00 00           	jne	0x140315d52 <.text+0x314d52>
14031551a: 48 8b 1f                    	movq	(%rdi), %rbx
14031551d: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315521: 48 39 13                    	cmpq	%rdx, (%rbx)
140315524: 0f 84 6e 0d 00 00           	je	0x140316298 <.text+0x315298>
14031552a: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14031552e: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315532: 48 ff c2                    	incq	%rdx
140315535: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315539: 48 8b 1f                    	movq	(%rdi), %rbx
14031553c: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315540: 48 39 13                    	cmpq	%rdx, (%rbx)
140315543: 0f 84 75 0d 00 00           	je	0x1403162be <.text+0x3152be>
140315549: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14031554d: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315551: 48 ff c2                    	incq	%rdx
140315554: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315558: 4c 8d 4e 78                 	leaq	0x78(%rsi), %r9
14031555c: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315561: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315566: 48 8d 15 6d 5b 6a 02        	leaq	0x26a5b6d(%rip), %rdx   # 0x1429bb0da
14031556d: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315572: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140315578: e8 23 bc 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
14031557d: 48 85 c0                    	testq	%rax, %rax
140315580: 0f 85 d4 07 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315586: 4c 8d 8e 80 00 00 00        	leaq	0x80(%rsi), %r9
14031558d: 48 8d 15 71 5b 6a 02        	leaq	0x26a5b71(%rip), %rdx   # 0x1429bb105
140315594: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315599: 41 b8 0b 00 00 00           	movl	$0xb, %r8d
14031559f: e8 ac 7c 1a 00              	callq	0x1404bd250 <.text+0x4bc250>
1403155a4: 48 85 c0                    	testq	%rax, %rax
1403155a7: 0f 85 ad 07 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403155ad: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
1403155b1: 48 8d 15 38 5b 6a 02        	leaq	0x26a5b38(%rip), %rdx   # 0x1429bb0f0
1403155b8: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403155bd: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1403155c3: e8 e8 a1 1a 00              	callq	0x1404bf7b0 <.text+0x4be7b0>
1403155c8: 48 85 c0                    	testq	%rax, %rax
1403155cb: 0f 85 89 07 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403155d1: 48 81 c6 84 00 00 00        	addq	$0x84, %rsi
1403155d8: e9 84 02 00 00              	jmp	0x140315861 <.text+0x314861>
1403155dd: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
1403155e1: 48 8b 1f                    	movq	(%rdi), %rbx
1403155e4: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1403155e8: 48 39 03                    	cmpq	%rax, (%rbx)
1403155eb: 0f 84 09 09 00 00           	je	0x140315efa <.text+0x314efa>
1403155f1: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1403155f5: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1403155f9: 48 ff c0                    	incq	%rax
1403155fc: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315600: 4c 8d 05 26 5c 6a 02        	leaq	0x26a5c26(%rip), %r8    # 0x1429bb22d
140315607: 41 b9 0d 00 00 00           	movl	$0xd, %r9d
14031560d: 48 89 f9                    	movq	%rdi, %rcx
140315610: e8 eb 0a 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315615: 48 85 c0                    	testq	%rax, %rax
140315618: 0f 85 34 07 00 00           	jne	0x140315d52 <.text+0x314d52>
14031561e: 48 8b 1f                    	movq	(%rdi), %rbx
140315621: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315625: 48 39 13                    	cmpq	%rdx, (%rbx)
140315628: 0f 84 b6 0c 00 00           	je	0x1403162e4 <.text+0x3152e4>
14031562e: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315632: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315636: 48 ff c2                    	incq	%rdx
140315639: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14031563d: 48 8b 1f                    	movq	(%rdi), %rbx
140315640: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315644: 48 39 13                    	cmpq	%rdx, (%rbx)
140315647: 0f 84 bd 0c 00 00           	je	0x14031630a <.text+0x31530a>
14031564d: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315651: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315655: 48 ff c2                    	incq	%rdx
140315658: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14031565c: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315660: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315665: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
14031566a: 48 8d 15 69 5a 6a 02        	leaq	0x26a5a69(%rip), %rdx   # 0x1429bb0da
140315671: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315676: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
14031567c: e8 1f bb 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140315681: 48 85 c0                    	testq	%rax, %rax
140315684: 0f 85 d0 06 00 00           	jne	0x140315d5a <.text+0x314d5a>
14031568a: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
14031568e: 48 8d 15 a5 5b 6a 02        	leaq	0x26a5ba5(%rip), %rdx   # 0x1429bb23a
140315695: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14031569a: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1403156a0: e8 4b 3a 1a 00              	callq	0x1404b90f0 <.text+0x4b80f0>
1403156a5: 48 85 c0                    	testq	%rax, %rax
1403156a8: 0f 85 ac 06 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403156ae: 48 83 c6 2c                 	addq	$0x2c, %rsi
1403156b2: 48 8d 15 91 5b 6a 02        	leaq	0x26a5b91(%rip), %rdx   # 0x1429bb24a
1403156b9: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403156be: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403156c4: 49 89 f1                    	movq	%rsi, %r9
1403156c7: e8 c4 6c 1a 00              	callq	0x1404bc390 <.text+0x4bb390>
1403156cc: e9 66 06 00 00              	jmp	0x140315d37 <.text+0x314d37>
1403156d1: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
1403156d5: 48 8b 1f                    	movq	(%rdi), %rbx
1403156d8: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1403156dc: 48 39 03                    	cmpq	%rax, (%rbx)
1403156df: 0f 84 44 08 00 00           	je	0x140315f29 <.text+0x314f29>
1403156e5: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1403156e9: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1403156ed: 48 ff c0                    	incq	%rax
1403156f0: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
1403156f4: 4c 8d 05 30 5a 6a 02        	leaq	0x26a5a30(%rip), %r8    # 0x1429bb12b
1403156fb: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
140315701: e9 92 03 00 00              	jmp	0x140315a98 <.text+0x314a98>
140315706: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
14031570a: 48 8b 1f                    	movq	(%rdi), %rbx
14031570d: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315711: 48 39 03                    	cmpq	%rax, (%rbx)
140315714: 0f 84 3e 08 00 00           	je	0x140315f58 <.text+0x314f58>
14031571a: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
14031571e: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315722: 48 ff c0                    	incq	%rax
140315725: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315729: 4c 8d 05 3c 5a 6a 02        	leaq	0x26a5a3c(%rip), %r8    # 0x1429bb16c
140315730: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140315736: e9 5d 03 00 00              	jmp	0x140315a98 <.text+0x314a98>
14031573b: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
14031573f: 48 8b 1f                    	movq	(%rdi), %rbx
140315742: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315746: 48 39 03                    	cmpq	%rax, (%rbx)
140315749: 0f 84 67 08 00 00           	je	0x140315fb6 <.text+0x314fb6>
14031574f: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315753: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315757: 48 ff c0                    	incq	%rax
14031575a: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
14031575e: 4c 8d 05 96 59 6a 02        	leaq	0x26a5996(%rip), %r8    # 0x1429bb0fb
140315765: 41 b9 0a 00 00 00           	movl	$0xa, %r9d
14031576b: 48 89 f9                    	movq	%rdi, %rcx
14031576e: e8 8d 09 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315773: 48 85 c0                    	testq	%rax, %rax
140315776: 0f 85 d6 05 00 00           	jne	0x140315d52 <.text+0x314d52>
14031577c: 48 8b 1f                    	movq	(%rdi), %rbx
14031577f: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315783: 48 39 13                    	cmpq	%rdx, (%rbx)
140315786: 0f 84 f0 0b 00 00           	je	0x14031637c <.text+0x31537c>
14031578c: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315790: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315794: 48 ff c2                    	incq	%rdx
140315797: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14031579b: 48 8b 1f                    	movq	(%rdi), %rbx
14031579e: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403157a2: 48 39 13                    	cmpq	%rdx, (%rbx)
1403157a5: 0f 84 f7 0b 00 00           	je	0x1403163a2 <.text+0x3153a2>
1403157ab: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403157af: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1403157b3: 48 ff c2                    	incq	%rdx
1403157b6: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403157ba: 4c 8d 4e 78                 	leaq	0x78(%rsi), %r9
1403157be: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1403157c3: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
1403157c8: 48 8d 15 0b 59 6a 02        	leaq	0x26a590b(%rip), %rdx   # 0x1429bb0da
1403157cf: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403157d4: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1403157da: e8 c1 b9 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
1403157df: 48 85 c0                    	testq	%rax, %rax
1403157e2: 0f 85 72 05 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403157e8: 4c 8d 8e 80 00 00 00        	leaq	0x80(%rsi), %r9
1403157ef: 48 8d 15 0f 59 6a 02        	leaq	0x26a590f(%rip), %rdx   # 0x1429bb105
1403157f6: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403157fb: 41 b8 0b 00 00 00           	movl	$0xb, %r8d
140315801: e8 4a 7a 1a 00              	callq	0x1404bd250 <.text+0x4bc250>
140315806: 48 85 c0                    	testq	%rax, %rax
140315809: 0f 85 4b 05 00 00           	jne	0x140315d5a <.text+0x314d5a>
14031580f: 4c 8d 8e 84 00 00 00        	leaq	0x84(%rsi), %r9
140315816: 48 8d 15 f3 58 6a 02        	leaq	0x26a58f3(%rip), %rdx   # 0x1429bb110
14031581d: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315822: 41 b8 09 00 00 00           	movl	$0x9, %r8d
140315828: e8 53 84 1a 00              	callq	0x1404bdc80 <.text+0x4bcc80>
14031582d: 48 85 c0                    	testq	%rax, %rax
140315830: 0f 85 24 05 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315836: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
14031583a: 48 8d 15 b5 58 6a 02        	leaq	0x26a58b5(%rip), %rdx   # 0x1429bb0f6
140315841: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315846: 41 b8 05 00 00 00           	movl	$0x5, %r8d
14031584c: e8 5f 9f 1a 00              	callq	0x1404bf7b0 <.text+0x4be7b0>
140315851: 48 85 c0                    	testq	%rax, %rax
140315854: 0f 85 00 05 00 00           	jne	0x140315d5a <.text+0x314d5a>
14031585a: 48 81 c6 88 00 00 00        	addq	$0x88, %rsi
140315861: 48 8d 15 b1 58 6a 02        	leaq	0x26a58b1(%rip), %rdx   # 0x1429bb119
140315868: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14031586d: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140315873: 49 89 f1                    	movq	%rsi, %r9
140315876: e8 35 52 1a 00              	callq	0x1404baab0 <.text+0x4b9ab0>
14031587b: e9 b7 04 00 00              	jmp	0x140315d37 <.text+0x314d37>
140315880: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315884: 48 8b 1f                    	movq	(%rdi), %rbx
140315887: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031588b: 48 39 03                    	cmpq	%rax, (%rbx)
14031588e: 0f 84 51 07 00 00           	je	0x140315fe5 <.text+0x314fe5>
140315894: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315898: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
14031589c: 48 ff c0                    	incq	%rax
14031589f: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
1403158a3: 4c 8d 05 a2 58 6a 02        	leaq	0x26a58a2(%rip), %r8    # 0x1429bb14c
1403158aa: 41 b9 11 00 00 00           	movl	$0x11, %r9d
1403158b0: 48 89 f9                    	movq	%rdi, %rcx
1403158b3: e8 48 08 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
1403158b8: 48 85 c0                    	testq	%rax, %rax
1403158bb: 0f 85 91 04 00 00           	jne	0x140315d52 <.text+0x314d52>
1403158c1: 48 8b 1f                    	movq	(%rdi), %rbx
1403158c4: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403158c8: 48 39 13                    	cmpq	%rdx, (%rbx)
1403158cb: 0f 84 f7 0a 00 00           	je	0x1403163c8 <.text+0x3153c8>
1403158d1: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403158d5: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1403158d9: 48 ff c2                    	incq	%rdx
1403158dc: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403158e0: 48 8b 1f                    	movq	(%rdi), %rbx
1403158e3: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403158e7: 48 39 13                    	cmpq	%rdx, (%rbx)
1403158ea: 0f 84 fe 0a 00 00           	je	0x1403163ee <.text+0x3153ee>
1403158f0: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403158f4: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1403158f8: 48 ff c2                    	incq	%rdx
1403158fb: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403158ff: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315903: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315908: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
14031590d: 48 8d 15 c6 57 6a 02        	leaq	0x26a57c6(%rip), %rdx   # 0x1429bb0da
140315914: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315919: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
14031591f: e8 7c b8 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140315924: 48 85 c0                    	testq	%rax, %rax
140315927: 0f 85 2d 04 00 00           	jne	0x140315d5a <.text+0x314d5a>
14031592d: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
140315931: 48 8d 15 b8 57 6a 02        	leaq	0x26a57b8(%rip), %rdx   # 0x1429bb0f0
140315938: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14031593d: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140315943: e8 a8 37 1a 00              	callq	0x1404b90f0 <.text+0x4b80f0>
140315948: 48 85 c0                    	testq	%rax, %rax
14031594b: 0f 85 09 04 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315951: 48 83 c6 2c                 	addq	$0x2c, %rsi
140315955: 48 8d 15 9a 57 6a 02        	leaq	0x26a579a(%rip), %rdx   # 0x1429bb0f6
14031595c: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315961: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140315967: 49 89 f1                    	movq	%rsi, %r9
14031596a: e8 81 37 1a 00              	callq	0x1404b90f0 <.text+0x4b80f0>
14031596f: e9 c3 03 00 00              	jmp	0x140315d37 <.text+0x314d37>
140315974: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315978: 48 8b 1f                    	movq	(%rdi), %rbx
14031597b: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031597f: 48 39 03                    	cmpq	%rax, (%rbx)
140315982: 0f 84 8c 06 00 00           	je	0x140316014 <.text+0x315014>
140315988: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
14031598c: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315990: 48 ff c0                    	incq	%rax
140315993: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315997: 4c 8d 05 61 58 6a 02        	leaq	0x26a5861(%rip), %r8    # 0x1429bb1ff
14031599e: 41 b9 11 00 00 00           	movl	$0x11, %r9d
1403159a4: 48 89 f9                    	movq	%rdi, %rcx
1403159a7: e8 54 07 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
1403159ac: 48 85 c0                    	testq	%rax, %rax
1403159af: 0f 85 9d 03 00 00           	jne	0x140315d52 <.text+0x314d52>
1403159b5: 48 8b 1f                    	movq	(%rdi), %rbx
1403159b8: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403159bc: 48 39 13                    	cmpq	%rdx, (%rbx)
1403159bf: 0f 84 4f 0a 00 00           	je	0x140316414 <.text+0x315414>
1403159c5: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403159c9: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1403159cd: 48 ff c2                    	incq	%rdx
1403159d0: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403159d4: 48 8b 1f                    	movq	(%rdi), %rbx
1403159d7: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403159db: 48 39 13                    	cmpq	%rdx, (%rbx)
1403159de: 0f 84 56 0a 00 00           	je	0x14031643a <.text+0x31543a>
1403159e4: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403159e8: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1403159ec: 48 ff c2                    	incq	%rdx
1403159ef: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403159f3: 4c 8d 4e 78                 	leaq	0x78(%rsi), %r9
1403159f7: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1403159fc: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315a01: 48 8d 15 d2 56 6a 02        	leaq	0x26a56d2(%rip), %rdx   # 0x1429bb0da
140315a08: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315a0d: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140315a13: e8 88 b7 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140315a18: 48 85 c0                    	testq	%rax, %rax
140315a1b: 0f 85 39 03 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315a21: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315a25: 48 8d 15 c4 56 6a 02        	leaq	0x26a56c4(%rip), %rdx   # 0x1429bb0f0
140315a2c: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315a31: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140315a37: e8 94 9c 1a 00              	callq	0x1404bf6d0 <.text+0x4be6d0>
140315a3c: 48 85 c0                    	testq	%rax, %rax
140315a3f: 0f 85 15 03 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315a45: 48 83 c6 40                 	addq	$0x40, %rsi
140315a49: 48 8d 15 a6 56 6a 02        	leaq	0x26a56a6(%rip), %rdx   # 0x1429bb0f6
140315a50: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315a55: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140315a5b: 49 89 f1                    	movq	%rsi, %r9
140315a5e: e8 6d 9c 1a 00              	callq	0x1404bf6d0 <.text+0x4be6d0>
140315a63: e9 cf 02 00 00              	jmp	0x140315d37 <.text+0x314d37>
140315a68: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315a6c: 48 8b 1f                    	movq	(%rdi), %rbx
140315a6f: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315a73: 48 39 03                    	cmpq	%rax, (%rbx)
140315a76: 0f 84 c7 05 00 00           	je	0x140316043 <.text+0x315043>
140315a7c: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315a80: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315a84: 48 ff c0                    	incq	%rax
140315a87: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315a8b: 4c 8d 05 cb 56 6a 02        	leaq	0x26a56cb(%rip), %r8    # 0x1429bb15d
140315a92: 41 b9 0f 00 00 00           	movl	$0xf, %r9d
140315a98: 48 89 f9                    	movq	%rdi, %rcx
140315a9b: e8 60 06 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315aa0: 48 85 c0                    	testq	%rax, %rax
140315aa3: 0f 85 a9 02 00 00           	jne	0x140315d52 <.text+0x314d52>
140315aa9: 48 8b 1f                    	movq	(%rdi), %rbx
140315aac: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315ab0: 48 39 13                    	cmpq	%rdx, (%rbx)
140315ab3: 0f 84 ac 02 00 00           	je	0x140315d65 <.text+0x314d65>
140315ab9: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315abd: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315ac1: 48 ff c2                    	incq	%rdx
140315ac4: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315ac8: 48 8b 1f                    	movq	(%rdi), %rbx
140315acb: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315acf: 48 39 13                    	cmpq	%rdx, (%rbx)
140315ad2: 0f 84 b3 02 00 00           	je	0x140315d8b <.text+0x314d8b>
140315ad8: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315adc: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315ae0: 48 ff c2                    	incq	%rdx
140315ae3: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315ae7: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315aeb: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315af0: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315af5: 48 8d 15 de 55 6a 02        	leaq	0x26a55de(%rip), %rdx   # 0x1429bb0da
140315afc: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315b01: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140315b07: e8 94 b6 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140315b0c: 48 85 c0                    	testq	%rax, %rax
140315b0f: 0f 85 45 02 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315b15: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
140315b19: 48 8d 15 d0 55 6a 02        	leaq	0x26a55d0(%rip), %rdx   # 0x1429bb0f0
140315b20: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315b25: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140315b2b: e8 50 81 1a 00              	callq	0x1404bdc80 <.text+0x4bcc80>
140315b30: 48 85 c0                    	testq	%rax, %rax
140315b33: 0f 85 21 02 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315b39: 48 83 c6 14                 	addq	$0x14, %rsi
140315b3d: 48 8d 15 b2 55 6a 02        	leaq	0x26a55b2(%rip), %rdx   # 0x1429bb0f6
140315b44: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315b49: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140315b4f: 49 89 f1                    	movq	%rsi, %r9
140315b52: e8 29 81 1a 00              	callq	0x1404bdc80 <.text+0x4bcc80>
140315b57: e9 db 01 00 00              	jmp	0x140315d37 <.text+0x314d37>
140315b5c: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315b60: 48 8b 1f                    	movq	(%rdi), %rbx
140315b63: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315b67: 48 39 03                    	cmpq	%rax, (%rbx)
140315b6a: 0f 84 02 05 00 00           	je	0x140316072 <.text+0x315072>
140315b70: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315b74: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315b78: 48 ff c0                    	incq	%rax
140315b7b: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315b7f: 4c 8d 05 0f 56 6a 02        	leaq	0x26a560f(%rip), %r8    # 0x1429bb195
140315b86: 41 b9 17 00 00 00           	movl	$0x17, %r9d
140315b8c: 48 89 f9                    	movq	%rdi, %rcx
140315b8f: e8 6c 05 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315b94: 48 85 c0                    	testq	%rax, %rax
140315b97: 0f 85 b5 01 00 00           	jne	0x140315d52 <.text+0x314d52>
140315b9d: 48 8b 1f                    	movq	(%rdi), %rbx
140315ba0: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315ba4: 48 39 13                    	cmpq	%rdx, (%rbx)
140315ba7: 0f 84 b3 08 00 00           	je	0x140316460 <.text+0x315460>
140315bad: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315bb1: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315bb5: 48 ff c2                    	incq	%rdx
140315bb8: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315bbc: 48 8b 1f                    	movq	(%rdi), %rbx
140315bbf: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315bc3: 48 39 13                    	cmpq	%rdx, (%rbx)
140315bc6: 0f 84 ba 08 00 00           	je	0x140316486 <.text+0x315486>
140315bcc: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315bd0: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315bd4: 48 ff c2                    	incq	%rdx
140315bd7: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315bdb: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315bdf: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315be4: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315be9: 48 8d 15 ea 54 6a 02        	leaq	0x26a54ea(%rip), %rdx   # 0x1429bb0da
140315bf0: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315bf5: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140315bfb: e8 a0 b5 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140315c00: 48 85 c0                    	testq	%rax, %rax
140315c03: 0f 85 51 01 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315c09: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
140315c0d: 48 8d 15 dc 54 6a 02        	leaq	0x26a54dc(%rip), %rdx   # 0x1429bb0f0
140315c14: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315c19: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140315c1f: e8 2c 76 1a 00              	callq	0x1404bd250 <.text+0x4bc250>
140315c24: 48 85 c0                    	testq	%rax, %rax
140315c27: 0f 85 2d 01 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315c2d: 48 83 c6 14                 	addq	$0x14, %rsi
140315c31: 48 8d 15 be 54 6a 02        	leaq	0x26a54be(%rip), %rdx   # 0x1429bb0f6
140315c38: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315c3d: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140315c43: 49 89 f1                    	movq	%rsi, %r9
140315c46: e8 05 76 1a 00              	callq	0x1404bd250 <.text+0x4bc250>
140315c4b: e9 e7 00 00 00              	jmp	0x140315d37 <.text+0x314d37>
140315c50: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315c54: 48 8b 1f                    	movq	(%rdi), %rbx
140315c57: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315c5b: 48 39 03                    	cmpq	%rax, (%rbx)
140315c5e: 0f 84 3d 04 00 00           	je	0x1403160a1 <.text+0x3150a1>
140315c64: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315c68: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315c6c: 48 ff c0                    	incq	%rax
140315c6f: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315c73: 4c 8d 05 02 55 6a 02        	leaq	0x26a5502(%rip), %r8    # 0x1429bb17c
140315c7a: 41 b9 0e 00 00 00           	movl	$0xe, %r9d
140315c80: 48 89 f9                    	movq	%rdi, %rcx
140315c83: e8 78 04 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315c88: 48 85 c0                    	testq	%rax, %rax
140315c8b: 0f 85 c1 00 00 00           	jne	0x140315d52 <.text+0x314d52>
140315c91: 48 8b 1f                    	movq	(%rdi), %rbx
140315c94: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315c98: 48 39 13                    	cmpq	%rdx, (%rbx)
140315c9b: 0f 84 0b 08 00 00           	je	0x1403164ac <.text+0x3154ac>
140315ca1: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315ca5: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315ca9: 48 ff c2                    	incq	%rdx
140315cac: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315cb0: 48 8b 1f                    	movq	(%rdi), %rbx
140315cb3: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315cb7: 48 39 13                    	cmpq	%rdx, (%rbx)
140315cba: 0f 84 12 08 00 00           	je	0x1403164d2 <.text+0x3154d2>
140315cc0: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315cc4: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315cc8: 48 ff c2                    	incq	%rdx
140315ccb: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315ccf: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315cd3: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315cd8: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315cdd: 48 8d 15 f6 53 6a 02        	leaq	0x26a53f6(%rip), %rdx   # 0x1429bb0da
140315ce4: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315ce9: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140315cef: e8 ac b4 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140315cf4: 48 85 c0                    	testq	%rax, %rax
140315cf7: 75 61                       	jne	0x140315d5a <.text+0x314d5a>
140315cf9: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
140315cfd: 48 8d 15 ec 53 6a 02        	leaq	0x26a53ec(%rip), %rdx   # 0x1429bb0f0
140315d04: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315d09: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140315d0f: e8 0c 3f 1a 00              	callq	0x1404b9c20 <.text+0x4b8c20>
140315d14: 48 85 c0                    	testq	%rax, %rax
140315d17: 75 41                       	jne	0x140315d5a <.text+0x314d5a>
140315d19: 48 83 c6 11                 	addq	$0x11, %rsi
140315d1d: 48 8d 15 d2 53 6a 02        	leaq	0x26a53d2(%rip), %rdx   # 0x1429bb0f6
140315d24: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315d29: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140315d2f: 49 89 f1                    	movq	%rsi, %r9
140315d32: e8 e9 3e 1a 00              	callq	0x1404b9c20 <.text+0x4b8c20>
140315d37: 48 85 c0                    	testq	%rax, %rax
140315d3a: 75 1e                       	jne	0x140315d5a <.text+0x314d5a>
140315d3c: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
140315d41: 0f b6 54 24 38              	movzbl	0x38(%rsp), %edx
140315d46: 48 8b 08                    	movq	(%rax), %rcx
140315d49: e8 52 83 d2 ff              	callq	0x14003e0a0 <.text+0x3d0a0>
140315d4e: 31 c0                       	xorl	%eax, %eax
140315d50: eb 08                       	jmp	0x140315d5a <.text+0x314d5a>
140315d52: 48 89 c1                    	movq	%rax, %rcx
140315d55: e8 46 3b 5b 02              	callq	0x1428c98a0 <.text+0x28c88a0>
140315d5a: 90                          	nop
140315d5b: 48 83 c4 48                 	addq	$0x48, %rsp
140315d5f: 5b                          	popq	%rbx
140315d60: 5f                          	popq	%rdi
140315d61: 5e                          	popq	%rsi
140315d62: 41 5e                       	popq	%r14
140315d64: c3                          	retq
140315d65: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315d6e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315d74: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315d7a: 48 89 d9                    	movq	%rbx, %rcx
140315d7d: e8 5e cc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315d82: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315d86: e9 2e fd ff ff              	jmp	0x140315ab9 <.text+0x314ab9>
140315d8b: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315d94: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315d9a: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315da0: 48 89 d9                    	movq	%rbx, %rcx
140315da3: e8 38 cc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315da8: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315dac: e9 27 fd ff ff              	jmp	0x140315ad8 <.text+0x314ad8>
140315db1: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315dba: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315dc0: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315dc6: 48 89 d9                    	movq	%rbx, %rcx
140315dc9: 49 89 d6                    	movq	%rdx, %r14
140315dcc: 48 89 c2                    	movq	%rax, %rdx
140315dcf: e8 0c cc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315dd4: 4c 89 f2                    	movq	%r14, %rdx
140315dd7: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315ddb: e9 54 f1 ff ff              	jmp	0x140314f34 <.text+0x313f34>
140315de0: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315de9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315def: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315df5: 48 89 d9                    	movq	%rbx, %rcx
140315df8: 49 89 d6                    	movq	%rdx, %r14
140315dfb: 48 89 c2                    	movq	%rax, %rdx
140315dfe: e8 dd cb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315e03: 4c 89 f2                    	movq	%r14, %rdx
140315e06: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315e0a: e9 11 f2 ff ff              	jmp	0x140315020 <.text+0x314020>
140315e0f: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315e18: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315e1e: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315e24: 48 89 d9                    	movq	%rbx, %rcx
140315e27: 49 89 d6                    	movq	%rdx, %r14
140315e2a: 48 89 c2                    	movq	%rax, %rdx
140315e2d: e8 ae cb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315e32: 4c 89 f2                    	movq	%r14, %rdx
140315e35: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315e39: e9 e7 f2 ff ff              	jmp	0x140315125 <.text+0x314125>
140315e3e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315e47: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315e4d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315e53: 48 89 d9                    	movq	%rbx, %rcx
140315e56: 49 89 d6                    	movq	%rdx, %r14
140315e59: 48 89 c2                    	movq	%rax, %rdx
140315e5c: e8 7f cb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315e61: 4c 89 f2                    	movq	%r14, %rdx
140315e64: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315e68: e9 ac f3 ff ff              	jmp	0x140315219 <.text+0x314219>
140315e6d: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315e76: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315e7c: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315e82: 48 89 d9                    	movq	%rbx, %rcx
140315e85: 49 89 d6                    	movq	%rdx, %r14
140315e88: 48 89 c2                    	movq	%rax, %rdx
140315e8b: e8 50 cb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315e90: 4c 89 f2                    	movq	%r14, %rdx
140315e93: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315e97: e9 71 f4 ff ff              	jmp	0x14031530d <.text+0x31430d>
140315e9c: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315ea5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315eab: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315eb1: 48 89 d9                    	movq	%rbx, %rcx
140315eb4: 49 89 d6                    	movq	%rdx, %r14
140315eb7: 48 89 c2                    	movq	%rax, %rdx
140315eba: e8 21 cb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315ebf: 4c 89 f2                    	movq	%r14, %rdx
140315ec2: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315ec6: e9 2e f5 ff ff              	jmp	0x1403153f9 <.text+0x3143f9>
140315ecb: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315ed4: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315eda: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315ee0: 48 89 d9                    	movq	%rbx, %rcx
140315ee3: 49 89 d6                    	movq	%rdx, %r14
140315ee6: 48 89 c2                    	movq	%rax, %rdx
140315ee9: e8 f2 ca 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315eee: 4c 89 f2                    	movq	%r14, %rdx
140315ef1: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315ef5: e9 f3 f5 ff ff              	jmp	0x1403154ed <.text+0x3144ed>
140315efa: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315f03: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315f09: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315f0f: 48 89 d9                    	movq	%rbx, %rcx
140315f12: 49 89 d6                    	movq	%rdx, %r14
140315f15: 48 89 c2                    	movq	%rax, %rdx
140315f18: e8 c3 ca 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315f1d: 4c 89 f2                    	movq	%r14, %rdx
140315f20: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315f24: e9 c8 f6 ff ff              	jmp	0x1403155f1 <.text+0x3145f1>
140315f29: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315f32: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315f38: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315f3e: 48 89 d9                    	movq	%rbx, %rcx
140315f41: 49 89 d6                    	movq	%rdx, %r14
140315f44: 48 89 c2                    	movq	%rax, %rdx
140315f47: e8 94 ca 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315f4c: 4c 89 f2                    	movq	%r14, %rdx
140315f4f: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315f53: e9 8d f7 ff ff              	jmp	0x1403156e5 <.text+0x3146e5>
140315f58: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315f61: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315f67: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315f6d: 48 89 d9                    	movq	%rbx, %rcx
140315f70: 49 89 d6                    	movq	%rdx, %r14
140315f73: 48 89 c2                    	movq	%rax, %rdx
140315f76: e8 65 ca 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315f7b: 4c 89 f2                    	movq	%r14, %rdx
140315f7e: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315f82: e9 93 f7 ff ff              	jmp	0x14031571a <.text+0x31471a>
140315f87: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315f90: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315f96: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315f9c: 48 89 d9                    	movq	%rbx, %rcx
140315f9f: 49 89 d6                    	movq	%rdx, %r14
140315fa2: 48 89 c2                    	movq	%rax, %rdx
140315fa5: e8 36 ca 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315faa: 4c 89 f2                    	movq	%r14, %rdx
140315fad: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315fb1: e9 61 ee ff ff              	jmp	0x140314e17 <.text+0x313e17>
140315fb6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315fbf: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315fc5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315fcb: 48 89 d9                    	movq	%rbx, %rcx
140315fce: 49 89 d6                    	movq	%rdx, %r14
140315fd1: 48 89 c2                    	movq	%rax, %rdx
140315fd4: e8 07 ca 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315fd9: 4c 89 f2                    	movq	%r14, %rdx
140315fdc: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315fe0: e9 6a f7 ff ff              	jmp	0x14031574f <.text+0x31474f>
140315fe5: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315fee: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315ff4: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315ffa: 48 89 d9                    	movq	%rbx, %rcx
140315ffd: 49 89 d6                    	movq	%rdx, %r14
140316000: 48 89 c2                    	movq	%rax, %rdx
140316003: e8 d8 c9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316008: 4c 89 f2                    	movq	%r14, %rdx
14031600b: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031600f: e9 80 f8 ff ff              	jmp	0x140315894 <.text+0x314894>
140316014: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031601d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316023: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316029: 48 89 d9                    	movq	%rbx, %rcx
14031602c: 49 89 d6                    	movq	%rdx, %r14
14031602f: 48 89 c2                    	movq	%rax, %rdx
140316032: e8 a9 c9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316037: 4c 89 f2                    	movq	%r14, %rdx
14031603a: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031603e: e9 45 f9 ff ff              	jmp	0x140315988 <.text+0x314988>
140316043: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031604c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316052: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316058: 48 89 d9                    	movq	%rbx, %rcx
14031605b: 49 89 d6                    	movq	%rdx, %r14
14031605e: 48 89 c2                    	movq	%rax, %rdx
140316061: e8 7a c9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316066: 4c 89 f2                    	movq	%r14, %rdx
140316069: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031606d: e9 0a fa ff ff              	jmp	0x140315a7c <.text+0x314a7c>
140316072: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031607b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316081: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316087: 48 89 d9                    	movq	%rbx, %rcx
14031608a: 49 89 d6                    	movq	%rdx, %r14
14031608d: 48 89 c2                    	movq	%rax, %rdx
140316090: e8 4b c9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316095: 4c 89 f2                    	movq	%r14, %rdx
140316098: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031609c: e9 cf fa ff ff              	jmp	0x140315b70 <.text+0x314b70>
1403160a1: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403160aa: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403160b0: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403160b6: 48 89 d9                    	movq	%rbx, %rcx
1403160b9: 49 89 d6                    	movq	%rdx, %r14
1403160bc: 48 89 c2                    	movq	%rax, %rdx
1403160bf: e8 1c c9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403160c4: 4c 89 f2                    	movq	%r14, %rdx
1403160c7: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1403160cb: e9 94 fb ff ff              	jmp	0x140315c64 <.text+0x314c64>
1403160d0: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403160d9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403160df: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403160e5: 48 89 d9                    	movq	%rbx, %rcx
1403160e8: e8 f3 c8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403160ed: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403160f1: e9 7b ee ff ff              	jmp	0x140314f71 <.text+0x313f71>
1403160f6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403160ff: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316105: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031610b: 48 89 d9                    	movq	%rbx, %rcx
14031610e: e8 cd c8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316113: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316117: e9 74 ee ff ff              	jmp	0x140314f90 <.text+0x313f90>
14031611c: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316125: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031612b: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316131: 48 89 d9                    	movq	%rbx, %rcx
140316134: e8 a7 c8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316139: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031613d: e9 1b ef ff ff              	jmp	0x14031505d <.text+0x31405d>
140316142: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031614b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316151: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316157: 48 89 d9                    	movq	%rbx, %rcx
14031615a: e8 81 c8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031615f: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316163: e9 14 ef ff ff              	jmp	0x14031507c <.text+0x31407c>
140316168: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316171: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316177: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031617d: 48 89 d9                    	movq	%rbx, %rcx
140316180: e8 5b c8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316185: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316189: e9 d4 ef ff ff              	jmp	0x140315162 <.text+0x314162>
14031618e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316197: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031619d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403161a3: 48 89 d9                    	movq	%rbx, %rcx
1403161a6: e8 35 c8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403161ab: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403161af: e9 cd ef ff ff              	jmp	0x140315181 <.text+0x314181>
1403161b4: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403161bd: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403161c3: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403161c9: 48 89 d9                    	movq	%rbx, %rcx
1403161cc: e8 0f c8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403161d1: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403161d5: e9 7c f0 ff ff              	jmp	0x140315256 <.text+0x314256>
1403161da: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403161e3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403161e9: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403161ef: 48 89 d9                    	movq	%rbx, %rcx
1403161f2: e8 e9 c7 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403161f7: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403161fb: e9 75 f0 ff ff              	jmp	0x140315275 <.text+0x314275>
140316200: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316209: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031620f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316215: 48 89 d9                    	movq	%rbx, %rcx
140316218: e8 c3 c7 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031621d: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316221: e9 24 f1 ff ff              	jmp	0x14031534a <.text+0x31434a>
140316226: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031622f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316235: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031623b: 48 89 d9                    	movq	%rbx, %rcx
14031623e: e8 9d c7 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316243: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316247: e9 1d f1 ff ff              	jmp	0x140315369 <.text+0x314369>
14031624c: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316255: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031625b: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316261: 48 89 d9                    	movq	%rbx, %rcx
140316264: e8 77 c7 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316269: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031626d: e9 c4 f1 ff ff              	jmp	0x140315436 <.text+0x314436>
140316272: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031627b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316281: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316287: 48 89 d9                    	movq	%rbx, %rcx
14031628a: e8 51 c7 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031628f: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316293: e9 bd f1 ff ff              	jmp	0x140315455 <.text+0x314455>
140316298: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403162a1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403162a7: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403162ad: 48 89 d9                    	movq	%rbx, %rcx
1403162b0: e8 2b c7 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403162b5: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403162b9: e9 6c f2 ff ff              	jmp	0x14031552a <.text+0x31452a>
1403162be: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403162c7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403162cd: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403162d3: 48 89 d9                    	movq	%rbx, %rcx
1403162d6: e8 05 c7 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403162db: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403162df: e9 65 f2 ff ff              	jmp	0x140315549 <.text+0x314549>
1403162e4: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403162ed: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403162f3: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403162f9: 48 89 d9                    	movq	%rbx, %rcx
1403162fc: e8 df c6 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316301: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316305: e9 24 f3 ff ff              	jmp	0x14031562e <.text+0x31462e>
14031630a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316313: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316319: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031631f: 48 89 d9                    	movq	%rbx, %rcx
140316322: e8 b9 c6 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316327: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031632b: e9 1d f3 ff ff              	jmp	0x14031564d <.text+0x31464d>
140316330: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316339: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031633f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316345: 48 89 d9                    	movq	%rbx, %rcx
140316348: e8 93 c6 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031634d: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316351: e9 fe ea ff ff              	jmp	0x140314e54 <.text+0x313e54>
140316356: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031635f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316365: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031636b: 48 89 d9                    	movq	%rbx, %rcx
14031636e: e8 6d c6 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316373: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316377: e9 f7 ea ff ff              	jmp	0x140314e73 <.text+0x313e73>
14031637c: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316385: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031638b: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316391: 48 89 d9                    	movq	%rbx, %rcx
140316394: e8 47 c6 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316399: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031639d: e9 ea f3 ff ff              	jmp	0x14031578c <.text+0x31478c>
1403163a2: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403163ab: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403163b1: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403163b7: 48 89 d9                    	movq	%rbx, %rcx
1403163ba: e8 21 c6 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403163bf: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403163c3: e9 e3 f3 ff ff              	jmp	0x1403157ab <.text+0x3147ab>
1403163c8: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403163d1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403163d7: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403163dd: 48 89 d9                    	movq	%rbx, %rcx
1403163e0: e8 fb c5 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403163e5: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403163e9: e9 e3 f4 ff ff              	jmp	0x1403158d1 <.text+0x3148d1>
1403163ee: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403163f7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403163fd: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316403: 48 89 d9                    	movq	%rbx, %rcx
140316406: e8 d5 c5 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031640b: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031640f: e9 dc f4 ff ff              	jmp	0x1403158f0 <.text+0x3148f0>
140316414: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031641d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316423: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316429: 48 89 d9                    	movq	%rbx, %rcx
14031642c: e8 af c5 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316431: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316435: e9 8b f5 ff ff              	jmp	0x1403159c5 <.text+0x3149c5>
14031643a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316443: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316449: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031644f: 48 89 d9                    	movq	%rbx, %rcx
140316452: e8 89 c5 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316457: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031645b: e9 84 f5 ff ff              	jmp	0x1403159e4 <.text+0x3149e4>
140316460: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316469: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031646f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316475: 48 89 d9                    	movq	%rbx, %rcx
140316478: e8 63 c5 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031647d: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316481: e9 27 f7 ff ff              	jmp	0x140315bad <.text+0x314bad>
140316486: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031648f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316495: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031649b: 48 89 d9                    	movq	%rbx, %rcx
14031649e: e8 3d c5 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403164a3: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403164a7: e9 20 f7 ff ff              	jmp	0x140315bcc <.text+0x314bcc>
1403164ac: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403164b5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403164bb: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403164c1: 48 89 d9                    	movq	%rbx, %rcx
1403164c4: e8 17 c5 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403164c9: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403164cd: e9 cf f7 ff ff              	jmp	0x140315ca1 <.text+0x314ca1>
1403164d2: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403164db: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403164e1: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403164e7: 48 89 d9                    	movq	%rbx, %rcx
1403164ea: e8 f1 c4 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403164ef: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403164f3: e9 c8 f7 ff ff              	jmp	0x140315cc0 <.text+0x314cc0>
1403164f8: cc                          	int3
1403164f9: cc                          	int3
1403164fa: cc                          	int3
1403164fb: cc                          	int3
1403164fc: cc                          	int3
1403164fd: cc                          	int3
1403164fe: cc                          	int3
1403164ff: cc                          	int3
140316500: 55                          	pushq	%rbp
140316501: 56                          	pushq	%rsi
140316502: 57                          	pushq	%rdi
140316503: 48 81 ec 90 00 00 00        	subq	$0x90, %rsp
14031650a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140316512: 48 c7 45 08 fe ff ff ff     	movq	$-0x2, 0x8(%rbp)
14031651a: 48 89 d7                    	movq	%rdx, %rdi
14031651d: 48 89 ce                    	movq	%rcx, %rsi
140316520: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14031652a: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
14031652e: 48 c7 45 e8 00 00 00 00     	movq	$0x0, -0x18(%rbp)
140316536: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
14031653e: 4c 8d 4a 08                 	leaq	0x8(%rdx), %r9
140316542: c6 45 07 01                 	movb	$0x1, 0x7(%rbp)
140316546: 48 8d 15 10 4d 6a 02        	leaq	0x26a4d10(%rip), %rdx   # 0x1429bb25d
14031654d: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
140316551: 41 b8 0b 00 00 00           	movl	$0xb, %r8d
140316557: e8 a4 d5 0c 00              	callq	0x1403e3b00 <.text+0x3e2b00>
14031655c: 48 85 c0                    	testq	%rax, %rax
14031655f: 75 22                       	jne	0x140316583 <.text+0x315583>
140316561: c6 45 07 01                 	movb	$0x1, 0x7(%rbp)
140316565: 48 8d 15 fc 4c 6a 02        	leaq	0x26a4cfc(%rip), %rdx   # 0x1429bb268
14031656c: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
140316570: 41 b8 0b 00 00 00           	movl	$0xb, %r8d
140316576: 49 89 f9                    	movq	%rdi, %r9
140316579: e8 a2 cc 0c 00              	callq	0x1403e3220 <.text+0x3e2220>
14031657e: 48 85 c0                    	testq	%rax, %rax
140316581: 74 2f                       	je	0x1403165b2 <.text+0x3155b2>
140316583: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
140316587: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14031658b: c6 06 06                    	movb	$0x6, (%rsi)
14031658e: e8 dd 75 2f 00              	callq	0x14060db70 <.text+0x60cb70>
140316593: 48 8b 55 d0                 	movq	-0x30(%rbp), %rdx
140316597: 48 89 d0                    	movq	%rdx, %rax
14031659a: 48 f7 d8                    	negq	%rax
14031659d: 70 3b                       	jo	0x1403165da <.text+0x3155da>
14031659f: 73 39                       	jae	0x1403165da <.text+0x3155da>
1403165a1: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
1403165a5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403165ab: e8 70 d6 2f 00              	callq	0x140613c20 <.text+0x612c20>
1403165b0: eb 28                       	jmp	0x1403165da <.text+0x3155da>
1403165b2: 0f 10 45 d0                 	movups	-0x30(%rbp), %xmm0
1403165b6: 0f 10 4d e0                 	movups	-0x20(%rbp), %xmm1
1403165ba: 0f 10 55 f0                 	movups	-0x10(%rbp), %xmm2
1403165be: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
1403165c2: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
1403165c6: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
1403165ca: c6 45 07 00                 	movb	$0x0, 0x7(%rbp)
1403165ce: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
1403165d2: 48 89 f1                    	movq	%rsi, %rcx
1403165d5: e8 e6 fe 86 00              	callq	0x140b864c0 <.text+0xb854c0>
1403165da: 48 89 f0                    	movq	%rsi, %rax
1403165dd: 48 81 c4 90 00 00 00        	addq	$0x90, %rsp
1403165e4: 5f                          	popq	%rdi
1403165e5: 5e                          	popq	%rsi
1403165e6: 5d                          	popq	%rbp
1403165e7: c3                          	retq
1403165e8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1403165f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403165f5: 55                          	pushq	%rbp
1403165f6: 56                          	pushq	%rsi
1403165f7: 57                          	pushq	%rdi
1403165f8: 48 83 ec 20                 	subq	$0x20, %rsp
1403165fc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140316603: 48 8b 55 d0                 	movq	-0x30(%rbp), %rdx
140316607: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
14031660f: 48 85 c0                    	testq	%rax, %rax
140316612: 74 0f                       	je	0x140316623 <.text+0x315623>
140316614: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
140316618: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031661e: e8 fd d5 2f 00              	callq	0x140613c20 <.text+0x612c20>
140316623: 90                          	nop
140316624: 48 83 c4 20                 	addq	$0x20, %rsp
140316628: 5f                          	popq	%rdi
140316629: 5e                          	popq	%rsi
14031662a: 5d                          	popq	%rbp
14031662b: c3                          	retq
14031662c: 0f 1f 40 00                 	nopl	(%rax)
140316630: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140316635: 55                          	pushq	%rbp
140316636: 56                          	pushq	%rsi
140316637: 57                          	pushq	%rdi
140316638: 48 83 ec 20                 	subq	$0x20, %rsp
14031663c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140316643: 80 7d 07 00                 	cmpb	$0x0, 0x7(%rbp)
140316647: 75 08                       	jne	0x140316651 <.text+0x315651>
140316649: 48 83 c4 20                 	addq	$0x20, %rsp
14031664d: 5f                          	popq	%rdi
14031664e: 5e                          	popq	%rsi
14031664f: 5d                          	popq	%rbp
140316650: c3                          	retq
140316651: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
140316655: e8 a6 a3 d6 ff              	callq	0x140080a00 <.text+0x7fa00>
14031665a: eb ed                       	jmp	0x140316649 <.text+0x315649>
14031665c: cc                          	int3
14031665d: cc                          	int3
14031665e: cc                          	int3
14031665f: cc                          	int3
140316660: 41 57                       	pushq	%r15
140316662: 41 56                       	pushq	%r14
140316664: 56                          	pushq	%rsi
140316665: 57                          	pushq	%rdi
140316666: 53                          	pushq	%rbx
140316667: 48 83 ec 30                 	subq	$0x30, %rsp
14031666b: 48 89 d6                    	movq	%rdx, %rsi
14031666e: 48 8b 01                    	movq	(%rcx), %rax
140316671: 48 8d 15 c8 4c 6a 02        	leaq	0x26a4cc8(%rip), %rdx   # 0x1429bb340
140316678: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14031667c: 48 01 d0                    	addq	%rdx, %rax
14031667f: ff e0                       	jmpq	*%rax
140316681: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316685: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316689: 48 8b 1e                    	movq	(%rsi), %rbx
14031668c: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316690: 48 39 03                    	cmpq	%rax, (%rbx)
140316693: 0f 84 d9 07 00 00           	je	0x140316e72 <.text+0x315e72>
140316699: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
14031669d: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1403166a1: 48 ff c0                    	incq	%rax
1403166a4: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
1403166a8: 4c 8d 05 c4 4b 6a 02        	leaq	0x26a4bc4(%rip), %r8    # 0x1429bb273
1403166af: 41 b9 07 00 00 00           	movl	$0x7, %r9d
1403166b5: 48 89 f1                    	movq	%rsi, %rcx
1403166b8: e8 43 fa 19 00              	callq	0x1404b6100 <.text+0x4b5100>
1403166bd: 48 85 c0                    	testq	%rax, %rax
1403166c0: 0f 85 d5 05 00 00           	jne	0x140316c9b <.text+0x315c9b>
1403166c6: 48 8b 1e                    	movq	(%rsi), %rbx
1403166c9: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403166cd: 48 39 13                    	cmpq	%rdx, (%rbx)
1403166d0: 0f 84 ef 09 00 00           	je	0x1403170c5 <.text+0x3160c5>
1403166d6: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403166da: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1403166de: 48 ff c2                    	incq	%rdx
1403166e1: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403166e5: 48 83 c7 10                 	addq	$0x10, %rdi
1403166e9: 48 89 f9                    	movq	%rdi, %rcx
1403166ec: 48 89 f2                    	movq	%rsi, %rdx
1403166ef: e8 bc da 1d 00              	callq	0x1404f41b0 <.text+0x4f31b0>
1403166f4: e9 84 04 00 00              	jmp	0x140316b7d <.text+0x315b7d>
1403166f9: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
1403166fd: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316701: 48 8b 1e                    	movq	(%rsi), %rbx
140316704: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316708: 48 39 03                    	cmpq	%rax, (%rbx)
14031670b: 0f 84 e9 05 00 00           	je	0x140316cfa <.text+0x315cfa>
140316711: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316715: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316719: 48 ff c0                    	incq	%rax
14031671c: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316720: 4c 8d 05 b9 4b 6a 02        	leaq	0x26a4bb9(%rip), %r8    # 0x1429bb2e0
140316727: 41 b9 06 00 00 00           	movl	$0x6, %r9d
14031672d: 48 89 f1                    	movq	%rsi, %rcx
140316730: e8 cb f9 19 00              	callq	0x1404b6100 <.text+0x4b5100>
140316735: 48 85 c0                    	testq	%rax, %rax
140316738: 0f 85 5d 05 00 00           	jne	0x140316c9b <.text+0x315c9b>
14031673e: 48 8b 1e                    	movq	(%rsi), %rbx
140316741: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316745: 48 39 13                    	cmpq	%rdx, (%rbx)
140316748: 0f 84 6d 08 00 00           	je	0x140316fbb <.text+0x315fbb>
14031674e: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140316752: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140316756: 48 ff c2                    	incq	%rdx
140316759: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14031675d: 48 83 c7 10                 	addq	$0x10, %rdi
140316761: 48 89 f9                    	movq	%rdi, %rcx
140316764: 48 89 f2                    	movq	%rsi, %rdx
140316767: e8 84 33 f4 ff              	callq	0x140259af0 <.text+0x258af0>
14031676c: e9 0c 04 00 00              	jmp	0x140316b7d <.text+0x315b7d>
140316771: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316775: 48 8b 3e                    	movq	(%rsi), %rdi
140316778: 48 8b 47 10                 	movq	0x10(%rdi), %rax
14031677c: 48 39 07                    	cmpq	%rax, (%rdi)
14031677f: 0f 84 a4 05 00 00           	je	0x140316d29 <.text+0x315d29>
140316785: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
140316789: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
14031678d: 48 ff c0                    	incq	%rax
140316790: 48 89 47 10                 	movq	%rax, 0x10(%rdi)
140316794: 4c 8d 05 2a 4b 6a 02        	leaq	0x26a4b2a(%rip), %r8    # 0x1429bb2c5
14031679b: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
1403167a1: 48 89 f1                    	movq	%rsi, %rcx
1403167a4: e8 57 f9 19 00              	callq	0x1404b6100 <.text+0x4b5100>
1403167a9: 48 85 c0                    	testq	%rax, %rax
1403167ac: 0f 85 e9 04 00 00           	jne	0x140316c9b <.text+0x315c9b>
1403167b2: 48 8b 3e                    	movq	(%rsi), %rdi
1403167b5: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403167b9: 48 39 17                    	cmpq	%rdx, (%rdi)
1403167bc: 0f 84 1f 08 00 00           	je	0x140316fe1 <.text+0x315fe1>
1403167c2: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403167c6: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1403167ca: 48 ff c2                    	incq	%rdx
1403167cd: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1403167d1: 48 8b 3e                    	movq	(%rsi), %rdi
1403167d4: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403167d8: 48 39 17                    	cmpq	%rdx, (%rdi)
1403167db: 0f 84 26 08 00 00           	je	0x140317007 <.text+0x316007>
1403167e1: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403167e5: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1403167e9: 48 ff c2                    	incq	%rdx
1403167ec: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1403167f0: 48 39 17                    	cmpq	%rdx, (%rdi)
1403167f3: 0f 84 34 08 00 00           	je	0x14031702d <.text+0x31602d>
1403167f9: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403167fd: c6 04 10 7d                 	movb	$0x7d, (%rax,%rdx)
140316801: 48 ff c2                    	incq	%rdx
140316804: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140316808: e9 75 03 00 00              	jmp	0x140316b82 <.text+0x315b82>
14031680d: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316811: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316815: 48 8b 1e                    	movq	(%rsi), %rbx
140316818: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031681c: 48 39 03                    	cmpq	%rax, (%rbx)
14031681f: 0f 84 33 05 00 00           	je	0x140316d58 <.text+0x315d58>
140316825: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316829: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
14031682d: 48 ff c0                    	incq	%rax
140316830: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316834: 4c 8d 05 5b 4a 6a 02        	leaq	0x26a4a5b(%rip), %r8    # 0x1429bb296
14031683b: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
140316841: e9 f8 02 00 00              	jmp	0x140316b3e <.text+0x315b3e>
140316846: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
14031684a: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
14031684e: 48 8b 1e                    	movq	(%rsi), %rbx
140316851: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316855: 48 39 03                    	cmpq	%rax, (%rbx)
140316858: 0f 84 29 05 00 00           	je	0x140316d87 <.text+0x315d87>
14031685e: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316862: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316866: 48 ff c0                    	incq	%rax
140316869: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
14031686d: 4c 8d 05 5d 4a 6a 02        	leaq	0x26a4a5d(%rip), %r8    # 0x1429bb2d1
140316874: 41 b9 04 00 00 00           	movl	$0x4, %r9d
14031687a: 48 89 f1                    	movq	%rsi, %rcx
14031687d: e8 7e f8 19 00              	callq	0x1404b6100 <.text+0x4b5100>
140316882: 48 85 c0                    	testq	%rax, %rax
140316885: 0f 85 10 04 00 00           	jne	0x140316c9b <.text+0x315c9b>
14031688b: 48 8b 1e                    	movq	(%rsi), %rbx
14031688e: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316892: 48 39 13                    	cmpq	%rdx, (%rbx)
140316895: 0f 84 b8 07 00 00           	je	0x140317053 <.text+0x316053>
14031689b: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14031689f: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1403168a3: 48 ff c2                    	incq	%rdx
1403168a6: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403168aa: 48 83 c7 10                 	addq	$0x10, %rdi
1403168ae: 48 89 f9                    	movq	%rdi, %rcx
1403168b1: 48 89 f2                    	movq	%rsi, %rdx
1403168b4: e8 97 6d d2 ff              	callq	0x14003d650 <.text+0x3c650>
1403168b9: e9 bf 02 00 00              	jmp	0x140316b7d <.text+0x315b7d>
1403168be: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
1403168c2: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
1403168c6: 48 8b 1e                    	movq	(%rsi), %rbx
1403168c9: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1403168cd: 48 39 03                    	cmpq	%rax, (%rbx)
1403168d0: 0f 84 e0 04 00 00           	je	0x140316db6 <.text+0x315db6>
1403168d6: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1403168da: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1403168de: 48 ff c0                    	incq	%rax
1403168e1: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
1403168e5: 4c 8d 05 c9 49 6a 02        	leaq	0x26a49c9(%rip), %r8    # 0x1429bb2b5
1403168ec: 41 b9 05 00 00 00           	movl	$0x5, %r9d
1403168f2: 48 89 f1                    	movq	%rsi, %rcx
1403168f5: e8 06 f8 19 00              	callq	0x1404b6100 <.text+0x4b5100>
1403168fa: 48 85 c0                    	testq	%rax, %rax
1403168fd: 0f 85 98 03 00 00           	jne	0x140316c9b <.text+0x315c9b>
140316903: 48 8b 1e                    	movq	(%rsi), %rbx
140316906: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031690a: 48 39 13                    	cmpq	%rdx, (%rbx)
14031690d: 0f 84 66 07 00 00           	je	0x140317079 <.text+0x316079>
140316913: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140316917: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
14031691b: 48 ff c2                    	incq	%rdx
14031691e: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140316922: 48 83 c7 10                 	addq	$0x10, %rdi
140316926: 48 89 f9                    	movq	%rdi, %rcx
140316929: 48 89 f2                    	movq	%rsi, %rdx
14031692c: e8 9f 5c d2 ff              	callq	0x14003c5d0 <.text+0x3b5d0>
140316931: e9 47 02 00 00              	jmp	0x140316b7d <.text+0x315b7d>
140316936: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
14031693a: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
14031693e: 48 8b 1e                    	movq	(%rsi), %rbx
140316941: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316945: 48 39 03                    	cmpq	%rax, (%rbx)
140316948: 0f 84 97 04 00 00           	je	0x140316de5 <.text+0x315de5>
14031694e: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316952: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316956: 48 ff c0                    	incq	%rax
140316959: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
14031695d: 4c 8d 05 1a 49 6a 02        	leaq	0x26a491a(%rip), %r8    # 0x1429bb27e
140316964: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
14031696a: e9 cf 01 00 00              	jmp	0x140316b3e <.text+0x315b3e>
14031696f: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316973: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316977: 48 8b 1e                    	movq	(%rsi), %rbx
14031697a: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031697e: 48 39 03                    	cmpq	%rax, (%rbx)
140316981: 0f 84 8d 04 00 00           	je	0x140316e14 <.text+0x315e14>
140316987: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
14031698b: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
14031698f: 48 ff c0                    	incq	%rax
140316992: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316996: 4c 8d 05 ed 48 6a 02        	leaq	0x26a48ed(%rip), %r8    # 0x1429bb28a
14031699d: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
1403169a3: e9 96 01 00 00              	jmp	0x140316b3e <.text+0x315b3e>
1403169a8: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
1403169ac: 48 8b 59 10                 	movq	0x10(%rcx), %rbx
1403169b0: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
1403169b4: 4c 8b 36                    	movq	(%rsi), %r14
1403169b7: 49 8b 46 10                 	movq	0x10(%r14), %rax
1403169bb: 49 39 06                    	cmpq	%rax, (%r14)
1403169be: 0f 84 7f 04 00 00           	je	0x140316e43 <.text+0x315e43>
1403169c4: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
1403169c8: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1403169cc: 48 ff c0                    	incq	%rax
1403169cf: 49 89 46 10                 	movq	%rax, 0x10(%r14)
1403169d3: 4c 8d 05 d3 48 6a 02        	leaq	0x26a48d3(%rip), %r8    # 0x1429bb2ad
1403169da: 41 b9 04 00 00 00           	movl	$0x4, %r9d
1403169e0: 48 89 f1                    	movq	%rsi, %rcx
1403169e3: e8 18 f7 19 00              	callq	0x1404b6100 <.text+0x4b5100>
1403169e8: 48 85 c0                    	testq	%rax, %rax
1403169eb: 0f 85 aa 02 00 00           	jne	0x140316c9b <.text+0x315c9b>
1403169f1: 4c 8b 36                    	movq	(%rsi), %r14
1403169f4: 49 8b 56 10                 	movq	0x10(%r14), %rdx
1403169f8: 49 39 16                    	cmpq	%rdx, (%r14)
1403169fb: 0f 84 9e 06 00 00           	je	0x14031709f <.text+0x31609f>
140316a01: 49 8b 46 08                 	movq	0x8(%r14), %rax
140316a05: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140316a09: 48 ff c2                    	incq	%rdx
140316a0c: 49 89 56 10                 	movq	%rdx, 0x10(%r14)
140316a10: 48 89 f1                    	movq	%rsi, %rcx
140316a13: 48 89 fa                    	movq	%rdi, %rdx
140316a16: 49 89 d8                    	movq	%rbx, %r8
140316a19: e8 12 0a 1a 00              	callq	0x1404b7430 <.text+0x4b6430>
140316a1e: e9 5a 01 00 00              	jmp	0x140316b7d <.text+0x315b7d>
140316a23: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316a27: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316a2b: 48 8b 1e                    	movq	(%rsi), %rbx
140316a2e: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316a32: 48 39 03                    	cmpq	%rax, (%rbx)
140316a35: 0f 84 66 04 00 00           	je	0x140316ea1 <.text+0x315ea1>
140316a3b: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316a3f: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316a43: 48 ff c0                    	incq	%rax
140316a46: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316a4a: 4c 8d 05 29 48 6a 02        	leaq	0x26a4829(%rip), %r8    # 0x1429bb27a
140316a51: 41 b9 04 00 00 00           	movl	$0x4, %r9d
140316a57: e9 e2 00 00 00              	jmp	0x140316b3e <.text+0x315b3e>
140316a5c: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316a60: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316a64: 48 8b 1e                    	movq	(%rsi), %rbx
140316a67: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316a6b: 48 39 03                    	cmpq	%rax, (%rbx)
140316a6e: 0f 84 5c 04 00 00           	je	0x140316ed0 <.text+0x315ed0>
140316a74: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316a78: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316a7c: 48 ff c0                    	incq	%rax
140316a7f: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316a83: 4c 8d 05 18 48 6a 02        	leaq	0x26a4818(%rip), %r8    # 0x1429bb2a2
140316a8a: 41 b9 05 00 00 00           	movl	$0x5, %r9d
140316a90: e9 a9 00 00 00              	jmp	0x140316b3e <.text+0x315b3e>
140316a95: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316a99: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316a9d: 48 8b 1e                    	movq	(%rsi), %rbx
140316aa0: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316aa4: 48 39 03                    	cmpq	%rax, (%rbx)
140316aa7: 0f 84 52 04 00 00           	je	0x140316eff <.text+0x315eff>
140316aad: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316ab1: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316ab5: 48 ff c0                    	incq	%rax
140316ab8: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316abc: 4c 8d 05 12 48 6a 02        	leaq	0x26a4812(%rip), %r8    # 0x1429bb2d5
140316ac3: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
140316ac9: 48 89 f1                    	movq	%rsi, %rcx
140316acc: e8 2f f6 19 00              	callq	0x1404b6100 <.text+0x4b5100>
140316ad1: 48 85 c0                    	testq	%rax, %rax
140316ad4: 0f 85 c1 01 00 00           	jne	0x140316c9b <.text+0x315c9b>
140316ada: 48 8b 1e                    	movq	(%rsi), %rbx
140316add: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316ae1: 48 39 13                    	cmpq	%rdx, (%rbx)
140316ae4: 0f 84 01 06 00 00           	je	0x1403170eb <.text+0x3160eb>
140316aea: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140316aee: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140316af2: 48 ff c2                    	incq	%rdx
140316af5: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140316af9: 48 83 c7 10                 	addq	$0x10, %rdi
140316afd: 48 89 f9                    	movq	%rdi, %rcx
140316b00: 48 89 f2                    	movq	%rsi, %rdx
140316b03: e8 f8 67 d2 ff              	callq	0x14003d300 <.text+0x3c300>
140316b08: eb 73                       	jmp	0x140316b7d <.text+0x315b7d>
140316b0a: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316b0e: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316b12: 48 8b 1e                    	movq	(%rsi), %rbx
140316b15: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316b19: 48 39 03                    	cmpq	%rax, (%rbx)
140316b1c: 0f 84 0c 04 00 00           	je	0x140316f2e <.text+0x315f2e>
140316b22: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316b26: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316b2a: 48 ff c0                    	incq	%rax
140316b2d: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316b31: 4c 8d 05 6f 47 6a 02        	leaq	0x26a476f(%rip), %r8    # 0x1429bb2a7
140316b38: 41 b9 06 00 00 00           	movl	$0x6, %r9d
140316b3e: 48 89 f1                    	movq	%rsi, %rcx
140316b41: e8 ba f5 19 00              	callq	0x1404b6100 <.text+0x4b5100>
140316b46: 48 85 c0                    	testq	%rax, %rax
140316b49: 0f 85 4c 01 00 00           	jne	0x140316c9b <.text+0x315c9b>
140316b4f: 48 8b 1e                    	movq	(%rsi), %rbx
140316b52: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316b56: 48 39 13                    	cmpq	%rdx, (%rbx)
140316b59: 0f 84 75 01 00 00           	je	0x140316cd4 <.text+0x315cd4>
140316b5f: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140316b63: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140316b67: 48 ff c2                    	incq	%rdx
140316b6a: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140316b6e: 48 83 c7 10                 	addq	$0x10, %rdi
140316b72: 48 89 f9                    	movq	%rdi, %rcx
140316b75: 48 89 f2                    	movq	%rsi, %rdx
140316b78: e8 73 2b 17 00              	callq	0x1404896f0 <.text+0x4886f0>
140316b7d: 48 85 c0                    	testq	%rax, %rax
140316b80: 75 21                       	jne	0x140316ba3 <.text+0x315ba3>
140316b82: 48 8b 36                    	movq	(%rsi), %rsi
140316b85: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
140316b89: 48 39 16                    	cmpq	%rdx, (%rsi)
140316b8c: 0f 84 1c 01 00 00           	je	0x140316cae <.text+0x315cae>
140316b92: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140316b96: c6 04 10 7d                 	movb	$0x7d, (%rax,%rdx)
140316b9a: 48 ff c2                    	incq	%rdx
140316b9d: 48 89 56 10                 	movq	%rdx, 0x10(%rsi)
140316ba1: 31 c0                       	xorl	%eax, %eax
140316ba3: 48 83 c4 30                 	addq	$0x30, %rsp
140316ba7: 5b                          	popq	%rbx
140316ba8: 5f                          	popq	%rdi
140316ba9: 5e                          	popq	%rsi
140316baa: 41 5e                       	popq	%r14
140316bac: 41 5f                       	popq	%r15
140316bae: c3                          	retq
140316baf: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316bb3: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316bb7: 48 8b 1e                    	movq	(%rsi), %rbx
140316bba: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316bbe: 48 39 03                    	cmpq	%rax, (%rbx)
140316bc1: 0f 84 96 03 00 00           	je	0x140316f5d <.text+0x315f5d>
140316bc7: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316bcb: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316bcf: 48 ff c0                    	incq	%rax
140316bd2: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316bd6: 4c 8d 05 dd 46 6a 02        	leaq	0x26a46dd(%rip), %r8    # 0x1429bb2ba
140316bdd: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
140316be3: 48 89 f1                    	movq	%rsi, %rcx
140316be6: e8 15 f5 19 00              	callq	0x1404b6100 <.text+0x4b5100>
140316beb: 48 85 c0                    	testq	%rax, %rax
140316bee: 0f 85 a7 00 00 00           	jne	0x140316c9b <.text+0x315c9b>
140316bf4: 48 8b 1e                    	movq	(%rsi), %rbx
140316bf7: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316bfb: 48 39 13                    	cmpq	%rdx, (%rbx)
140316bfe: 0f 84 0d 05 00 00           	je	0x140317111 <.text+0x316111>
140316c04: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140316c08: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140316c0c: 48 ff c2                    	incq	%rdx
140316c0f: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140316c13: 48 83 c7 10                 	addq	$0x10, %rdi
140316c17: 48 89 f9                    	movq	%rdi, %rcx
140316c1a: 48 89 f2                    	movq	%rsi, %rdx
140316c1d: e8 9e db 1d 00              	callq	0x1404f47c0 <.text+0x4f37c0>
140316c22: e9 56 ff ff ff              	jmp	0x140316b7d <.text+0x315b7d>
140316c27: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316c2b: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316c2f: 48 8b 1e                    	movq	(%rsi), %rbx
140316c32: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316c36: 48 39 03                    	cmpq	%rax, (%rbx)
140316c39: 0f 84 4d 03 00 00           	je	0x140316f8c <.text+0x315f8c>
140316c3f: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316c43: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316c47: 48 ff c0                    	incq	%rax
140316c4a: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316c4e: 4c 8d 05 5c 46 6a 02        	leaq	0x26a465c(%rip), %r8    # 0x1429bb2b1
140316c55: 41 b9 04 00 00 00           	movl	$0x4, %r9d
140316c5b: 48 89 f1                    	movq	%rsi, %rcx
140316c5e: e8 9d f4 19 00              	callq	0x1404b6100 <.text+0x4b5100>
140316c63: 48 85 c0                    	testq	%rax, %rax
140316c66: 75 33                       	jne	0x140316c9b <.text+0x315c9b>
140316c68: 48 8b 1e                    	movq	(%rsi), %rbx
140316c6b: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316c6f: 48 39 13                    	cmpq	%rdx, (%rbx)
140316c72: 0f 84 bf 04 00 00           	je	0x140317137 <.text+0x316137>
140316c78: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140316c7c: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140316c80: 48 ff c2                    	incq	%rdx
140316c83: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140316c87: 48 83 c7 10                 	addq	$0x10, %rdi
140316c8b: 48 89 f9                    	movq	%rdi, %rcx
140316c8e: 48 89 f2                    	movq	%rsi, %rdx
140316c91: e8 9a 31 f4 ff              	callq	0x140259e30 <.text+0x258e30>
140316c96: e9 e2 fe ff ff              	jmp	0x140316b7d <.text+0x315b7d>
140316c9b: 48 89 c1                    	movq	%rax, %rcx
140316c9e: 48 83 c4 30                 	addq	$0x30, %rsp
140316ca2: 5b                          	popq	%rbx
140316ca3: 5f                          	popq	%rdi
140316ca4: 5e                          	popq	%rsi
140316ca5: 41 5e                       	popq	%r14
140316ca7: 41 5f                       	popq	%r15
140316ca9: e9 f2 2b 5b 02              	jmp	0x1428c98a0 <.text+0x28c88a0>
140316cae: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316cb7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316cbd: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316cc3: 48 89 f1                    	movq	%rsi, %rcx
140316cc6: e8 15 bd 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316ccb: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
140316ccf: e9 be fe ff ff              	jmp	0x140316b92 <.text+0x315b92>
140316cd4: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316cdd: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316ce3: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316ce9: 48 89 d9                    	movq	%rbx, %rcx
140316cec: e8 ef bc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316cf1: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316cf5: e9 65 fe ff ff              	jmp	0x140316b5f <.text+0x315b5f>
140316cfa: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316d03: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316d09: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316d0f: 48 89 d9                    	movq	%rbx, %rcx
140316d12: 49 89 d6                    	movq	%rdx, %r14
140316d15: 48 89 c2                    	movq	%rax, %rdx
140316d18: e8 c3 bc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316d1d: 4c 89 f2                    	movq	%r14, %rdx
140316d20: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316d24: e9 e8 f9 ff ff              	jmp	0x140316711 <.text+0x315711>
140316d29: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316d32: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316d38: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316d3e: 48 89 f9                    	movq	%rdi, %rcx
140316d41: 48 89 d3                    	movq	%rdx, %rbx
140316d44: 48 89 c2                    	movq	%rax, %rdx
140316d47: e8 94 bc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316d4c: 48 89 da                    	movq	%rbx, %rdx
140316d4f: 48 8b 47 10                 	movq	0x10(%rdi), %rax
140316d53: e9 2d fa ff ff              	jmp	0x140316785 <.text+0x315785>
140316d58: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316d61: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316d67: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316d6d: 48 89 d9                    	movq	%rbx, %rcx
140316d70: 49 89 d6                    	movq	%rdx, %r14
140316d73: 48 89 c2                    	movq	%rax, %rdx
140316d76: e8 65 bc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316d7b: 4c 89 f2                    	movq	%r14, %rdx
140316d7e: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316d82: e9 9e fa ff ff              	jmp	0x140316825 <.text+0x315825>
140316d87: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316d90: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316d96: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316d9c: 48 89 d9                    	movq	%rbx, %rcx
140316d9f: 49 89 d6                    	movq	%rdx, %r14
140316da2: 48 89 c2                    	movq	%rax, %rdx
140316da5: e8 36 bc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316daa: 4c 89 f2                    	movq	%r14, %rdx
140316dad: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316db1: e9 a8 fa ff ff              	jmp	0x14031685e <.text+0x31585e>
140316db6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316dbf: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316dc5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316dcb: 48 89 d9                    	movq	%rbx, %rcx
140316dce: 49 89 d6                    	movq	%rdx, %r14
140316dd1: 48 89 c2                    	movq	%rax, %rdx
140316dd4: e8 07 bc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316dd9: 4c 89 f2                    	movq	%r14, %rdx
140316ddc: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316de0: e9 f1 fa ff ff              	jmp	0x1403168d6 <.text+0x3158d6>
140316de5: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316dee: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316df4: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316dfa: 48 89 d9                    	movq	%rbx, %rcx
140316dfd: 49 89 d6                    	movq	%rdx, %r14
140316e00: 48 89 c2                    	movq	%rax, %rdx
140316e03: e8 d8 bb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316e08: 4c 89 f2                    	movq	%r14, %rdx
140316e0b: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316e0f: e9 3a fb ff ff              	jmp	0x14031694e <.text+0x31594e>
140316e14: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316e1d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316e23: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316e29: 48 89 d9                    	movq	%rbx, %rcx
140316e2c: 49 89 d6                    	movq	%rdx, %r14
140316e2f: 48 89 c2                    	movq	%rax, %rdx
140316e32: e8 a9 bb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316e37: 4c 89 f2                    	movq	%r14, %rdx
140316e3a: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316e3e: e9 44 fb ff ff              	jmp	0x140316987 <.text+0x315987>
140316e43: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316e4c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316e52: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316e58: 4c 89 f1                    	movq	%r14, %rcx
140316e5b: 49 89 d7                    	movq	%rdx, %r15
140316e5e: 48 89 c2                    	movq	%rax, %rdx
140316e61: e8 7a bb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316e66: 4c 89 fa                    	movq	%r15, %rdx
140316e69: 49 8b 46 10                 	movq	0x10(%r14), %rax
140316e6d: e9 52 fb ff ff              	jmp	0x1403169c4 <.text+0x3159c4>
140316e72: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316e7b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316e81: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316e87: 48 89 d9                    	movq	%rbx, %rcx
140316e8a: 49 89 d6                    	movq	%rdx, %r14
140316e8d: 48 89 c2                    	movq	%rax, %rdx
140316e90: e8 4b bb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316e95: 4c 89 f2                    	movq	%r14, %rdx
140316e98: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316e9c: e9 f8 f7 ff ff              	jmp	0x140316699 <.text+0x315699>
140316ea1: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316eaa: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316eb0: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316eb6: 48 89 d9                    	movq	%rbx, %rcx
140316eb9: 49 89 d6                    	movq	%rdx, %r14
140316ebc: 48 89 c2                    	movq	%rax, %rdx
140316ebf: e8 1c bb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316ec4: 4c 89 f2                    	movq	%r14, %rdx
140316ec7: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316ecb: e9 6b fb ff ff              	jmp	0x140316a3b <.text+0x315a3b>
140316ed0: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316ed9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316edf: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316ee5: 48 89 d9                    	movq	%rbx, %rcx
140316ee8: 49 89 d6                    	movq	%rdx, %r14
140316eeb: 48 89 c2                    	movq	%rax, %rdx
140316eee: e8 ed ba 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316ef3: 4c 89 f2                    	movq	%r14, %rdx
140316ef6: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316efa: e9 75 fb ff ff              	jmp	0x140316a74 <.text+0x315a74>
140316eff: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
